use super::{Column, Query, Result};
use postgres::rows::Row;

pub trait Table : Sized {
    type ID: Column;
    type QUERY: Query<Self>;

    fn table_name() -> &'static str;

    fn load_from_reader(row: &Row) -> Result<Self>;

    fn id(&self) -> &<<Self as Table>::ID as Column>::Type;
}
