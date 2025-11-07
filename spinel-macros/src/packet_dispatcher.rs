use crate::parsers::AttrsParser;
use crate::util::{get_inner_type, get_write_method_for_type};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemStruct, Type, parse_macro_input};

pub fn packet_dispatcher_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let packet_attrs = parse_macro_input!(attr as AttrsParser);
    let item_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &item_struct.ident;
    let packet_id_lit = packet_attrs.id;

    let mut encode_body = quote! {};

    match &item_struct.fields {
        syn::Fields::Named(fields) => {
            for field in &fields.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                let ty_path = if let Type::Path(path) = field_type {
                    path
                } else {
                    continue;
                };
                let segment_ident = ty_path.path.segments.last().unwrap().ident.clone();
                let segment_str = segment_ident.to_string();

                if segment_str == "Option" || segment_str == "Optional" {
                    let inner_type = get_inner_type(ty_path)
                        .expect("Option/Optional must have a generic parameter.");

                    if let Type::Tuple(type_tuple) = &inner_type {
                        if type_tuple.elems.len() == 2 {
                            encode_body.extend(quote! {
                                buffer.write_bool(self.#field_name.is_some());
                                if let Some(value) = &self.#field_name {
                                    buffer.write_string(&value.0);
                                    buffer.write_position(&value.1);
                                }
                            });
                            continue;
                        }
                    }

                    let (write_method, is_ref) = get_write_method_for_type(&inner_type);

                    let inner_segment_ident = if let Type::Path(p) = &inner_type {
                        p.path.segments.last().unwrap().ident.clone()
                    } else {
                        format_ident!("")
                    };

                    let value_expr = if inner_segment_ident == format_ident!("VarInt")
                        || inner_segment_ident == format_ident!("VarLong")
                    {
                        quote! { value.0 }
                    } else if is_ref {
                        quote! { value }
                    } else {
                        quote! { *value }
                    };

                    encode_body.extend(quote! {
                        buffer.write_bool(self.#field_name.is_some());
                        if let Some(value) = &self.#field_name {
                            buffer.#write_method(#value_expr);
                        }
                    });
                    continue;
                }

                let (write_method, is_ref) = get_write_method_for_type(field_type);
                let write_method_str = write_method.to_string();

                if write_method_str == "write_array_custom" {
                    let inner_type =
                        get_inner_type(ty_path).expect("Array/Vec must have a generic parameter.");

                    let base_expr = if segment_str == "Array" {
                        quote! { self.#field_name.0 }
                    } else {
                        quote! { self.#field_name }
                    };

                    let mut loop_body = quote! {};

                    if let Type::Tuple(type_tuple) = inner_type {
                        for (i, elem_type) in type_tuple.elems.iter().enumerate() {
                            let index = syn::Index::from(i);

                            let is_optional = if let Type::Path(p) = elem_type {
                                p.path
                                    .segments
                                    .last()
                                    .map_or(false, |s| s.ident == "Option" || s.ident == "Optional")
                            } else {
                                false
                            };

                            if is_optional {
                                let inner_opt_type =
                                    get_inner_type(if let Type::Path(p) = elem_type {
                                        p
                                    } else {
                                        unreachable!()
                                    })
                                    .expect("Optional in tuple must have a generic parameter");
                                let (inner_writer, inner_is_ref) =
                                    get_write_method_for_type(&inner_opt_type);

                                let value_expr = if inner_is_ref {
                                    quote! { value }
                                } else {
                                    quote! { *value }
                                };

                                let inner_segment = if let Type::Path(p) = &inner_opt_type {
                                    p.path.segments.last().map(|s| s.ident.clone())
                                } else {
                                    None
                                };
                                let final_value_expr = if inner_segment
                                    .as_ref()
                                    .map_or(false, |id| id == "VarInt" || id == "VarLong")
                                {
                                    quote! { value.0 }
                                } else {
                                    value_expr
                                };

                                loop_body.extend(quote! {
                                    buffer.write_bool(item.#index.is_some());
                                    if let Some(value) = &item.#index {
                                        buffer.#inner_writer(#final_value_expr);
                                    }
                                });
                            } else {
                                let (elem_writer, elem_is_ref) =
                                    get_write_method_for_type(elem_type);
                                let access_expr = if elem_is_ref {
                                    quote! { &item.#index }
                                } else {
                                    quote! { item.#index }
                                };

                                let elem_segment_ident = if let Type::Path(p) = elem_type {
                                    p.path.segments.last().unwrap().ident.clone()
                                } else {
                                    format_ident!("")
                                };
                                let final_access_expr = if elem_segment_ident
                                    == format_ident!("VarInt")
                                    || elem_segment_ident == format_ident!("VarLong")
                                {
                                    quote! { #access_expr.0 }
                                } else {
                                    access_expr
                                };

                                loop_body.extend(quote! {
                                    buffer.#elem_writer(#final_access_expr);
                                });
                            }
                        }
                    } else {
                        let (elem_writer, elem_is_ref) = get_write_method_for_type(&inner_type);
                        let access_expr = if elem_is_ref {
                            quote! { item }
                        } else {
                            quote! { *item }
                        };

                        let elem_segment_ident = if let Type::Path(p) = &inner_type {
                            p.path.segments.last().unwrap().ident.clone()
                        } else {
                            format_ident!("")
                        };
                        let final_access_expr = if elem_segment_ident == format_ident!("VarInt")
                            || elem_segment_ident == format_ident!("VarLong")
                        {
                            quote! { item.0 }
                        } else {
                            access_expr
                        };

                        loop_body.extend(quote! {
                            buffer.#elem_writer(#final_access_expr);
                        });
                    }

                    encode_body.extend(quote! {
                        buffer.write_varint(#base_expr.len() as i32);
                        for item in &#base_expr {
                            #loop_body
                        }
                    });

                    continue;
                }

                let value_expr = if segment_str == "VarInt" || segment_str == "VarLong" {
                    quote! { self.#field_name.0 }
                } else if is_ref {
                    quote! { &self.#field_name }
                } else {
                    quote! { self.#field_name }
                };

                encode_body.extend(quote! { buffer.#write_method(#value_expr); });
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
