use super::{Table, Result};

pub trait Queryable<TABLE: Table> {
    fn execute(&self) -> Result<Vec<TABLE>>;
}
