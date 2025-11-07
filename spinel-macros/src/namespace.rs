use crate::parsers::set_crate_namespace;
use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};

pub fn declare_namespace_logic(input: TokenStream) -> TokenStream {
    let namespace_lit = parse_macro_input!(input as LitStr);
    set_crate_namespace(namespace_lit.value());

    TokenStream::new()
}
