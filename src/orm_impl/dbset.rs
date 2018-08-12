use super::{Column, QueryBuilder, Result, Table};
use postgres::Connection;
use std::marker::PhantomData;
use std::rc::Rc;

/// Contains a queryable point where you can load/save/delete one or multiple records from a database
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

    /// Load a single entry by it's ID. If there are no results, then Ok(None) is returned.
    pub fn load_by_id(
        &mut self,
        _id: <<TABLE as Table>::ID as Column>::Type,
    ) -> Result<Option<TABLE>> {
        Ok(None)
    }

    /// Start a strongly typed query on this record. This returns a `Result<Vec<Obj>>` when you call `.execute()`
    pub fn query(&self) -> <TABLE as Table>::QUERY {
        QueryBuilder::new(self.connection.clone()).into()
    }

    /// Save a single record into the database. This will also automatically update the `id` field.
    pub fn save(&mut self, _t: &mut TABLE) -> Result<()> {
        Ok(())
    }
}
