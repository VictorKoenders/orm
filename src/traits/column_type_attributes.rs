use crate::ColumnAttribute;

#[derive(Debug)]
pub struct Null;
#[derive(Debug)]
pub struct NotNull;
#[derive(Debug)]
pub struct PrimaryKey;

impl ColumnAttribute for Null {
    fn to_pg_string(&self) -> String {
        String::from("NULL")
    }
    fn to_sqlite_string(&self) -> String {
        String::from("NULL")
    }
}

impl ColumnAttribute for NotNull {
    fn to_pg_string(&self) -> String {
        String::from("NOT NULL")
    }
    fn to_sqlite_string(&self) -> String {
        String::from("NOT NULL")
    }
}

impl ColumnAttribute for PrimaryKey {
    fn to_pg_string(&self) -> String {
        String::from("PRIMARY KEY")
    }
    fn to_sqlite_string(&self) -> String {
        String::from("PRIMARY KEY AUTOINCREMENT")
    }
}
