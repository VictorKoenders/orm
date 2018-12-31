use postgres::types::ToSql;
use postgres::GenericConnection;
use std::marker::PhantomData;

pub struct DbSet<T> {
    _marker: PhantomData<T>,
}

impl<T: DbTable> DbSet<T> {
    #[doc(hidden)]
    pub fn __new() -> DbSet<T> {
        DbSet {
            _marker: PhantomData,
        }
    }

    pub fn filter<FN, RESULT>(&self, cb: FN) -> Result<Vec<T>>
    where
        FN: FnOnce(<T as DbTable>::QueryBuilder) -> RESULT,
        RESULT: Queryable<T>,
    {
        let builder = cb(Default::default());
        builder.get_results()
    }
}

pub trait Queryable<T> {
    fn get_results(&self) -> Result<Vec<T>>;
    fn get_result(&self) -> Result<Option<T>> {
        Ok(self.get_results()?.into_iter().next())
    }
}

impl<T> Queryable<T> for DbSet<T> {
    fn get_results(&self) -> Result<Vec<T>> {
        //TODO: "SELECT * FROM {{table}}"
        unimplemented!()
    }
}

pub trait DbTable {
    type QueryBuilder: Default;
    fn table_name() -> &'static str;

    fn update_database_schema(updater: &mut TableUpdater) -> Result<()>;
}

pub struct TableUpdater<'a> {
    pub conn: &'a mut GenericConnection,
}

impl<'a> TableUpdater<'a> {
    pub fn table<'b>(&'b self, name: &'b str) -> TableBuilder<'a, 'b> {
        TableBuilder {
            name,
            updater: self,
        }
    }
}

pub struct TableBuilder<'a, 'b> {
    name: &'b str,
    updater: &'b TableUpdater<'a>,
}

impl<'a, 'b> TableBuilder<'a, 'b> {
    pub fn column<T: DbColumn>(self, _column: T) -> Self {
        self
    }

    pub fn build(self) -> Result<()> {
        Ok(())
    }
}

pub trait QueryBuilder {
    fn set<COLUMN, VALUE, RESULT>(self, v: VALUE) -> RESULT;
}

pub type Result<T> = std::result::Result<T, failure::Error>;

pub struct Expression<PARENT, COLUMN> {
    parent: PARENT,
    _column: PhantomData<COLUMN>,
}

impl<PARENT, COLUMN> Expression<PARENT, COLUMN> {
    pub fn new(parent: PARENT) -> Self {
        Self {
            parent,
            _column: PhantomData,
        }
    }

    pub fn eq<VAL>(self, val: VAL) -> <PARENT as ExpressionNext<COLUMN, Eq<VAL>>>::Result
    where
        PARENT: ExpressionNext<COLUMN, Eq<VAL>>,
        COLUMN: DbColumn,
        VAL: DbEquals<<COLUMN as DbColumn>::Type>,
    {
        self.parent.next(Eq { val })
    }
}

pub trait ExpressionNext<COLUMN, T> {
    type Result;

    fn next(self, val: T) -> Self::Result;
}

pub trait DbEquals<OTHER> {}
pub enum ExpressionCompare {
    Equals,
}

pub trait DbColumn {
    type Type;
    fn name() -> &'static str;
}

impl<'a> DbEquals<&'a str> for String {}
impl<'a> DbEquals<String> for &'a str {}

impl<A> DbEquals<A> for A {}

pub trait AsQueryFilter {
    fn as_query_filter(&self, index: usize) -> Option<(String, &ToSql)>;
}

impl AsQueryFilter for () {
    fn as_query_filter(&self, _index: usize) -> Option<(String, &ToSql)> {
        None
    }
}

pub struct Eq<T> {
    val: T,
}

impl<T: ToSql + 'static> AsQueryFilter for Eq<T> {
    fn as_query_filter(&self, index: usize) -> Option<(String, &ToSql)> {
        Some((format!("= ${}", index + 1), &self.val))
    }
}
