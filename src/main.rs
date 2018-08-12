extern crate dotenv;
extern crate orm;

use orm::DbSet;

#[derive(Debug)]
// #[derive(Table)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub birthdate: String,
}

// TODO: Generate this with a #[derive(Table)] on `struct User`
mod user {
    #[allow(dead_code, non_camel_case_types)]
    pub struct id;
    impl ::orm::Column for id {
        type Type = i32;
        fn name() -> &'static str {
            "id"
        }
    }
    #[allow(dead_code, non_camel_case_types)]
    pub struct name;
    impl ::orm::Column for name {
        type Type = String;
        fn name() -> &'static str {
            "name"
        }
    }
    #[allow(dead_code, non_camel_case_types)]
    pub struct birthdate;
    impl ::orm::Column for birthdate {
        type Type = String;
        fn name() -> &'static str {
            "birthdate"
        }
    }

    pub struct Query {
        builder: ::orm::QueryBuilder,
    }

    impl Query {
        pub fn id(self) -> ::orm::PartialCriteria<::User, Self, id> {
            ::orm::PartialCriteria::new(self.builder)
        }
        pub fn name(self) -> ::orm::PartialCriteria<::User, Self, name> {
            ::orm::PartialCriteria::new(self.builder)
        }
        pub fn birthdate(self) -> ::orm::PartialCriteria<::User, Self, birthdate> {
            ::orm::PartialCriteria::new(self.builder)
        }
    }

    impl From<::orm::QueryBuilder> for Query {
        fn from(builder: ::orm::QueryBuilder) -> Query {
            Query { builder }
        }
    }

    impl ::orm::Query<::User> for Query {
        fn execute(self) -> ::orm::Result<Vec<::User>> {
            self.builder.execute()
        }
    }
}

// TODO: Generate this with a #[derive(Table)] on `struct User`
impl orm::Table for User {
    type ID = user::id;
    type QUERY = user::Query;

    fn table_name() -> &'static str {
        "Users"
    }

    fn load_from_reader(row: &orm::postgres::rows::Row) -> orm::Result<Self> {
        let id = match row.get_opt(0) {
            Some(Ok(id)) => id,
            Some(Err(e)) => return Err(e.into()),
            None => return Err(orm::failure::err_msg("Field 'id' not found")),
        };
        let name = match row.get_opt(1) {
            Some(Ok(id)) => id,
            Some(Err(e)) => return Err(e.into()),
            None => return Err(orm::failure::err_msg("Field 'name' not found")),
        };
        let birthdate = match row.get_opt(2) {
            Some(Ok(id)) => id,
            Some(Err(e)) => return Err(e.into()),
            None => return Err(orm::failure::err_msg("Field 'birthdate' not found")),
        };
        Ok(User {
            id,
            name,
            birthdate,
        })
    }

    fn id(&self) -> &<Self::ID as orm::Column>::Type {
        &self.id
    }
}

// #[derive(DbContext)]
pub struct Context {
    pub users: DbSet<User>,
}

// TODO: Generate this with a #[derive(DbContext)] on `struct Context`
impl orm::DbContext for Context {
    fn connect(_url: impl AsRef<str>) -> ::orm::Result<Self> {
        let connection =
            ::orm::postgres::Connection::connect(_url.as_ref(), ::orm::postgres::TlsMode::None)?;
        let connection = ::std::rc::Rc::new(connection);
        Ok(Context {
            users: DbSet::__new(connection.clone()),
        })
    }
}

fn main() {
    if let Err(e) = run() {
        println!("Failed: {:?}", e);
    }
}

fn run() -> ::orm::Result<()> {
    dotenv::dotenv().expect("Could not load .env file");
    use orm::DbContext;
    let mut context =
        Context::connect(std::env::var("DATABASE_URL").expect("Could not load DATABASE_URL"))?;
    {
        if let Some(mut user) = context.users.load_by_id(5)? {
            println!("{:?}", user);

            user.name = format!("{0} {0}", user.name);
            context.users.save(&mut user)?;
        }
    }
    {
        use orm::Query;
        let users = context
            .users
            .query()
            .id()
            .eq(1)
            .name()
            .eq(String::from("test"))
            .execute()?;
        println!("{:?}", users);
    }

    Ok(())
}
