use crate::{DataComponentDescriptor, DataComponentMap, Material};
use serde::Deserialize;
use spinel_nbt::{Nbt, NbtCompound};
use std::collections::BTreeMap;
use std::sync::OnceLock;

impl Material {
    pub fn prototype(&self) -> DataComponentMap {
        material_prototypes()
            .get(&self.id())
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MaterialPrototypeExtraction {
    SpinelExtractor { items: Vec<MaterialPrototypeEntry> },
    Minestom(BTreeMap<String, MaterialPrototypeEntry>),
}

#[derive(Deserialize)]
struct MaterialPrototypeEntry {
    id: i32,
    components: Option<BTreeMap<String, serde_json::Value>>,
}

fn material_prototypes() -> &'static BTreeMap<i32, DataComponentMap> {
    static MATERIAL_PROTOTYPES: OnceLock<BTreeMap<i32, DataComponentMap>> = OnceLock::new();
    MATERIAL_PROTOTYPES.get_or_init(parse_material_prototypes)
}

fn parse_material_prototypes() -> BTreeMap<i32, DataComponentMap> {
    serde_json::from_str::<MaterialPrototypeExtraction>(include_str!("../../assets/items.json"))
        .map(|extraction| match extraction {
            MaterialPrototypeExtraction::SpinelExtractor { items } => items,
            MaterialPrototypeExtraction::Minestom(entries) => entries.into_values().collect(),
        })
        .map(|entries| {
            entries
                .into_iter()
                .map(|entry| (entry.id, component_map_from_json(entry.components)))
                .collect()
        })
        .unwrap_or_default()
}

fn component_map_from_json(
    components: Option<BTreeMap<String, serde_json::Value>>,
) -> DataComponentMap {
    components
        .unwrap_or_default()
        .into_iter()
        .filter(|(key, _)| DataComponentDescriptor::from_key(key).is_some())
        .fold(
            DataComponentMap::new(),
            |mut component_map, (key, value)| {
                let _ = component_map.set_nbt_by_key(&key, component_json_to_nbt(&key, value));
                component_map
            },
        )
}

fn component_json_to_nbt(component_key: &str, value: serde_json::Value) -> Nbt {
    match component_key {
        "minecraft:custom_name" | "minecraft:item_name" => Nbt::String(value.to_string()),
        "minecraft:lore" => lore_component_json_to_nbt(value),
        _ => json_to_nbt(value),
    }
}

fn lore_component_json_to_nbt(value: serde_json::Value) -> Nbt {
    match value {
        serde_json::Value::Array(lines) => Nbt::List(
            lines
                .into_iter()
                .map(|line| Nbt::String(line.to_string()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
        value => json_to_nbt(value),
    }
}

fn json_to_nbt(value: serde_json::Value) -> Nbt {
    match value {
        serde_json::Value::Null => Nbt::End,
        serde_json::Value::Bool(value) => Nbt::Byte(i8::from(value)),
        serde_json::Value::Number(value) => number_to_nbt(value),
        serde_json::Value::String(value) => Nbt::String(value),
        serde_json::Value::Array(values) => Nbt::List(
            values
                .into_iter()
                .map(json_to_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
        serde_json::Value::Object(values) => Nbt::Compound(values.into_iter().fold(
            NbtCompound::new(),
            |mut compound, (key, value)| {
                compound.insert(key, json_to_nbt(value));
                compound
            },
        )),
    }
}

fn number_to_nbt(value: serde_json::Number) -> Nbt {
    if let Some(value) = value.as_i64() {
        return i32::try_from(value)
            .map(Nbt::Int)
            .unwrap_or_else(|_| Nbt::Long(value));
    }

    if let Some(value) = value.as_u64() {
        return i32::try_from(value)
            .map(Nbt::Int)
            .unwrap_or_else(|_| Nbt::Long(value as i64));
    }

    Nbt::Double(value.as_f64().unwrap_or_default())
}
