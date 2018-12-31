pub mod user {
    #[derive(Default)]
    #[allow(unused)]
    pub struct QueryBuilder<TID, TNAME> {
        #[allow(unused)]
        id: TID,
        #[allow(unused)]
        name: TNAME,
    }

    #[allow(unused)]
    pub struct ID;
    impl crate::DbColumn for ID {
        type Type = i32;
        fn name() -> &'static str {
            "id"
        }
    }
    #[allow(unused)]
    pub struct NAME;
    impl crate::DbColumn for NAME {
        type Type = String;
        fn name() -> &'static str {
            "name"
        }
    }

    impl crate::DbTable for crate::User {
        type QueryBuilder = QueryBuilder<(), ()>;
        fn table_name() -> &'static str {
            "user"
        }

        fn update_database_schema(updater: &mut crate::TableUpdater) -> crate::Result<()> {
            updater.table("user").column(ID).column(NAME).build()
        }
    }

    impl<TID, TNAME> crate::Queryable<crate::User> for QueryBuilder<TID, TNAME>
    where
        TID: crate::AsQueryFilter,
        TNAME: crate::AsQueryFilter,
    {
        fn get_results(&self) -> crate::Result<Vec<crate::User>> {
            let mut query = format!(
                "SELECT * FROM \"{}\"",
                <crate::User as crate::DbTable>::table_name()
            );
            let mut values = Vec::new();

            if let Some((format, val)) = self.id.as_query_filter(values.len()) {
                query += &format!(
                    " {} \"{}\"{}",
                    if values.is_empty() { "WHERE" } else { "AND" },
                    <ID as crate::DbColumn>::name(),
                    format
                );
                values.push(val);
            }

            if let Some((format, val)) = self.name.as_query_filter(values.len()) {
                query += &format!(
                    " {} \"{}\"{}",
                    if values.is_empty() { "WHERE" } else { "AND" },
                    <NAME as crate::DbColumn>::name(),
                    format
                );
                values.push(val);
            }

            println!("Query: {:?}", query);
            unimplemented!()
        }
    }

    impl<TNAME> QueryBuilder<(), TNAME> {
        pub fn id(self) -> crate::Expression<Self, ID> {
            crate::Expression::new(self)
        }
    }

    impl<T, TNAME> crate::ExpressionNext<ID, T> for QueryBuilder<(), TNAME> {
        type Result = QueryBuilder<T, TNAME>;
        fn next(self, val: T) -> Self::Result {
            QueryBuilder {
                id: val,
                name: self.name,
            }
        }
    }

    impl<TID> QueryBuilder<TID, ()> {
        pub fn name(self) -> crate::Expression<Self, NAME> {
            crate::Expression::new(self)
        }
    }

    impl<T, TID> crate::ExpressionNext<NAME, T> for QueryBuilder<TID, ()> {
        type Result = QueryBuilder<TID, T>;
        fn next(self, val: T) -> Self::Result {
            QueryBuilder {
                id: self.id,
                name: val,
            }
        }
    }
}

pub mod dbcontext {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref POOL: r2d2::Pool<r2d2_postgres::PostgresConnectionManager> = {
            let url = std::env::var("DATABASE_URL")
                .expect("Could not get environment var \"DATABASE_URL\"");
            let manager =
                r2d2_postgres::PostgresConnectionManager::new(url, r2d2_postgres::TlsMode::None)
                    .expect("Could not set up connection to the server");
            r2d2::Pool::new(manager).expect("Could not set up a connection pool")
        };
    }
}

impl crate::DbContext {
    pub fn new() -> crate::Result<crate::DbContext> {
        let conn = dbcontext::POOL.get()?;

        let mut transaction = conn.transaction()?;
        <crate::User as crate::DbTable>::update_database_schema(&mut crate::TableUpdater {
            conn: &mut transaction,
        })?;
        transaction.finish()?;

        Ok(crate::DbContext {
            users: crate::DbSet::__new(),
        })
    }
}
