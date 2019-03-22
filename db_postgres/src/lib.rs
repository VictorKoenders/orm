pub use db_core::{EstimateStrLen, Result};

pub struct Connection(pq_sys::PGconn);

impl db_core::Connection for Connection {
    type ConnectionParam = String;
    type Row = Row;

    fn connect(s: String) -> Result<Connection> {
        unimplemented!()
    }

    fn execute(&self, builder: db_core::QueryBuilder) -> Result<Vec<Row>> {
        let mut query = String::with_capacity(builder.estimate_str_len() * 2);

        unimplemented!()
    }
}

pub struct Row();

impl db_core::Row for Row {}
