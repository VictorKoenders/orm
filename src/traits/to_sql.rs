use std::marker::PhantomData;

pub trait ToSql {
    #[cfg(feature = "pg")]
    fn to_pg_sql(self) -> Box<postgres::types::ToSql>;
    #[cfg(feature = "sqlite")]
    fn to_sqlite_sql(self) -> Box<rusqlite::types::ToSql>;
}

macro_rules! impl_to_sql {
    ($ty:ty) => {
        impl_to_sql!($ty, (pg), (sqlite));
    };
    ($ty:ty, (pg $($pg_ty:tt)*), (sqlite $($sqlite_ty:tt)*)) => {
        impl ToSql for $ty {
            #[cfg(feature = "pg")]
            fn to_pg_sql(self) -> Box<postgres::types::ToSql> {
                Box::new(self $($pg_ty)*)
            }
            #[cfg(feature = "sqlite")]
            fn to_sqlite_sql(self) -> Box<rusqlite::types::ToSql> {
                Box::new(self $($sqlite_ty)*)
            }
        }
    }
}

impl_to_sql!(u8, (pg as i16), (sqlite));
impl_to_sql!(u16, (pg as i16), (sqlite));
impl_to_sql!(u32, (pg as i32), (sqlite));
impl_to_sql!(u64, (pg as i64), (sqlite as i64));

impl_to_sql!(i8, (pg), (sqlite));
impl_to_sql!(i16, (pg), (sqlite));
impl_to_sql!(i32, (pg), (sqlite));
impl_to_sql!(i64, (pg), (sqlite));

impl_to_sql!(Option<u8>, (pg.map(|i| i as i16)), (sqlite));
impl_to_sql!(Option<u16>, (pg.map(|i| i as i16)), (sqlite));
impl_to_sql!(Option<u32>, (pg.map(|i| i as i32)), (sqlite));
impl_to_sql!( Option<u64>, (pg.map(|i| i as i64)), (sqlite.map(|i| i as i64)));

impl_to_sql!(Option<i8>, (pg), (sqlite));
impl_to_sql!(Option<i16>, (pg), (sqlite));
impl_to_sql!(Option<i32>, (pg), (sqlite));
impl_to_sql!(Option<i64>, (pg), (sqlite));

impl_to_sql!(String, (pg), (sqlite));
