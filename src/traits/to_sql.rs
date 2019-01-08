use std::marker::PhantomData;

cfg_if! {
    if #[cfg(feature = "pg")] {
        pub trait ToSql : postgres::types::ToSql {
            fn as_pg_arg(&self) -> &postgres::types::ToSql
                where Self: Sized{
                self
            }
        }
    } else if #[cfg(feature = "sqlite")] {
        pub trait ToSql : sqlite::Bindable {
            fn as_bindable(&self) -> &sqlite {
                self
            }
        }
    }
}

impl<T: ToSql> ToSql for Option<T> {}

impl ToSql for i8 {}
impl ToSql for i16 {}
impl ToSql for i32 {}

impl ToSql for String {}
impl<'a> ToSql for &'a str {}
