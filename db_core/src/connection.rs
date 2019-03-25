use crate::{QueryBuilder, Result, Row};

pub trait Connection<'a>: Sized {
    type ConnectionParam;
    type Row: Row;

    fn connect(p: Self::ConnectionParam) -> Result<Self>;
    fn execute(&'a self, builder: QueryBuilder<'a>) -> Result<Vec<Self::Row>>;
}
