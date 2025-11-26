use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{FnArg, ItemFn, PatType, Type, parse_macro_input};

use crate::parsers::AttrsParser;
use crate::util::{
    map_field_to_rust_type, map_syn_type_to_decoder_method, map_type_to_decoder_method_name,
    resolve_enum_from_expr, resolve_priority_token,
};

pub fn packet_listener_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let packet_attrs = parse_macro_input!(attr as AttrsParser);
    let fn_ident = &input_fn.sig.ident;

    let state_expr = packet_attrs
        .state
        .expect("Packet listener must have a 'state' attribute.");

    let id = match packet_attrs.id {
        Some(syn::Lit::Int(lit_int)) => lit_int.to_token_stream(),
        Some(syn::Lit::Str(lit_str)) => {
            let id_str = lit_str.value();
            let state_str = extract_state_string(&Some(state_expr.clone()));
            let resolved_id = resolve_packet_id(&id_str, state_str.as_deref()).expect(&format!(
                "Failed to resolve packet ID '{}' for state {:?}",
                id_str, state_str
            ));
            quote! { #resolved_id }
        }
        Some(_) => panic!("Packet ID must be an integer or a string literal."),
        None => quote! {-1},
    };

    let modules = packet_attrs.modules;
    let modules_slice = quote! { &[#(#modules),*] };

    let state = resolve_enum_from_expr(
        state_expr,
        "spinel::network::ConnectionState",
        "ConnectionState",
    );

    let priority = resolve_priority_token(packet_attrs.priority, "Priority", "Medium");

    let events = packet_attrs.events;
    let events_slice = quote! { &[#(#events),*] };

    let wrapper_fn_ident = format_ident!("__wrapper_for_{}", fn_ident);

    let (generated_packet_struct, generated_wrapper_fn) = if let Some(fields_attr) =
        packet_attrs.fields
    {
        let fn_name_str = fn_ident.to_string();
        let camel_case_fn_name: String = fn_name_str
            .split('_')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect();
        let packet_struct_ident = format_ident!("PacketFor{}", camel_case_fn_name);
        let fields = &fields_attr.fields;

        for arg in &mut input_fn.sig.inputs {
            if let FnArg::Typed(PatType { ty, .. }) = arg {
                if let Type::Path(type_path) = &mut **ty {
                    if let Some(segment) = type_path.path.segments.last_mut() {
                        if segment.ident == "Packet" {
                            segment.ident = packet_struct_ident.clone();
                        }
                    }
                }
            }
        }

        let struct_fields = fields.iter().map(|field| {
            let name = &field.name;
            let ty = map_field_to_rust_type(field);
            quote! { pub #name: #ty, }
        });

        let packet_struct = quote! {
            #[derive(Debug)]
            struct #packet_struct_ident {
                #(#struct_fields)*
            }
        };

        let field_parsers = fields.iter().map(|field| {
            let name = &field.name;
            let ty_str = field.ty.to_string();

            let decoder_logic = if ty_str == "Array" || ty_str == "Vec" { // <-- THIS IS THE FIX!
                let inner_type = field.generic_param.as_ref().unwrap();

                if let Type::Tuple(tuple_type) = inner_type {
                    let tuple_elements_rust_type = quote! { #tuple_type };
                    let tuple_readers = tuple_type.elems.iter().map(|elem_ty| {
                        let decoder = map_syn_type_to_decoder_method(elem_ty);
                        quote! { client.#decoder()? }
                    });

                    quote! {
                        (|| -> Result<Vec<#tuple_elements_rust_type>, std::io::Error> {
                            use std::io::{Error, ErrorKind};
                            let count = client.read_varint()? as usize;
                            const MAX_ARRAY_ELEMENTS: usize = 8192;
                            if count > MAX_ARRAY_ELEMENTS {
                                return Err(Error::new(ErrorKind::InvalidData, "Array size exceeds limit"));
                            }
                            let mut vec = Vec::with_capacity(count);
                            for _ in 0..count {
                                vec.push((
                                    #(#tuple_readers),*
                                ));
                            }
                            Ok(vec)
                        })()
                    }
                } else if let Type::Path(type_path) = inner_type {
                    let inner_ident = &type_path.path.segments.last().unwrap().ident;
                    let inner_rust_type = crate::util::map_type_to_rust_type(inner_ident);
                    let inner_decoder_method = crate::util::map_type_to_decoder_method_name(inner_ident);

                    quote! {
                        (|| -> Result<Vec<#inner_rust_type>, std::io::Error> {
                            use std::io::{Error, ErrorKind};
                            let count = client.read_varint()? as usize;
                            const MAX_ARRAY_ELEMENTS: usize = 8192;
                            if count > MAX_ARRAY_ELEMENTS {
                                return Err(Error::new(ErrorKind::InvalidData, "Array size exceeds limit"));
                            }
                            let mut vec = Vec::with_capacity(count);
                            for _ in 0..count {
                                vec.push(client.#inner_decoder_method()?);
                            }
                            Ok(vec)
                        })()
                    }
                } else {
                    panic!("Unsupported generic type for Array/Vec: {}", quote!(#inner_type).to_string());
                }
            } else if ty_str == "Optional" {
                let inner_type = field.generic_param.as_ref().unwrap();
                 let inner_decoder_method = if let Type::Path(p) = inner_type {
                    map_type_to_decoder_method_name(&p.path.segments.last().unwrap().ident)
                } else {
                    panic!("Unsupported generic type for Optional: {}", quote!(#inner_type).to_string())
                };

                quote! {
                    (|| -> Result<_, std::io::Error> {
                        if client.read_bool()? {
                            Ok(Some(client.#inner_decoder_method()?))
                        } else {
                            Ok(None)
                        }
                    })()
                }
            } else if (ty_str == "String" || ty_str == "Identifier") && field.len_arg.is_some() {
                let len_lit = field.len_arg.as_ref().unwrap();
                quote! { client.read_string_with_limit(#len_lit) }
            } else {
                let decoder_method = map_type_to_decoder_method_name(&field.ty);
                quote! { client.#decoder_method() }
            };

            quote! {
                let #name = match #decoder_logic {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Packet read failed for field '{}': {:?}", stringify!(#name), e);
                        return false;
                    }
                };
            }
        });

        let field_names = fields.iter().map(|f| &f.name);

        let wrapper_fn = quote! {
            #[doc(hidden)]
            fn #wrapper_fn_ident(client: &mut spinel::internal::Client, server_ptr: *mut ()) -> bool {
                #(#field_parsers)*

                let packet = #packet_struct_ident {
                    #(#field_names),*
                };

                let server = unsafe { &mut *(server_ptr as *mut spinel::core::server::MinecraftServer) };
                #fn_ident(client, packet, server)
            }
        };
        (Some(packet_struct), wrapper_fn)
    } else {
        let wrapper_fn = quote! {
            #[doc(hidden)]
            fn #wrapper_fn_ident(client: &mut spinel::internal::Client, server_ptr: *mut ()) -> bool {
                let server = unsafe { &mut *(server_ptr as *mut spinel::core::server::MinecraftServer) };
                #fn_ident(client, server)
            }
        };
        (None, wrapper_fn)
    };

    let static_metadata_name = format_ident!("__SPINEL_PACKET_LISTENER_{}", fn_ident);

    let inventory_submit = quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static #static_metadata_name: spinel::internal::PacketListener = spinel::internal::PacketListener {
            id: #id as i32,
            state: #state,
            priority: #priority,
            events: #events_slice,
            handler: #wrapper_fn_ident,
            modules: #modules_slice,
        };
        inventory::submit! { &#static_metadata_name }
    };

    let mut final_output = TokenStream2::new();
    if let Some(ps) = generated_packet_struct {
        final_output.extend(ps);
    }
    final_output.extend(input_fn.into_token_stream());
    final_output.extend(generated_wrapper_fn);
    final_output.extend(inventory_submit);

    final_output.into()
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

    let direction = &packets.serverbound;

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
