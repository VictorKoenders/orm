use crate::{Connection as ConnectionTrait, Result, TableDefinition, ToSql, QueryBinder};
use std::rc::Rc;

pub struct Connection {
    conn: Rc<postgres::Connection>,
}

impl ConnectionTrait for Connection {
    type QueryBinder = PgQueryBinder;

    fn query(&self, str: &str) -> PgQueryBinder {
        PgQueryBinder {
            conn: Rc::clone(&self.conn),
            query: str.to_owned(),
            args: Vec::new(),
        }
    }

    fn update_table_by_definition(&self, definition: &TableDefinition) -> Result<()> {
        println!("Updating table {:#?}", definition);
        unimplemented!();
    }
}

pub struct PgQueryBinder {
    conn: Rc<postgres::Connection>,
    query: String,
    args: Vec<Box<postgres::types::ToSql>>,
}

impl QueryBinder for PgQueryBinder {
    fn bind<T: ToSql>(&mut self, t: T) {
        self.args.push(t.to_pg_sql());
    }

    fn execute<T>(self) -> Result<T> {
        println!("Executing {:?} with args {:?}", self.query, self.args);

        let args: Vec<_> = self.args.iter().map(|a| a.as_ref()).collect();
        self.conn.query(&self.query, &args)?;
        unimplemented!()
    }
}
