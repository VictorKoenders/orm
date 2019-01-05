#![allow(proc_macro_derive_resolution_fallback)]

extern crate orm;

use orm::{Context, DbSet, Table};

#[derive(Table, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Context)]
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

    assert_eq!(
        vec![User {
            id: 1,
            name: String::from("test")
        }],
        users
    );

    println!("Loading user named \"test\": {:?}", users);

    let users = context
        .users
        .filter(|u| u.id().eq(2))
        .expect("Could not load user");

    assert_eq!(
        vec![User {
            id: 2,
            name: String::from("Trangar")
        }],
        users
    );

    println!("Loading user ID 2: {:?}", users);

    let users = context
        .users
        .filter(|u| u.id().eq(2).name().eq("test"))
        .expect("Could not load user");

    assert_eq!(Vec::<User>::new(), users);

    println!("Loading user named \"test\" with ID 2: {:?}", users);
}
