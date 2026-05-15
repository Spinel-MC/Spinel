use crate::common::packet::definition::codec::PacketCodecGenerator;
use crate::common::utils::{get_base_path, get_inner_type};
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::Type;

pub struct PacketEncodeLogic;

impl PacketEncodeLogic {
    pub fn generate(
        field_type: &Type,
        access_expression: TokenStream2,
        state_name: Option<&str>,
        raw_type_hint: Option<&str>,
    ) -> syn::Result<TokenStream2> {
        let type_name = field_type.to_token_stream().to_string();
        let network_path = get_base_path("network");

        if PacketCodecGenerator::is_var_int_wrapper(&type_name) {
            return Ok(quote! {
                <#network_path::types::var_int::VarIntWrapper as #network_path::DataType>::encode(#access_expression, buffer)?;
            });
        }

        if PacketCodecGenerator::is_var_int(&type_name, raw_type_hint) {
            return Ok(quote! {
                <#network_path::types::var_int::VarIntWrapper as #network_path::DataType>::encode(
                    &#network_path::types::var_int::VarIntWrapper(*#access_expression as i32),
                    buffer,
                )?;
            });
        }

        if PacketCodecGenerator::is_var_long_wrapper(&type_name) {
            return Ok(quote! {
                <#network_path::types::var_long::VarLongWrapper as #network_path::DataType>::encode(#access_expression, buffer)?;
            });
        }

        if PacketCodecGenerator::is_var_long(&type_name, raw_type_hint) {
            return Ok(quote! {
                <#network_path::types::var_long::VarLongWrapper as #network_path::DataType>::encode(
                    &#network_path::types::var_long::VarLongWrapper(*#access_expression as i64),
                    buffer,
                )?;
            });
        }

        if PacketCodecGenerator::is_text_component(&type_name) {
            let wrapper_type = PacketCodecGenerator::text_component_wrapper(state_name);
            return Ok(quote! {
                <#wrapper_type as #network_path::DataType>::encode(
                    &#wrapper_type((#access_expression).clone()),
                    buffer,
                )?;
            });
        }

        if let Some(collection_logic) =
            Self::collection_logic(field_type, access_expression.clone(), state_name)
        {
            return collection_logic;
        }

        Ok(
            quote! { <#field_type as #network_path::DataType>::encode(#access_expression, buffer)?; },
        )
    }

    fn collection_logic(
        field_type: &Type,
        access_expression: TokenStream2,
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
                let inner_logic = PacketCodecGenerator::generate_encode_logic(
                    &inner_type,
                    quote! { inner },
                    state_name,
                    None,
                )?;
                Ok(quote! {
                    if let Some(inner) = #access_expression {
                        <bool as #network_path::DataType>::encode(&true, buffer)?;
                        #inner_logic
                    } else {
                        <bool as #network_path::DataType>::encode(&false, buffer)?;
                    }
                })
            }));
        }

        if type_name == "Vec" || type_name == "Array" {
            return Some(get_inner_type(type_path).and_then(|inner_type| {
                let inner_logic =
                    PacketCodecGenerator::generate_encode_logic(&inner_type, quote! { item }, state_name, None)?;
                let iterable = if type_name == "Array" {
                    quote! { &(#access_expression).0 }
                } else {
                    quote! { #access_expression }
                };
                Ok(quote! {
                    <#network_path::types::var_int::VarIntWrapper as #network_path::DataType>::encode(
                        &#network_path::types::var_int::VarIntWrapper((#iterable).len() as i32),
                        buffer,
                    )?;
                    for item in #iterable {
                        #inner_logic
                    }
                })
            }));
        }

        None
    }
}
