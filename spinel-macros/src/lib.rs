mod client;
mod common;
mod server;

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, PatType, Type, parse_macro_input};

use common::parsers::{AttrsParser, EventAttrParser};

// Routing Logic

#[proc_macro_attribute]
pub fn event_dispatcher(attr: TokenStream, item: TokenStream) -> TokenStream {
    common::event::event_dispatcher_logic(attr, item)
}

#[proc_macro_attribute]
pub fn event_listener(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let event_attrs = parse_macro_input!(attr as EventAttrParser);

    // Determine context based on the second argument (context arg)
    // Arg 0: event
    // Arg 1: context (Server or Client)
    let context_arg_index = 1;
    let context_type = if let Some(arg) = input_fn.sig.inputs.iter().nth(context_arg_index) {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            match &**ty {
                Type::Reference(type_ref) => type_ref.elem.clone(),
                _ => ty.clone(),
            }
        } else {
            panic!("#[event_listener] function's second argument must be typed");
        }
    } else {
        panic!("#[event_listener] function must take two arguments: (event, context)");
    };

    let event_struct_full_path =
        if let Some(FnArg::Typed(PatType { ty, .. })) = input_fn.sig.inputs.first() {
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

    let type_str = quote!(#context_type).to_string();
    if type_str.contains("MinecraftServer") || type_str.contains("Server") {
        server::event::generate(event_attrs, input_fn, event_struct_full_path, *context_type).into()
    } else {
        client::event::generate(event_attrs, input_fn, event_struct_full_path, *context_type).into()
    }
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, item: TokenStream) -> TokenStream {
    common::packet::definition::packet_struct_logic(attr, item)
}

#[deprecated(since = "0.1.0", note = "Use `#[packet]` instead")]
#[proc_macro_attribute]
pub fn packet_dispatcher(attr: TokenStream, item: TokenStream) -> TokenStream {
    common::packet::definition::packet_struct_logic(attr, item)
}

#[proc_macro_attribute]
pub fn packet_listener(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let packet_attrs = parse_macro_input!(attr as AttrsParser);
    let context_arg = input_fn
        .sig
        .inputs
        .last()
        .expect("Listener missing arguments");

    let context_type = if let FnArg::Typed(PatType { ty, .. }) = context_arg {
        match &**ty {
            Type::Reference(type_reference) => type_reference.elem.clone(),
            _ => ty.clone(),
        }
    } else {
        panic!("Context arg typed");
    };

    let connection_type = if let Some(arg) = input_fn.sig.inputs.first() {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            match &**ty {
                Type::Reference(type_reference) => type_reference.elem.clone(),
                _ => ty.clone(),
            }
        } else {
            panic!("Connection arg typed");
        }
    } else {
        panic!("Connection arg missing");
    };

    let context_type_str = quote!(#context_type).to_string();

    if context_type_str.contains("MinecraftServer") || context_type_str.contains("Server") {
        server::packet::listener::generate(packet_attrs, input_fn, *context_type, *connection_type)
            .into()
    } else {
        client::packet::listener::generate(packet_attrs, input_fn, *context_type, *connection_type)
            .into()
    }
}

#[proc_macro]
pub fn import_module(input: TokenStream) -> TokenStream {
    common::modules::import_module_logic(input)
}

#[proc_macro]
pub fn server_module(input: TokenStream) -> TokenStream {
    server::modules::server_module_logic(input)
}

#[proc_macro]
pub fn client_module(input: TokenStream) -> TokenStream {
    client::modules::client_module_logic(input)
}

#[proc_macro]
pub fn declare_namespace(input: TokenStream) -> TokenStream {
    common::namespace::declare_namespace_logic(input)
}

#[proc_macro]
pub fn declare_module_dependency(input: TokenStream) -> TokenStream {
    common::dependency::declare_module_dependency_logic(input)
}
