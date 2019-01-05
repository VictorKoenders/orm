#![recursion_limit = "1024"]

extern crate proc_macro;

mod table;
mod context;

use proc_macro::TokenStream;

#[proc_macro_derive(Table)]
pub fn derive_table(item: TokenStream) -> TokenStream {
    let obj: syn::ItemStruct = syn::parse(item).expect("Could not parse table as a valid struct");
    crate::table::generate_implementation(&obj).expect("Could not generate table implementation")
}

#[proc_macro_derive(Context)]
pub fn derive_context(item: TokenStream) -> TokenStream {
    let obj: syn::ItemStruct = syn::parse(item).expect("Could not parse table as a valid struct");
    crate::context::generate_implementation(&obj).expect("Could not generate context implementation")
}
