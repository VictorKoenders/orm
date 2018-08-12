use super::{Column, Query, QueryBuilder, Table};
use postgres::types::ToSql;
use std::marker::PhantomData;

pub struct PartialCriteria<TABLE: Table, QUERY: Query<TABLE>, COLUMN: Column> {
    builder: QueryBuilder,
    pd_table: PhantomData<TABLE>,
    pd_query: PhantomData<QUERY>,
    pd_column: PhantomData<COLUMN>,
}

impl<TABLE, QUERY, COLUMN> PartialCriteria<TABLE, QUERY, COLUMN>
where
    TABLE: Table,
    QUERY: Query<TABLE>,
    COLUMN: Column,
{
    pub fn new(builder: QueryBuilder) -> Self {
        Self {
            builder,

            pd_table: PhantomData,
            pd_query: PhantomData,
            pd_column: PhantomData,
        }
    }

    pub fn eq(mut self, value: COLUMN::Type) -> QUERY
    where
        COLUMN::Type: ToSql + 'static,
    {
        self.builder
            .add_criteria::<COLUMN>(TABLE::table_name(), value);
        self.builder.into()
    }
}
