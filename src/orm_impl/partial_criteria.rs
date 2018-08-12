use super::{Column, Query, QueryBuilder, Table};
use postgres::types::ToSql;
use std::marker::PhantomData;

/// A partial criteria. This is created when you call `DbSet.query().field()` before comparing it with a value
/// 
/// The generics represent the following values:
/// - TABLE: The table being loaded
/// - QUERY: The table query object, generated with `#[derive(Table)]`
/// - COLUMN: The column we're querying, an instance of [Column](trait.Column.html)
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
    #[doc(hidden)]
    pub fn new(builder: QueryBuilder) -> Self {
        Self {
            builder,

            pd_table: PhantomData,
            pd_query: PhantomData,
            pd_column: PhantomData,
        }
    }

    /// Only queries the entries in the database that match this exact value
    pub fn eq(mut self, value: COLUMN::Type) -> QUERY
    where
        COLUMN::Type: ToSql + 'static,
    {
        self.builder
            .add_criteria::<COLUMN>(TABLE::table_name(), value);
        self.builder.into()
    }
}
