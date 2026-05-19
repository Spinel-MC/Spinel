use crate::ASSETS_DIRECTORY;
use crate::tag_registry_specs::{TagRegistrySpec, tag_registry_specs};
use serde::Deserialize;
use std::{collections::BTreeMap, fs, io};

const DYNAMIC_REGISTRY_TAGS_FILE: &str = "dynamic_registry_tags.json";
const STATIC_REGISTRY_TAGS_FILE: &str = "static_registry_tags.json";

pub(crate) fn dynamic_tags() -> io::Result<String> {
    let dynamic_tag_assets = RegistryTagAssets::load(DYNAMIC_REGISTRY_TAGS_FILE)?;
    let static_tag_assets = RegistryTagAssets::load(STATIC_REGISTRY_TAGS_FILE)?;
    let dynamic_registries = tag_registry_specs()
        .iter()
        .map(|registry| dynamic_registry_tags(registry, &dynamic_tag_assets))
        .collect::<Vec<_>>()
        .join(",\n");
    let static_registries = static_registry_specs()
        .iter()
        .map(|registry| static_registry_tags(registry, &static_tag_assets))
        .collect::<Vec<_>>()
        .join(",\n");
    Ok(format!(
        "use crate::registry_tags::{{dynamic_registry_tag, static_registry_tag}};\nimpl Registries {{\n    pub fn dynamic_tag_entries(&self) -> Result<Vec<RegistryTags>, RegistryTagError> {{\n        Ok(vec![\n{dynamic_registries}\n        ])\n    }}\n\n    pub fn static_tag_entries(&self) -> Result<Vec<RegistryTags>, RegistryTagError> {{\n        Ok(vec![\n{static_registries}\n        ])\n    }}\n}}\n"
    ))
}

fn dynamic_registry_tags(registry: &TagRegistrySpec, tag_assets: &RegistryTagAssets) -> String {
    let tags = tag_assets
        .tags(registry.tag_path)
        .iter()
        .map(|tag| dynamic_tag_entry(registry, tag))
        .collect::<Vec<_>>()
        .join(",\n");
    format!(
        "            RegistryTags::new({registry_const}, vec![\n{tags}\n            ])",
        registry_const = registry.registry_const
    )
}

fn static_registry_tags(
    registry: &StaticTagRegistrySpec,
    tag_assets: &RegistryTagAssets,
) -> String {
    let tags = tag_assets
        .tags(registry.tag_path)
        .iter()
        .map(|tag| static_tag_entry(registry, tag))
        .collect::<Vec<_>>()
        .join(",\n");
    format!(
        "            RegistryTags::new({registry_const}, vec![\n{tags}\n            ])",
        registry_const = registry.registry_const
    )
}

fn dynamic_tag_entry(registry: &TagRegistrySpec, tag: &TagEntry) -> String {
    let entries = tag
        .values
        .iter()
        .map(|entry| format!("\"{entry}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "                dynamic_registry_tag(&self.{field_name}, {registry_const}, \"{tag_name}\", &[{entries}])?",
        field_name = registry.field_name,
        registry_const = registry.registry_const,
        tag_name = tag.name
    )
}

fn static_tag_entry(registry: &StaticTagRegistrySpec, tag: &TagEntry) -> String {
    let entries = tag
        .values
        .iter()
        .map(|entry| format!("\"{entry}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "                static_registry_tag(&self.{field_name}, {registry_const}, \"{tag_name}\", &[{entries}])?",
        field_name = registry.field_name,
        registry_const = registry.registry_const,
        tag_name = tag.name
    )
}

#[derive(Deserialize)]
struct RegistryTagAssets(BTreeMap<String, BTreeMap<String, Vec<String>>>);

impl RegistryTagAssets {
    fn load(file_name: &str) -> io::Result<Self> {
        let path = format!("{ASSETS_DIRECTORY}/{file_name}");
        let json = fs::read_to_string(path)?;
        serde_json::from_str(&json).map_err(io::Error::other)
    }

    fn tags(&self, registry_path: &str) -> Vec<TagEntry> {
        self.0
            .get(registry_path)
            .into_iter()
            .flat_map(|tags| tags.iter())
            .map(|(name, values)| TagEntry {
                name: name.clone(),
                values: values.clone(),
            })
            .collect()
    }
}

struct TagEntry {
    name: String,
    values: Vec<String>,
}

struct StaticTagRegistrySpec {
    registry_const: &'static str,
    field_name: &'static str,
    tag_path: &'static str,
}

fn static_registry_specs() -> &'static [StaticTagRegistrySpec] {
    &[
        StaticTagRegistrySpec {
            registry_const: "BLOCKS_REGISTRY",
            field_name: "blocks",
            tag_path: "block",
        },
        StaticTagRegistrySpec {
            registry_const: "ITEM_REGISTRY",
            field_name: "items",
            tag_path: "item",
        },
    ]
}
