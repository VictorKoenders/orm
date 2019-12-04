use orm_pq::prelude::*;

#[derive(OrmTable, Debug)]
struct User {
    pub id: Uuid,
    pub name: String,
    pub date_of_birth: DateTime<Utc>,
    pub password: Option<Lazy<Password>>,
}

#[derive(OrmTable, Debug)]
struct Password {
    pub id: u32,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub user: Lazy<User>,
}

#[derive(OrmContext)]
struct Context {
    pub user: DbSet<User>,
    pub password: DbSet<Password>,
}

fn main() {
    let context = Context::postgres()
        .connection_string("postgres://Trangar:development@localhost/orm_test")
        .connect()
        .expect("Could not connect");
    // or
    // let pool = Context::postgres().connection_string(...).create_pool().expect("Could not create pool");
    // let context = pool.create_context().expect("Could not create connection");

    let user: Option<User> = context
        .user
        .filter(|u| u.name().eq("Trangar"))
        .load_one()
        .expect("Could not load user");
    if let Some(user) = user {
        println!("Found user {:?}", user);
    } else {
        let user = User {
            id: Uuid::new_v4(),
            name: String::from("Trangar"),
            date_of_birth: Utc::now(),
            password: None,
        };
        context.user.add(&mut user);
        println!("Inserted user: {:?}", user);
    }
}
