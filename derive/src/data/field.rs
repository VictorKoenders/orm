use syn::spanned::Spanned;
use syn::{Error, Field as synField, Ident, Result, Type};

pub struct Field {
    pub name_string: String,
    pub name: Ident,
    pub name_upper: Ident,
    pub name_lower: Ident,
    pub name_upper_t: Ident,

    pub code_type: Type,
    pub db_type: String,
    pub db_type_attributes: Vec<String>,
}

impl Field {
    pub fn from(f: &synField) -> Result<Field> {
        let name = match f.ident.clone() {
            Some(i) => i,
            None => return Err(Error::new(f.span(), "Field needs to have a name")),
        };
        let name_string = name.to_string();
        let name_upper = Ident::new(&name_string.to_uppercase(), f.span());
        let name_lower = Ident::new(&name_string.to_lowercase(), f.span());
        let name_upper_t = Ident::new(&format!("T{}", name_string.to_uppercase()), f.span());

        let code_type = f.ty.clone();

        // TODO: Figure out the field type and attributes
        let db_type = "TEXT".to_owned();
        let db_type_attributes = vec!["NOT NULL".to_owned()];

        Ok(Field {
            name_string,
            name,
            name_upper,
            name_lower,
            name_upper_t,

            code_type,
            db_type,
            db_type_attributes,
        })
    }
}

/*
let type_ = &field.ty;
    let mut db_type = get_database_type_from_field_type(type_)?;
    if is_identifier(field, fields) {
        if db_type != "UUID" {
            db_type = String::from("SERIAL PRIMARY KEY");
        } else {
            db_type += " PRIMARY KEY DEFAULT(uuid_generate_v4())";
        }
    }
*/
