mod parsers;
mod util;

mod event_dispatcher;
mod event_listener;
mod packet_dispatcher;
mod packet_listener;
mod import_module;
mod namespace;
mod module_dependency;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn event_dispatcher(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::event_dispatcher::event_dispatcher_logic(attr, item)
}

#[proc_macro_attribute]
pub fn event_listener(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::event_listener::event_listener_logic(attr, item)
}

#[proc_macro_attribute]
pub fn packet_dispatcher(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::packet_dispatcher::packet_dispatcher_logic(attr, item)
}

#[proc_macro_attribute]
pub fn packet_listener(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::packet_listener::packet_listener_logic(attr, item)
}

#[proc_macro]
pub fn import_module(input: TokenStream) -> TokenStream {
    crate::import_module::import_module_logic(input)
}

#[proc_macro]
pub fn declare_namespace(input: TokenStream) -> TokenStream {
    crate::namespace::declare_namespace_logic(input)
}

#[proc_macro]
pub fn declare_module_dependency(input: TokenStream) -> TokenStream {
    crate::module_dependency::declare_module_dependency_logic(input)
}