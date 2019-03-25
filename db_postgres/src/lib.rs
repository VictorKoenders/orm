use db_core::failure::bail;
pub use db_core::{Connection as ConnectionTrait, EstimateStrLen, Result};
use pq_sys::*;
use std::ffi::{CStr, CString};

pub struct Connection(*mut PGconn);

const TABLE_COLUMN_PREFIX: &str = "\"";
const TABLE_COLUMN_POSTFIX: &str = "\"";
const TABLE_COLUMN_JOIN: &str = ".";
const COLUMN_ALIAS: &str = " AS ";

impl<'a> db_core::Connection<'a> for Connection {
    type ConnectionParam = &'a str;
    type Row = Row<'a>;

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

    fn execute(&self, builder: db_core::QueryBuilder<'a>) -> Result<Vec<Row<'a>>> {
        let query = build_query(&builder);
        let _params = get_query_parameters(&builder);

        println!("Query: {}", query);

        unimplemented!()
    }
}

fn get_query_parameters<'a, 'b>(
    builder: &'a db_core::QueryBuilder<'b>,
) -> Vec<&'a Box<db_core::Argument<'b>>> {
    let mut result = Vec::with_capacity(builder.criteria.len());
    for criteria in &builder.criteria {
        if let db_core::FieldOrArgument::Argument(a) = &criteria.right {
            result.push(a);
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
        if index == 0 { query += " WHERE "; }
        else { query += " AND "; }
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

pub struct Row<'a> {
    _result: &'a PGresult,
}

impl<'a> db_core::Row for Row<'a> {
    fn read_string_at_index(&mut self, _index: usize) -> Result<String> {
        unimplemented!()
    }
    fn read_string_by_name(&mut self, _name: &str) -> Result<String> {
        unimplemented!()
    }
}
