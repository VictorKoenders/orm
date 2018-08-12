use std::marker::PhantomData;
use super::{Table, Queryable, Query, Column, Criteria};

pub struct PartialCriteria<TABLE: Table, QUERY: Query<TABLE>, COLUMN: Column> {
    inner: Box<Queryable<TABLE>>,
    pd_table: PhantomData<TABLE>,
    pd_query: PhantomData<QUERY>,
    pd_column: PhantomData<COLUMN>,
}

impl<TABLE: Table, QUERY: Query<TABLE>, COLUMN: Column> PartialCriteria<TABLE, QUERY, COLUMN> {
    pub fn new(inner: Box<Queryable<TABLE>>) -> Self {
        Self {
            inner,

            pd_table: PhantomData,
            pd_query: PhantomData,
            pd_column: PhantomData,
        }
    }

    pub fn eq(self, value: COLUMN::Type) -> Criteria<TABLE, QUERY, COLUMN> {
        Criteria::new(self.inner, value)
    }

    pub fn gt(self, value: COLUMN::Type) -> Criteria<TABLE, QUERY, COLUMN> {
        Criteria::new(self.inner, value)
    }
}
