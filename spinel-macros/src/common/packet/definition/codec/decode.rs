use crate::common::packet::definition::codec::PacketCodecGenerator;
use crate::common::utils::{get_base_path, get_inner_type};
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type};

pub struct PacketDecodeLogic;

impl PacketDecodeLogic {
    pub fn generate(
        field_name: &Ident,
        field_type: &Type,
        state_name: Option<&str>,
        raw_type_hint: Option<&str>,
    ) -> syn::Result<TokenStream2> {
        let type_name = field_type.to_token_stream().to_string();
        let network_path = get_base_path("network");

        if PacketCodecGenerator::is_var_int_wrapper(&type_name) {
            return Ok(quote! {
                let #field_name: #field_type = <#network_path::types::var_int::VarIntWrapper as #network_path::DataType>::decode(connection)?;
            });
        }

        if PacketCodecGenerator::is_var_int(&type_name, raw_type_hint) {
            return Ok(quote! {
                let #field_name: #field_type = <#network_path::types::var_int::VarIntWrapper as #network_path::DataType>::decode(connection)
                    .map(|value: #network_path::types::var_int::VarIntWrapper| value.0 as #field_type)?;
            });
        }

        if PacketCodecGenerator::is_var_long_wrapper(&type_name) {
            return Ok(quote! {
                let #field_name: #field_type = <#network_path::types::var_long::VarLongWrapper as #network_path::DataType>::decode(connection)?;
            });
        }

        if PacketCodecGenerator::is_var_long(&type_name, raw_type_hint) {
            return Ok(quote! {
                let #field_name: #field_type = <#network_path::types::var_long::VarLongWrapper as #network_path::DataType>::decode(connection)
                    .map(|value: #network_path::types::var_long::VarLongWrapper| value.0 as #field_type)?;
            });
        }

        if PacketCodecGenerator::is_text_component(&type_name) {
            let wrapper_type = PacketCodecGenerator::text_component_wrapper(state_name);
            return Ok(quote! {
                let #field_name: #field_type = <#wrapper_type as #network_path::DataType>::decode(connection)
                    .map(|value: #wrapper_type| value.0 as #field_type)?;
            });
        }

        if let Some(collection_logic) = Self::collection_logic(field_name, field_type, state_name) {
            return collection_logic;
        }

        Ok(
            quote! { let #field_name: #field_type = <#field_type as #network_path::DataType>::decode(connection)?; },
        )
    }

    fn collection_logic(
        field_name: &Ident,
        field_type: &Type,
        state_name: Option<&str>,
    ) -> Option<syn::Result<TokenStream2>> {
        let Type::Path(type_path) = field_type else {
            return None;
        };
        let last_segment = type_path.path.segments.last()?;
        let type_name = last_segment.ident.to_string();
        let network_path = get_base_path("network");

        if type_name == "Option" || type_name == "Optional" {
            return Some(get_inner_type(type_path).and_then(|inner_type| {
                let inner_logic =
                    PacketCodecGenerator::generate_decode_logic(field_name, &inner_type, state_name, None)?;
                Ok(quote! {
                    let #field_name: #field_type = if <bool as #network_path::DataType>::decode(connection)? {
                        #inner_logic
                        Some(#field_name)
                    } else {
                        None
                    };
                })
            }));
        }

        if type_name == "Vec" || type_name == "Array" {
            return Some(get_inner_type(type_path).and_then(|inner_type| {
                let inner_logic = PacketCodecGenerator::generate_decode_logic(
                    &format_ident!("item"),
                    &inner_type,
                    state_name,
                    None,
                )?;
                let constructor = if type_name == "Array" {
                    quote! { #network_path::types::Array(items) }
                } else {
                    quote! { items }
                };
                Ok(quote! {
                    let len = <#network_path::types::var_int::VarIntWrapper as #network_path::DataType>::decode(connection)?.0 as usize;
                    let mut items = Vec::with_capacity(len);
                    for _ in 0..len {
                        #inner_logic
                        items.push(item);
                    }
                    let #field_name: #field_type = #constructor;
                })
            }));
        }

        None
    }
}
