use crate::ColumnAttribute;

pub struct Null;
pub struct NotNull;
pub struct PrimaryKey;

impl ColumnAttribute for Null {
    fn to_string(&self) -> String {
        String::from("NULL")
    }
}

impl ColumnAttribute for NotNull {
    fn to_string(&self) -> String {
        String::from("NOT NULL")
    }
}

impl ColumnAttribute for PrimaryKey {
    fn to_string(&self) -> String {
        String::from("PRIMARY KEY")
    }
}
