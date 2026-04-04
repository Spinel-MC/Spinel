use crate::common::parsers::AttrsParser;
use crate::common::utils::{get_base_path, resolve_priority_token};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, PatType, Type};

pub fn generate(
    packet_attrs: AttrsParser,
    input_fn: ItemFn,
    context_type: Type,
    connection_type: Type,
) -> TokenStream2 {
    let fn_ident = &input_fn.sig.ident;

    let packet_type = if let Some(FnArg::Typed(PatType { ty, .. })) = input_fn.sig.inputs.get(1) {
        quote! { #ty }
    } else {
        panic!(
            "Packet listener function '{}' must have 3 arguments: (connection, packet, context)",
            fn_ident
        );
    };

    let modules = packet_attrs.modules;
    let modules_slice = quote! { &[#(#modules),*] };
    let priority = resolve_priority_token(packet_attrs.priority, "Priority", "Medium");
    let events = packet_attrs.events;
    let events_slice = quote! { &[#(#events),*] };

    let wrapper_fn_ident = format_ident!("__wrapper_for_{}", fn_ident);
    let net_path = get_base_path("network");

    let wrapper_fn = quote! {
        #[doc(hidden)]
        fn #wrapper_fn_ident(connection: &mut #connection_type, context_ptr: *mut ()) -> bool {
            let packet = match <#packet_type as #net_path::DataType>::decode(connection) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to decode packet for listener '{}': {:?}", stringify!(#fn_ident), e);
                    return false;
                }
            };
            let context = unsafe { &mut *(context_ptr as *mut #context_type) };
            #fn_ident(connection, packet, context)
        }
    };

    let client_path = get_base_path("client");
    let event_path = get_base_path("events");
    let static_metadata_name = format_ident!("__SPINEL_PACKET_LISTENER_{}", fn_ident);

    quote! {
        #input_fn
        #wrapper_fn

        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static #static_metadata_name: #client_path::ClientPacketListener =
            #client_path::ClientPacketListener {
                id: #packet_type::get_id_const(),
                state: #packet_type::get_state_const(),
                priority: #priority,
                events: #events_slice,
                handler: #wrapper_fn_ident,
                modules: #modules_slice,
            };
        #event_path::inventory::submit! { &#static_metadata_name }
    }
}
