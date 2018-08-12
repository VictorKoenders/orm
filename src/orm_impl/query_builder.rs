use postgres::{Connection, types::ToSql};
use super::{Result, Column, Table};

pub struct QueryBuilder<'a> {
    connection: &'a Connection,
    table: String,
    criteria: Vec<&'static str>,
    params: Vec<&'a ToSql>,
}

impl<'a> QueryBuilder<'a> {
    pub(crate) fn new(connection: &'a Connection) -> Self {
        Self {
            connection,
            table: String::new(),
            criteria: Vec::new(),
            params: Vec::new(),
        }
    }
    pub fn execute<TABLE: Table>(self) -> Result<Vec<TABLE>> {
        let mut query = format!("SELECT * FROM {}", self.table);
        for (index, criteria) in self.criteria.into_iter().enumerate() {
            query += if index == 0 { " WHERE " } else { " AND " };
            query += criteria;
            query += " = $";
            query += &(index + 1).to_string();
        }
        let rows = self.connection.query(&query, &self.params)?;
        let mut result = Vec::with_capacity(rows.len());
        for row in rows.iter() {
            result.push(TABLE::load_from_reader(&row)?);
        }
        Ok(result)
    }
    pub fn add_criteria<COLUMN: Column>(&mut self, value: &'a COLUMN::Type)
        where COLUMN::Type : ToSql {
        self.criteria.push(COLUMN::name());
        self.params.push(value);
    }
}
