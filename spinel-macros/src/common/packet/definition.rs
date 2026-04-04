use crate::common::packet::metadata::{
    PacketField, extract_recipient_string, extract_state_string, resolve_packet_entry,
};
use crate::common::parsers::AttrsParser;
use crate::common::utils::{
    get_base_path, get_inner_type, map_type_to_storage_type, resolve_enum_from_expr,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{Ident, ItemStruct, Type, parse_macro_input};

pub fn packet_struct_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let packet_attrs = parse_macro_input!(attr as AttrsParser);
    let mut item_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &item_struct.ident;
    let state_expr = packet_attrs
        .state
        .clone()
        .expect("Packet must have a 'state' attribute.");

    let state_str_opt = extract_state_string(&Some(state_expr.clone()));
    let recipient_opt = extract_recipient_string(&packet_attrs.recipient);

    let (packet_id_lit, packet_fields) = match packet_attrs.id {
        Some(syn::Lit::Int(lit_int)) => (quote! { #lit_int }, None),
        Some(syn::Lit::Str(lit_str)) => {
            let id_str = lit_str.value();
            let recipient = recipient_opt
                .as_deref()
                .expect(&format!("Recipient required for {}", id_str));
            let entry = resolve_packet_entry(&id_str, state_str_opt.as_deref(), recipient)
                .expect("Packet not found");
            let id_val = i32::from_str_radix(entry.id.trim_start_matches("0x"), 16).unwrap();
            (quote! { #id_val }, entry.fields)
        }
        Some(_) => panic!("Packet ID must be int or string"),
        None => panic!("Packet dispatcher needs ID"),
    };

    if packet_attrs.generate_fields {
        if let Some(fields) = packet_fields.clone() {
            let mut new_fields = syn::FieldsNamed {
                brace_token: syn::token::Brace::default(),
                named: Punctuated::new(),
            };

            for field_def in fields {
                let field_name = format_ident!("{}", field_def.name);
                let type_str = field_def.field_type;

                let ty_tokens = map_type_to_storage_type(&type_str);
                let ty: Type = syn::parse_str(&ty_tokens.to_string()).expect(&format!(
                    "Failed to parse generated type '{}' for field '{}'",
                    ty_tokens, field_def.name
                ));

                let field: syn::Field = syn::parse_quote! {
                    pub #field_name: #ty
                };
                new_fields.named.push(field);
            }

            item_struct.fields = syn::Fields::Named(new_fields);
        }
    }

    let mut new_fn_tokens = quote! {};
    if packet_attrs.generate_constructor {
        match &item_struct.fields {
            syn::Fields::Named(fields) => {
                let mut new_args = quote! {};
                let mut new_body = quote! {};
                for field in &fields.named {
                    let name = field.ident.as_ref().unwrap();
                    let ty = &field.ty;
                    new_args.extend(quote! { #name: #ty, });
                    new_body.extend(quote! { #name, });
                }
                new_fn_tokens = quote! { pub fn new(#new_args) -> Self { Self { #new_body } } };
            }
            syn::Fields::Unit => {
                new_fn_tokens = quote! { pub fn new() -> Self { Self } };
            }
            _ => panic!("Tuple structs not supported"),
        }
    }

    let mut encode_body = quote! {};
    let mut decode_body = quote! {};

    match &item_struct.fields {
        syn::Fields::Named(fields) => {
            let mut field_defs_iter: Option<std::slice::Iter<PacketField>> =
                if let Some(pf) = &packet_fields {
                    Some(pf.iter())
                } else {
                    None
                };

            for field in &fields.named {
                let name = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                let raw_type_hint = if let Some(iter) = &mut field_defs_iter {
                    iter.next().map(|f| f.field_type.clone())
                } else {
                    None
                };

                encode_body.extend(generate_encode_logic(
                    ty,
                    quote! { &self.#name },
                    state_str_opt.as_deref(),
                    raw_type_hint.as_deref(),
                ));

                decode_body.extend(generate_decode_logic(
                    name,
                    ty,
                    state_str_opt.as_deref(),
                    raw_type_hint.as_deref(),
                ));
            }
        }
        _ => {}
    }

    let net_path = get_base_path("network");
    let state_resolved = resolve_enum_from_expr(state_expr, "", "");
    let field_names = match &item_struct.fields {
        syn::Fields::Named(fields) => {
            let idents = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            quote! { { #(#idents),* } }
        }
        syn::Fields::Unit => quote! {},
        _ => panic!("Unsupported field type"),
    };

    quote! {
        #item_struct
        impl #struct_name {
            pub fn get_id() -> i32 { #packet_id_lit as i32 }
            pub const fn get_id_const() -> i32 { #packet_id_lit as i32 }
            pub const fn get_state_const() -> #net_path::ConnectionState { #state_resolved }

            #new_fn_tokens
            pub fn encode_to_buffer(&self) -> #net_path::encoder::NetworkBuffer {
                let mut buffer = #net_path::encoder::NetworkBuffer::new();
                <Self as #net_path::DataType>::encode(self, &mut buffer).unwrap();
                buffer
            }
            pub fn dispatch<S: #net_path::PacketSender>(self, sender: &mut S) {
                let payload_bytes = self.encode_to_buffer().into_buffer();
                sender.send_packet(Self::get_id(), &payload_bytes);
            }
        }

        impl #net_path::DataType for #struct_name {
            fn encode<W: ::std::io::Write>(&self, buffer: &mut W) -> ::std::io::Result<()> {
                #encode_body
                Ok(())
            }
            fn decode<R: ::std::io::Read>(connection: &mut R) -> ::std::io::Result<Self> {
                #decode_body
                Ok(Self #field_names)
            }
        }

        impl #net_path::PacketStruct for #struct_name {
            fn get_id() -> i32 { #packet_id_lit as i32 }
            fn get_state() -> #net_path::ConnectionState { #state_resolved }
        }
    }
    .into()
}

fn generate_encode_logic(
    ty: &Type,
    access_expr: TokenStream2,
    state: Option<&str>,
    raw_type_hint: Option<&str>,
) -> TokenStream2 {
    let type_string = quote!(#ty).to_string();
    let net_path = get_base_path("network");

    let is_varint =
        raw_type_hint.map_or(false, |h| h == "VarInt" || h == "int") || type_string == "VarInt";
    let is_varlong =
        raw_type_hint.map_or(false, |h| h == "VarLong" || h == "long") || type_string == "VarLong";

    if is_varint {
        return quote! {
             <#net_path::types::var_int::VarIntWrapper as #net_path::DataType>::encode(&#net_path::types::var_int::VarIntWrapper(*#access_expr as i32), buffer)?;
        };
    }

    if is_varlong {
        return quote! {
             <#net_path::types::var_long::VarLongWrapper as #net_path::DataType>::encode(&#net_path::types::var_long::VarLongWrapper(*#access_expr as i64), buffer)?;
        };
    }

    if type_string.contains("TextComponent")
        && !type_string.contains("Json")
        && !type_string.contains("Nbt")
    {
        let wrapper_type = match state {
            Some("Play") | Some("Configuration") => {
                quote! { #net_path::wrappers::NbtTextComponent }
            }
            _ => quote! { #net_path::wrappers::JsonTextComponent },
        };
        return quote! {
            <#wrapper_type as #net_path::DataType>::encode(&#wrapper_type((#access_expr).clone()), buffer)?;
        };
    }

    if let Type::Path(p) = ty {
        let segment = p.path.segments.last().unwrap();
        let ident_str = segment.ident.to_string();

        if ident_str == "Option" || ident_str == "Optional" {
            let inner_type = get_inner_type(p).expect("Option must have generic");
            let inner_logic = generate_encode_logic(&inner_type, quote! { inner }, state, None);
            return quote! {
                if let Some(inner) = #access_expr {
                    <bool as #net_path::DataType>::encode(&true, buffer)?;
                    #inner_logic
                } else {
                    <bool as #net_path::DataType>::encode(&false, buffer)?;
                }
            };
        } else if ident_str == "Vec" || ident_str == "Array" {
            let inner_type = get_inner_type(p).expect("Vec must have generic");
            let inner_logic = generate_encode_logic(&inner_type, quote! { item }, state, None);
            let iterable = if ident_str == "Array" {
                quote! { &(#access_expr).0 }
            } else {
                quote! { #access_expr }
            };
            return quote! {
                <#net_path::types::var_int::VarIntWrapper as #net_path::DataType>::encode(&#net_path::types::var_int::VarIntWrapper((#iterable).len() as i32), buffer)?;
                for item in #iterable {
                    #inner_logic
                }
            };
        }
    }

    quote! { <#ty as #net_path::DataType>::encode(#access_expr, buffer)?; }
}

fn generate_decode_logic(
    name: &Ident,
    ty: &Type,
    state: Option<&str>,
    raw_type_hint: Option<&str>,
) -> TokenStream2 {
    let type_string = quote!(#ty).to_string();
    let net_path = get_base_path("network");

    let is_varint =
        raw_type_hint.map_or(false, |h| h == "VarInt" || h == "int") || type_string == "VarInt";
    let is_varlong =
        raw_type_hint.map_or(false, |h| h == "VarLong" || h == "long") || type_string == "VarLong";

    if is_varint {
        return quote! {
            let #name: #ty = <#net_path::types::var_int::VarIntWrapper as #net_path::DataType>::decode(connection).map(|v: #net_path::types::var_int::VarIntWrapper| v.0 as #ty)?;
        };
    }

    if is_varlong {
        return quote! {
            let #name: #ty = <#net_path::types::var_long::VarLongWrapper as #net_path::DataType>::decode(connection).map(|v: #net_path::types::var_long::VarLongWrapper| v.0 as #ty)?;
        };
    }

    if type_string.contains("TextComponent")
        && !type_string.contains("Json")
        && !type_string.contains("Nbt")
    {
        let wrapper_type = match state {
            Some("Play") | Some("Configuration") => {
                quote! { #net_path::wrappers::NbtTextComponent }
            }
            _ => quote! { #net_path::wrappers::JsonTextComponent },
        };
        return quote! {
            let #name: #ty = <#wrapper_type as #net_path::DataType>::decode(connection).map(|v: #wrapper_type| v.0 as #ty)?;
        };
    }

    if let Type::Path(p) = ty {
        let segment = p.path.segments.last().unwrap();
        let ident_str = segment.ident.to_string();

        if ident_str == "Option" || ident_str == "Optional" {
            let inner_type = get_inner_type(p).expect("Option must have generic");
            let inner_logic = generate_decode_logic(name, &inner_type, state, None);
            return quote! {
                let #name: #ty = if <bool as #net_path::DataType>::decode(connection)? {
                    #inner_logic
                    Some(#name)
                } else {
                    None
                };
            };
        } else if ident_str == "Vec" || ident_str == "Array" {
            let inner_type = get_inner_type(p).expect("Vec must have generic");
            let inner_logic =
                generate_decode_logic(&format_ident!("item"), &inner_type, state, None);
            let constructor = if ident_str == "Array" {
                quote! { #net_path::types::Array(items) }
            } else {
                quote! { items }
            };
            return quote! {
                let len = <#net_path::types::var_int::VarIntWrapper as #net_path::DataType>::decode(connection)?.0 as usize;
                let mut items = Vec::with_capacity(len);
                for _ in 0..len {
                    #inner_logic
                    items.push(item);
                }
                let #name: #ty = #constructor;
            };
        }
    }

    quote! { let #name: #ty = <#ty as #net_path::DataType>::decode(connection)?; }
}
