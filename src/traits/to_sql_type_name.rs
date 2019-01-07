pub trait ToSqlTypeName {
    fn name() -> &'static str;
}

impl ToSqlTypeName for i32 {
    fn name() -> &'static str {
        "INT"
    }
}

impl ToSqlTypeName for String {
    fn name() -> &'static str {
        "TEXT"
    }
}
