use crate::common::parsers::AttrsParser;
use crate::common::utils::{get_base_path, resolve_priority_token};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{FnArg, ImplItem, ItemImpl, Meta, Type, TypePath, parse2};

pub fn generate(mut input_impl: ItemImpl) -> TokenStream2 {
    let server_path = get_base_path("server");
    let listener_type = input_impl.self_ty.clone();
    let mut packet_registrations = Vec::new();

    for impl_item in input_impl.items.iter_mut() {
        let ImplItem::Fn(method) = impl_item else {
            continue;
        };

        let Some(attribute_index) = method
            .attrs
            .iter()
            .position(|attribute| attribute.path().is_ident("packet_handler"))
        else {
            continue;
        };

        let attribute = method.attrs.remove(attribute_index);
        let packet_attrs = match attribute.meta {
            Meta::Path(_) => AttrsParser {
                events: Vec::new(),
                modules: Vec::new(),
                priority: None,
                id: None,
                state: None,
                recipient: None,
                generate_fields: false,
                generate_constructor: false,
            },
            Meta::List(meta_list) => parse2::<AttrsParser>(meta_list.tokens)
                .unwrap_or_else(|error| panic!("invalid packet_handler attribute: {error}")),
            Meta::NameValue(_) => panic!("invalid packet_handler attribute"),
        };
        let priority = resolve_priority_token(packet_attrs.priority, "Medium");
        let method_name = &method.sig.ident;
        let packet_type = packet_type_from_method(method);

        packet_registrations.push(quote! {
            global_packet_handler.add_listener_with_priority::<#packet_type>(
                #priority,
                #listener_type::#method_name,
            );
        });
    }

    quote! {
        #input_impl

        impl #server_path::network::PacketHandler for #listener_type {
            fn register_packet_handlers(
                self,
                global_packet_handler: &mut #server_path::network::GlobalPacketHandler,
            ) {
                #(#packet_registrations)*
            }
        }
    }
}

fn packet_type_from_method(method: &syn::ImplItemFn) -> TypePath {
    let Some(packet_argument) = method.sig.inputs.iter().nth(1) else {
        panic!("#[packet_handler] method must take client, packet, and server arguments");
    };

    let FnArg::Typed(packet_argument) = packet_argument else {
        panic!("#[packet_handler] packet argument must be typed");
    };

    let Type::Path(packet_type) = &*packet_argument.ty else {
        panic!("#[packet_handler] packet argument must be a path type");
    };

    packet_type.clone()
}
