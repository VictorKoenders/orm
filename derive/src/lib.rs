#![recursion_limit = "1024"]

extern crate proc_macro;

mod context;
mod data;
mod table;

use proc_macro::TokenStream;

#[proc_macro_derive(Table, attributes(key, unique))]
pub fn derive_table(item: TokenStream) -> TokenStream {
    let obj: syn::ItemStruct = syn::parse(item).expect("Could not parse table as a valid struct");
    let table =
        crate::data::Table::from_struct(&obj).expect("Could not parse table as a valid struct");
    crate::table::generate_implementation(&table).expect("Could not generate table implementation")
}

#[proc_macro_derive(Context)]
pub fn derive_context(item: TokenStream) -> TokenStream {
    let obj: syn::ItemStruct = syn::parse(item).expect("Could not parse table as a valid struct");
    crate::context::generate_implementation(&obj)
        .expect("Could not generate context implementation")
}
