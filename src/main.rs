#![allow(unused_variables, unused_mut, unused_imports)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, failure::Error>;

mod dbset;
mod expression;
pub mod generated;
mod inner_context;
mod table_builder;
mod table_updater;
mod traits;

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

use crate::generated::*;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub struct DbContext {
    pub users: DbSet<User>,
}

fn main() {
    dotenv::dotenv().unwrap();
    let context = DbContext::new(&std::env::var("DATABASE_URL").unwrap())
        .expect("Could not connect to database");

    let users = context
        .users
        .filter(|u| u.name().eq("test"))
        .expect("Could not load user");
    println!("Users: {:?}", users);

    let users = context
        .users
        .filter(|u| u.id().eq(2))
        .expect("Could not load user");
    println!("Users: {:?}", users);

    let users = context
        .users
        .filter(|u| u.id().eq(2).name().eq("test"))
        .expect("Could not load user");
    println!("Users: {:?}", users);
}
