use crate::{QueryBuilder, Result, TableUpdater, Connection};

pub trait Table: Sized {
    type QueryBuilder: QueryBuilder;
    fn table_name() -> &'static str;
    fn update_database_schema<T: Connection>(updater: &mut TableUpdater<T>) -> Result<()>;
    fn from_reader(row: &postgres::rows::Row) -> Result<Self>;
}
