use crate::{Column, ColumnAttribute, ColumnType, Result, ToSql};

pub trait Connection {
    type QueryBinder: QueryBinder;

    fn query(&self, str: &str) -> Self::QueryBinder;
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

pub trait QueryBinder {
    fn bind<T: ToSql>(&mut self, t: T);
    fn execute<T>(self) -> Result<T>;
}

