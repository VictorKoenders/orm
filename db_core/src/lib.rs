pub extern crate failure;

pub type Result<T> = std::result::Result<T, failure::Error>;

mod connection;
mod query_builder;
mod row;

pub use connection::Connection;
pub use query_builder::*;
pub use row::Row;
