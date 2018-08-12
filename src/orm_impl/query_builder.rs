use super::{Column, Result, Table};
use postgres::{types::ToSql, Connection};
use std::rc::Rc;

/// A querybuilder, used internally by the [Query](trait.Query.html) object to generate an actual query.
pub struct QueryBuilder {
    connection: Rc<Connection>,
    tables: Vec<&'static str>,
    criteria: Vec<(&'static str, &'static str)>,
    params: Vec<Box<ToSql>>,
}

impl QueryBuilder {
    pub(crate) fn new(connection: Rc<Connection>) -> Self {
        Self {
            connection,
            tables: Vec::new(),
            criteria: Vec::new(),
            params: Vec::new(),
        }
    }
    /// Execute this querybuilder and return the resulting data. The database rows are passed to Table's [load_from_reader](trait.Table.html#tymethod.load_from_reader) method to create an instance.
    pub fn execute<TABLE: Table>(self) -> Result<Vec<TABLE>> {
        let mut query = format!("SELECT * FROM {}", self.tables[0]);
        for (index, (table, criteria)) in self.criteria.into_iter().enumerate() {
            query += if index == 0 { " WHERE " } else { " AND " };
            query += table;
            query += ".";
            query += criteria;
            query += " = $";
            query += &(index + 1).to_string();
        }
        let params = self
            .params
            .iter()
            .map(|p| p.as_ref())
            .collect::<Vec<&ToSql>>();
        println!("{:?} {:?}", query, params);
        let rows = self.connection.query(&query, &params)?;
        let mut result = Vec::with_capacity(rows.len());
        for row in rows.iter() {
            result.push(TABLE::load_from_reader(&row)?);
        }
        Ok(result)
    }

    /// Add a criteria to this query
    pub fn add_criteria<COLUMN: Column>(&mut self, table: &'static str, value: COLUMN::Type)
    where
        COLUMN::Type: ToSql + 'static,
    {
        if let None = self.tables.iter().find(|t| t == &&table) {
            self.tables.push(table);
        }
        self.criteria.push((table, COLUMN::name()));
        self.params.push(Box::new(value));
    }
}
