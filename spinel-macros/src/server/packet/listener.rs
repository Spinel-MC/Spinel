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
    PacketListenerGenerator::new(packet_attrs, input_fn, context_type, connection_type)
        .generate()
        .unwrap_or_else(|error| error.to_compile_error())
}

struct PacketListenerGenerator {
    packet_attrs: AttrsParser,
    input_fn: ItemFn,
    context_type: Type,
    connection_type: Type,
}

impl PacketListenerGenerator {
    fn new(
        packet_attrs: AttrsParser,
        input_fn: ItemFn,
        context_type: Type,
        connection_type: Type,
    ) -> Self {
        Self {
            packet_attrs,
            input_fn,
            context_type,
            connection_type,
        }
    }

    fn generate(self) -> syn::Result<TokenStream2> {
        let function_name = self.input_fn.sig.ident.clone();
        let packet_type = self.packet_type_tokens()?;
        let modules = self.packet_attrs.modules;
        let events = self.packet_attrs.events;
        let priority = resolve_priority_token(self.packet_attrs.priority, "Medium");
        let wrapper_function_name = format_ident!("__wrapper_for_{}", function_name);
        let network_path = get_base_path("network");
        let server_path = get_base_path("server");
        let event_path = get_base_path("events");
        let static_metadata_name = format_ident!("__SPINEL_PACKET_LISTENER_{}", function_name);
        let input_fn = self.input_fn;
        let context_type = self.context_type;
        let connection_type = self.connection_type;

        Ok(quote! {
            #input_fn

            #[doc(hidden)]
            fn #wrapper_function_name(connection: &mut #connection_type, context_ptr: *mut ()) -> bool {
                let packet = match <#packet_type as #network_path::DataType>::decode(connection) {
                    Ok(packet) => packet,
                    Err(error) => {
                        let context = unsafe { &mut *(context_ptr as *mut #context_type) };
                        let mut inbound_packet_error_event =
                            #server_path::events::network::inbound_packet_error::InboundPacketErrorEvent::new(
                                #server_path::events::network::inbound_packet_error::InboundPacketErrorStage::PacketDecode,
                                connection.state,
                                Some(#packet_type::get_id_const()),
                                Some(stringify!(#packet_type).to_string()),
                                error.to_string(),
                            );
                        inbound_packet_error_event.dispatch(context, connection);
                        return false;
                    }
                };

                let context = unsafe { &mut *(context_ptr as *mut #context_type) };
                #function_name(connection, packet, context)
            }

            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            static #static_metadata_name: #server_path::ServerPacketListener =
                #server_path::ServerPacketListener {
                    id: #packet_type::get_id_const(),
                    state: #packet_type::get_state_const(),
                    priority: #priority,
                    events: &[#(#events),*],
                    handler: #wrapper_function_name,
                    modules: &[#(#modules),*],
                };

            #event_path::inventory::submit! { &#static_metadata_name }
        })
    }

    fn packet_type_tokens(&self) -> syn::Result<TokenStream2> {
        let Some(FnArg::Typed(PatType { ty, .. })) = self.input_fn.sig.inputs.get(1) else {
            return Err(syn::Error::new_spanned(
                &self.input_fn.sig.ident,
                "packet listener must take three arguments: (connection, packet, context)",
            ));
        };

        Ok(quote! { #ty })
    }
}
