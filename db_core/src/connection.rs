use crate::query_builder::QueryBuilder;
use crate::row::Row;
use crate::Result;

pub trait Connection<'a>: Sized {
    type ConnectionParam;
    type QueryResult: QueryResult<'a>;

    fn connect(p: Self::ConnectionParam) -> Result<Self>;
    fn execute(&'a self, builder: QueryBuilder<'a>) -> Result<Self::QueryResult>;
    fn get_existing_schema(&'a self) -> Result<crate::database_updater::DatabaseUpdater<'a>>;
}

pub trait QueryResult<'a> {
    type Row: Row;

    fn len(&mut self) -> Result<usize>;
    fn get_row(&'a self, index: usize) -> Result<Self::Row>;
}
