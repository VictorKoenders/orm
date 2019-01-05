use crate::{Connection, TableBuilder};

pub struct TableUpdater<'a> {
    pub conn: &'a mut Connection,
}

impl<'a> TableUpdater<'a> {
    pub fn table<'b>(&'b self, name: &'b str) -> TableBuilder<'a, 'b> {
        TableBuilder::new(name, self)
    }
}
