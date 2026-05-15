use crate::common::utils::get_base_path;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Expr, GenericArgument, PathArguments, Type, TypePath};

pub fn resolve_enum_from_expr(expression: Expr) -> syn::Result<TokenStream2> {
    let network_path = get_base_path("network");
    let Expr::Path(expression_path) = expression else {
        return Err(syn::Error::new_spanned(
            expression,
            "attribute must be an enum path",
        ));
    };

    let Some(path_segment) = expression_path.path.segments.last() else {
        return Err(syn::Error::new_spanned(
            expression_path,
            "attribute path must contain a variant",
        ));
    };

    let variant_identifier = &path_segment.ident;
    let variant_name = variant_identifier.to_string();

    Ok(match variant_name.as_str() {
        "Handshaking" | "Status" | "Login" | "Configuration" | "Play" => {
            quote! { #network_path::ConnectionState::#variant_identifier }
        }
        "Client" | "Server" => quote! { #network_path::Recipient::#variant_identifier },
        _ => quote! { #expression_path },
    })
}

pub fn get_inner_type(type_path: &TypePath) -> syn::Result<Type> {
    let Some(last_segment) = type_path.path.segments.last() else {
        return Err(syn::Error::new_spanned(
            type_path,
            "generic type path is missing a segment",
        ));
    };

    let PathArguments::AngleBracketed(arguments) = &last_segment.arguments else {
        return Err(syn::Error::new_spanned(
            type_path,
            "generic type requires angle-bracketed arguments",
        ));
    };

    let Some(GenericArgument::Type(inner_type)) = arguments.args.first() else {
        return Err(syn::Error::new_spanned(
            arguments,
            "generic type requires an inner type argument",
        ));
    };

    Ok(inner_type.clone())
}
