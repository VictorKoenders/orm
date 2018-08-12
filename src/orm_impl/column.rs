/// Defines a column of a table. A column exists of a Type and a name.
pub trait Column: Sized {
    /// Defines the type of this column
    type Type;

    /// Defines the name of this column
    fn name() -> &'static str;
}
