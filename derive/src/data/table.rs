use super::Field;
use syn::spanned::Spanned;
use syn::{Ident, ItemStruct, Result};

pub struct Table {
    pub database_table_name: String,
    pub mod_name: Ident,
    pub name: Ident,
    pub fields: Vec<Field>,
}

impl Table {
    pub fn from_struct(s: &ItemStruct) -> Result<Table> {
        let name = s.ident.clone();
        let database_table_name = name.to_string().to_lowercase();
        let mod_name = Ident::new(&database_table_name, s.span());

        let mut fields = Vec::new();
        for field in s.fields.iter() {
            fields.push(Field::from(field)?);
        }

        Ok(Table {
            database_table_name,
            mod_name,
            name,
            fields,
        })
    }
}
