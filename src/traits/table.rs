use crate::{QueryBuilder, Result, TableUpdater};

pub trait Table: Sized {
    type QueryBuilder: QueryBuilder;
    fn table_name() -> &'static str;
    fn update_database_schema(updater: &mut TableUpdater) -> Result<()>;
    fn from_reader(row: &postgres::rows::Row) -> Result<Self>;
}
