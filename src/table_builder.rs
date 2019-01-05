use crate::{Column, Result, TableUpdater, ToSql, ToSqlTypeName};
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

pub struct TableBuilder<'a, 'b> {
    name: &'b str,
    fields: HashMap<String, &'static str>,
    updater: &'b TableUpdater<'a>,
}

impl<'a, 'b> TableBuilder<'a, 'b> {
    pub fn new(name: &'b str, updater: &'b TableUpdater<'a>) -> TableBuilder<'a, 'b> {
        TableBuilder {
            name,
            fields: HashMap::new(),
            updater,
        }
    }

    pub fn column<T: Column>(mut self, _column: T) -> Self
    where
        T::Type: ToSqlTypeName,
    {
        let type_name = <T::Type as ToSqlTypeName>::name();
        self.fields.insert(T::name().to_owned(), type_name);
        self
    }

    pub fn build(mut self) -> Result<()> {
        let result = self.updater.conn.query(TABLE_COLUMN_QUERY, &[&self.name])?;
        if result.is_empty() {
            println!("Creating new table {:?}", self.name);
            let query = CREATE_TABLE_QUERY.replace("$1", &self.name);
            println!("{}", query);
            self.updater.conn.query(&query, &[])?;
        } else {
            println!("Updating table {:?}", self.name);

            for row in result.iter() {
                let name: String = row.get(0);
                let old_type: String = row.get(1);

                match self.fields.entry(name) {
                    Entry::Occupied(o) => {
                        let (name, new_type) = o.remove_entry();
                        println!(
                            "Field {} already exists! (Type: {} -> {})",
                            name, old_type, new_type,
                        );
                    }
                    Entry::Vacant(e) => {
                        println!("Column {:?} found in DB but not locally, ignoring", e.key());
                    }
                }
            }
        }

        for (name, new_type) in self.fields.into_iter() {
            println!("Creating column {} ({})", name, new_type);
            let query = CREATE_COLUMN_QUERY
                .replace("$1", self.name)
                .replace("$2", &name)
                .replace("$3", new_type);
            println!("{}", query);
            self.updater.conn.query(&query, &[])?;
        }

        println!("Done updating table {}", self.name);

        Ok(())
    }
}

const TABLE_COLUMN_QUERY: &str = r#"SELECT
	column_name, data_type
FROM information_schema.columns
WHERE table_name = $1"#;

const CREATE_TABLE_QUERY: &str = "CREATE TABLE \"$1\"()";
const CREATE_COLUMN_QUERY: &str = "ALTER TABLE \"$1\" ADD COLUMN \"$2\" $3";
