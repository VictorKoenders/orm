use crate::ToSql;

pub trait Column {
    type Type: ToSql;
    fn name() -> &'static str;
}
