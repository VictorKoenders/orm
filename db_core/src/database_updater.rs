use crate::table_builder::{Table, Column};

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
    pub fn ensure_table_exists(&mut self, table: Table<'a>) {
        self.new_definition.push(table);
    }

    pub fn diff(&self) -> Vec<DatabaseUpdaterChange> {
        let mut result = Vec::new();
        for old_table in &self.old_definition {
            let new_table = match self.new_definition.iter().find(|d| d.name == old_table.name) {
                Some(t) => t,
                None => {
                    result.push(DatabaseUpdaterChange::DropTable(old_table));
                    continue;
                }
            };

            for column in &new_table.columns {
                let old_column = match old_table.columns.iter().find(|c| c.name == column.name) {
                    Some(c) => c,
                    None => {
                        result.push(DatabaseUpdaterChange::AddTableColumn(new_table, column));
                        continue;
                    }
                };
                if old_column != column {
                    result.push(DatabaseUpdaterChange::AlterTableColumn(new_table, column));
                }
            }
            for column in &old_table.columns {
                if new_table.columns.iter().all(|c| c.name != column.name) {
                    result.push(DatabaseUpdaterChange::DropTableColumn(old_table, column));
                }
            }
        }

        for new_table in &self.new_definition {
            if self.old_definition.iter().all(|c| c.name != new_table.name) {
                result.push(DatabaseUpdaterChange::CreateTable(new_table));
            }
        }

        result
    }
}

pub enum DatabaseUpdaterChange<'a> {
    CreateTable(&'a Table<'a>),
    DropTable(&'a Table<'a>),
    AddTableColumn(&'a Table<'a>, &'a Column<'a>),
    AlterTableColumn(&'a Table<'a>, &'a Column<'a>),
    DropTableColumn(&'a Table<'a>, &'a Column<'a>),
}

