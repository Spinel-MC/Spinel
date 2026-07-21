use crate::common::parsers::EventAttrParser;
use crate::common::utils::{get_base_path, resolve_priority_token};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{FnArg, ImplItem, ItemImpl, Meta, Type, TypePath, parse2};

pub fn generate(mut input_impl: ItemImpl) -> TokenStream2 {
    let server_path = get_base_path("server");
    let listener_type = input_impl.self_ty.clone();
    let mut event_registrations = Vec::new();

    for impl_item in input_impl.items.iter_mut() {
        let ImplItem::Fn(method) = impl_item else {
            continue;
        };

        let Some(attribute_index) = method
            .attrs
            .iter()
            .position(|attribute| attribute.path().is_ident("event_handler"))
        else {
            continue;
        };

        let attribute = method.attrs.remove(attribute_index);
        let event_attrs = match attribute.meta {
            Meta::Path(_) => EventAttrParser {
                event: None,
                with_client: false,
                with_server: false,
                priority: None,
                dependent: false,
                modules: Vec::new(),
                r#async: false,
            },
            Meta::List(meta_list) => parse2::<EventAttrParser>(meta_list.tokens)
                .unwrap_or_else(|error| panic!("invalid event_handler attribute: {error}")),
            Meta::NameValue(_) => panic!("invalid event_handler attribute"),
        };
        let priority = resolve_priority_token(event_attrs.priority, "Medium");
        let method_name = &method.sig.ident;
        let event_type = event_type_from_method(method);

        event_registrations.push(quote! {
            global_event_handler.add_listener_with_priority::<#event_type>(
                #priority,
                #listener_type::#method_name,
            );
        });
    }

    quote! {
        #input_impl

        impl #server_path::events::EventHandler for #listener_type {
            fn register_event_handlers(
                &self,
                global_event_handler: &mut #server_path::events::GlobalEventHandler,
            ) {
                #(#event_registrations)*
            }
        }
    }
}

fn event_type_from_method(method: &syn::ImplItemFn) -> TypePath {
    let Some(first_argument) = method.sig.inputs.first() else {
        panic!("#[event_handler] method must take an event argument");
    };

    let FnArg::Typed(event_argument) = first_argument else {
        panic!("#[event_handler] method must not take self");
    };

    let Type::Reference(event_reference) = &*event_argument.ty else {
        panic!("#[event_handler] event argument must be a mutable reference");
    };

    let Type::Path(event_type) = &*event_reference.elem else {
        panic!("#[event_handler] event argument must be a path type");
    };

    event_type.clone()
}
