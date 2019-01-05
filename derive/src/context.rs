use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{GenericArgument, Ident, ItemStruct, PathArguments, Result, Type};

pub fn generate_implementation(obj: &ItemStruct) -> Result<TokenStream> {
    let mut fields = Vec::new();
    let mut field_types = Vec::new();
    for field in obj.fields.iter() {
        let ident = match field.ident.as_ref() {
            Some(i) => i,
            None => return Err(syn::Error::new(field.span(), "Fields need to be named")),
        };

        let inner = match get_dbset_inner_type(&field.ty) {
            Some(i) => i,
            None => {
                return Err(syn::Error::new(
                    field.span(),
                    "Fields need to be of type DbSet<T>",
                ));
            }
        };

        fields.push(ident.clone());
        field_types.push(inner);
    }

    let ident = obj.ident.clone();
    let fields_2 = fields.clone();

    let result = quote! {
        impl #ident {
            pub fn new(url: &str) -> orm::Result<#ident> {
                let context = orm::InnerContext::new(url)?;
                let conn = context.pool.get()?;

                let mut transaction = conn.transaction()?;
                #(
                    <#field_types as orm::Table>::update_database_schema(&mut orm::TableUpdater {
                        conn: &mut transaction,
                    })?;
                )*
                transaction.commit()?;

                Ok(#ident {
                    #(
                        #fields_2: orm::DbSet::__new(context.clone()),
                    )*
                })
            }
        }
    };

    Ok(result.into())
}

fn get_dbset_inner_type(ty: &Type) -> Option<&Ident> {
    let path = match ty {
        Type::Path(p) => p,
        _ => {
            return None;
        }
    };
    let first = path.path.segments.first();
    let segment = match &first {
        Some(pair) => pair.value(),
        _ => {
            return None;
        }
    };

    if segment.ident != "DbSet" {
        return None;
    }

    let args = match &segment.arguments {
        PathArguments::AngleBracketed(b) => b,
        _ => return None,
    };

    let first = args.args.first();

    let first = match &first {
        Some(f) => f.value(),
        None => return None,
    };

    let path = match &first {
        GenericArgument::Type(Type::Path(path)) => path,
        _ => return None,
    };

    let first = path.path.segments.first();

    let segment = match &first {
        Some(segment) => segment.value(),
        _ => return None,
    };

    Some(&segment.ident)
}
