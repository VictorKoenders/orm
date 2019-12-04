use std::marker::PhantomData;

pub struct DbSet<T: crate::traits::OrmTable>{
    pd: PhantomData<T>,
}

impl<T: crate::traits::OrmTable> DbSet<T> {
    pub fn filter(&self, cb: impl FnOnce(T::Filter) -> T::Filter) -> Query {
        unimplemented!()
    }
}

/*
#[derive(OrmTable, Debug)]
struct User {
    pub id: Uuid,
    pub name: String,
    pub date_of_birth: DateTime<Utc>,
    pub password: Option<Lazy<Password>>,
}
*/
struct __UserFilter<ID, NAME, DATE_OF_BIRTH, PASSWORD> {
    id: ID,
    name: NAME,
    date_of_birth: DATE_OF_BIRTH,
    password: PASSWORD
}

impl<NAME, DATE_OF_BIRTH, PASSWORD> __UserFilter<(), NAME, DATE_OF_BIRTH, PASSWORD> {
    pub fn id(self) -> QuerySegment {
        unimplemented!()
    }
}

pub struct QuerySegment{}

pub struct Query;

