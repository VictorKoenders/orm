use crate::table_builder::Table;

pub struct DatabaseUpdater<'a> {
    old_definition: Vec<Table<'a>>,
    new_definition: Vec<Table<'a>>,
}

impl<'a> DatabaseUpdater<'a> {
    pub fn from_old_definition(old_definition: Vec<Table<'a>>) -> DatabaseUpdater<'a> {
        let old_definition_len = old_definition.len();
        DatabaseUpdater {
            old_definition,
            new_definition: Vec::with_capacity(old_definition_len),
        }
    }
    pub fn ensure_table_exists(&'a mut self, table: Table<'a>) {
        self.new_definition.push(table);
    }
}
