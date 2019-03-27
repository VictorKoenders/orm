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
    fn get_row(&'a mut self, index: usize) -> Result<Self::Row>;
}


