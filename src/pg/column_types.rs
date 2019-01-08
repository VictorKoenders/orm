use crate::ColumnType;

pub trait GetColumnType {
    fn get_column_type() -> &'static ColumnType;
}

macro_rules! impl_type {
    ($static_name:ident, $struct_name:ident, $($ty:ty),*) => {
        #[derive(Debug)]
        pub struct $struct_name;
        impl ColumnType for $struct_name {
            fn to_type(&self) -> &'static str {
                stringify!($static_name)
            }
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

impl_type!(TINYINT, TinyInt, i16, u16, i8, u8);
impl_type!(INT, Int, i32, u32);
impl_type!(TEXT, Text, String);

