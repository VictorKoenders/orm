use std::marker::PhantomData;
use super::{Column, Table, Result, QueryBuilder};
use postgres::Connection;
use std::rc::Rc;

pub struct DbSet<TABLE: Table> {
    pd_table: PhantomData<TABLE>,
    connection: Rc<Connection>,
}

impl<TABLE: Table> DbSet<TABLE> {
    #[doc(hidden)]
    pub fn __new(connection: Rc<Connection>) -> Self {
        Self {
            pd_table: PhantomData,
            connection,
        }
    }

    pub fn load_by_id(&mut self, _id: <<TABLE as Table>::ID as Column>::Type) -> Result<Option<TABLE>> {
        Ok(None)
    }

    pub fn query(&self) -> <TABLE as Table>::Query {
        QueryBuilder::new(&self.connection).into()
    }

    pub fn save(&mut self, _t: &mut TABLE) -> Result<()> {
        Ok(())
    }
}
