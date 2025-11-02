use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemFn, PatType, Type};
use crate::parsers::EventAttrParser;
use crate::util::resolve_priority_token;

pub fn event_listener_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let event_attrs = parse_macro_input!(attr as EventAttrParser);
    let fn_ident = &input_fn.sig.ident;
    let wrapper_fn_ident = format_ident!("__spinel_event_wrapper_{}", fn_ident);

    let event_name_lit = event_attrs.event;
    let dependent_lit = event_attrs.dependent;
    
    let modules = event_attrs.modules;
    let modules_slice = quote! { &[#(#modules),*] };

    let priority = resolve_priority_token(event_attrs.priority, "Priority", "Medium");

    let event_struct_full_path = if let Some(FnArg::Typed(PatType { ty, .. })) = input_fn.sig.inputs.first() {
        if let Type::Reference(type_ref) = &**ty {
            if let Type::Path(path_ty) = &*type_ref.elem {
                quote! { #path_ty }
            } else {
                panic!("#[event_listener] function's event argument must be a path type")
            }
        } else {
            panic!("#[event_listener] function's first argument must be a mutable reference")
        }
    } else {
        panic!("#[event_listener] function must take at least one argument")
    };

    let wrapper_fn = quote! {
        #[doc(hidden)]
        fn #wrapper_fn_ident(event_ptr: *mut (), server_ptr: *mut ()) {
            let event = unsafe { &mut *(event_ptr as *mut #event_struct_full_path) };
            let server = unsafe { &mut *(server_ptr as *mut spinel::core::server::MinecraftServer) };
            #fn_ident(event, server);
        }
    };

    let static_item_name = format_ident!("__SPINEL_LISTENER_{}", fn_ident);
    
    let mut final_output = TokenStream2::new();

    final_output.extend(quote! {
        #input_fn
        #wrapper_fn

        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static #static_item_name: spinel::internal::RegisteredListener =
            spinel::internal::RegisteredListener {
                event_name: #event_name_lit,
                listener: spinel::events::ListenerFn { call: #wrapper_fn_ident },
                priority: #priority,
                dependent: #dependent_lit,
                modules: #modules_slice,
            };
        inventory::submit! { &#static_item_name }
    });

    let static_event_reg_name = format_ident!("__SPINEL_EVENT_REGISTRATION_{}", fn_ident);
    final_output.extend(quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static #static_event_reg_name: spinel::internal::RegisteredEvent =
            spinel::internal::RegisteredEvent {
                name: #event_name_lit,
                is_independent: !#dependent_lit,
            };
        inventory::submit! { &#static_event_reg_name }
    });
    
    
    final_output.into()
}