use crate::common::parsers::EventAttrParser;
use crate::common::utils::{get_base_path, resolve_priority_token};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{ItemFn, Type};

pub fn generate(
    event_attrs: EventAttrParser,
    input_fn: ItemFn,
    event_struct_full_path: TokenStream2,
    context_type: Type,
) -> TokenStream2 {
    let fn_ident = &input_fn.sig.ident;
    let wrapper_fn_ident = format_ident!("__spinel_event_wrapper_{}", fn_ident);
    let event_lib_path = get_base_path("events");

    let event_name_lit = if let Some(name) = event_attrs.event {
        quote! { #name }
    } else {
        quote! { <#event_struct_full_path as #event_lib_path::Event>::NAME }
    };

    let dependent_lit = event_attrs.dependent;
    let modules = event_attrs.modules;
    let modules_slice = quote! { &[#(#modules),*] };
    let priority = resolve_priority_token(event_attrs.priority, "Priority", "Medium");

    let wrapper_fn = quote! {
        #[doc(hidden)]
        fn #wrapper_fn_ident(event_ptr: *mut (), client_ptr: *mut ()) {
            let event = unsafe { &mut *(event_ptr as *mut #event_struct_full_path) };
            let client = unsafe { &mut *(client_ptr as *mut #context_type) };
            #fn_ident(event, client);
        }
    };

    let static_item_name = format_ident!("__SPINEL_LISTENER_{}", fn_ident);

    let mut final_output = TokenStream2::new();

    final_output.extend(quote! {
        #input_fn
        #wrapper_fn

        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static #static_item_name: #event_lib_path::RegisteredListener =
            #event_lib_path::RegisteredListener {
                event_name: #event_name_lit,
                listener: #event_lib_path::ListenerFn { call: #wrapper_fn_ident },
                priority: #priority,
                dependent: #dependent_lit,
                modules: #modules_slice,
            };
        #event_lib_path::inventory::submit! { &#static_item_name }
    });

    let static_event_reg_name = format_ident!("__SPINEL_EVENT_REGISTRATION_{}", fn_ident);
    final_output.extend(quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static #static_event_reg_name: #event_lib_path::RegisteredEvent =
            #event_lib_path::RegisteredEvent {
                name: #event_name_lit,
                is_independent: !#dependent_lit,
            };
        #event_lib_path::inventory::submit! { &#static_event_reg_name }
    });

    final_output
}
