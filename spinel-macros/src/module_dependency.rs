use crate::parsers::get_crate_namespace;
use crate::util::resolve_id;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    LitStr, Token, parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
};

struct ModuleDependencyParser {
    subject_module: LitStr,
    _comma: Token![,],
    dependencies: Punctuated<LitStr, Token![,]>,
}

impl Parse for ModuleDependencyParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let subject_module: LitStr = input.parse()?;
        let _comma: Token![,] = input.parse()?;

        let mut dependencies = Punctuated::new();
        if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            dependencies = content.parse_terminated(Parse::parse, Token![,])?;
        } else {
            dependencies.push(input.parse()?);
        }

        Ok(Self {
            subject_module,
            _comma,
            dependencies,
        })
    }
}

pub fn declare_module_dependency_logic(input: TokenStream) -> TokenStream {
    let parser = parse_macro_input!(input as ModuleDependencyParser);
    let mut expanded = quote! {};

    let subject_module_str = parser.subject_module.value();

    let crate_namespace = get_crate_namespace();
    let qualified_subject = resolve_id(&subject_module_str, &crate_namespace);

    for (i, dep_lit) in parser.dependencies.iter().enumerate() {
        let dep_str = dep_lit.value();
        let qualified_dependency = resolve_id(&dep_str, &crate_namespace);

        let sanitized_subject = qualified_subject.replace(":", "_").to_uppercase();
        let static_name = format_ident!("__SPINEL_MODULE_DEPENDENCY_{}_{}", sanitized_subject, i);

        expanded.extend(quote! {
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static #static_name: spinel::internal::RegisteredModuleDependency =
                spinel::internal::RegisteredModuleDependency {
                    subject_module: #qualified_subject,
                    dependent_on: #qualified_dependency,
                };
            inventory::submit!(&#static_name);
        });
    }

    expanded.into()
}
