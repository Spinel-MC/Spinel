use crate::Identifier;
use serde_json::{Number, Value};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Debug, Clone)]
pub enum BiomeAttributes {
    Static(&'static [BiomeAttribute]),
    Owned(Vec<OwnedBiomeAttribute>),
}

impl BiomeAttributes {
    pub(crate) fn push(&mut self, key: Identifier, attribute: Value) {
        let mut attributes = match self {
            Self::Static(_) => Vec::new(),
            Self::Owned(attributes) => std::mem::take(attributes),
        };
        attributes.push(OwnedBiomeAttribute { key, attribute });
        *self = Self::Owned(attributes);
    }

    pub(crate) fn nbt(&self) -> Option<Nbt> {
        let attributes = match self {
            Self::Static(attributes) => static_attributes_nbt(attributes),
            Self::Owned(attributes) => owned_attributes_nbt(attributes),
        };
        (!attributes.0.is_empty()).then_some(Nbt::Compound(attributes))
    }
}

impl Default for BiomeAttributes {
    fn default() -> Self {
        Self::Static(&[])
    }
}

#[derive(Debug, Clone)]
pub struct BiomeAttribute {
    pub key: Identifier,
    pub value_json: &'static str,
}

#[derive(Debug, Clone)]
pub struct OwnedBiomeAttribute {
    pub key: Identifier,
    pub attribute: Value,
}

fn static_attributes_nbt(attributes: &[BiomeAttribute]) -> NbtCompound {
    attributes
        .iter()
        .filter_map(|attribute| {
            serde_json::from_str(attribute.value_json)
                .ok()
                .map(|value| (attribute.key.to_string(), json_nbt(&value)))
        })
        .fold(NbtCompound::new(), insert_attribute_nbt)
}

fn owned_attributes_nbt(attributes: &[OwnedBiomeAttribute]) -> NbtCompound {
    attributes
        .iter()
        .map(|attribute| (attribute.key.to_string(), json_nbt(&attribute.attribute)))
        .fold(NbtCompound::new(), insert_attribute_nbt)
}

fn insert_attribute_nbt(
    mut attributes_nbt: NbtCompound,
    (attribute_key, attribute_nbt): (String, Nbt),
) -> NbtCompound {
    attributes_nbt.insert(attribute_key, attribute_nbt);
    attributes_nbt
}

fn json_nbt(value: &Value) -> Nbt {
    match value {
        Value::Null => Nbt::End,
        Value::Bool(boolean) => Nbt::Byte(i8::from(*boolean)),
        Value::Number(number) => number_nbt(number),
        Value::String(string) => Nbt::String(string.clone()),
        Value::Array(values) => Nbt::List(values.iter().map(json_nbt).collect()),
        Value::Object(fields) => fields
            .iter()
            .map(|(field_name, field_value)| (field_name.clone(), json_nbt(field_value)))
            .fold(NbtCompound::new(), insert_attribute_nbt)
            .into(),
    }
}

fn number_nbt(number: &Number) -> Nbt {
    if let Some(integer) = number.as_i64() {
        return Nbt::Long(integer);
    }
    Nbt::Double(number.as_f64().unwrap_or_default())
}
