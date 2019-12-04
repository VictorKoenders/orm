use quote::{quote, ToTokens};
use syn::export::TokenStream;
use syn::DeriveInput;

pub fn build(input: DeriveInput) -> TokenStream {
    let struct_data = match &input.data {
        syn::Data::Struct(s) => s,
        _ => unimplemented!("OrmContext is not derivable for this type"),
    };

    let fields = match &struct_data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => unimplemented!("OrmContext only supports structs with named fields"),
    };

    /*
    let mut s = format!("Found struct {} with {} fields", input.ident, fields.len());
    for field in fields.iter() {
        s += &format!(
            "\n - {} ({})",
            field.ident.as_ref().unwrap(),
            field.ty.to_token_stream()
        );
    }
    */

    let ident = input.ident;

    (quote! {
        impl OrmContext for #ident {
        }
    })
    .into()
}
