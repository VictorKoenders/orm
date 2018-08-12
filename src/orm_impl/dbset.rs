use std::marker::PhantomData;
use super::{Column, Table, Result, Query};

pub struct DbSet<TABLE: Table> {
    pd_table: PhantomData<TABLE>,
}

impl<TABLE: Table> ::std::clone::Clone for DbSet<TABLE> {
    fn clone(&self) -> Self {
        Self {
            pd_table: PhantomData,
        }
    }
}

impl<TABLE: Table> DbSet<TABLE> {
    #[doc(hidden)]
    pub fn __new() -> Self {
        Self {
            pd_table: PhantomData,
        }
    }

    pub fn load_by_id(&mut self, _id: <<TABLE as Table>::ID as Column>::Type) -> Result<Option<TABLE>> {
        Ok(None)
    }

    pub fn query(&self) -> <TABLE as Table>::QUERY {
        let cloned: DbSet<TABLE> = self.clone();
        <TABLE as Table>::QUERY::on(cloned)
    }

    pub fn save(&mut self, _t: &mut TABLE) -> Result<()> {
        Ok(())
    }
}
