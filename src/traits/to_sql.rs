use std::marker::PhantomData;

pub trait ToSql: postgres::types::ToSql {
    fn as_pg_arg(&self) -> &postgres::types::ToSql
    where
        Self: Sized,
    {
        self
    }
}

impl ToSql for i32 {}
impl ToSql for String {}
impl<'a> ToSql for &'a str {}
