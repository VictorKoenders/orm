use crate::query_builder::QueryBuilder;
use crate::row::Row;
use crate::Result;

pub trait Connection<'a>: Sized {
    type ConnectionParam;
    type QueryResult: QueryResult<'a>;

    fn connect(p: Self::ConnectionParam) -> Result<Self>;
    fn execute(&self, builder: QueryBuilder<'a>) -> Result<Self::QueryResult>;
    fn get_existing_schema(&self) -> Result<crate::database_updater::DatabaseUpdater<'a>>;
    fn update_schema(&self, updates: &[crate::database_updater::DatabaseUpdaterChange]) -> Result<()>;
}

pub trait QueryResult<'a> {
    type Row: Row;

    fn len(&mut self) -> Result<usize>;
    fn is_empty(&mut self) -> Result<bool> {
        self.len().map(|l| l == 0)
    }

    fn get_row(&'a self, index: usize) -> Result<Self::Row>;
}
