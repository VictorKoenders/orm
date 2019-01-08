use crate::ColumnType;

pub trait GetColumnType {
    fn get_column_type() -> &'static ColumnType;
}

macro_rules! impl_type {
    ($static_name:ident, $struct_name:ident, pg: $pg:expr, sqlite: $sqlite:expr, $($ty:ty),*) => {
        #[derive(Debug)]
        pub struct $struct_name;
        impl ColumnType for $struct_name {
            fn pg_type(&self) -> &str { $pg }
            fn sqlite_type(&self) -> &str { $sqlite }
        }
        pub static $static_name: $struct_name = $struct_name;
        $(
            impl GetColumnType for $ty {
                fn get_column_type() -> &'static ColumnType {
                    &$static_name
                }
            }
        )*
    }
}

impl_type!(TINYINT, TinyInt, pg: "TINYINT", sqlite: "INTEGER", i8, u8);
impl_type!(SMALLINT, SmallInt, pg: "TINYINT", sqlite: "INTEGER", i16, u16);
impl_type!(INT, Int, pg: "INT", sqlite: "INTEGER", i32, u32);
impl_type!(BIGINT, BigInt, pg: "BIGINT", sqlite: "INTEGER", i64, u64);
impl_type!(TEXT, Text, pg: "TEXT", sqlite: "TEXT", String);
