mod decode;
mod encode;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Type};

pub struct PacketCodecGenerator;

impl PacketCodecGenerator {
    pub fn generate_encode_logic(
        field_type: &Type,
        access_expression: TokenStream2,
        state_name: Option<&str>,
        raw_type_hint: Option<&str>,
    ) -> syn::Result<TokenStream2> {
        encode::PacketEncodeLogic::generate(
            field_type,
            access_expression,
            state_name,
            raw_type_hint,
        )
    }

    pub fn generate_decode_logic(
        field_name: &Ident,
        field_type: &Type,
        state_name: Option<&str>,
        raw_type_hint: Option<&str>,
    ) -> syn::Result<TokenStream2> {
        decode::PacketDecodeLogic::generate(field_name, field_type, state_name, raw_type_hint)
    }

    fn is_var_int(type_name: &str, raw_type_hint: Option<&str>) -> bool {
        raw_type_hint.is_some_and(|hint| hint == "VarInt" || hint == "int") || type_name == "VarInt"
    }

    fn is_var_long(type_name: &str, raw_type_hint: Option<&str>) -> bool {
        raw_type_hint.is_some_and(|hint| hint == "VarLong" || hint == "long")
            || type_name == "VarLong"
    }

    fn is_var_int_wrapper(type_name: &str) -> bool {
        type_name.ends_with("VarIntWrapper")
    }

    fn is_var_long_wrapper(type_name: &str) -> bool {
        type_name.ends_with("VarLongWrapper")
    }

    fn is_text_component(type_name: &str) -> bool {
        type_name.contains("TextComponent")
            && !type_name.contains("Json")
            && !type_name.contains("Nbt")
    }

    fn text_component_wrapper(state_name: Option<&str>) -> TokenStream2 {
        let network_path = crate::common::utils::get_base_path("network");

        match state_name {
            Some("Play") | Some("Configuration") => {
                quote! { #network_path::wrappers::NbtTextComponent }
            }
            _ => quote! { #network_path::wrappers::JsonTextComponent },
        }
    }
}
