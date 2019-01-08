use crate::{InnerContext, QueryBuilder, Queryable, Result, Table};
use std::marker::PhantomData;

pub struct DbSet<T> {
    context: InnerContext,
    _marker: PhantomData<T>,
}

impl<T> Clone for DbSet<T> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            _marker: PhantomData,
        }
    }
}


impl<T: Table> DbSet<T> {
    #[doc(hidden)]
    pub fn __new(context: InnerContext) -> DbSet<T> {
        DbSet {
            _marker: PhantomData,
            context,
        }
    }

    pub fn filter<FN, RESULT>(&self, cb: FN) -> Result<Vec<T>>
    where
        FN: FnOnce(<T as Table>::QueryBuilder) -> RESULT,
        RESULT: Queryable<T>,
    {
        let builder = cb(<<T as Table>::QueryBuilder as QueryBuilder>::new(
            self.context.clone(),
        ));
        builder.get_results()
    }

    pub fn to_list(&self) -> Result<Vec<T>> {
        self.get_results()
    }
}

impl<T> Queryable<T> for DbSet<T> {
    fn get_results(&self) -> Result<Vec<T>> {
        //TODO: "SELECT * FROM {{table}}"
        unimplemented!()
    }
}
