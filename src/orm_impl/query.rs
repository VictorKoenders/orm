use super::{QueryBuilder, Table, Result};

/// The base type of a query, used to fill a QueryBuilder with strongly typed columns and comparisons
pub trait Query<TABLE: Table>: From<QueryBuilder> {
    /// Execute this query, internally calling [QueryBuilder::execute](struct.QueryBuilder.html#method.execute).
    fn execute(self) -> Result<Vec<TABLE>>;
}
