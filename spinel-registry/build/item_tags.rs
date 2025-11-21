use std::{collections::HashMap, fs, path::Path};

use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TagJson {
    values: Vec<String>,
}

fn read_all_tags(tag_dir: &str) -> HashMap<String, Vec<String>> {
    let mut tags = HashMap::new();

    fn read_directory(dir: &Path, base_path: &Path, tags: &mut HashMap<String, Vec<String>>) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                read_directory(&path, base_path, tags);
            } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let relative_path = path.strip_prefix(base_path).unwrap();
                let tag_name = relative_path
                    .with_extension("")
                    .to_str()
                    .unwrap()
                    .replace('\\', "/");

                if tag_name.contains("/_") || tag_name.starts_with('_') {
                    continue;
                }

                let content = fs::read_to_string(&path).unwrap();
                let tag: TagJson = serde_json::from_str(&content)
                    .unwrap_or_else(|e| panic!("Failed to parse {}: {}", tag_name, e));

                tags.insert(tag_name, tag.values);
            }
        }
    }

    let base_path = Path::new(tag_dir);
    read_directory(base_path, base_path, &mut tags);

    tags
}

fn resolve_tag(
    tag_name: &str,
    all_tags: &HashMap<String, Vec<String>>,
    resolved_cache: &mut HashMap<String, Vec<String>>,
    visiting: &mut Vec<String>,
) -> Vec<String> {
    if let Some(cached) = resolved_cache.get(tag_name) {
        return cached.clone();
    }

    if visiting.contains(&tag_name.to_string()) {
        panic!("Circular tag dependency detected: {:?}", visiting);
    }

    visiting.push(tag_name.to_string());

    let values = all_tags
        .get(tag_name)
        .unwrap_or_else(|| panic!("Tag not found: {}", tag_name));

    let mut resolved = Vec::new();

    for value in values {
        if let Some(nested_tag) = value.strip_prefix('#') {
            let nested_tag = nested_tag.strip_prefix("minecraft:").unwrap_or(nested_tag);

            let nested_values = resolve_tag(nested_tag, all_tags, resolved_cache, visiting);
            resolved.extend(nested_values);
        } else {
            let item_key = value.strip_prefix("minecraft:").unwrap_or(value);
            resolved.push(item_key.to_string());
        }
    }

    visiting.pop();

    let mut seen = std::collections::HashSet::new();
    resolved.retain(|x| seen.insert(x.clone()));

    resolved_cache.insert(tag_name.to_string(), resolved.clone());
    resolved
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=build_assets/datapacks/default/data/minecraft/tags/item/");

    let tag_dir = "build_assets/datapacks/default/data/minecraft/tags/item";
    let all_tags = read_all_tags(tag_dir);

    let mut resolved_tags: HashMap<String, Vec<String>> = HashMap::new();
    let mut resolved_cache = HashMap::new();

    for tag_name in all_tags.keys() {
        let mut visiting = Vec::new();
        let resolved = resolve_tag(tag_name, &all_tags, &mut resolved_cache, &mut visiting);
        resolved_tags.insert(tag_name.clone(), resolved);
    }

    let mut sorted_tags: Vec<_> = resolved_tags.into_iter().collect();
    sorted_tags.sort_by(|a, b| a.0.cmp(&b.0));

    let mut stream = TokenStream::new();

    stream.extend(quote! {
        use crate::items::ItemRegistry;
        use crate::types::Identifier;
    });

    for (tag_name, items) in &sorted_tags {
        let tag_ident = Ident::new(
            &format!("{}_TAG", tag_name.to_shouty_snake_case()),
            Span::call_site(),
        );

        let item_strs = items.iter().map(|s| s.as_str());

        stream.extend(quote! {
            pub const #tag_ident: &[&str] = &[#(#item_strs),*];
        });
    }

    let mut register_stream = TokenStream::new();
    for (tag_name, _) in &sorted_tags {
        let tag_ident = Ident::new(
            &format!("{}_TAG", tag_name.to_shouty_snake_case()),
            Span::call_site(),
        );
        let tag_key = tag_name.clone();

        register_stream.extend(quote! {
            registry.register_tag(
                Identifier::vanilla_static(#tag_key),
                #tag_ident
            );
        });
    }

    stream.extend(quote! {
        pub fn register_item_tags(registry: &mut ItemRegistry) {
            #register_stream
        }
    });

    stream
}
