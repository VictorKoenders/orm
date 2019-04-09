use std::borrow::Cow;

/// Contains the definition of a table in the database
#[derive(Debug)]
pub struct Table<'a> {
    /// The name of the table
    pub name: Cow<'a, str>,
    /// The columns in the table
    pub columns: Vec<Column<'a>>,
    /// Any special constraints on the table, e.g. multi-column indexes
    pub constraints: Vec<TableConstraint<'a>>,
}

impl<'a> Table<'a> {
    pub fn with_name(name: impl Into<Cow<'a, str>>) -> Table<'a> {
        Table {
            name: name.into(),
            columns: Vec::new(),
            constraints: Vec::new(),
        }
    }
}

/// Special constraints on a table
#[derive(Debug)]
pub enum TableConstraint<'a> {
    /// An index that contains multiple columns
    MultiColumnIndex(Vec<Cow<'a, str>>),

    /// A unique constraint that spans multiple columns
    MultiColumnUnique(Vec<Cow<'a, str>>),
}

/// A column in a table
#[derive(Debug)]
pub struct Column<'a> {
    /// The name of the column
    pub name: Cow<'a, str>,

    /// All foreign keys this column has. Reverse foreign keys are not stored and will have to be
    /// looked up manually.
    pub foreign_keys: Vec<ForeignKey<'a>>,

    /// The default value of this column, if any
    pub default: Option<ColumnDefault<'a>>,

    /// The type of this column
    pub r#type: ColumnType<'a>,

    /// The flags of this column, e.g. primary key, index, unique, not-nullable
    pub flags: ColumnFlags,
}

impl<'a> PartialEq for Column<'a> {
    fn eq(&self, other: &Column) -> bool {
        if self.name != other.name { return false; }
        if self.foreign_keys.len() != other.foreign_keys.len() { return false; }
        // TODO: Compare all foreign keys
        if self.default != other.default { return false; }
        if self.r#type != other.r#type { return false; }
        if self.flags != other.flags { return false; }
        true
    }
}


impl<'a> Column<'a> {
    pub fn with_name(name: impl Into<Cow<'a, str>>) -> Column<'a> {
        Column {
            name: name.into(),
            foreign_keys: Vec::new(),
            default: None,
            r#type: ColumnType::Custom("".into()),
            flags: ColumnFlags::default(),
        }
    }
}

bitflags! {
    /// The flags a column can have, e.g. primary key, index, unique, not-nullable
    #[derive(Default)]
    pub struct ColumnFlags: u32 {
        /// Determines that the column is a primary key
        const PRIMARY  = 0b00000000_00000001;

        /// Determines that the column has an index. This should have the same effect as adding a
        /// `TableConstraint::MultiColumnIndex(vec![column_name])` to the table
        const INDEX    = 0b00000000_00000010;

        /// Determines that the column is unique. This should have the same effect as adding a
        /// `TableConstraint::MultiColumnUnique(vec![column_name])` to the table
        const UNIQUE   = 0b00000000_00000100;

        /// Determines that the column cannot contain a NULL value
        const NOT_NULL = 0b00000000_00001000;
    }
}

/// The type a column can have
#[derive(Debug, PartialEq)]
pub enum ColumnType<'a> {
    /// The column is a smallint. This matches the rust type i16
    SmallInt,

    /// The column is an int. This matches the rust type i32
    Int,

    /// The column is a bigint. This matches the rust type i64
    BigInt,

    /// The column is a text with an optional fixed sized
    Text(Option<usize>),

    /// The column is a byte array with an optional fixed sized
    ByteArray(Option<usize>),

    /// The column is a database-specific type that we cannot express. Implementations are expected
    /// to use this str directly.
    Custom(Cow<'a, str>),
}

#[derive(Debug, PartialEq)]
pub enum ColumnDefault<'a> {
    Custom(Cow<'a, str>),
}

#[derive(Debug, PartialEq)]
pub struct ForeignKey<'a> {
    pub table: Cow<'a, str>,
    pub column: Cow<'a, str>,
}
