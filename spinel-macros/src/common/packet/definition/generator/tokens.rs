use crate::common::packet::definition::codec::PacketCodecGenerator;
use crate::common::packet::definition::generator::PacketDefinitionGenerator;
use crate::common::packet::metadata::PacketField;
use crate::common::utils::{get_base_path, resolve_enum_from_expr};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

impl PacketDefinitionGenerator {
    pub(super) fn constructor_tokens(&self) -> syn::Result<TokenStream2> {
        if !self.packet_attributes.generate_constructor {
            return Ok(quote! {});
        }

        match &self.item_struct.fields {
            syn::Fields::Named(fields) => {
                let field_names = fields
                    .named
                    .iter()
                    .map(|field| {
                        field.ident.as_ref().ok_or_else(|| {
                            syn::Error::new_spanned(field, "packet field must be named")
                        })
                    })
                    .collect::<syn::Result<Vec<&Ident>>>()?;
                let arguments =
                    fields
                        .named
                        .iter()
                        .zip(field_names.iter())
                        .map(|(field, field_name)| {
                            let field_type = &field.ty;
                            quote! { #field_name: #field_type }
                        });
                Ok(quote! { pub fn new(#(#arguments),*) -> Self { Self { #(#field_names),* } } })
            }
            syn::Fields::Unit => Ok(quote! { pub fn new() -> Self { Self } }),
            other_fields => Err(syn::Error::new_spanned(
                other_fields,
                "tuple packet structs are not supported",
            )),
        }
    }

    pub(super) fn encode_body(
        &self,
        packet_fields: Option<&Vec<PacketField>>,
        state_name: Option<&str>,
    ) -> syn::Result<TokenStream2> {
        let syn::Fields::Named(fields) = &self.item_struct.fields else {
            return Ok(quote! {});
        };
        let mut encode_body = quote! {};
        let mut field_definitions = packet_fields.map(|fields| fields.iter());

        for field in &fields.named {
            let field_name = field
                .ident
                .as_ref()
                .ok_or_else(|| syn::Error::new_spanned(field, "packet field must be named"))?;
            let raw_type_hint = field_definitions
                .as_mut()
                .and_then(|definitions| definitions.next())
                .map(|field_definition| field_definition.field_type.as_str());
            encode_body.extend(PacketCodecGenerator::generate_encode_logic(
                &field.ty,
                quote! { &self.#field_name },
                state_name,
                raw_type_hint,
            )?);
        }

        Ok(encode_body)
    }

    pub(super) fn decode_body(
        &self,
        packet_fields: Option<&Vec<PacketField>>,
        state_name: Option<&str>,
    ) -> syn::Result<TokenStream2> {
        let syn::Fields::Named(fields) = &self.item_struct.fields else {
            return Ok(quote! {});
        };
        let mut decode_body = quote! {};
        let mut field_definitions = packet_fields.map(|fields| fields.iter());

        for field in &fields.named {
            let field_name = field
                .ident
                .as_ref()
                .ok_or_else(|| syn::Error::new_spanned(field, "packet field must be named"))?;
            let raw_type_hint = field_definitions
                .as_mut()
                .and_then(|definitions| definitions.next())
                .map(|field_definition| field_definition.field_type.as_str());
            decode_body.extend(PacketCodecGenerator::generate_decode_logic(
                field_name,
                &field.ty,
                state_name,
                raw_type_hint,
            )?);
        }

        Ok(decode_body)
    }

    pub(super) fn packet_tokens(
        &self,
        packet_id_literal: TokenStream2,
        state_expression: syn::Expr,
        recipient_expression: syn::Expr,
        constructor_tokens: TokenStream2,
        encode_tokens: TokenStream2,
        decode_tokens: TokenStream2,
    ) -> syn::Result<TokenStream2> {
        let struct_name = &self.item_struct.ident;
        let item_struct = &self.item_struct;
        let network_path = get_base_path("network");
        let state_tokens = resolve_enum_from_expr(state_expression)?;
        let recipient_tokens = resolve_enum_from_expr(recipient_expression)?;
        let decode_constructor = self.decode_constructor_tokens()?;

        Ok(quote! {
            #item_struct
            impl #struct_name {
                pub fn get_id() -> i32 { #packet_id_literal as i32 }
                pub const fn get_id_const() -> i32 { #packet_id_literal as i32 }
                pub const fn get_state_const() -> #network_path::ConnectionState { #state_tokens }
                #constructor_tokens
                pub fn encode_to_buffer(&self) -> ::std::io::Result<#network_path::encoder::NetworkBuffer> {
                    let mut buffer = #network_path::encoder::NetworkBuffer::new();
                    <Self as #network_path::DataType>::encode(self, &mut buffer)?;
                    Ok(buffer)
                }
                pub fn dispatch<S: #network_path::PacketSender>(self, sender: &mut S) -> ::std::io::Result<()> {
                    let payload_bytes = self.encode_to_buffer()?.into_buffer();
                    sender.send_packet(Self::get_id(), &payload_bytes)
                }
            }
            impl #network_path::DataType for #struct_name {
                fn encode<W: ::std::io::Write>(&self, buffer: &mut W) -> ::std::io::Result<()> {
                    #encode_tokens
                    Ok(())
                }
                fn decode<R: ::std::io::Read>(connection: &mut R) -> ::std::io::Result<Self> {
                    #decode_tokens
                    Ok(Self #decode_constructor)
                }
            }
            impl #network_path::PacketStruct for #struct_name {
                fn get_id() -> i32 { #packet_id_literal as i32 }
                fn get_state() -> #network_path::ConnectionState { #state_tokens }
            }

            #network_path::register_packet_codec!(#struct_name, #recipient_tokens);
        })
    }

    fn decode_constructor_tokens(&self) -> syn::Result<TokenStream2> {
        match &self.item_struct.fields {
            syn::Fields::Named(fields) => {
                let field_names = fields
                    .named
                    .iter()
                    .map(|field| {
                        field.ident.as_ref().ok_or_else(|| {
                            syn::Error::new_spanned(field, "packet field must be named")
                        })
                    })
                    .collect::<syn::Result<Vec<&Ident>>>()?;
                Ok(quote! { { #(#field_names),* } })
            }
            syn::Fields::Unit => Ok(quote! {}),
            other_fields => Err(syn::Error::new_spanned(
                other_fields,
                "unsupported packet field shape",
            )),
        }
    }
}
