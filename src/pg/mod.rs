use crate::{Connection, Result, TableDefinition, ToSql};

impl<'a> Connection for postgres::transaction::Transaction<'a> {
    type QueryResult = postgres::rows::Rows;

    fn query(&self, str: &str, args: &[&ToSql]) -> Result<Self::QueryResult> {
        self.query(str, &[]).map_err(Into::into)
    }

    fn update_table_by_definition(&self, definition: &TableDefinition) -> Result<()> {
        println!("Updating table {:#?}", definition);
        unimplemented!();
    }
}
