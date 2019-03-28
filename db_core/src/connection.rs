use crate::{QueryBuilder, Result, Row};

pub trait Connection<'a>: Sized {
    type ConnectionParam;
    type QueryResult: QueryResult<'a>;

    fn connect(p: Self::ConnectionParam) -> Result<Self>;
    fn execute(&'a self, builder: QueryBuilder<'a>) -> Result<Self::QueryResult>;
}

pub trait QueryResult<'a> {
    type Row: Row;

    fn len(&mut self) -> Result<usize>;
    fn is_empty(&mut self) -> Result<bool> {
        self.len().map(|l| l == 0)
    }
    fn get_row(&'a mut self, index: usize) -> Result<Self::Row>;
}
