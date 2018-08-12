use super::{Table, DbSet};

pub trait Query<TABLE: Table> {
    fn on(set: DbSet<TABLE>) -> Self;
}
