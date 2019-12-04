use proc_macro2::Span;
use quote::{ToTokens, __rt::TokenStream, format_ident, quote};
use syn::{DeriveInput, Ident};

pub fn build(input: DeriveInput) -> TokenStream {
    let struct_data = match &input.data {
        syn::Data::Struct(s) => s,
        _ => unimplemented!("OrmTable is not derivable for this type"),
    };

    let fields = match &struct_data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => unimplemented!("OrmTable only supports structs with named fields"),
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
    let fields = fields.iter().collect::<Vec<_>>();
    let (filter_ident, filter_impl) = build_filter(format!("__{}Filter", ident), &fields);

    (quote! {
        impl OrmTable for #ident {
            type Filter = #filter_ident;
        }
        #filter_impl
    })
    .into()
}

fn build_filter<'a>(name: String, fields: &[&syn::Field]) -> (TokenStream, TokenStream) {
    let ident = Ident::new(&name, Span::call_site());
    let field_impls = fields
        .iter()
        .enumerate()
        .map(|(i, f)| build_filter_field_impl(fields, &ident, i, f));
    let generics = fields.iter().map(|f| f.ident.to_upper());
    let generics = quote! { <#(#generics), *>};
    let empty_generics = fields.iter().map(|f| quote! { () });
    let empty_generics = quote! { <#(#empty_generics), *>};
    let fields = fields.iter().map(|f| {
        let name = &f.ident;
        let type_name = f.ident.to_upper();
        quote! {
        #name: #type_name,
        }
    });

    let filter_impl = quote! {
        struct #ident #generics { #(#fields)* }

        #(#field_impls)*
    };
    let filter_type = quote! { #ident #empty_generics };
    (filter_type, filter_impl)
}

trait IdentExt {
    fn to_upper(&self) -> Ident;
}
impl IdentExt for Option<Ident> {
    fn to_upper(&self) -> Ident { 
        self.as_ref().unwrap().to_upper()
    }
}
impl IdentExt for Ident {
    fn to_upper(&self) -> Ident { 
        let str = self.to_string().to_ascii_uppercase().replace("_", "");
        format_ident!("{}", str)
    }
}

fn build_filter_field_impl(
    fields: &[&syn::Field],
    filter_ident: &Ident,
    index: usize,
    field: &syn::Field,
) -> TokenStream {
    let other_generics = fields
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, f)| f.ident.to_upper());

    let generics = fields.iter().enumerate().map(|(i, f)| {
        if i != index {
            let ident = f.ident.to_upper();
            quote! { #ident }
        } else {
            quote! { () }
        }
    });
    let field_ident = &field.ident;
    quote! {
        impl <#(#other_generics),*> #filter_ident <#(#generics),*> {
            pub fn #field_ident(self) -> QuerySegment {
                unimplemented!()
            }
        }
    }
}
