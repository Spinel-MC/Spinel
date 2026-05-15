use crate::common::utils::get_base_path;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Type;

pub fn map_type_to_storage_type(type_name: &str) -> syn::Result<TokenStream2> {
    let utils_path = get_base_path("utils");

    match type_name {
        "JsonTextComponent" | "TextComponent" | "NbtTextComponent" | "Component" => {
            Ok(quote! { #utils_path::component::text::TextComponent })
        }
        _ => map_type_to_rust_type(type_name, None),
    }
}

pub fn map_type_to_rust_type(
    type_name: &str,
    state_name: Option<&str>,
) -> syn::Result<TokenStream2> {
    let network_path = get_base_path("network");
    let nbt_path = get_base_path("nbt");

    let mapped_type = match type_name {
        "bool" | "boolean" | "Boolean" => quote! { bool },
        "i8" | "byte" | "Byte" => quote! { i8 },
        "u8" | "UnsignedByte" => quote! { u8 },
        "i16" | "short" | "Short" => quote! { i16 },
        "u16" | "UnsignedShort" => quote! { u16 },
        "i32" | "int" | "Int" | "Integer" | "VarInt" | "int32" => quote! { i32 },
        "i64" | "long" | "Long" | "VarLong" | "int64" => quote! { i64 },
        "f32" | "float" | "Float" => quote! { f32 },
        "f64" | "double" | "Double" => quote! { f64 },
        "String" => quote! { String },
        "UUID" | "Uuid" => quote! { ::uuid::Uuid },
        "Position" | "BlockPos" => quote! { #network_path::types::Position },
        "ChunkPos" => quote! { #network_path::types::ChunkPos },
        "GlobalPos" => quote! { #network_path::types::GlobalPos },
        "Vector3f" => quote! { #network_path::types::Vector3f },
        "Quaternion" => quote! { #network_path::types::Quaternionf },
        "Vec3" => quote! { #network_path::types::Vector3d },
        "LpVec3" => quote! { #network_path::types::Velocity },
        "BlockHitResult" => quote! { #network_path::types::BlockHitResult },
        "Instant" => quote! { #network_path::types::NetworkInstant },
        "Slot" | "ItemStack" | "HashedStack" => quote! { #network_path::types::Slot },
        "Identifier" | "ResourceLocation" | "ResourceKey" | "Holder" | "Level" | "Registry"
        | "RegistryKey" => quote! { #network_path::types::Identifier },
        "RegistryValue" | "Enum" => quote! { i32 },
        "BitSet" | "FixedBitSet" => quote! { #network_path::types::BitSet },
        "ByteArray" => quote! { #network_path::types::ByteArray },
        "VarIntArray" => quote! { #network_path::types::VarIntArray },
        "LongArray" => quote! { #network_path::types::LongArray },
        "PublicKey" => quote! { #network_path::types::PublicKey },
        "ContainerId" => quote! { #network_path::types::ContainerId },
        "ClientInformation" => quote! { #network_path::types::ClientInformation },
        "unknown" => quote! { ::std::vec::Vec<u8> },
        "JsonTextComponent" | "TextComponent" | "Component" => text_component_type(state_name),
        "NbtTextComponent" => quote! { #network_path::wrappers::NbtTextComponent },
        "NBT" | "NbtCompound" | "Tag" | "CompoundTag" => quote! { #nbt_path::NbtCompound },
        "PackedRegistryEntry" => {
            quote! { (#network_path::types::Identifier, ::std::option::Option<#nbt_path::NbtCompound>) }
        }
        "NetworkPayload" => {
            quote! { ::std::collections::HashMap<String, #network_path::types::IntList> }
        }
        _ => return map_complex_type(type_name, state_name),
    };

    Ok(mapped_type)
}

fn text_component_type(state_name: Option<&str>) -> TokenStream2 {
    let network_path = get_base_path("network");

    match state_name {
        Some("Play") | Some("Configuration") => {
            quote! { #network_path::wrappers::NbtTextComponent }
        }
        _ => quote! { #network_path::wrappers::JsonTextComponent },
    }
}

fn map_complex_type(type_name: &str, state_name: Option<&str>) -> syn::Result<TokenStream2> {
    if type_name.contains('<') || type_name.contains("Content") || type_name.contains('(') {
        return map_json_string_to_rust_type(type_name, state_name);
    }

    if type_name
        .chars()
        .all(|character| character.is_alphanumeric() || character == '_')
    {
        let type_identifier = format_ident!("{}", type_name);
        return Ok(quote! { #type_identifier });
    }

    let parsed_type: Type = syn::parse_str(type_name).map_err(|_| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("failed to parse type string: {}", type_name),
        )
    })?;
    Ok(quote! { #parsed_type })
}

fn map_json_string_to_rust_type(
    type_name: &str,
    state_name: Option<&str>,
) -> syn::Result<TokenStream2> {
    let trimmed_type_name = type_name.trim();

    if trimmed_type_name.starts_with('(') && trimmed_type_name.ends_with(')') {
        return map_tuple_type(
            &trimmed_type_name[1..trimmed_type_name.len() - 1],
            state_name,
        );
    }

    if let Some(generic_start) = trimmed_type_name.find('<') {
        return map_generic_type(trimmed_type_name, generic_start, state_name);
    }

    map_type_to_rust_type(type_name, state_name)
}

fn map_tuple_type(tuple_body: &str, state_name: Option<&str>) -> syn::Result<TokenStream2> {
    let inner_types = split_comma_separated(tuple_body)
        .iter()
        .map(|type_name| map_json_string_to_rust_type(type_name, state_name))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { ( #(#inner_types),* ) })
}

fn map_generic_type(
    type_name: &str,
    generic_start: usize,
    state_name: Option<&str>,
) -> syn::Result<TokenStream2> {
    let Some(generic_end) = type_name.rfind('>') else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("generic type is missing closing bracket: {}", type_name),
        ));
    };

    let outer_type_name = type_name[..generic_start].trim();
    let inner_type_names = split_comma_separated(type_name[generic_start + 1..generic_end].trim());
    let inner_types = inner_type_names
        .iter()
        .map(|inner_type_name| map_json_string_to_rust_type(inner_type_name, state_name))
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(match outer_type_name {
        "Array" | "List" | "Collection" | "Set" | "PrefixedArray" => {
            let inner_type = &inner_types[0];
            let network_path = get_base_path("network");
            quote! { #network_path::types::Array<#inner_type> }
        }
        "Optional" | "Option" | "Nullable" => {
            let inner_type = &inner_types[0];
            quote! { ::std::option::Option<#inner_type> }
        }
        "Map" => {
            let key_type = &inner_types[0];
            let value_type = &inner_types[1];
            quote! { ::std::collections::HashMap<#key_type, #value_type> }
        }
        "RegistryValue" | "Enum" => quote! { i32 },
        "EnumSet" => {
            let inner_type = &inner_types[0];
            let network_path = get_base_path("network");
            quote! { #network_path::types::Array<#inner_type> }
        }
        "Either" => {
            let left_type = &inner_types[0];
            let right_type = &inner_types[1];
            quote! { (#left_type, #right_type) }
        }
        "ResourceKey" | "Holder" | "Registry" | "RegistryKey" | "Level" => {
            let network_path = get_base_path("network");
            quote! { #network_path::types::Identifier }
        }
        _ => {
            let outer_identifier = format_ident!("{}", outer_type_name);
            quote! { #outer_identifier<#(#inner_types),*> }
        }
    })
}

fn split_comma_separated(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut nesting_depth = 0;
    let mut current_part = String::new();

    for character in input.chars() {
        match character {
            '<' | '(' => {
                nesting_depth += 1;
                current_part.push(character);
            }
            '>' | ')' => {
                nesting_depth -= 1;
                current_part.push(character);
            }
            ',' if nesting_depth == 0 => {
                parts.push(current_part.trim().to_string());
                current_part.clear();
            }
            _ => current_part.push(character),
        }
    }

    if !current_part.is_empty() {
        parts.push(current_part.trim().to_string());
    }

    parts
}
