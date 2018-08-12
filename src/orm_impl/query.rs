use super::{Table, QueryBuilder};

pub trait Query<TABLE: Table> : for<'a> From<QueryBuilder<'a>> {
}
