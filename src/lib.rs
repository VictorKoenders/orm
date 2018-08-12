#![deny(missing_docs)]

//! ORM is a simple object relational mapper implemented in Rust. It tries to make the development cycle as short and smooth as possible, even if this were to sacrifice some speed.
//! 
//! ORM works with a single object called a [DbContext](trait.DbContext.html). This object contains one or more (DbSets)[struct.DbSet.html].
//! 
//! The generic argument of a `DbSet` is a struct that derives from [Table](trait.Table.html). 
//! 
//! ```rust
//! 
//! #[derive(DbContext)]
//! pub struct Context {
//!     pub users: DbSet<User>,
//! }
//! 
//! #[derive(Debug)]
//! #[derive(Table)]
//! pub struct User {
//!     pub id: i32,
//!     pub name: String,
//!     pub birthdate: String,
//! }
//! 
//! fn main() { 
//!     // TODO: Get this from a .env file with the dotenv crate
//!     let mut context = Context::connect("postgres://postgres:postgres@localhost/orm").unwrap();
//! 
//!     // Load all users named 'Bob'
//!     let users = context.users.query().name.eq("Bob").execute().unwrap();
//!     println!("{:?}", users);
//! }
//! ```

/// Re-export the failure crate for the derive macros to use
pub extern crate failure;
/// Re-export the postgres crate for the derive macros to use
pub extern crate postgres;

mod orm_impl;
pub use orm_impl::*;
