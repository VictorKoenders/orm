pub extern crate failure;
#[macro_use]
extern crate bitflags;

pub type Result<T> = std::result::Result<T, failure::Error>;

mod connection;
mod query_builder;
mod row;
mod table_builder;

pub use connection::*;
pub use query_builder::*;
pub use row::*;
pub use table_builder::*;
