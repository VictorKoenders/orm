use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ItemStruct, Result};

pub fn generate_implementation(obj: &ItemStruct) -> Result<TokenStream> {
    let mut fields = Vec::new();
    for field in obj.fields.iter() {
        let ident = match field.ident.as_ref() {
            Some(i) => i,
            None => return Err(syn::Error::new(field.span(), "Fields need to be named")),
        };
        fields.push(ident.clone());
    }

    let ident = obj.ident.clone();
    let fields_2 = fields.clone();

    let result = quote! {
        impl #ident {
            pub fn new(url: &str) -> orm::Result<#ident> {
                let context = orm::InnerContext::new(url)?;
                let conn = context.pool.get()?;

                let mut transaction = conn.transaction()?;
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
