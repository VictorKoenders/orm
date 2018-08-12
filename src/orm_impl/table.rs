use super::{Column, Query, Result};
use postgres::rows::Row;

/// The base trait for a Table.
///
/// This define it's [table name](#tymethod.table_name), [ID column](#associatedtype.ID) and [Query](#associatedtype.QUERY) type.
/// It also a function [id](#tymethod.id) to get the current ID, and a function [load_from_reader](#tymethod.load_from_reader) to create an instance from a database row
pub trait Table: Sized {
    /// The Column definition of the ID of this type
    type ID: Column;

    /// The type used to query this table, for strongly-typed field querying
    type QUERY: Query<Self>;

    /// Get the table name of this table
    fn table_name() -> &'static str;

    /// Create a new instance of an object from a database row
    fn load_from_reader(row: &Row) -> Result<Self>;

    /// Get the database ID of this object
    fn id(&self) -> &<<Self as Table>::ID as Column>::Type;
}
