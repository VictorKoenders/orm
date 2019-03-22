use crate::{QueryBuilder, Result, Row};

pub trait Connection: Sized {
    type ConnectionParam;
    type Row: Row;

    fn connect(p: Self::ConnectionParam) -> Result<Self>;
    fn execute(&self, builder: QueryBuilder) -> Result<Vec<Self::Row>>;
}
