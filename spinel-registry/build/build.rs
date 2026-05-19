use heck::{ToShoutySnakeCase, ToSnakeCase};
use std::{collections::BTreeMap, fs, io};

mod block_entries;
mod dynamic_registry_assets;
mod item_entries;
mod tag_entries;
mod tag_registry_specs;

use block_entries::{block_entries_by_key, sorted_block_entries};
use dynamic_registry_assets::DYNAMIC_REGISTRY_ASSETS;
use item_entries::item_entries;

const GENERATED_DIRECTORY: &str = "src/generated";
const ASSETS_DIRECTORY: &str = "assets";
const DYNAMIC_REGISTRY_KEYS_FILE: &str = "assets/dynamic_registry_keys.json";

fn main() {
    BuildScript.run().unwrap_or_else(|error| {
        panic!("registry build failed: {error}");
    });
}

struct BuildScript;

impl BuildScript {
    fn run(self) -> io::Result<()> {
        self.emit_rerun_instructions();
        fs::create_dir_all(GENERATED_DIRECTORY)?;
        self.write("vanilla_world_blocks.rs", self.world_blocks()?);
        self.write("vanilla_blocks.rs", self.static_blocks()?);
        self.write("vanilla_items.rs", self.static_items()?);
        self.write(
            "vanilla_biomes.rs",
            self.dynamic_registry("biomes", "Biome")?,
        );
        self.write(
            "vanilla_dimension_types.rs",
            self.dynamic_registry("dimension_types", "DimensionType")?,
        );
        self.write("vanilla_dynamic_tags.rs", tag_entries::dynamic_tags()?);
        self.write_dynamic_registry_modules()
    }

    fn emit_rerun_instructions(&self) {
        println!("cargo:rerun-if-changed=build/build.rs");
        println!("cargo:rerun-if-changed={ASSETS_DIRECTORY}");
    }

    fn write(&self, module_name: &str, contents: String) {
        let output_path = format!("{GENERATED_DIRECTORY}/{module_name}");
        fs::write(output_path, contents).unwrap_or_else(|error| {
            panic!("failed to write {module_name}: {error}");
        });
    }

    fn world_blocks(&self) -> io::Result<String> {
        let blocks = block_entries_by_key()?;
        let variants = blocks
            .iter()
            .map(|block| format!("    {},\n", block.variant))
            .collect::<String>();
        let state_ids = blocks
            .iter()
            .map(|block| {
                format!(
                    "            Self::{} => {},\n",
                    block.variant, block.state_id
                )
            })
            .collect::<String>();
        let paths = blocks
            .iter()
            .map(|block| {
                format!(
                    "            Self::{} => \"{}\",\n",
                    block.variant, block.path
                )
            })
            .collect::<String>();
        let all = blocks
            .iter()
            .map(|block| format!("            Self::{},\n", block.variant))
            .collect::<String>();
        Ok(format!(
            "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\npub enum Block {{\n{variants}}}\nimpl Block {{\n    pub const ALL: &'static [Self] = &[\n{all}    ];\n    pub const fn state_id(self) -> i32 {{\n        match self {{\n{state_ids}        }}\n    }}\n    pub const fn path(self) -> &'static str {{\n        match self {{\n{paths}        }}\n    }}\n    pub const fn from_state_id(state_id: i32) -> Option<Self> {{\n        let mut block_index = 0usize;\n        while block_index < Self::ALL.len() {{\n            let block = Self::ALL[block_index];\n            if block.state_id() == state_id {{\n                return Some(block);\n            }}\n            block_index += 1;\n        }}\n        None\n    }}\n}}\n"
        ))
    }

    fn static_blocks(&self) -> io::Result<String> {
        let registrations = sorted_block_entries()?
            .iter()
            .map(|block| {
                format!(
                    "    let _ = registry.register(RegistryKey::vanilla_static(\"{}\"), Block::{});\n",
                    block.path, block.variant
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::{{RegistryKey, StaticRegistry}};\nuse crate::vanilla_world_blocks::Block;\npub fn register_blocks(registry: &mut StaticRegistry<Block>) {{\n{registrations}}}\n"
        ))
    }

    fn dynamic_registry(&self, asset_name: &str, type_name: &str) -> io::Result<String> {
        let asset_path = format!("{ASSETS_DIRECTORY}/{asset_name}.json");
        let entries = self.extracted_registry_keys(&asset_path, asset_name)?;
        Ok(self.dynamic_module(type_name, &entries))
    }

    fn dynamic_registry_asset(&self, registry_path: &str, type_name: &str) -> io::Result<String> {
        Ok(self.dynamic_module(type_name, &self.dynamic_registry_keys(registry_path)?))
    }

    fn dynamic_module(&self, type_name: &str, keys: &[String]) -> String {
        let constants = keys
            .iter()
            .map(|key| {
                format!(
                    "    pub const {}: RegistryKey<Self> = RegistryKey::vanilla_static(\"{}\");\n",
                    const_name(key),
                    vanilla_path(key)
                )
            })
            .collect::<String>();
        let registrations = keys
            .iter()
            .map(|key| format!("    let _ = registry.register_vanilla({type_name}::{}, {type_name}::default());\n", const_name(key)))
            .collect::<String>();
        let function_name = format!("register_{}", plural_snake(type_name));
        format!(
            "use crate::{{DynamicRegistry, RegistryKey}};\nuse crate::{}::{type_name};\nimpl {type_name} {{\n{constants}}}\npub fn {function_name}(registry: &mut DynamicRegistry<{type_name}>) {{\n{registrations}}}\n",
            type_name.to_snake_case()
        )
    }

    fn static_items(&self) -> io::Result<String> {
        let registrations = item_entries()?
            .iter()
            .map(|item| {
                format!(
                    "    let _ = registry.register(RegistryKey::vanilla_static(\"{}\"), Item::new(Identifier::vanilla_static(\"{}\")));\n",
                    item.path, item.path
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::{{Identifier, RegistryKey, StaticRegistry}};\nuse crate::items::Item;\npub fn register_items(registry: &mut StaticRegistry<Item>) {{\n{registrations}}}\n"
        ))
    }

    fn write_dynamic_registry_modules(&self) -> io::Result<()> {
        DYNAMIC_REGISTRY_ASSETS.iter().try_for_each(|registry| {
            let contents = self.dynamic_registry_asset(registry.path, registry.type_name)?;
            self.write(registry.module_name, contents);
            Ok(())
        })
    }

    fn extracted_registry_keys(&self, path: &str, field_name: &str) -> io::Result<Vec<String>> {
        let json = fs::read_to_string(path)?;
        let registries: BTreeMap<String, BTreeMap<String, serde_json::Value>> =
            serde_json::from_str(&json).map_err(io::Error::other)?;
        Ok(registries
            .get(field_name)
            .map(|entries| entries.keys().cloned().collect())
            .unwrap_or_default())
    }

    fn dynamic_registry_keys(&self, registry_path: &str) -> io::Result<Vec<String>> {
        let json = fs::read_to_string(DYNAMIC_REGISTRY_KEYS_FILE)?;
        let registries: BTreeMap<String, Vec<String>> =
            serde_json::from_str(&json).map_err(io::Error::other)?;
        Ok(registries.get(registry_path).cloned().unwrap_or_default())
    }
}

fn vanilla_path(key: &str) -> &str {
    key.strip_prefix("minecraft:").unwrap_or(key)
}

fn plural_snake(type_name: &str) -> String {
    format!("{}s", type_name.to_snake_case())
}

fn const_name(key: &str) -> String {
    let name = vanilla_path(key).to_shouty_snake_case();
    if name
        .chars()
        .next()
        .is_some_and(|first_char| first_char.is_ascii_digit())
    {
        return format!("_{name}");
    }
    name
}
