use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Expr, GenericArgument, PathArguments, Type, TypePath};

pub fn get_base_path(module: &str) -> TokenStream2 {
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
    let is_internal = pkg_name == "spinel" || pkg_name.starts_with("spinel-");

    if !is_internal {
        return match module {
            "events" => quote!(::spinel::events),
            "network" => quote!(::spinel::network),
            "core" => quote!(::spinel::core),
            "server" => quote!(::spinel::server),
            "client" => quote!(::spinel::client),
            "nbt" => quote!(::spinel::nbt),
            "utils" => quote!(::spinel::utils),
            _ => quote!(::spinel),
        };
    }

    match (pkg_name.as_str(), module) {
        ("spinel-server", "server") => quote!(crate),
        ("spinel-client", "client") => quote!(crate),
        ("spinel-network", "network") => quote!(crate),
        ("spinel-events", "events") => quote!(crate),
        ("spinel-core", "core") => quote!(crate),
        ("spinel-utils", "utils") => quote!(crate),
        ("spinel-nbt", "nbt") => quote!(crate),

        (_, "events") => quote!(::spinel_events),
        (_, "network") => quote!(::spinel_network),
        (_, "core") => quote!(::spinel_core),
        (_, "server") => quote!(::spinel_server),
        (_, "client") => quote!(::spinel_client),
        (_, "nbt") => quote!(::spinel_nbt),
        (_, "utils") => quote!(::spinel_utils),
        _ => quote!(::spinel),
    }
}

pub fn resolve_id(raw_id: &str, default_namespace: &str) -> String {
    if raw_id.contains(':') {
        raw_id.to_string()
    } else {
        format!("{}:{}", default_namespace, raw_id)
    }
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

pub fn resolve_priority_token(
    priority_expr: Option<Expr>,
    _enum_name: &str,
    default_variant: &str,
) -> TokenStream2 {
    match priority_expr {
        Some(expr) => match expr {
            Expr::Path(_) => quote! { #expr },
            _ => panic!("`priority` attribute must be an enum path"),
        },
        None => {
            let path = get_base_path("utils");
            let variant = format_ident!("{}", default_variant);
            quote! { #path::Priority::#variant }
        }
    }
}

pub fn map_type_to_rust_type(ty_str: &str, state: Option<&str>) -> TokenStream2 {
    let net = get_base_path("network");
    let nbt = get_base_path("nbt");

    match ty_str {
        "bool" | "boolean" | "Boolean" => quote! { bool },
        "i8" | "byte" | "Byte" => quote! { i8 },
        "u8" => quote! { u8 },
        "i16" | "short" | "Short" => quote! { i16 },
        "u16" => quote! { u16 },
        "i32" | "int" | "Integer" | "VarInt" | "int32" => quote! { i32 },
        "i64" | "long" | "Long" | "VarLong" | "int64" => quote! { i64 },
        "f32" | "float" | "Float" => quote! { f32 },
        "f64" | "double" | "Double" => quote! { f64 },
        "String" => quote! { String },

        "UUID" | "Uuid" => quote! { ::uuid::Uuid },
        "Position" | "BlockPos" => quote! { #net::types::Position },
        "Slot" | "ItemStack" | "HashedStack" => quote! { #net::types::Slot },
        "Identifier" | "ResourceLocation" | "ResourceKey" | "Holder" | "Level" | "Registry"
        | "RegistryKey" => quote! { #net::types::Identifier },
        "ClientInformation" => quote! { #net::types::ClientInformation },

        "JsonTextComponent" | "TextComponent" | "Component" => match state {
            Some("Play") | Some("Configuration") => {
                quote! { #net::wrappers::NbtTextComponent }
            }
            _ => {
                quote! { #net::wrappers::JsonTextComponent }
            }
        },
        "NbtTextComponent" => quote! { #net::wrappers::NbtTextComponent },
        "NbtCompound" | "Tag" | "CompoundTag" => quote! { #nbt::NbtCompound },

        "PackedRegistryEntry" => {
            quote! { (#net::types::Identifier, ::std::option::Option<#nbt::NbtCompound>) }
        }
        "NetworkPayload" => {
            quote! { ::std::collections::HashMap<String, #net::types::IntList> }
        }

        _ => {
            if ty_str.contains('<') || ty_str.contains("Content") || ty_str.contains('(') {
                return map_json_string_to_rust_type(ty_str, state);
            }

            if ty_str.chars().all(|c| c.is_alphanumeric() || c == '_') {
                let ident = format_ident!("{}", ty_str);
                quote! { #ident }
            } else {
                let parsed: Type = syn::parse_str(ty_str)
                    .unwrap_or_else(|_| panic!("Failed to parse type string: {}", ty_str));
                quote! { #parsed }
            }
        }
    }
}

pub fn map_json_string_to_rust_type(ty_str: &str, state: Option<&str>) -> TokenStream2 {
    let ty_trimmed = ty_str.trim();

    if ty_trimmed.starts_with('(') && ty_trimmed.ends_with(')') {
        let content = &ty_trimmed[1..ty_trimmed.len() - 1];
        let parts = parse_comma_separated(content);
        let inner_rusts: Vec<TokenStream2> = parts
            .iter()
            .map(|p| map_json_string_to_rust_type(p, state))
            .collect();
        return quote! { ( #(#inner_rusts),* ) };
    }

    if let Some(start) = ty_str.find('<') {
        if let Some(end) = ty_str.rfind('>') {
            let outer = &ty_str[..start].trim();
            let inner_raw = &ty_str[start + 1..end].trim();
            let net = get_base_path("network");

            let inner_parts = parse_comma_separated(inner_raw);
            let inner_rusts: Vec<TokenStream2> = inner_parts
                .iter()
                .map(|p| map_json_string_to_rust_type(p, state))
                .collect();

            return match *outer {
                "Array" | "List" | "Collection" | "Set" => {
                    let inner = &inner_rusts[0];
                    quote! { #net::types::Array<#inner> }
                }
                "Optional" | "Option" => {
                    let inner = &inner_rusts[0];
                    quote! { ::std::option::Option<#inner> }
                }
                "Map" => {
                    let key = &inner_rusts[0];
                    let val = &inner_rusts[1];
                    quote! { ::std::collections::HashMap<#key, #val> }
                }
                "ResourceKey" | "Holder" | "Registry" | "RegistryKey" | "Level" => {
                    quote! { #net::types::Identifier }
                }
                _ => {
                    let outer_ident = format_ident!("{}", outer);
                    quote! { #outer_ident<#(#inner_rusts),*> }
                }
            };
        }
    }
    map_type_to_rust_type(ty_str, state)
}

fn parse_comma_separated(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut current = String::new();
    for c in input.chars() {
        match c {
            '<' | '(' => {
                depth += 1;
                current.push(c);
            }
            '>' | ')' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                parts.push(current.trim().to_string());
                current = String::new();
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

pub fn map_type_to_storage_type(ty_str: &str) -> TokenStream2 {
    let utils = get_base_path("utils");
    match ty_str {
        "JsonTextComponent" | "TextComponent" | "NbtTextComponent" | "Component" => {
            quote! { #utils::component::text::TextComponent }
        }
        _ => map_type_to_rust_type(ty_str, None),
    }
}

pub fn resolve_enum_from_expr(
    expr: Expr,
    _enum_full_path: &str,
    _enum_name_for_panic: &str,
) -> TokenStream2 {
    let net = get_base_path("network");
    match expr {
        Expr::Path(expr_path) => {
            let segment = expr_path.path.segments.last().unwrap();
            let ident = &segment.ident;
            let ident_str = ident.to_string();

            match ident_str.as_str() {
                "Handshaking" | "Status" | "Login" | "Configuration" | "Play" => {
                    quote! { #net::ConnectionState::#ident }
                }
                "Client" | "Server" => {
                    quote! { #net::Recipient::#ident }
                }
                _ => {
                    quote! { #expr_path }
                }
            }
        }
        _ => panic!("Attribute must be an enum path (e.g., ConnectionState::Play)"),
    }
}

pub fn get_inner_type(ty_path: &TypePath) -> Option<Type> {
    if let PathArguments::AngleBracketed(args) = &ty_path.path.segments.last().unwrap().arguments {
        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
            return Some(inner_ty.clone());
        }
    }
    None
}
