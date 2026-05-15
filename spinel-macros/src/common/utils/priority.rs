use crate::common::utils::get_base_path;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Expr;

pub fn resolve_priority_token(
    priority_expression: Option<Expr>,
    default_variant: &str,
) -> TokenStream2 {
    match priority_expression {
        Some(Expr::Path(priority_path)) => quote! { #priority_path },
        Some(invalid_expression) => syn::Error::new_spanned(
            invalid_expression,
            "`priority` attribute must be an enum path",
        )
        .to_compile_error(),
        None => default_priority_token(default_variant),
    }
}

fn default_priority_token(default_variant: &str) -> TokenStream2 {
    let utils_path = get_base_path("utils");
    let priority_variant = format_ident!("{}", default_variant);
    quote! { #utils_path::Priority::#priority_variant }
}
