pub use db_core::connection::Connection as ConnectionTrait;
pub use db_core::database_updater::DatabaseUpdater;
use db_core::failure::{bail, format_err};
pub use db_core::query_builder::EstimateStrLen;
pub use db_core::row::ReadType;
pub use db_core::table_builder::{Column, ColumnDefault, ColumnFlags, ColumnType, Table};
pub use db_core::Result;
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

impl Connection {
    fn execute_query(
        &self,
        query: &str,
        params: &[&db_core::query_builder::Argument],
    ) -> Result<QueryResult<'static>> {
        let params = params
            .into_iter()
            .map(|p| {
                let str = p.to_query_string();
                CString::new(str).map_err(Into::into)
            })
            .collect::<Result<Vec<CString>>>()?;

        println!("Query: \"{}\"", query);
        println!("Params: {:?}", params);

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
                1,                              // resultFormat: c_int (0 = plain text, 1 = binary)
            )
        };

        match NonNull::new(result) {
            Some(ptr) => {
                let result = QueryResult::new(ptr);
                let error_message = result.error_message();
                if !error_message.is_empty() {
                    Err(format_err!("Could not execute query: {}", error_message))
                } else {
                    Ok(result)
                }
            }
            None => Err(format_err!("Could not execute query: {}", unsafe {
                let error_ptr = PQerrorMessage(self.0);
                let bytes = CStr::from_ptr(error_ptr).to_bytes();
                std::str::from_utf8_unchecked(bytes)
            })),
        }
    }
}

impl<'a> ConnectionTrait<'a> for Connection {
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

    fn execute(
        &self,
        builder: db_core::query_builder::QueryBuilder<'a>,
    ) -> Result<QueryResult<'a>> {
        let (query, params) = build_query(&builder);
        self.execute_query(&query, &params)
    }

    fn get_existing_schema(&self) -> Result<DatabaseUpdater<'a>> {
        use db_core::connection::QueryResult;
        use db_core::row::Row;
        const COLUMN_QUERY: &str = r#"SELECT
    table_name,
    column_name,
    column_default,
    CAST(CASE WHEN is_nullable = 'YES' THEN 1 ELSE 0 END as bool) AS is_nullable,
    udt_name
FROM information_schema.columns
WHERE table_schema = $1
ORDER BY table_name"#;

        let mut result = self.execute_query(COLUMN_QUERY, &[&"public"])?;
        let len = result.len()?;
        println!("Found {} columns", result.len()?);
        let mut tables = Vec::new();
        let mut current_table: Option<Table> = None;
        for i in 0..len {
            let row = result.get_row(i)?;

            let table_name: String = row.read_at_index(0)?;
            let table: &mut Table = {
                let table_name_equals =
                    current_table.as_ref().map(|t| t.name.as_ref()) == Some(&table_name);

                // If the table name doesn't match, but we have a table, clear the current table
                // and add it to the `tables` vec
                if current_table.is_some() && !table_name_equals {
                    // we checked that current_table.is_some(), so this unwrap is safe
                    let table = current_table.take().unwrap();
                    tables.push(table);
                }
                current_table.get_or_insert_with(|| Table::with_name(table_name))
            };

            let column_name: &str = row.read_at_index(1)?;
            let column_default: Option<&str> = row.read_at_index(2)?;
            let column_nullable: bool = row.read_at_index(3)?;
            let column_type: &str = row.read_at_index(4)?;

            let mut column = Column::with_name(column_name.to_owned());
            if let Some(default) = column_default {
                column.default = Some(ColumnDefault::Custom(default.to_owned().into()));
            }
            column.r#type = type_string_to_column_type(column_type.to_owned());
            if !column_nullable {
                column.flags |= ColumnFlags::NOT_NULL;
            }

            table.columns.push(column);
        }
        if let Some(table) = current_table.take() {
            tables.push(table);
        }

        Ok(DatabaseUpdater::from_old_definition(tables))
    }

    fn update_schema(&self, _updates: &[db_core::database_updater::DatabaseUpdaterChange]) -> Result<()>{
        Ok(())
    }
}

fn type_string_to_column_type(s: String) -> ColumnType<'static> {
    match s.as_ref() {
        "text" | "name" => ColumnType::Text(None),
        "int4" => ColumnType::Int,
        "timestamptz" => ColumnType::Custom("TIMESTAMPTZ".into()),
        "uuid" => ColumnType::Custom("UUID".into()),
        _ => {
            println!("Unknown type: {:?}", s);
            ColumnType::Custom(s.into())
        }
    }
}

fn append_field_to_query(query: &mut String, field: &db_core::query_builder::Field) {
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

fn build_query<'a, 'b>(builder: &'a db_core::query_builder::QueryBuilder<'b>) -> (String, Vec<&'a db_core::query_builder::Argument<'b>>) {
    let mut query = String::with_capacity(builder.estimate_str_len() * 2);
    let mut criteria_arguments = Vec::with_capacity(builder.criteria.len());

    let table = &builder.table;
    let joined_tables = &builder.joined_tables;
    let select = &builder.select;
    let criteria = &builder.criteria;

    query += "SELECT ";
    if select.is_empty() {
        query += "*";
    } else {
        for (index, select) in select.iter().enumerate() {
            if index != 0 {
                query += ", ";
            }
            append_field_to_query(&mut query, select);
        }
    }

    query += " FROM ";
    query += TABLE_COLUMN_PREFIX;
    query += table;
    query += TABLE_COLUMN_POSTFIX;

    for joined_table in joined_tables {
        query += " LEFT JOIN ";
        query += TABLE_COLUMN_PREFIX;
        query += &joined_table.joined_table;
        query += TABLE_COLUMN_POSTFIX;

        for (index, criteria) in joined_table.criteria.iter().enumerate() {
            if index == 0 { query += " ON "; }
            else { query += " AND "; }
            append_field_to_query(&mut query, &criteria.left);
            query += criteria.comparison.as_query_str();

            match &criteria.right {
                db_core::query_builder::FieldOrArgument::Field(f) => {
                    append_field_to_query(&mut query, f)
                }
                db_core::query_builder::FieldOrArgument::Argument(a) => {
                    criteria_arguments.push(a.as_ref());

                    query += "$";
                    query += &(criteria_arguments.len().to_string());
                    query += " ";
                }
            }
        }
    }

    for (index, criteria) in criteria.iter().enumerate() {
        if index == 0 {
            query += " WHERE ";
        } else {
            query += " AND ";
        }
        append_field_to_query(&mut query, &criteria.left);
        query += criteria.comparison.as_query_str();

        match &criteria.right {
            db_core::query_builder::FieldOrArgument::Field(f) => {
                append_field_to_query(&mut query, f)
            }
            db_core::query_builder::FieldOrArgument::Argument(a) => {
                criteria_arguments.push(a.as_ref());

                query += "$";
                query += &(criteria_arguments.len().to_string());
            }
        }
    }

    (query, criteria_arguments)
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { PQfinish(self.0) }
    }
}

pub struct QueryResult<'a> {
    ptr: NonNull<PGresult>,
    row_count: usize,
    column_count: usize,
    _pd: PhantomData<&'a ()>,
}

impl QueryResult<'_> {
    fn new(ptr: NonNull<PGresult>) -> QueryResult<'static> {
        QueryResult {
            ptr,
            row_count: unsafe { PQntuples(ptr.as_ptr()) } as usize,
            column_count: unsafe { PQnfields(ptr.as_ptr()) } as usize,
            _pd: PhantomData,
        }
    }

    pub fn error_message(&self) -> &str {
        let ptr = unsafe { PQresultErrorMessage(self.ptr.as_ptr()) };
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_str().unwrap_or_default()
    }
}

impl<'a> db_core::connection::QueryResult<'a> for QueryResult<'a> {
    type Row = Row<'a>;

    fn len(&mut self) -> Result<usize> {
        Ok(self.row_count)
    }

    fn get_row(&'a self, index: usize) -> Result<Row<'a>> {
        Ok(Row {
            result: unsafe { self.ptr.as_ref() },
            row_index: index,
            column_count: self.column_count,
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
    column_count: usize,
}

impl Row<'_> {
    pub fn len(&self) -> usize {
        self.column_count
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn name_of_index(&self, index: usize) -> Result<&str> {
        let cstr = unsafe { std::ffi::CStr::from_ptr(PQfname(self.result, index as i32)) };
        cstr.to_str().map_err(Into::into)
    }
}

impl db_core::row::Row for Row<'_> {
    fn read_at_index<'a, T: ReadType<'a>>(&self, index: usize) -> Result<T> {
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
    fn read_by_name<'a, T: ReadType<'a>>(&self, name: &str) -> Result<T> {
        let cstr = CString::new(name).unwrap_or_default();
        let fnum = unsafe { PQfnumber(self.result, cstr.as_ptr()) };
        match fnum {
            -1 => Err(format_err!("Field {:?} not found", name)),
            x => self.read_at_index(x as usize),
        }
    }
}
