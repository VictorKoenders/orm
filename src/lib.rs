#![allow(unused_variables, unused_mut, unused_imports)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate cfg_if;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, failure::Error>;

mod dbset;
mod expression;
mod inner_context;
mod table_builder;
mod table_updater;
mod traits;

cfg_if! {
    if #[cfg(feature = "pg")] {
        mod pg;
        pub use self::pg::*;
    } else if #[cfg(feature = "sqlite")] {
        mod sqlite;
        pub use self::sqlite::*;
    }
}

pub use orm_derive::{Context, Table};

pub use self::dbset::*;
pub use self::expression::*;
pub use self::inner_context::*;
pub use self::table_builder::*;
pub use self::table_updater::*;
pub use self::traits::*;

pub enum ExpressionCompare {
    Equals,
}

pub struct Eq<T> {
    val: T,
}
