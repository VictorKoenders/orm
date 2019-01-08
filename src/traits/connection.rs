use crate::{Result, ColumnType, ColumnAttribute, Column, ToSql};

pub trait Connection {
    type QueryResult;

    fn query(&self, str: &str, args: &[&ToSql]) -> Result<Self::QueryResult>;
    fn update_table_by_definition(&self, definition: &TableDefinition) -> Result<()>;
}

#[derive(Debug)]
pub struct TableDefinition {
    pub name: &'static str,
    pub fields: Vec<TableDefinitionField>,
}

impl TableDefinition {
    pub fn new(name: &'static str) -> TableDefinition {
        TableDefinition {
            name,
            fields: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct TableDefinitionField {
    pub name: &'static str,
    pub type_: &'static ColumnType,
    pub attributes: &'static [&'static ColumnAttribute],
}

impl TableDefinitionField {
    pub fn new<COL: Column>() -> TableDefinitionField {
        let name = COL::name();
        let type_ = COL::db_type();
        let attributes = COL::db_type_attributes();

        TableDefinitionField {
            name,
            type_,
            attributes,
        }
    }
}
