use std::marker::PhantomData;
use super::{Table, Query, Column, Queryable, Result};

pub struct Criteria<TABLE: Table, QUERY: Query<TABLE>, COLUMN: Column> {
    inner: Box<Queryable<TABLE>>,
    value: COLUMN::Type,

    pd_table: PhantomData<TABLE>,
    pd_query: PhantomData<QUERY>,
    pd_column: PhantomData<COLUMN>,
}

impl<TABLE: Table, QUERY: Query<TABLE>, COLUMN: Column> Criteria<TABLE, QUERY, COLUMN> {
    pub fn new(inner: Box<Queryable<TABLE>>, value: COLUMN::Type) -> Self {
        Criteria {
            inner,
            value,

            pd_table: PhantomData,
            pd_query: PhantomData,
            pd_column: PhantomData,
        }
    }
}

impl<TABLE: Table, QUERY: Query<TABLE>, COLUMN: Column> Queryable<TABLE> for Criteria<TABLE, QUERY, COLUMN> 
    where COLUMN::Type : ::std::fmt::Display {
    fn generate_query(&self, out: &mut String) -> Result<()> {
        *out += &format!(" AND {} = {}", COLUMN::name(), self.value);
        self.inner.generate_query(out)?;

        Ok(())
    }
}
