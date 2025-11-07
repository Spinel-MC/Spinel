use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
};

struct ModuleListParser {
    modules: Punctuated<LitStr, Token![,]>,
}

impl Parse for ModuleListParser {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            modules: Punctuated::parse_terminated(input)?,
        })
    }
}

pub fn import_module_logic(input: TokenStream) -> TokenStream {
    let parser = parse_macro_input!(input as ModuleListParser);
    let mut expanded = quote! {};

    for (i, module_lit) in parser.modules.iter().enumerate() {
        let module_str = module_lit.value();

        let sanitized_name = module_str.replace(":", "_").to_uppercase();
        let static_name = format_ident!("__SPINEL_MODULE_REGISTRATION_{}_{}", sanitized_name, i);

        expanded.extend(quote! {
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static #static_name: spinel::internal::RegisteredModule =
                spinel::internal::RegisteredModule {
                    name: #module_lit,
                };
            inventory::submit!(&#static_name);
        });
    }

    expanded.into()
}
