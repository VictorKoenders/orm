use postgres::{Connection, types::ToSql, rows::Rows};
use super::Result;

pub struct QueryBuilder<'a> {
    connection: &'a Connection,
    pub(crate) query: String,
    pub(crate) params: Vec<&'a ToSql>,
}

impl<'a> QueryBuilder<'a> {
    pub fn execute(self) -> Result<Rows> {
        self.connection.query(&self.query, &self.params).map_err(Into::into)
    }
}
