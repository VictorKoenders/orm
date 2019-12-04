extern crate proc_macro;

use syn::export::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod context;
mod table;

#[proc_macro_derive(OrmTable)]
pub fn orm_table_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let result = crate::table::build(input);
    // panic!("{}", result);
    result.into()
}

#[proc_macro_derive(OrmContext)]
pub fn orm_context_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    crate::context::build(input)
}
