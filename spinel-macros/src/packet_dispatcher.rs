use crate::parsers::AttrsParser;
use crate::util::{get_inner_type, get_write_method_for_type};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{ItemStruct, Type, parse_macro_input};

pub fn packet_dispatcher_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let packet_attrs = parse_macro_input!(attr as AttrsParser);
    let item_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &item_struct.ident;
    let state_expr = packet_attrs.state.clone();
    let packet_id_lit = match packet_attrs.id {
        Some(syn::Lit::Int(lit_int)) => lit_int.to_token_stream(),
        Some(syn::Lit::Str(lit_str)) => {
            let id_str = lit_str.value();
            let state_str = extract_state_string(&state_expr);
            let resolved_id = resolve_packet_id(&id_str, state_str.as_deref()).expect(&format!(
                "Failed to resolve packet ID '{}' for state {:?}",
                id_str, state_str
            ));
            quote! { #resolved_id }
        }
        Some(_) => panic!("Packet ID must be an integer or a string literal."),
        None => panic!("Packet dispatcher must have an 'id' attribute."),
    };

    let mut encode_body = quote! {};

    match &item_struct.fields {
        syn::Fields::Named(fields) => {
            for field in &fields.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                let access_expr = quote! { &self.#field_name };

                let logic = generate_serialization_logic(field_type, access_expr);
                encode_body.extend(logic);
            }
        }
        syn::Fields::Unit => {}
        syn::Fields::Unnamed(_) => {
            panic!("packet_dispatcher does not support tuple structs.");
        }
    }

    quote! {
        #item_struct
        impl #struct_name {
            pub fn get_id() -> i32 { #packet_id_lit as i32 }
            pub fn encode(&self) -> spinel::internal::encoder::NetworkBuffer {
                let mut buffer = spinel::internal::encoder::NetworkBuffer::new();
                #encode_body
                buffer
            }
            pub fn dispatch(self, client: &mut spinel::network::Client) {
                let packet_id = Self::get_id();
                let payload_bytes = self.encode().into_buffer();
                client.send_packet(packet_id, &payload_bytes);
            }
        }
    }
    .into()
}

fn generate_serialization_logic(ty: &Type, access_expr: TokenStream2) -> TokenStream2 {
    if let Type::Path(p) = ty {
        if let Some(segment) = p.path.segments.last() {
            if segment.ident == "Option" || segment.ident == "Optional" {
                let inner_type = get_inner_type(p).expect("Option/Optional must have generic");

                let inner_logic = generate_serialization_logic(&inner_type, quote! { value });

                return quote! {
                    buffer.write_bool((#access_expr).is_some());
                    if let Some(value) = &#access_expr {
                        #inner_logic
                    }
                };
            }
        }
    }

    if let Type::Tuple(type_tuple) = ty {
        let mut tuple_body = quote! {};
        for (i, elem_type) in type_tuple.elems.iter().enumerate() {
            let index = syn::Index::from(i);
            let elem_access = quote! { &(#access_expr).#index };
            tuple_body.extend(generate_serialization_logic(elem_type, elem_access));
        }
        return tuple_body;
    }

    let (write_method, is_ref) = get_write_method_for_type(ty);
    let write_method_str = write_method.to_string();

    if write_method_str == "write_array_custom" {
        let inner_type = get_inner_type(if let Type::Path(p) = ty {
            p
        } else {
            panic!("Array/Vec must be a path")
        })
        .expect("Array/Vec must have a generic parameter.");

        let is_array_wrapper = if let Type::Path(p) = ty {
            p.path.segments.last().unwrap().ident == "Array"
        } else {
            false
        };

        let vec_expr = if is_array_wrapper {
            quote! { &(#access_expr).0 }
        } else {
            quote! { #access_expr }
        };

        let inner_logic = generate_serialization_logic(&inner_type, quote! { item });

        return quote! {
            buffer.write_varint((#vec_expr).len() as i32);
            for item in #vec_expr {
                #inner_logic
            }
        };
    }

    let value_expr = if is_ref {
        quote! { #access_expr }
    } else {
        quote! { (#access_expr).clone() }
    };

    let segment_ident = if let Type::Path(p) = ty {
        p.path.segments.last().unwrap().ident.to_string()
    } else {
        "".to_string()
    };

    let final_value_expr = if segment_ident == "VarInt" || segment_ident == "VarLong" {
        quote! { #value_expr.0 }
    } else {
        value_expr
    };

    quote! { buffer.#write_method(#final_value_expr); }
}

#[derive(serde::Deserialize)]
struct PacketsJson {
    serverbound: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    clientbound: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
}

fn extract_state_string(state_expr: &Option<syn::Expr>) -> Option<String> {
    state_expr.as_ref().and_then(|expr| {
        if let syn::Expr::Path(expr_path) = expr {
            expr_path
                .path
                .segments
                .last()
                .map(|seg| seg.ident.to_string())
        } else {
            None
        }
    })
}

fn resolve_packet_id(name: &str, state: Option<&str>) -> Option<i32> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let packets_path =
        std::path::Path::new(&manifest_dir).join("../spinel-registry/build_assets/packets.json");

    if !packets_path.exists() {
        panic!(
            "packets.json not found at {:?}. Please run SteelExtractor.",
            packets_path
        );
    }

    let file = std::fs::File::open(packets_path).expect("Failed to open packets.json");
    let reader = std::io::BufReader::new(file);
    let packets: PacketsJson =
        serde_json::from_reader(reader).expect("Failed to parse packets.json");

    let direction = &packets.clientbound;

    if let Some(state_name) = state {
        let protocol_key = match state_name {
            "Handshaking" => "handshake",
            "Status" => "status",
            "Login" => "login",
            "Configuration" => "config",
            "Play" => "play",
            _ => return None,
        };

        if let Some(protocol) = direction.get(protocol_key) {
            if let Some(hex_id) = protocol.get(name) {
                return Some(i32::from_str_radix(hex_id.trim_start_matches("0x"), 16).unwrap());
            }
        }
        return None;
    }

    for protocol in direction.values() {
        if let Some(hex_id) = protocol.get(name) {
            return Some(i32::from_str_radix(hex_id.trim_start_matches("0x"), 16).unwrap());
        }
    }

    None
}
