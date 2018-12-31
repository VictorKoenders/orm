#![allow(unused_variables, unused_mut, unused_imports)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod generated;
pub mod lib_stub;

use crate::generated::*;
use crate::lib_stub::*;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub struct DbContext {
    pub users: DbSet<User>,
}

fn main() {
    let context = DbContext::new().expect("Could not connect to database");

    let users = context
        .users
        .filter(|u| u.id().eq(5).name().eq("test"))
        .expect("Could not load user");
    println!("Users: {:?}", users);
}
