use crate::{Column, Result, TableUpdater, ToSql, ToSqlTypeName, ColumnType, ColumnAttribute, Connection, TableDefinition, TableDefinitionField};
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

pub struct TableBuilder<'a, 'b, T: Connection> {
    definition: TableDefinition,
    updater: &'a TableUpdater<'b, T>,
}

impl<'a, 'b, T: Connection> TableBuilder<'a, 'b, T> {
    pub fn new(name: &'static str, updater: &'a TableUpdater<'b, T>) -> TableBuilder<'a, 'b, T> {
        TableBuilder {
            definition: TableDefinition::new(name),
            updater,
        }
    }

    pub fn column<COL: Column>(mut self, _column: COL) -> Self
    {
        self.definition.fields.push(TableDefinitionField::new::<COL>());
        self
    }

    pub fn build(mut self) -> Result<()> {
        self.updater.conn.update_table_by_definition(&self.definition)?;
        Ok(())
    }
}

