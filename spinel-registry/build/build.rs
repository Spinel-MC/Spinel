use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
};

mod block_entries;
mod damage_type_entries;
mod dynamic_registry_assets;
mod entity_entries;
mod item_entries;
mod mob_effect_entries;
mod sound_entries;
mod static_protocol_entries;
mod tag_entries;
mod tag_registry_specs;

use block_entries::{
    BlockShapeBoxEntry, BlockStateEntry, block_entries_by_key, block_shapes, sorted_block_entries,
    sorted_block_state_entries,
};
use dynamic_registry_assets::DYNAMIC_REGISTRY_ASSETS;
use entity_entries::entity_entries;
use item_entries::item_entries;
use mob_effect_entries::mob_effect_entries;
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
        self.write("vanilla_block_states.rs", self.block_states()?);
        self.write("vanilla_blocks.rs", self.static_blocks()?);
        self.write("vanilla_items.rs", self.static_items()?);
        self.write("vanilla_entity_types.rs", self.entity_types()?);
        self.write("vanilla_sound_events.rs", self.sound_events()?);
        self.write(
            "vanilla_villager_types.rs",
            self.static_protocol_values("villager_type", "VillagerType")?,
        );
        self.write(
            "vanilla_villager_professions.rs",
            self.static_protocol_values("villager_profession", "VillagerProfession")?,
        );
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
        let ids = blocks
            .iter()
            .map(|block| format!("            Self::{} => {},\n", block.variant, block.id))
            .collect::<String>();
        let id_matches = blocks
            .iter()
            .map(|block| {
                format!(
                    "            {} => Some(Self::{}),\n",
                    block.id, block.variant
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
        let path_matches = blocks
            .iter()
            .map(|block| {
                format!(
                    "            \"{}\" => Some(Self::{}),\n",
                    block.path, block.variant
                )
            })
            .collect::<String>();
        let air_flags = blocks
            .iter()
            .map(|block| format!("            Self::{} => {},\n", block.variant, block.is_air))
            .collect::<String>();
        let solid_flags = blocks
            .iter()
            .map(|block| {
                format!(
                    "            Self::{} => {},\n",
                    block.variant, block.is_solid
                )
            })
            .collect::<String>();
        let liquid_flags = blocks
            .iter()
            .map(|block| {
                format!(
                    "            Self::{} => {},\n",
                    block.variant, block.is_liquid
                )
            })
            .collect::<String>();
        let replaceable_flags = blocks
            .iter()
            .map(|block| {
                format!(
                    "            Self::{} => {},\n",
                    block.variant, block.is_replaceable
                )
            })
            .collect::<String>();
        let hardness_values = blocks
            .iter()
            .map(|block| {
                format!(
                    "            Self::{} => {},\n",
                    block.variant,
                    rust_f32(block.hardness)
                )
            })
            .collect::<String>();
        let friction_values = blocks
            .iter()
            .map(|block| {
                format!(
                    "            Self::{} => {},\n",
                    block.variant,
                    rust_f32(block.friction)
                )
            })
            .collect::<String>();
        let requires_tool_flags = blocks
            .iter()
            .map(|block| {
                format!(
                    "            Self::{} => {},\n",
                    block.variant, block.requires_tool
                )
            })
            .collect::<String>();
        let block_entity_types = blocks
            .iter()
            .map(|block| {
                let block_entity_type = block
                    .block_entity_type
                    .as_deref()
                    .map(|path| format!("Some(BlockEntityType::{})", path.to_upper_camel_case()))
                    .unwrap_or_else(|| "None".to_string());
                format!(
                    "            Self::{} => {},\n",
                    block.variant, block_entity_type
                )
            })
            .collect::<String>();
        let all = blocks
            .iter()
            .map(|block| format!("            Self::{},\n", block.variant))
            .collect::<String>();
        Ok(format!(
            "use crate::{{block_entity_type::BlockEntityType, vanilla_block_states::BlockState}};\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\npub enum Block {{\n{variants}}}\nimpl Block {{\n    pub const ALL: &'static [Self] = &[\n{all}    ];\n    pub const fn id(self) -> i32 {{\n        match self {{\n{ids}        }}\n    }}\n    pub const fn state_id(self) -> i32 {{\n        match self {{\n{state_ids}        }}\n    }}\n    pub const fn default_state(self) -> BlockState {{\n        BlockState::from_state_id_or_panic(self.state_id())\n    }}\n    pub const fn path(self) -> &'static str {{\n        match self {{\n{paths}        }}\n    }}\n    pub const fn is_air(self) -> bool {{\n        match self {{\n{air_flags}        }}\n    }}\n    pub const fn is_solid(self) -> bool {{\n        match self {{\n{solid_flags}        }}\n    }}\n    pub const fn is_liquid(self) -> bool {{\n        match self {{\n{liquid_flags}        }}\n    }}\n    pub const fn is_replaceable(self) -> bool {{\n        match self {{\n{replaceable_flags}        }}\n    }}\n    pub const fn hardness(self) -> f32 {{\n        match self {{\n{hardness_values}        }}\n    }}\n    pub const fn friction(self) -> f32 {{\n        match self {{\n{friction_values}        }}\n    }}\n    pub const fn requires_tool(self) -> bool {{\n        match self {{\n{requires_tool_flags}        }}\n    }}\n    pub const fn block_entity_type(self) -> Option<BlockEntityType> {{\n        match self {{\n{block_entity_types}        }}\n    }}\n    pub const fn emitted_light_level(self) -> u8 {{ self.default_state().light_emission() }}\n    pub const fn from_id(id: i32) -> Option<Self> {{\n        match id {{\n{id_matches}            _ => None,\n        }}\n    }}\n    pub fn from_key(key: &str) -> Option<Self> {{\n        let path = key.strip_prefix(\"minecraft:\").unwrap_or(key);\n        match path {{\n{path_matches}            _ => None,\n        }}\n    }}\n    pub const fn from_state_id(state_id: i32) -> Option<Self> {{\n        match BlockState::from_state_id(state_id) {{\n            Some(state) => Some(state.block()),\n            None => None,\n        }}\n    }}\n}}\n"
        ))
    }

    fn block_states(&self) -> io::Result<String> {
        let states = sorted_block_state_entries()?;
        let shapes = block_shapes()?;
        let state_count = states.len();
        states.iter().enumerate().for_each(|(expected_id, state)| {
            assert_eq!(
                state.id, expected_id as i32,
                "block state ids must be contiguous"
            );
        });
        let blocks = states
            .iter()
            .map(|state| format!("    Block::{},\n", state.block_variant))
            .collect::<String>();
        let light_emissions = numeric_array(states.iter().map(|state| state.light_emission));
        let light_blocks = numeric_array(states.iter().map(|state| state.light_block));
        let skylight_flags =
            boolean_array(states.iter().map(|state| state.propagates_skylight_down));
        let shape_occlusion_flags = boolean_array(
            states
                .iter()
                .map(|state| state.uses_shape_for_light_occlusion),
        );
        let collision_shapes = numeric_array(states.iter().map(|state| state.collision_shape));
        let occlusion_shapes = numeric_array(states.iter().map(|state| state.occlusion_shape));
        let face_occlusion_shapes = states
            .iter()
            .map(|state| {
                format!(
                    "    [{}, {}, {}, {}, {}, {}],\n",
                    face_shape(state, "down"),
                    face_shape(state, "up"),
                    face_shape(state, "north"),
                    face_shape(state, "south"),
                    face_shape(state, "west"),
                    face_shape(state, "east"),
                )
            })
            .collect::<String>();
        let properties = states
            .iter()
            .map(|state| {
                let entries = state
                    .properties
                    .iter()
                    .map(|(name, value)| {
                        format!(
                            "BlockStateProperty {{ name: {:?}, value: {:?} }}, ",
                            name, value
                        )
                    })
                    .collect::<String>();
                format!("    &[{entries}],\n")
            })
            .collect::<String>();
        let shape_boxes = shapes
            .iter()
            .map(|shape| {
                let boxes = shape.iter().map(block_shape_box).collect::<String>();
                format!("    &[{boxes}],\n")
            })
            .collect::<String>();
        Ok(format!(
            "use crate::vanilla_world_blocks::Block;\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\npub struct BlockState(u16);\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\npub struct BlockStateProperty {{\n    pub name: &'static str,\n    pub value: &'static str,\n}}\n\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct BlockShapeBox {{\n    pub min_x: f64,\n    pub min_y: f64,\n    pub min_z: f64,\n    pub max_x: f64,\n    pub max_y: f64,\n    pub max_z: f64,\n}}\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\npub enum BlockFaceDirection {{ Down, Up, North, South, West, East }}\n\nimpl BlockFaceDirection {{\n    const fn index(self) -> usize {{\n        match self {{ Self::Down => 0, Self::Up => 1, Self::North => 2, Self::South => 3, Self::West => 4, Self::East => 5 }}\n    }}\n}}\n\nimpl BlockState {{\n    pub const COUNT: usize = {state_count};\n    pub const fn from_state_id(state_id: i32) -> Option<Self> {{\n        if state_id < 0 || state_id as usize >= Self::COUNT {{ return None; }}\n        Some(Self(state_id as u16))\n    }}\n    pub(crate) const fn from_state_id_or_panic(state_id: i32) -> Self {{\n        match Self::from_state_id(state_id) {{ Some(state) => state, None => panic!(\"invalid generated block state id\") }}\n    }}\n    pub const fn state_id(self) -> i32 {{ self.0 as i32 }}\n    pub const fn block(self) -> Block {{ BLOCKS[self.0 as usize] }}\n    pub const fn light_emission(self) -> u8 {{ LIGHT_EMISSIONS[self.0 as usize] }}\n    pub const fn light_block(self) -> u8 {{ LIGHT_BLOCKS[self.0 as usize] }}\n    pub const fn propagates_skylight_down(self) -> bool {{ PROPAGATES_SKYLIGHT_DOWN[self.0 as usize] }}\n    pub const fn uses_shape_for_light_occlusion(self) -> bool {{ USES_SHAPE_FOR_LIGHT_OCCLUSION[self.0 as usize] }}\n    pub fn properties(self) -> &'static [BlockStateProperty] {{ STATE_PROPERTIES[self.0 as usize] }}\n    pub fn property(self, name: &str) -> Option<&'static str> {{ self.properties().iter().find(|property| property.name == name).map(|property| property.value) }}\n    pub fn with_property(self, name: &str, value: &str) -> Option<Self> {{\n        if self.property(name).is_none() {{ return None; }}\n        (0..Self::COUNT).find_map(|state_id| {{\n            let candidate = Self(state_id as u16);\n            if candidate.block() != self.block() {{ return None; }}\n            let candidate_properties = candidate.properties();\n            if candidate_properties.len() != self.properties().len() {{ return None; }}\n            let properties_match = self.properties().iter().all(|property| {{\n                let expected_value = if property.name == name {{ value }} else {{ property.value }};\n                candidate.property(property.name) == Some(expected_value)\n            }});\n            properties_match.then_some(candidate)\n        }})\n    }}\n    pub fn collision_shape(self) -> &'static [BlockShapeBox] {{ BLOCK_SHAPES[COLLISION_SHAPES[self.0 as usize] as usize] }}\n    pub fn occlusion_shape(self) -> &'static [BlockShapeBox] {{ BLOCK_SHAPES[OCCLUSION_SHAPES[self.0 as usize] as usize] }}\n    pub fn face_occlusion_shape(self, face: BlockFaceDirection) -> &'static [BlockShapeBox] {{\n        let shape = FACE_OCCLUSION_SHAPES[self.0 as usize][face.index()];\n        BLOCK_SHAPES[shape as usize]\n    }}\n}}\n\nimpl From<Block> for BlockState {{ fn from(block: Block) -> Self {{ block.default_state() }} }}\n\nconst BLOCKS: [Block; {state_count}] = [\n{blocks}];\nconst LIGHT_EMISSIONS: [u8; {state_count}] = [{light_emissions}];\nconst LIGHT_BLOCKS: [u8; {state_count}] = [{light_blocks}];\nconst PROPAGATES_SKYLIGHT_DOWN: [bool; {state_count}] = [{skylight_flags}];\nconst USES_SHAPE_FOR_LIGHT_OCCLUSION: [bool; {state_count}] = [{shape_occlusion_flags}];\nconst COLLISION_SHAPES: [u16; {state_count}] = [{collision_shapes}];\nconst OCCLUSION_SHAPES: [u16; {state_count}] = [{occlusion_shapes}];\nconst FACE_OCCLUSION_SHAPES: [[u16; 6]; {state_count}] = [\n{face_occlusion_shapes}];\nconst STATE_PROPERTIES: [&[BlockStateProperty]; {state_count}] = [\n{properties}];\nconst BLOCK_SHAPES: [&[BlockShapeBox]; {}] = [\n{shape_boxes}];\n",
            shapes.len()
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
        if type_name == "DamageType" {
            return self.damage_types();
        }
        if type_name == "Enchantment" {
            return self.enchantment_module();
        }
        if type_name == "MobEffect" {
            return self.mob_effect_module();
        }
        Ok(self.dynamic_module(type_name, &self.dynamic_registry_keys(registry_path)?))
    }

    fn damage_types(&self) -> io::Result<String> {
        let entries = damage_type_entries::damage_type_entries()?;
        let constants = entries
            .iter()
            .map(|entry| {
                format!(
                    "    pub const {}: RegistryKey<Self> = RegistryKey::vanilla_static(\"{}\");\n",
                    const_name(&entry.path),
                    entry.path
                )
            })
            .collect::<String>();
        let registrations = entries
            .iter()
            .map(|entry| {
                format!(
                    "    let _ = registry.register_vanilla(DamageType::{}, DamageType::new(\"{}\", DamageScaling::{}, {}, DamageEffects::{}, DeathMessageType::{}));\n",
                    const_name(&entry.path),
                    entry.message_id,
                    enum_variant(&entry.scaling),
                    rust_f32(entry.exhaustion),
                    enum_variant(&entry.effects),
                    enum_variant(&entry.death_message_type),
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::{{DynamicRegistry, RegistryKey}};\nuse crate::damage_type::{{DamageEffects, DamageScaling, DamageType, DeathMessageType}};\nimpl DamageType {{\n{constants}}}\npub fn register_damage_types(registry: &mut DynamicRegistry<DamageType>) {{\n{registrations}}}\n"
        ))
    }

    fn enchantment_module(&self) -> io::Result<String> {
        let keys = self.dynamic_registry_keys("enchantment")?;
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
        Ok(format!(
            "use crate::{{DynamicRegistry, Identifier, RegistryKey}};\nuse crate::enchantment::Enchantment;\nuse spinel_nbt::parse_snbt_compound;\nuse std::collections::BTreeMap;\nconst ENCHANTMENT_ENTRIES: &str = include_str!(\"../../assets/enchantments.json\");\nimpl Enchantment {{\n{constants}}}\npub fn register_enchantments(registry: &mut DynamicRegistry<Enchantment>) {{\n    let entries: BTreeMap<String, String> = serde_json::from_str(ENCHANTMENT_ENTRIES).expect(\"SpinelExtractor enchantments.json is malformed\");\n    for (key, entry) in entries {{\n        let identifier: Identifier = key.parse().expect(\"SpinelExtractor enchantment key is malformed\");\n        let raw_nbt = parse_snbt_compound(&entry).expect(\"SpinelExtractor enchantment payload is malformed SNBT\");\n        let _ = registry.register_vanilla(RegistryKey::new(identifier), Enchantment::raw(raw_nbt));\n    }}\n}}\n"
        ))
    }

    fn mob_effect_module(&self) -> io::Result<String> {
        let entries = mob_effect_entries()?;
        let constants = entries
            .iter()
            .map(|entry| {
                format!(
                    "    pub const {}: RegistryKey<Self> = RegistryKey::vanilla_static(\"{}\");\n",
                    const_name(&entry.name),
                    entry.name
                )
            })
            .collect::<String>();
        let registrations = entries.iter().map(|entry| format!("    let _ = registry.register_vanilla(MobEffect::{}, MobEffect::new({}, \"{}\".to_owned(), {}, {}));\n", const_name(&entry.name), entry.id, entry.translation_key, entry.color, entry.instantaneous)).collect::<String>();
        Ok(format!(
            "use crate::{{DynamicRegistry, RegistryKey}};\nuse crate::mob_effect::MobEffect;\nimpl MobEffect {{\n{constants}}}\npub fn register_mob_effects(registry: &mut DynamicRegistry<MobEffect>) {{\n{registrations}}}\n"
        ))
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
        let default_attribute_tables = entity_types
            .iter()
            .map(|entity_type| self.entity_default_attribute_table(entity_type))
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
        let default_attribute_matches = entity_types
            .iter()
            .map(|entity_type| {
                format!(
                    "            Self::{} => {}_DEFAULT_ATTRIBUTES,\n",
                    const_name(&entity_type.path),
                    const_name(&entity_type.path)
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::Attribute;\nuse crate::entity::{{EntityAttachmentOffset, EntityDefaultAttribute, EntityPacketType, EntityType}};\n{attachment_tables}{default_attribute_tables}impl EntityType {{\n{constants}    pub const ALL: &'static [Self] = &[\n{all}    ];\n    pub fn attachments(self) -> &'static [EntityAttachmentOffset] {{\n        match self {{\n{attachment_matches}            _ => &[],\n        }}\n    }}\n    pub fn default_attributes(self) -> &'static [EntityDefaultAttribute] {{\n        match self {{\n{default_attribute_matches}            _ => &[],\n        }}\n    }}\n}}\n"
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

    fn static_protocol_values(&self, registry_name: &str, type_name: &str) -> io::Result<String> {
        let entries = static_protocol_entries::static_protocol_entries(registry_name)?;
        let constants = entries
            .iter()
            .map(|entry| {
                format!(
                    "    pub const {}: Self = Self::new({}, Identifier::vanilla_static(\"{}\"));\n",
                    const_name(&entry.name),
                    entry.id,
                    vanilla_path(&entry.name)
                )
            })
            .collect::<String>();
        let id_matches = entries
            .iter()
            .map(|entry| {
                format!(
                    "            {} => Some(Self::{}),\n",
                    entry.id,
                    const_name(&entry.name)
                )
            })
            .collect::<String>();
        let key_matches = entries
            .iter()
            .map(|entry| {
                format!(
                    "            \"{}\" => Some(Self::{}),\n",
                    vanilla_path(&entry.name),
                    const_name(&entry.name)
                )
            })
            .collect::<String>();
        Ok(format!(
            "use crate::{{Identifier, {type_name}}};\nimpl {type_name} {{\n{constants}    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {{\n        match protocol_id {{\n{id_matches}            _ => None,\n        }}\n    }}\n    pub fn from_key(key: &Identifier) -> Option<Self> {{\n        match key.path.as_ref() {{\n{key_matches}            _ => None,\n        }}\n    }}\n}}\n"
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

    fn entity_default_attribute_table(&self, entity_type: &entity_entries::EntityEntry) -> String {
        let attributes = entity_type
            .default_attributes
            .iter()
            .map(|(name, base_value)| {
                format!(
                    "    EntityDefaultAttribute::new(Attribute::{}, {}),\n",
                    const_name(name),
                    rust_float(*base_value)
                )
            })
            .collect::<String>();
        format!(
            "const {}_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[\n{}];\n",
            const_name(&entity_type.path),
            attributes
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

fn enum_variant(value: &str) -> String {
    value.to_upper_camel_case()
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

fn rust_f32(value: f32) -> String {
    let value_text = value.to_string();
    if value_text.contains('.') {
        return value_text;
    }
    format!("{value_text}.0")
}

fn numeric_array<T: std::fmt::Display>(values: impl Iterator<Item = T>) -> String {
    values.map(|value| format!("{value},")).collect::<String>()
}

fn boolean_array(values: impl Iterator<Item = bool>) -> String {
    values.map(|value| format!("{value},")).collect::<String>()
}

fn face_shape(state: &BlockStateEntry, face: &str) -> u16 {
    state
        .face_occlusion_shapes
        .get(face)
        .copied()
        .unwrap_or(state.occlusion_shape)
}

fn block_shape_box(shape_box: &BlockShapeBoxEntry) -> String {
    format!(
        "BlockShapeBox {{ min_x: {}, min_y: {}, min_z: {}, max_x: {}, max_y: {}, max_z: {} }}, ",
        rust_f64(shape_box.min_x),
        rust_f64(shape_box.min_y),
        rust_f64(shape_box.min_z),
        rust_f64(shape_box.max_x),
        rust_f64(shape_box.max_y),
        rust_f64(shape_box.max_z),
    )
}

fn rust_f64(value: f64) -> String {
    if value.fract() == 0.0 {
        return format!("{value:.1}");
    }
    value.to_string()
}
