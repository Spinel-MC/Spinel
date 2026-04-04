use crate::common::parsers::ModuleListParser;
use crate::common::utils::get_base_path;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

pub fn import_module_logic(input: TokenStream) -> TokenStream {
    let parser = parse_macro_input!(input as ModuleListParser);
    let mut expanded = quote! {};

    let event_path = get_base_path("events");

    for (i, module_lit) in parser.modules.iter().enumerate() {
        let module_str = module_lit.value();
        let sanitized_name = module_str.replace(":", "_").to_uppercase();

        let server_static_name = format_ident!(
            "__SPINEL_SERVER_MODULE_REGISTRATION_{}_{}",
            sanitized_name,
            i
        );
        let client_static_name = format_ident!(
            "__SPINEL_CLIENT_MODULE_REGISTRATION_{}_{}",
            sanitized_name,
            i
        );

        expanded.extend(quote! {
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static #server_static_name: #event_path::RegisteredServerModule =
                #event_path::RegisteredServerModule {
                    name: #module_lit,
                };
            #event_path::inventory::submit!(&#server_static_name);

            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static #client_static_name: #event_path::RegisteredClientModule =
                #event_path::RegisteredClientModule {
                    name: #module_lit,
                };
            #event_path::inventory::submit!(&#client_static_name);
        });
    }

    expanded.into()
}
