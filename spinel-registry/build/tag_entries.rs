use crate::ASSETS_DIRECTORY;
use crate::tag_registry_specs::{TagRegistrySpec, tag_registry_specs};
use serde::Deserialize;
use std::{collections::BTreeMap, fs, io};

const DYNAMIC_REGISTRY_TAGS_FILE: &str = "dynamic_registry_tags.json";
const STATIC_REGISTRY_TAGS_FILE: &str = "static_registry_tags.json";

pub(crate) fn dynamic_tags() -> io::Result<String> {
    let dynamic_tag_assets = RegistryTagAssets::load(DYNAMIC_REGISTRY_TAGS_FILE)?;
    let static_tag_assets = RegistryTagAssets::load(STATIC_REGISTRY_TAGS_FILE)?;
    let dynamic_tables = tag_registry_specs()
        .iter()
        .map(|registry| {
            tag_table(
                registry.table_const(),
                &dynamic_tag_assets.tags(registry.tag_path),
            )
        })
        .collect::<String>();
    let static_tables = static_registry_specs()
        .iter()
        .map(|registry| {
            tag_table(
                registry.table_const(),
                &static_tag_assets.tags(registry.tag_path),
            )
        })
        .collect::<String>();
    let dynamic_entries = tag_registry_specs()
        .iter()
        .map(dynamic_registry_entry)
        .collect::<String>();
    let static_entries = static_registry_specs()
        .iter()
        .map(static_registry_entry)
        .collect::<String>();
    Ok(format!(
        "use crate::registry_tags::{{dynamic_registry_tags, static_registry_tags}};\n\ntype GeneratedTagTable = &'static [(&'static str, &'static [&'static str])];\n\n{dynamic_tables}{static_tables}impl Registries {{\n    pub fn dynamic_tag_entries(&self) -> Result<Vec<RegistryTags>, RegistryTagError> {{\n        Ok(vec![\n{dynamic_entries}        ])\n    }}\n\n    pub fn static_tag_entries(&self) -> Result<Vec<RegistryTags>, RegistryTagError> {{\n        Ok(vec![\n{static_entries}        ])\n    }}\n}}\n"
    ))
}

fn tag_table(table_const: String, tags: &[TagEntry]) -> String {
    let entries = tags.iter().map(tag_table_entry).collect::<String>();
    format!("const {table_const}: GeneratedTagTable = &[\n{entries}];\n\n")
}

fn tag_table_entry(tag: &TagEntry) -> String {
    let values = tag
        .values
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("    (\"{}\", &[\n        {values}\n    ]),\n", tag.name)
}

fn dynamic_registry_entry(registry: &TagRegistrySpec) -> String {
    format!(
        "            RegistryTags::new({registry_const}, dynamic_registry_tags(&self.{field_name}, {registry_const}, {table_const})?),\n",
        field_name = registry.field_name,
        registry_const = registry.registry_const,
        table_const = registry.table_const()
    )
}

fn static_registry_entry(registry: &StaticTagRegistrySpec) -> String {
    format!(
        "            RegistryTags::new({registry_const}, static_registry_tags(&self.{field_name}, {registry_const}, {table_const})?),\n",
        field_name = registry.field_name,
        registry_const = registry.registry_const,
        table_const = registry.table_const()
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

impl StaticTagRegistrySpec {
    fn table_const(&self) -> String {
        format!("{}_TAGS", self.registry_const)
    }
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
