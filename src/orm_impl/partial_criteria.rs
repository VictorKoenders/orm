use super::{Column, Query, QueryBuilder, Table};
use postgres::types::ToSql;
use std::marker::PhantomData;

pub struct PartialCriteria<'a, TABLE: Table, QUERY: Query<TABLE>, COLUMN: Column> {
    builder: QueryBuilder<'a>,
    pd_table: PhantomData<TABLE>,
    pd_query: PhantomData<QUERY>,
    pd_column: PhantomData<COLUMN>,
}

impl<'a, TABLE, QUERY, COLUMN> PartialCriteria<'a, TABLE, QUERY, COLUMN>
where
    TABLE: Table,
    QUERY: Query<TABLE>,
    COLUMN: Column,
{
    pub fn new(builder: QueryBuilder<'a>) -> Self {
        Self {
            builder,

            pd_table: PhantomData,
            pd_query: PhantomData,
            pd_column: PhantomData,
        }
    }

    pub fn eq(mut self, value: &'a COLUMN::Type) -> QUERY
    where
        COLUMN::Type: ToSql,
    {
        self.builder.add_criteria::<COLUMN>(value);
        self.builder.into()
    }
}
