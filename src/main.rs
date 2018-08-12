extern crate orm;

use orm::DbSet;
mod user {
    #[allow(dead_code, non_camel_case_types)]
    pub struct id;
    impl ::orm::Column for id {
        type Type = u32;
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

    pub struct Query<'a> {
        builder: ::orm::QueryBuilder<'a>,
    }

    impl<'a> Query<'a> {
        pub fn id(self) -> ::orm::PartialCriteria<'a, ::User, Self, id> {
            ::orm::PartialCriteria::new(self.builder)
        }
        pub fn name(self) -> ::orm::PartialCriteria<'a, ::User, Self, name> {
            ::orm::PartialCriteria::new(self.builder)
        }
        pub fn birthdate(self) -> ::orm::PartialCriteria<'a, ::User, Self, birthdate> {
            ::orm::PartialCriteria::new(self.builder)
        }
        pub fn execute(self) -> ::orm::Result<Vec<::User>> {
            self.builder.execute()
        }
    }

    impl<'a> From<::orm::QueryBuilder<'a>> for Query<'a> {
        fn from(builder: ::orm::QueryBuilder<'a>) -> Query<'a> {
            Query { builder }
        }
    }

    impl<'a> ::orm::Query<::User> for Query<'a> {
    }
}

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

#[derive(Debug)]
// #[derive(Table)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub birthdate: String,
}

// #[derive(DbContext)]
pub struct Context {
    pub users: DbSet<User>,
}

fn run() -> ::orm::Result<()> {
    use orm::{DbContext, Queryable};
    let mut context = Context::connect("...")?;
    {
        if let Some(mut user) = context.users.load_by_id(5)? {
            println!("{:?}", user);

            user.name = format!("{0} {0}", user.name);
            context.users.save(&mut user)?;
        }
    }
    /*{
        let users = context.users.query().id().eq(1).execute()?;
        println!("{:?}", users);
    }*/

    Ok(())
}
