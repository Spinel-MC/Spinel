// File: spinel-macros/src/util.rs
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Type, Expr, Lit, TypePath, PathArguments, GenericArgument};

use crate::parsers::Field; 

pub fn resolve_id(raw_id: &str, default_namespace: &str) -> String {
    if raw_id.contains(':') {
        raw_id.to_string()
    } else {
        format!("{}:{}", default_namespace, raw_id)
    }
}

pub fn resolve_priority_token(priority_expr: Option<Expr>, enum_name: &str, default_variant: &str) -> TokenStream2 {
    let full_enum_path = format!("spinel::utils::{}", enum_name); 
    match priority_expr {
        Some(expr) => match expr {
            Expr::Lit(expr_lit) => {
                if let Lit::Str(lit_str) = expr_lit.lit {
                    let path_str = format!("{}::{}", full_enum_path, lit_str.value());
                    let path: syn::Path = syn::parse_str(&path_str).unwrap_or_else(|e| panic!("Invalid priority string: {} - {}", lit_str.value(), e));
                    quote! { #path }
                } else {
                    panic!("`priority` attribute must be a string literal or an enum path");
                }
            },
            Expr::Path(_) => quote! { #expr },
            _ => panic!("`priority` attribute must be a string literal or an enum path"),
        },
        None => {
            let path_str = format!("{}::{}", full_enum_path, default_variant);
            let path: syn::Path = syn::parse_str(&path_str).unwrap();
            quote! { #path }
        }
    }
}

pub fn resolve_enum_from_expr(expr: Expr, enum_full_path: &str, _enum_name_for_panic: &str) -> TokenStream2 {
    match expr {
        Expr::Lit(expr_lit) => {
            if let Lit::Str(lit_str) = expr_lit.lit {
                let path_str = format!("{}::{}", enum_full_path, lit_str.value());
                let path: syn::Path = syn::parse_str(&path_str).unwrap_or_else(|e| panic!("Invalid {} string: {} - {}", _enum_name_for_panic, lit_str.value(), e));
                quote! { #path }
            } else {
                panic!("`state` attribute must be a string literal or an enum path");
            }
        },
        Expr::Path(_) => quote! { #expr },
        _ => panic!("`state` attribute must be a string literal or an enum path"),
    }
}


// Maps the Rust Type Path (e.g., i32 or UUID) used in a struct definition 
// to the correct `NetworkBuffer` write method and whether it needs a reference.
pub fn get_write_method_for_type(ty: &Type) -> (Ident, bool) {
    let ty_path = if let Type::Path(path) = ty {
        path
    } else {
        panic!("Unsupported complex type for packet_dispatcher serialization: {}", quote!{#ty});
    };

    let segment_ident = ty_path.path.segments.last().unwrap().ident.to_string();

    let (method_name_str, needs_ref) = match segment_ident.as_str() {
        "bool" | "Bool" => ("write_bool", false),
        "i8" | "Byte" => ("write_byte", false),
        "u8" | "UnsignedByte" | "Angle" => ("write_unsigned_byte", false),
        "i16" | "Short" => ("write_short", false),
        "u16" | "UnsignedShort" => ("write_unsigned_short", false),
        "i32" | "Int" => ("write_int", false),
        "i64" | "Long" => ("write_long", false),
        "f32" | "Float" => ("write_float", false),
        "f64" | "Double" => ("write_double", false),
        "VarInt" => ("write_varint", false), 
        "VarLong" => ("write_varlong", false), 
        "String" | "Identifier" => ("write_string", true),
        "Uuid" => ("write_uuid", true),
        "TextComponent" => ("write_json_text_component", true),
        "Position" => ("write_position", true),
        "Slot" => ("write_slot", true),
        "ChunkData" => ("write_chunk_data", true),
        "LightData" => ("write_light_data", true),
        "GameProfile" => ("write_game_profile", true),
        "Array" => ("write_array_custom", true),
        "NbtCompound" => ("write_nbt_compound", true),
        "Vec" => {
            let is_u8_vec = ty_path.path.segments.last().map_or(false, |s| {
                if let syn::PathArguments::AngleBracketed(args) = &s.arguments {
                    if let Some(syn::GenericArgument::Type(Type::Path(inner_path))) = args.args.first() {
                        return inner_path.path.segments.last().map_or(false, |inner_s| inner_s.ident == "u8");
                    }
                }
                false
            });

            if is_u8_vec {
                ("write_byte_array", true)
            } else {
                ("write_array_custom", true)
            }
        }
        "Option" | "Optional" => {
             if let Some(inner_ty) = get_inner_type(ty_path) {
                return get_write_method_for_type(&inner_ty);
             } else {
                panic!("Option/Optional type requires a generic parameter.");
             }
        }
        _ => panic!("Unsupported type for packet_dispatcher serialization: {}", segment_ident),
    };
    (format_ident!("{}", method_name_str), needs_ref)
}

// Helper to extract the inner generic type `T` from `Vec<T>` or `Array<T>`.
pub fn get_inner_type(ty_path: &TypePath) -> Option<Type> {
    if let PathArguments::AngleBracketed(args) = &ty_path.path.segments.last().unwrap().arguments {
        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
            return Some(inner_ty.clone());
        }
    }
    None
}

// Maps a macro field definition (including generics) to its actual Rust type.
pub fn map_field_to_rust_type(field: &Field) -> TokenStream2 {
    let ty_str = field.ty.to_string();
    if ty_str == "Array" || ty_str == "Vec" {
        let inner_type = field.generic_param.as_ref()
            .expect("Array/Vec type requires a generic parameter, e.g., Array<Identifier>");
        
        let inner_rust_type = if let Type::Tuple(_) = inner_type {
            quote! { #inner_type }
        } else if let Type::Path(type_path) = inner_type {
             map_type_to_rust_type(&type_path.path.segments.last().unwrap().ident)
        } else {
            panic!("Unsupported generic type for Array/Vec: {}", quote!(#inner_type).to_string())
        };
        return quote! { Vec<#inner_rust_type> };
    }
    if ty_str == "Optional" {
        let inner_type = field.generic_param.as_ref()
            .expect("Optional type requires a generic parameter");
        let inner_rust_type = if let Type::Path(type_path) = inner_type {
             map_type_to_rust_type(&type_path.path.segments.last().unwrap().ident)
        } else {
             panic!("Unsupported generic type for Optional: {}", quote!(#inner_type).to_string())
        };
        return quote! { Option<#inner_rust_type> };
    }
    map_type_to_rust_type(&field.ty)
}

// Maps a simple type identifier (used in fields attribute or generics) to its Rust primitive.
pub fn map_type_to_rust_type(ty_ident: &Ident) -> TokenStream2 {
    let ty_str = ty_ident.to_string();
    match ty_str.as_str() {
        "Bool" => quote! { bool },
        "Byte" => quote! { i8 },
        "UnsignedByte" | "Angle" => quote! { u8 },
        "Short" => quote! { i16 },
        "UnsignedShort" => quote! { u16 },
        "Int" => quote! { i32 },
        "Long" => quote! { i64 },
        "Float" => quote! { f32 },
        "Double" => quote! { f64 },
        "VarInt" => quote! { i32 },
        "VarLong" => quote! { i64 },
        "String" | "Identifier" => quote! { String },
        "UUID" | "Uuid" => quote! { uuid::Uuid },
        "JsonTextComponent" | "TextComponent" => quote! { spinel::utils::component::text::TextComponent },
        "Position" => quote! { spinel::network::types::Position },
        "ByteArray" => quote! { Vec<u8> },
        "Slot" => quote! { spinel::network::types::Slot },
        _ => quote! { #ty_ident },
    }
}

// Maps a `syn::Type` to the corresponding `Client` read method name.
pub fn map_syn_type_to_decoder_method(ty: &Type) -> Ident {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return map_type_to_decoder_method_name(&segment.ident);
        }
    }
    panic!("Unsupported field type in tuple for #[packet_listener]: {}", quote!(#ty).to_string());
}

// Maps a simple type identifier to the corresponding `Client` read method name.
pub fn map_type_to_decoder_method_name(ty_ident: &Ident) -> Ident {
    let method_name = match ty_ident.to_string().as_str() {
        "Bool" | "bool" => "read_bool",
        "Byte" | "i8" => "read_byte",
        "UnsignedByte" | "u8" => "read_unsigned_byte",
        "Short" | "i16" => "read_short",
        "UnsignedShort" | "u16" => "read_u16",
        "Int" | "i32" => "read_int",
        "Long" | "i64" => "read_long",
        "Float" | "f32" => "read_float",
        "Double" | "f64" => "read_double",
        "VarInt" => "read_varint",
        "VarLong" => "read_varlong",
        "String" => "read_string",
        "Identifier" => "read_identifier",
        "UUID" => "read_uuid",
        "JsonTextComponent" | "TextComponent" => "read_json_text_component",
        "Position" => "read_position",
        "Angle" => "read_angle",
        "ByteArray" => "read_byte_array",
        "Slot" => "read_slot",
        _ => panic!("Unsupported field type in #[packet_listener]: {}", ty_ident),
    };
    format_ident!("{}", method_name)
}