pub mod user {
    #[allow(unused)]
    pub struct QueryBuilder<TID, TNAME> {
        #[allow(unused)]
        id: TID,
        #[allow(unused)]
        name: TNAME,

        db: crate::InnerContext,
    }

    #[allow(unused)]
    pub struct ID;
    impl crate::Column for ID {
        type Type = i32;
        fn name() -> &'static str {
            "id"
        }
    }
    #[allow(unused)]
    pub struct NAME;
    impl crate::Column for NAME {
        type Type = String;
        fn name() -> &'static str {
            "name"
        }
    }

    impl crate::Table for crate::User {
        type QueryBuilder = QueryBuilder<(), ()>;
        fn table_name() -> &'static str {
            "user"
        }

        fn update_database_schema(updater: &mut crate::TableUpdater) -> crate::Result<()> {
            updater.table("user").column(ID).column(NAME).build()
        }

        fn from_reader(row: &postgres::rows::Row) -> crate::Result<crate::User> {
            Ok(crate::User {
                id: row.get_opt(0).unwrap()?,
                name: row.get_opt(1).unwrap()?,
            })
        }
    }

    impl<TID, TNAME> crate::Queryable<crate::User> for QueryBuilder<TID, TNAME>
    where
        TID: crate::AsQueryFilter,
        TNAME: crate::AsQueryFilter,
    {
        fn get_results(&self) -> crate::Result<Vec<crate::User>> {
            let mut query = String::from("SELECT ");
            query += "\"";
            query += <ID as crate::Column>::name();
            query += "\", \"";
            query += <NAME as crate::Column>::name();
            query += "\"";
            query += " FROM \"";
            query += <crate::User as crate::Table>::table_name();
            query += "\"";
            let mut values = Vec::new();

            if let Some((format, val)) = self.id.as_query_filter(values.len()) {
                query += &format!(
                    " {} \"{}\"{}",
                    if values.is_empty() { "WHERE" } else { "AND" },
                    <ID as crate::Column>::name(),
                    format
                );
                values.push(val);
            }

            if let Some((format, val)) = self.name.as_query_filter(values.len()) {
                query += &format!(
                    " {} \"{}\"{}",
                    if values.is_empty() { "WHERE" } else { "AND" },
                    <NAME as crate::Column>::name(),
                    format
                );
                values.push(val);
            }

            println!("Query: {:?}", query);
            println!("Arguments: {:?}", values);

            let conn = self.db.pool.get()?;

            let rows = conn.query(&query, values.as_slice())?;
            let mut results = Vec::with_capacity(rows.len());
            for row in rows.iter() {
                results.push(<crate::User as crate::Table>::from_reader(&row)?);
            }
            Ok(results)
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
                db: self.db,
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
                db: self.db,
            }
        }
    }

    impl crate::QueryBuilder for QueryBuilder<(), ()> {
        fn new(inner: crate::InnerContext) -> Self {
            QueryBuilder {
                id: (),
                name: (),
                db: inner,
            }
        }
    }
}

impl crate::DbContext {
    pub fn new(url: &str) -> crate::Result<crate::DbContext> {
        let context = crate::InnerContext::new(url)?;
        let conn = context.pool.get()?;

        let mut transaction = conn.transaction()?;
        <crate::User as crate::Table>::update_database_schema(&mut crate::TableUpdater {
            conn: &mut transaction,
        })?;
        println!("Committing transaction");
        transaction.commit()?;
        println!("Done");

        Ok(crate::DbContext {
            users: crate::DbSet::__new(context.clone()),
        })
    }
}
