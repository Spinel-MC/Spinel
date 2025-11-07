use crate::parsers::EventAttrParser;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse::Parser, parse_macro_input};

pub fn event_dispatcher_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let event_attrs = parse_macro_input!(attr as EventAttrParser);
    let mut item_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &item_struct.ident;
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let event_name_lit = event_attrs.event;
    let client_accessor;

    let async_keyword = if event_attrs.r#async {
        quote! { async }
    } else {
        quote! {}
    };

    if event_attrs.with_client {
        let has_client_ptr = if let syn::Fields::Named(ref fields) = item_struct.fields {
            fields
                .named
                .iter()
                .any(|f| f.ident.as_ref().map_or(false, |i| i == "client_ptr"))
        } else {
            false
        };

        if !has_client_ptr {
            if let syn::Fields::Named(ref mut fields) = item_struct.fields {
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! {
                            #[doc(hidden)]
                            pub client_ptr: Option<usize>
                        })
                        .unwrap(),
                );
            } else {
                panic!(
                    "#[event_dispatcher(with_client = true)] can only be used with structs having named fields."
                );
            }
        }

        client_accessor = quote! {
            pub fn client(&mut self) -> &mut spinel::network::Client {
                let ptr = self.client_ptr.expect("client() called before event was dispatched.") as *mut spinel::network::Client;
                unsafe { &mut *ptr }
            }
        };
    } else {
        client_accessor = quote! {};
    }

    let client_param = if event_attrs.with_client {
        quote! { client: &mut spinel::network::Client }
    } else {
        quote! {}
    };

    let setup_client_ptr = if event_attrs.with_client {
        quote! { self.client_ptr = Some(client as *mut _ as usize); }
    } else {
        quote! {}
    };

    let dispatch_fn_header =
        quote! { pub #async_keyword fn dispatch<S>(&mut self, server: &mut S, #client_param) };

    let dispatch_body = quote! {
        #setup_client_ptr

        fn resolve_ambiguous_module(
            unqualified_name: &str,
            all_modules_map: &std::collections::HashMap<String, Vec<String>>,
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

        let mut all_module_names: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
        for module in spinel::internal::inventory::iter::<&'static spinel::internal::RegisteredModule>() {
            all_module_names.insert(module.name);
        }
        for dep in spinel::internal::inventory::iter::<&'static spinel::internal::RegisteredModuleDependency>() {
            all_module_names.insert(dep.subject_module);
            all_module_names.insert(dep.dependent_on);
        }

        let mut all_known_modules: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for qualified_name in all_module_names {
            let unqualified_name = qualified_name.split(':').last().unwrap_or(qualified_name).to_string();
            all_known_modules.entry(unqualified_name).or_default().push(qualified_name.to_string());
        }

        let explicitly_imported_raw: std::collections::HashSet<&'static str> = spinel::internal::inventory::iter::<&'static spinel::internal::RegisteredModule>()
            .map(|m| m.name)
            .collect();
        let mut explicitly_activated_qualified = std::collections::HashSet::new();
        for raw_name in explicitly_imported_raw {
            let resolved = resolve_ambiguous_module(raw_name, &all_known_modules);
            explicitly_activated_qualified.insert(resolved);
        }

        let mut dependency_graph: std::collections::HashMap<String, std::collections::HashSet<String>> = std::collections::HashMap::new();
        for dep in spinel::internal::inventory::iter::<&'static spinel::internal::RegisteredModuleDependency>() {
            let resolved_subject = resolve_ambiguous_module(dep.subject_module, &all_known_modules);
            let resolved_dependency = resolve_ambiguous_module(dep.dependent_on, &all_known_modules);
            dependency_graph.entry(resolved_subject).or_default().insert(resolved_dependency);
        }

        let mut resolved_modules: std::collections::HashSet<String> = explicitly_activated_qualified.clone();
        let mut queue: std::collections::VecDeque<String> = explicitly_activated_qualified.into_iter().collect();
        while let Some(current_module) = queue.pop_front() {
            if let Some(dependencies) = dependency_graph.get(&current_module) {
                for dep in dependencies {
                    if resolved_modules.insert(dep.clone()) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        let has_independent_listener = spinel::internal::inventory::iter::<&'static spinel::internal::RegisteredListener>()
            .any(|l| l.event_name == #event_name_lit && !l.dependent);

        let mut listeners: Vec<_> = spinel::internal::inventory::iter::<&'static spinel::internal::RegisteredListener>()
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

        listeners.sort_by_key(|l| std::cmp::Reverse(l.priority.to_order()));

        let server_ptr = server as *mut _ as *mut ();
        let self_ptr = self as *mut _ as *mut ();

        for listener in listeners {
            (listener.listener.call)(self_ptr, server_ptr);
        }
    };

    let generated_impl = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #client_accessor

            #dispatch_fn_header {
                #dispatch_body
            }
        }
    };

    let expanded = quote! {
        #item_struct
        #generated_impl
    };

    expanded.into()
}
