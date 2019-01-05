use crate::Eq;
use postgres::types::ToSql;

pub trait AsQueryFilter {
    fn as_query_filter(&self, index: usize) -> Option<(String, &ToSql)>;
}

impl AsQueryFilter for () {
    fn as_query_filter(&self, _index: usize) -> Option<(String, &ToSql)> {
        None
    }
}

impl<T: ToSql + 'static> AsQueryFilter for Eq<T> {
    fn as_query_filter(&self, index: usize) -> Option<(String, &ToSql)> {
        Some((format!("= ${}", index + 1), &self.val))
    }
}
