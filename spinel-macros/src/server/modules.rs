use crate::common::parsers::ModuleListParser;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

pub fn server_module_logic(input: TokenStream) -> TokenStream {
    let parser = parse_macro_input!(input as ModuleListParser);
    let mut expanded = quote! {};

    for (i, module_lit) in parser.modules.iter().enumerate() {
        let module_str = module_lit.value();

        let sanitized_name = module_str.replace(":", "_").to_uppercase();
        let static_name = format_ident!(
            "__SPINEL_SERVER_MODULE_REGISTRATION_{}_{}",
            sanitized_name,
            i
        );

        expanded.extend(quote! {
            #[cfg(feature = "server")]
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static #static_name: spinel::internal::RegisteredServerModule =
                spinel::internal::RegisteredServerModule {
                    name: #module_lit,
                };
            #[cfg(feature = "server")]
            inventory::submit!(&#static_name);
        });
    }

    expanded.into()
}
