use super::{QueryBuilder, Table};

pub trait Query<TABLE: Table>: From<QueryBuilder> {}
