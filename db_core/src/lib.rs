pub extern crate failure;
#[macro_use]
extern crate bitflags;

pub type Result<T> = std::result::Result<T, failure::Error>;

mod connection;
mod table_builder;
mod query_builder;
mod row;

pub use connection::*;
pub use query_builder::*;
pub use table_builder::*;
pub use row::*;

