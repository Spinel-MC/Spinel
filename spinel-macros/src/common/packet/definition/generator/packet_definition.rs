use crate::common::packet::metadata::{PacketField, PacketMetadataResolver};
use crate::common::parsers::AttrsParser;
use crate::common::utils::{map_type_to_storage_type, to_snake_case};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{ItemStruct, Lit, Type};

pub struct PacketDefinitionGenerator {
    pub(super) packet_attributes: AttrsParser,
    pub(super) item_struct: ItemStruct,
}

impl PacketDefinitionGenerator {
    pub fn from_tokens(attr: TokenStream, item: TokenStream) -> syn::Result<Self> {
        Ok(Self {
            packet_attributes: syn::parse(attr)?,
            item_struct: syn::parse(item)?,
        })
    }

    pub fn generate(mut self) -> syn::Result<TokenStream2> {
        let state_expression = self.packet_attributes.state.clone().ok_or_else(|| {
            syn::Error::new_spanned(
                &self.item_struct.ident,
                "packet must have a `state` attribute",
            )
        })?;
        let state_name =
            PacketMetadataResolver::extract_state_string(&Some(state_expression.clone()));
        let recipient_name =
            PacketMetadataResolver::extract_recipient_string(&self.packet_attributes.recipient);
        let recipient_expression = self.packet_attributes.recipient.clone().ok_or_else(|| {
            syn::Error::new_spanned(
                &self.item_struct.ident,
                "packet must have a `recipient` attribute",
            )
        })?;
        let (packet_id_literal, packet_fields) =
            self.resolve_packet_id_and_fields(state_name.as_deref(), recipient_name.as_deref())?;

        self.populate_generated_fields(packet_fields.clone())?;
        let constructor_tokens = self.constructor_tokens()?;
        let encode_tokens = self.encode_body(packet_fields.as_ref(), state_name.as_deref())?;
        let decode_tokens = self.decode_body(packet_fields.as_ref(), state_name.as_deref())?;
        self.packet_tokens(
            packet_id_literal,
            state_expression,
            recipient_expression,
            constructor_tokens,
            encode_tokens,
            decode_tokens,
        )
    }

    fn resolve_packet_id_and_fields(
        &self,
        state_name: Option<&str>,
        recipient_name: Option<&str>,
    ) -> syn::Result<(TokenStream2, Option<Vec<PacketField>>)> {
        match &self.packet_attributes.id {
            Some(Lit::Int(packet_id)) => Ok((quote! { #packet_id }, None)),
            Some(Lit::Str(packet_name)) => {
                self.resolve_named_packet_entry(packet_name, state_name, recipient_name)
            }
            Some(other_id) => Err(syn::Error::new_spanned(
                other_id,
                "packet id must be an int or string literal",
            )),
            None => Err(syn::Error::new_spanned(
                &self.item_struct.ident,
                "packet definition requires an `id` attribute",
            )),
        }
    }

    fn resolve_named_packet_entry(
        &self,
        packet_name: &syn::LitStr,
        state_name: Option<&str>,
        recipient_name: Option<&str>,
    ) -> syn::Result<(TokenStream2, Option<Vec<PacketField>>)> {
        let recipient_name = recipient_name.ok_or_else(|| {
            syn::Error::new_spanned(
                packet_name,
                format!("recipient is required for {}", packet_name.value()),
            )
        })?;
        let packet_entry = PacketMetadataResolver::resolve_packet_entry(
            &packet_name.value(),
            state_name,
            recipient_name,
        )
        .ok_or_else(|| {
            syn::Error::new_spanned(packet_name, "packet metadata entry was not found")
        })?;
        let packet_id =
            i32::from_str_radix(packet_entry.id.trim_start_matches("0x"), 16).map_err(|_| {
                syn::Error::new_spanned(packet_name, "packet id must be a hexadecimal integer")
            })?;
        Ok((quote! { #packet_id }, packet_entry.fields))
    }

    fn populate_generated_fields(
        &mut self,
        packet_fields: Option<Vec<PacketField>>,
    ) -> syn::Result<()> {
        if !self.packet_attributes.generate_fields {
            return Ok(());
        }

        let Some(packet_fields) = packet_fields else {
            return Ok(());
        };

        let mut generated_fields = syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: Punctuated::new(),
        };

        for packet_field in packet_fields {
            let field_name = format_ident!("{}", to_snake_case(&packet_field.name));
            let field_type_tokens = map_type_to_storage_type(&packet_field.field_type)?;
            let field_type: Type = syn::parse2(field_type_tokens)?;
            let field: syn::Field = syn::parse_quote! { pub #field_name: #field_type };
            generated_fields.named.push(field);
        }

        self.item_struct.fields = syn::Fields::Named(generated_fields);
        Ok(())
    }
}
