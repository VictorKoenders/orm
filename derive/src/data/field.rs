use syn::spanned::Spanned;
use syn::{Error, Field as synField, Ident, Result, Type};

pub struct Field {
    pub name_string: String,
    pub name: Ident,
    pub name_upper: Ident,
    pub name_lower: Ident,
    pub name_upper_t: Ident,

    pub code_type: Type,
    pub db_type: Type,
    pub db_type_attributes: Vec<Ident>,
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

        let mut db_type = code_type.clone();
        let mut db_type_attributes = vec![
        ];

        if let Some(new_db_type) = get_inner_option_type(&db_type) {
            db_type = new_db_type.clone();
            db_type_attributes.push(
                Ident::new("Null", f.span())
            );
        } else {
            db_type_attributes.push(
                Ident::new("NotNull", f.span())
            );
        }

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

fn get_inner_option_type(ty: &Type) -> Option<&Type> {
    let path = if let Type::Path(p) = ty {
        p
    } else {
        return None;
    };

    let first_segment = path.path.segments.iter().next()?;
    if first_segment.ident != "Option" { return None; }
    let args = if let syn::PathArguments::AngleBracketed(a) = &first_segment.arguments {
        &a.args
    } else {
        return None;
    };

    let first_arg = args.iter().next()?;

    match first_arg {
        syn::GenericArgument::Type(t) => Some(t),
        _ => None
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
