pub extern crate failure;
#[macro_use]
extern crate bitflags;

// TODO: Get rid of failure dependency
pub type Result<T> = std::result::Result<T, failure::Error>;

pub mod connection;
pub mod database_updater;
pub mod query_builder;
pub mod row;
pub mod table_builder;

pub use connection::*;
pub use query_builder::*;
pub use row::*;
pub use table_builder::*;
