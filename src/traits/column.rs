use crate::ToSql;

pub trait Column {
    type Type: ToSql;
    fn name() -> &'static str;
    fn db_type() -> &'static ColumnType;
    fn db_type_attributes() -> &'static [&'static ColumnAttribute];
}

pub trait ColumnType: std::fmt::Debug {
    fn pg_type(&self) -> &str;
    fn sqlite_type(&self) -> &str;
}

pub trait ColumnAttribute: std::fmt::Debug {
    fn to_pg_string(&self) -> String;
    fn to_sqlite_string(&self) -> String;
}
