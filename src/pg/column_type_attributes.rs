use crate::ColumnAttribute;

#[derive(Debug)]
pub struct Null;
#[derive(Debug)]
pub struct NotNull;
#[derive(Debug)]
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
