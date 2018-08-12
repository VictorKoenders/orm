use super::{Result, Table};

pub trait Queryable<TABLE: Table> {
    fn execute(&self) -> Result<Vec<TABLE>>;
}
