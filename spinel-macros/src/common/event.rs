use crate::common::parsers::EventAttrParser;
use crate::common::utils::{get_base_path, to_snake_case};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Field, Fields, ItemStruct, LitStr, parse::Parser, parse_macro_input};

pub fn event_dispatcher_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let event_attrs = parse_macro_input!(attr as EventAttrParser);
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    let mut connection_accessor = quote! {};
    let mut connection_param = quote! {};
    let mut setup_connection_ptr = quote! {};

    let is_server_side = event_attrs.with_client;
    let is_client_side = event_attrs.with_server;

    let event_lib_path = get_base_path("events");

    if is_server_side {
        let server_path = get_base_path("server");
        let connection_type = quote! { #server_path::network::Client };
        inject_pointer_field(&mut item_struct);

        connection_accessor = quote! {
            pub fn client(&mut self) -> &mut #connection_type {
                let ptr = self.connection_ptr.expect("client() called before event was dispatched.") as *mut #connection_type;
                unsafe { &mut *ptr }
            }
        };
        connection_param = quote! { connection: &mut #connection_type };
        setup_connection_ptr =
            quote! { self.connection_ptr = Some(connection as *mut _ as usize); };
    } else if is_client_side {
        let client_path = get_base_path("client");
        let connection_type = quote! { #client_path::network::Server };
        inject_pointer_field(&mut item_struct);

        connection_accessor = quote! {
            pub fn server(&mut self) -> &mut #connection_type {
                let ptr = self.connection_ptr.expect("server() called before event was dispatched.") as *mut #connection_type;
                unsafe { &mut *ptr }
            }
        };
        connection_param = quote! { connection: &mut #connection_type };
        setup_connection_ptr =
            quote! { self.connection_ptr = Some(connection as *mut _ as usize); };
    }

    let struct_name = &item_struct.ident;
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let event_name_lit = if let Some(name) = event_attrs.event {
        name
    } else {
        let name_str = struct_name.to_string();
        let name_stripped = if name_str.ends_with("Event") {
            &name_str[..name_str.len() - 5]
        } else {
            &name_str
        };
        let snake_case_name = to_snake_case(name_stripped);
        LitStr::new(&snake_case_name, struct_name.span())
    };

    let async_keyword = if event_attrs.r#async {
        quote! { async }
    } else {
        quote! {}
    };

    let dispatch_fn_header =
        quote! { pub #async_keyword fn dispatch<C>(&mut self, context: &mut C, #connection_param) };

    let dispatch_body = quote! {
        #setup_connection_ptr

        fn resolve_ambiguous_module(
            unqualified_name: &str,
            all_modules_map: &::std::collections::HashMap<String, Vec<String>>,
        ) -> String {
            if unqualified_name.contains(':') {
                return unqualified_name.to_string();
            }
            match all_modules_map.get(unqualified_name) {
                None => format!("unknown:{}", unqualified_name),
                Some(candidates) => {
                    if candidates.len() == 1 {
                        candidates[0].clone()
                    } else {
                        let minecraft_candidate = format!("minecraft:{}", unqualified_name);
                        if candidates.iter().any(|c| c == &minecraft_candidate) {
                            minecraft_candidate
                        } else {
                            panic!(
                                "Module name '{}' is ambiguous. Candidates found: {:?}. Please specify a namespace (e.g., 'namespace:{}').",
                                unqualified_name, candidates, unqualified_name
                            );
                        }
                    }
                }
            }
        }

        let mut all_module_names: ::std::collections::HashSet<&'static str> = ::std::collections::HashSet::new();

        for module in #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredModule>() {
            all_module_names.insert(module.name);
        }

        #[cfg(feature = "server")]
        for module in #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredServerModule>() {
            all_module_names.insert(module.name);
        }

        #[cfg(feature = "client")]
        for module in #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredClientModule>() {
            all_module_names.insert(module.name);
        }

        for dep in #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredModuleDependency>() {
            all_module_names.insert(dep.subject_module);
            all_module_names.insert(dep.dependent_on);
        }

        let mut all_known_modules: ::std::collections::HashMap<String, Vec<String>> = ::std::collections::HashMap::new();
        for qualified_name in all_module_names {
            let unqualified_name = qualified_name.split(':').last().unwrap_or(qualified_name).to_string();
            all_known_modules.entry(unqualified_name).or_default().push(qualified_name.to_string());
        }

        let mut explicitly_imported_raw: ::std::collections::HashSet<&'static str> = #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredModule>()
            .map(|m| m.name)
            .collect();

        #[cfg(feature = "server")]
        {
            explicitly_imported_raw.extend(
                #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredServerModule>()
                    .map(|m| m.name)
            );
        }

        #[cfg(feature = "client")]
        {
            explicitly_imported_raw.extend(
                #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredClientModule>()
                    .map(|m| m.name)
            );
        }

        let mut explicitly_activated_qualified = ::std::collections::HashSet::new();
        for raw_name in explicitly_imported_raw {
            let resolved = resolve_ambiguous_module(raw_name, &all_known_modules);
            explicitly_activated_qualified.insert(resolved);
        }

        let mut dependency_graph: ::std::collections::HashMap<String, ::std::collections::HashSet<String>> = ::std::collections::HashMap::new();
        for dep in #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredModuleDependency>() {
            let resolved_subject = resolve_ambiguous_module(dep.subject_module, &all_known_modules);
            let resolved_dependency = resolve_ambiguous_module(dep.dependent_on, &all_known_modules);
            dependency_graph.entry(resolved_subject).or_default().insert(resolved_dependency);
        }

        let mut resolved_modules: ::std::collections::HashSet<String> = explicitly_activated_qualified.clone();
        let mut queue: ::std::collections::VecDeque<String> = explicitly_activated_qualified.into_iter().collect();
        while let Some(current_module) = queue.pop_front() {
            if let Some(dependencies) = dependency_graph.get(&current_module) {
                for dep in dependencies {
                    if resolved_modules.insert(dep.clone()) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        let has_independent_listener = #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredListener>()
            .any(|l| l.event_name == #event_name_lit && !l.dependent);

        let mut listeners: Vec<_> = #event_lib_path::inventory::iter::<&'static #event_lib_path::RegisteredListener>()
            .filter(|l| {
                let name_ok = l.event_name == #event_name_lit;
                let dependency_ok = !l.dependent || has_independent_listener;
                let module_ok = l.modules.is_empty() || l.modules.iter().any(|unqualified_name| {
                    let resolved_name = resolve_ambiguous_module(unqualified_name, &all_known_modules);
                    resolved_modules.contains(&resolved_name)
                });

                name_ok && dependency_ok && module_ok
            })
            .collect();

        listeners.sort_by_key(|l| ::std::cmp::Reverse(l.priority.to_order()));

        let context_ptr = context as *mut _ as *mut ();
        let self_ptr = self as *mut _ as *mut ();

        for listener in listeners {
            (listener.listener.call)(self_ptr, context_ptr);
        }
    };

    let generated_impl = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #connection_accessor

            #dispatch_fn_header {
                #dispatch_body
            }
        }
    };

    let event_impl = quote! {
        impl #event_lib_path::Event for #struct_name {
            const NAME: &'static str = #event_name_lit;
        }
    };

    let expanded = quote! {
        #item_struct
        #generated_impl
        #event_impl
    };

    expanded.into()
}

fn inject_pointer_field(item_struct: &mut ItemStruct) {
    let has_connection_ptr = if let Fields::Named(ref fields) = item_struct.fields {
        fields
            .named
            .iter()
            .any(|f| f.ident.as_ref().map_or(false, |i| i == "connection_ptr"))
    } else {
        false
    };

    if !has_connection_ptr {
        if let Fields::Named(ref mut fields) = item_struct.fields {
            fields.named.push(
                Field::parse_named
                    .parse2(quote! {
                        #[doc(hidden)]
                        pub connection_ptr: Option<usize>
                    })
                    .unwrap(),
            );
        } else {
            panic!(
                "#[event_dispatcher] with connection injection can only be used with structs having named fields."
            );
        }
    }
}
