use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
};

mod block_entries;
mod dynamic_registry_assets;
mod entity_entries;
mod item_entries;
mod sound_entries;
mod tag_entries;
mod tag_registry_specs;

use block_entries::{block_entries_by_key, sorted_block_entries};
use dynamic_registry_assets::DYNAMIC_REGISTRY_ASSETS;
use entity_entries::entity_entries;
use item_entries::item_entries;
use sound_entries::sound_entries;

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
        self.write("vanilla_entity_types.rs", self.entity_types()?);
        self.write("vanilla_sound_events.rs", self.sound_events()?);
        self.write("vanilla_materials.rs", self.materials()?);
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
                    "    let _ = registry.register(RegistryKey::vanilla_static(\"{}\"), Material::{});\n",
                    item.path,
                    const_name(&item.path)
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::{{Material, RegistryKey, StaticRegistry}};\npub fn register_items(registry: &mut StaticRegistry<Material>) {{\n{registrations}}}\n"
        ))
    }

    fn entity_types(&self) -> io::Result<String> {
        let entity_types = entity_entries()?;
        let constants = entity_types
            .iter()
            .map(|entity_type| {
                format!(
                    "    pub const {}: Self = Self::new({}, \"{}\", \"{}\", EntityPacketType::{}, {}, {}, {}, {}, {});\n",
                    const_name(&entity_type.path),
                    entity_type.id,
                    entity_type.path,
                    entity_type.translation_key,
                    entity_type.packet_type.to_upper_camel_case(),
                    rust_float(entity_type.width),
                    rust_float(entity_type.height),
                    rust_float(entity_type.eye_height),
                    entity_type.client_tracking_range,
                    entity_type.fire_immune
                )
            })
            .collect::<String>();
        let all = entity_types
            .iter()
            .map(|entity_type| format!("        Self::{},\n", const_name(&entity_type.path)))
            .collect::<String>();
        let attachment_tables = entity_types
            .iter()
            .map(|entity_type| self.entity_attachment_table(entity_type))
            .collect::<String>();
        let attachment_matches = entity_types
            .iter()
            .map(|entity_type| {
                format!(
                    "            Self::{} => {}_ATTACHMENTS,\n",
                    const_name(&entity_type.path),
                    const_name(&entity_type.path)
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::entity::{{EntityAttachmentOffset, EntityPacketType, EntityType}};\n{attachment_tables}impl EntityType {{\n{constants}    pub const ALL: &'static [Self] = &[\n{all}    ];\n    pub fn attachments(self) -> &'static [EntityAttachmentOffset] {{\n        match self {{\n{attachment_matches}            _ => &[],\n        }}\n    }}\n}}\n"
        ))
    }

    fn sound_events(&self) -> io::Result<String> {
        let sound_events = sound_entries()?;
        let constants = sound_events
            .iter()
            .map(|sound_event| {
                format!(
                    "    pub const {}: Self = Self::new({}, Identifier::vanilla_static(\"{}\"));\n",
                    const_name(&sound_event.name),
                    sound_event.id,
                    sound_event.name
                )
            })
            .collect::<String>();
        let id_matches = sound_events
            .iter()
            .map(|sound_event| {
                format!(
                    "            {} => Some(Self::{}),\n",
                    sound_event.id,
                    const_name(&sound_event.name)
                )
            })
            .collect::<String>();
        let key_matches = sound_events
            .iter()
            .map(|sound_event| {
                format!(
                    "            \"{}\" => Some(Self::{}),\n",
                    sound_event.name,
                    const_name(&sound_event.name)
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::{{BuiltinSoundEvent, Identifier}};\nimpl BuiltinSoundEvent {{\n{constants}    pub fn from_id(id: i32) -> Option<Self> {{\n        match id {{\n{id_matches}            _ => None,\n        }}\n    }}\n    pub fn from_key(key: &Identifier) -> Option<Self> {{\n        match key.path.as_ref() {{\n{key_matches}            _ => None,\n        }}\n    }}\n}}\n"
        ))
    }

    fn entity_attachment_table(&self, entity_type: &entity_entries::EntityEntry) -> String {
        let attachments = entity_type
            .attachments
            .iter()
            .filter(|(_, offset)| offset.len() == 3)
            .map(|(name, offset)| {
                format!(
                    "    EntityAttachmentOffset::new(\"{}\", {}, {}, {}),\n",
                    name,
                    rust_float(offset[0]),
                    rust_float(offset[1]),
                    rust_float(offset[2])
                )
            })
            .collect::<String>();
        format!(
            "const {}_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[\n{}];\n",
            const_name(&entity_type.path),
            attachments
        )
    }

    fn materials(&self) -> io::Result<String> {
        let items = item_entries()?;
        let block_paths = block_entries_by_key()?
            .into_iter()
            .map(|block| block.path)
            .collect::<BTreeSet<_>>();
        let constants = items
            .iter()
            .map(|item| {
                let block = item
                    .block_item
                    .as_ref()
                    .or_else(|| block_paths.get(&item.path))
                    .map(|block| format!("Some(Block::{})", const_name(block)))
                    .unwrap_or_else(|| "None".to_string());
                format!(
                    "    pub const {}: Self = Self::new({}, Identifier::vanilla_static(\"{}\"), {});\n",
                    const_name(&item.path),
                    item.id,
                    item.path,
                    block
                )
            })
            .collect::<String>();
        let id_matches = items
            .iter()
            .map(|item| {
                format!(
                    "            {} => Some(Self::{}),\n",
                    item.id,
                    const_name(&item.path)
                )
            })
            .collect::<String>();
        let key_matches = items
            .iter()
            .map(|item| {
                format!(
                    "            \"{}\" => Some(Self::{}),\n",
                    item.path,
                    const_name(&item.path)
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::{{Identifier, Material}};\nuse crate::vanilla_world_blocks::Block;\nimpl Material {{\n{constants}    pub fn from_id(id: i32) -> Option<Self> {{\n        match id {{\n{id_matches}            _ => None,\n        }}\n    }}\n    pub fn from_key(key: &str) -> Option<Self> {{\n        let material_path = key.strip_prefix(\"minecraft:\").unwrap_or(key);\n        match material_path {{\n{key_matches}            _ => None,\n        }}\n    }}\n}}\n"
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

fn rust_float(value: f64) -> String {
    let value_text = value.to_string();
    if value_text.contains('.') {
        return value_text;
    }
    format!("{value_text}.0")
}
