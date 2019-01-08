use crate::{Connection, Result, TableDefinition, ToSql};

impl<'a> Connection for postgres::transaction::Transaction<'a> {
    type QueryResult = postgres::rows::Rows;

    fn query(&self, str: &str, args: &[&ToSql]) -> Result<Self::QueryResult> {
        let args = args.iter().map(|a| a.as_pg_arg()).collect::<Vec<_>>();
        let args: Vec<&postgres::types::ToSql> =
            args.iter().map(|a| a.as_ref()).collect::<Vec<_>>();
        self.query(str, &args).map_err(Into::into)
    }

    fn update_table_by_definition(&self, definition: &TableDefinition) -> Result<()> {
        println!("Updating table {:#?}", definition);
        unimplemented!();
    }
}
