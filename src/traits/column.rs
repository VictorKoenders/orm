use crate::ToSql;

pub trait Column {
    type Type: ToSql;
    fn name() -> &'static str;
    fn db_type() -> &'static str;
}
