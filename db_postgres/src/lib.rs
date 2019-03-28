use db_core::failure::{bail, format_err};
pub use db_core::{Connection as ConnectionTrait, EstimateStrLen, ReadType, Result};
use pq_sys::*;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw;
use std::ptr::NonNull;

pub struct Connection(*mut PGconn);

const TABLE_COLUMN_PREFIX: &str = "\"";
const TABLE_COLUMN_POSTFIX: &str = "\"";
const TABLE_COLUMN_JOIN: &str = ".";
const COLUMN_ALIAS: &str = " AS ";

impl<'a> db_core::Connection<'a> for Connection {
    type ConnectionParam = &'a str;
    type QueryResult = QueryResult<'a>;

    fn connect(database_url: &'a str) -> Result<Connection> {
        let connection_string = CString::new(database_url)?;
        let connection_ptr = unsafe { PQconnectdb(connection_string.as_ptr()) };
        let connection_status = unsafe { PQstatus(connection_ptr) };

        if connection_status != ConnStatusType::CONNECTION_OK {
            bail!("{}", unsafe {
                let error_ptr = PQerrorMessage(connection_ptr);
                let bytes = CStr::from_ptr(error_ptr).to_bytes();
                std::str::from_utf8_unchecked(bytes)
            });
        }

        Ok(Connection(connection_ptr))
    }

    fn execute(&self, builder: db_core::QueryBuilder<'a>) -> Result<QueryResult<'a>> {
        let query = build_query(&builder);
        let params = get_query_parameters(&builder);
        let params = params
            .into_iter()
            .map(|p| {
                let str = p.to_query_string();
                CString::new(str).map_err(Into::into)
            })
            .collect::<Result<Vec<CString>>>()?;

        println!("Query: {}", query);
        println!("Parameters: {:?}", params);

        let param_ptrs = params
            .iter()
            .map(|c| c.as_ptr())
            .collect::<Vec<*const raw::c_char>>();
        let query = CString::new(query)?;

        let result = unsafe {
            PQexecParams(
                self.0,                         // conn: *mut PGconn
                query.as_ptr(),                 // command: *const c_char
                param_ptrs.len() as raw::c_int, // nParams: c_int
                std::ptr::null(),               // paramTypes: *const Oid
                param_ptrs.as_ptr(),            // paramValues: *const *const c_char
                std::ptr::null(),               // paramLengths: *const c_int
                std::ptr::null(),               // paramFormats: *const c_int
                0,                              // resultFormat: c_int (0 = plain text, 1 = binary)
            )
        };

        match NonNull::new(result) {
            Some(ptr) => Ok(QueryResult::new(ptr)),
            None => Err(format_err!("Could not execute query: {}", unsafe {
                let error_ptr = PQerrorMessage(self.0);
                let bytes = CStr::from_ptr(error_ptr).to_bytes();
                std::str::from_utf8_unchecked(bytes)
            })),
        }
    }
}

fn get_query_parameters<'a, 'b>(
    builder: &'a db_core::QueryBuilder<'b>,
) -> Vec<&'a db_core::Argument<'b>> {
    let mut result = Vec::with_capacity(builder.criteria.len());
    for criteria in &builder.criteria {
        if let db_core::FieldOrArgument::Argument(a) = &criteria.right {
            result.push(a.as_ref());
        }
    }
    result
}

fn append_field_to_query(query: &mut String, field: &db_core::Field) {
    if let Some(table) = &field.table {
        *query += TABLE_COLUMN_PREFIX;
        *query += &table;
        *query += TABLE_COLUMN_POSTFIX;
        *query += TABLE_COLUMN_JOIN;
    }
    *query += TABLE_COLUMN_PREFIX;
    *query += &field.field;
    *query += TABLE_COLUMN_POSTFIX;

    if let Some(alias) = &field.alias {
        *query += COLUMN_ALIAS;
        *query += &alias;
    }
}

fn build_query(builder: &db_core::QueryBuilder) -> String {
    let mut query = String::with_capacity(builder.estimate_str_len() * 2);
    query += "SELECT ";
    if builder.select.is_empty() {
        query += "*";
    } else {
        for (index, select) in builder.select.iter().enumerate() {
            if index != 0 {
                query += ",";
            }
            append_field_to_query(&mut query, select);
        }
    }

    query += " FROM ";
    query += TABLE_COLUMN_PREFIX;
    query += &builder.table;
    query += TABLE_COLUMN_POSTFIX;

    let mut argument_index = 1;

    for (index, criteria) in builder.criteria.iter().enumerate() {
        if index == 0 {
            query += " WHERE ";
        } else {
            query += " AND ";
        }
        append_field_to_query(&mut query, &criteria.left);
        query += criteria.comparison.as_query_str();

        match &criteria.right {
            db_core::FieldOrArgument::Field(f) => append_field_to_query(&mut query, f),
            db_core::FieldOrArgument::Argument(_) => {
                query += "$";
                query += &argument_index.to_string();
                argument_index += 1;
            }
        }
    }

    query
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { PQfinish(self.0) }
    }
}

pub struct QueryResult<'a> {
    ptr: NonNull<PGresult>,
    len: usize,
    _pd: PhantomData<&'a ()>,
}

impl QueryResult<'_> {
    fn new(ptr: NonNull<PGresult>) -> QueryResult<'static> {
        QueryResult {
            ptr,
            len: unsafe { PQntuples(ptr.as_ptr()) } as usize,
            _pd: PhantomData,
        }
    }

    pub fn error_message(&self) -> &str {
        let ptr = unsafe { PQresultErrorMessage(self.ptr.as_ptr()) };
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_str().unwrap_or_default()
    }
}

impl<'a> db_core::QueryResult<'a> for QueryResult<'a> {
    type Row = Row<'a>;

    fn len(&mut self) -> Result<usize> {
        Ok(self.len)
    }

    fn get_row(&mut self, index: usize) -> Result<Row> {
        Ok(Row {
            result: unsafe { self.ptr.as_ref() },
            row_index: index,
        })
    }
}

impl Drop for QueryResult<'_> {
    fn drop(&mut self) {
        unsafe { PQclear(self.ptr.as_ptr()) }
    }
}

pub struct Row<'a> {
    result: &'a PGresult,
    row_index: usize,
}

impl<'a> db_core::Row for Row<'a> {
    fn read_at_index<T: ReadType>(&mut self, index: usize) -> Result<T> {
        let row_index = self.row_index as i32;
        let index = index as i32;
        if unsafe { PQgetisnull(self.result, row_index, index) } != 0 {
            T::from_pq_bytes(None)
        } else {
            let slice = unsafe {
                let ptr = PQgetvalue(self.result, row_index, index) as *const u8;
                let len = PQgetlength(self.result, row_index, index);
                std::slice::from_raw_parts(ptr, len as usize)
            };
            T::from_pq_bytes(Some(slice))
        }
    }
    fn read_by_name<T: ReadType>(&mut self, name: &str) -> Result<T> {
        let cstr = CString::new(name).unwrap_or_default();
        let fnum = unsafe { PQfnumber(self.result, cstr.as_ptr()) };
        match fnum {
            -1 => Err(format_err!("Field {:?} not found", name)),
            x => self.read_at_index(x as usize),
        }
    }
}
