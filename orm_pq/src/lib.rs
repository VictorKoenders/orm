pub mod prelude {
    pub use crate::OrmContextExt;
    #[cfg(feature = "chrono")]
    pub use chrono::{DateTime, Utc};
    pub use orm_core::prelude::*;
    #[cfg(feature = "uuid")]
    pub use uuid::Uuid;
}

use orm_core::traits::OrmContext;

pub trait OrmContextExt<T: OrmContext> {
    fn postgres() -> PostgresConnectionBuilder<T>;
}

impl<T> OrmContextExt<T> for T
where
    T: OrmContext,
{
    fn postgres() -> PostgresConnectionBuilder<T> {
        PostgresConnectionBuilder::new()
    }
}

pub struct PostgresConnectionBuilder<T>
where
    T: OrmContext,
{
    marker: std::marker::PhantomData<T>,
}

impl<T: OrmContext> PostgresConnectionBuilder<T> {
    fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
    pub fn connection_string(self, _s: impl ToString) -> Self {
        unimplemented!()
    }
    pub fn connect(self) -> Result<T, String> {
        unimplemented!()
    }
}
