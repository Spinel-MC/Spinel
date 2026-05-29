use spinel_nbt::{Nbt, NbtCompound};
use std::collections::HashMap;

pub(super) fn compound_from_nbt(component_nbt: &Nbt) -> Option<&NbtCompound> {
    match component_nbt {
        Nbt::Compound(compound) => Some(compound),
        _ => None,
    }
}

pub(super) fn string_from_nbt(component_nbt: &Nbt) -> Option<String> {
    match component_nbt {
        Nbt::String(value) => Some(value.clone()),
        _ => None,
    }
}

pub(super) fn string_field(compound: &NbtCompound, name: &str) -> Option<String> {
    match compound.get(name) {
        Some(Nbt::String(value)) => Some(value.clone()),
        _ => None,
    }
}

pub(super) fn bool_field_or(
    compound: &NbtCompound,
    name: &str,
    default_value: bool,
) -> Option<bool> {
    match compound.get(name) {
        Some(Nbt::Byte(value)) => Some(*value != 0),
        Some(Nbt::Int(value)) => Some(*value != 0),
        None => Some(default_value),
        _ => None,
    }
}

pub(super) fn i32_field_or(compound: &NbtCompound, name: &str, default_value: i32) -> Option<i32> {
    match compound.get(name) {
        Some(Nbt::Int(value)) => Some(*value),
        Some(Nbt::Short(value)) => Some(i32::from(*value)),
        Some(Nbt::Byte(value)) => Some(i32::from(*value)),
        None => Some(default_value),
        _ => None,
    }
}

pub(super) fn f32_field_or(compound: &NbtCompound, name: &str, default_value: f32) -> Option<f32> {
    match compound.get(name) {
        Some(Nbt::Float(value)) => Some(*value),
        Some(Nbt::Double(value)) => Some(*value as f32),
        Some(Nbt::Int(value)) => Some(*value as f32),
        None => Some(default_value),
        _ => None,
    }
}

pub(super) fn f64_field(compound: &NbtCompound, name: &str) -> Option<f64> {
    match compound.get(name) {
        Some(Nbt::Double(value)) => Some(*value),
        Some(Nbt::Float(value)) => Some(f64::from(*value)),
        Some(Nbt::Int(value)) => Some(f64::from(*value)),
        _ => None,
    }
}

pub(super) fn f64_field_or(compound: &NbtCompound, name: &str, default_value: f64) -> Option<f64> {
    match compound.get(name) {
        Some(Nbt::Double(value)) => Some(*value),
        Some(Nbt::Float(value)) => Some(f64::from(*value)),
        Some(Nbt::Int(value)) => Some(f64::from(*value)),
        None => Some(default_value),
        _ => None,
    }
}

pub(super) fn string_list_field_or_empty(
    compound: &NbtCompound,
    name: &str,
) -> Option<Vec<String>> {
    match compound.get(name) {
        Some(Nbt::List(values)) => values
            .iter()
            .map(|value| match value {
                Nbt::String(value) => Some(value.clone()),
                _ => None,
            })
            .collect(),
        None => Some(Vec::new()),
        _ => None,
    }
}

pub(super) fn f32_list_field_or_empty(compound: &NbtCompound, name: &str) -> Option<Vec<f32>> {
    match compound.get(name) {
        Some(Nbt::List(values)) => values
            .iter()
            .map(|value| match value {
                Nbt::Float(value) => Some(*value),
                Nbt::Double(value) => Some(*value as f32),
                Nbt::Int(value) => Some(*value as f32),
                _ => None,
            })
            .collect(),
        None => Some(Vec::new()),
        _ => None,
    }
}

pub(super) fn bool_list_field_or_empty(compound: &NbtCompound, name: &str) -> Option<Vec<bool>> {
    match compound.get(name) {
        Some(Nbt::List(values)) => values
            .iter()
            .map(|value| match value {
                Nbt::Byte(value) => Some(*value != 0),
                Nbt::Int(value) => Some(*value != 0),
                _ => None,
            })
            .collect(),
        None => Some(Vec::new()),
        _ => None,
    }
}

pub(super) fn i32_list_field_or_empty(compound: &NbtCompound, name: &str) -> Option<Vec<i32>> {
    match compound.get(name) {
        Some(Nbt::List(values)) => values
            .iter()
            .map(|value| match value {
                Nbt::Int(value) => Some(*value),
                Nbt::Short(value) => Some(i32::from(*value)),
                Nbt::Byte(value) => Some(i32::from(*value)),
                _ => None,
            })
            .collect(),
        None => Some(Vec::new()),
        _ => None,
    }
}

pub(super) fn string_map_from_compound(compound: &NbtCompound) -> Option<HashMap<String, String>> {
    compound
        .0
        .iter()
        .map(|(key, value)| match value {
            Nbt::String(value) => Some((key.clone(), value.clone())),
            _ => None,
        })
        .collect()
}
