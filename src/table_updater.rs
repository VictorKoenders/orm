use crate::{Connection, TableBuilder};

pub struct TableUpdater<'a, T: Connection> {
    pub conn: &'a mut T,
}

impl<'a, T: Connection> TableUpdater<'a, T> {
    pub fn table<'b>(&'b self, name: &'static str) -> TableBuilder<'a, 'b, T> {
        TableBuilder::new(name, self)
    }
}
