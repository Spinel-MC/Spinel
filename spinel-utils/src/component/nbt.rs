use serde_json::{Map, Value};
use spinel_nbt::{Nbt, NbtCompound};

pub fn json_to_nbt_compound(map: Map<String, Value>) -> NbtCompound {
    let mut compound = NbtCompound::new();
    for (key, value) in map {
        if let Some(nbt) = json_to_nbt(value) {
            compound.insert(key, nbt);
        }
    }
    compound
}

pub fn json_to_nbt(value: Value) -> Option<Nbt> {
    match value {
        Value::Null => None,
        Value::Bool(value) => Some(Nbt::Byte(i8::from(value))),
        Value::Number(number) => number_to_nbt(&number),
        Value::String(value) => Some(Nbt::String(value)),
        Value::Array(values) => Some(Nbt::List(
            values
                .into_iter()
                .filter_map(json_to_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )),
        Value::Object(map) => Some(Nbt::Compound(json_to_nbt_compound(map))),
    }
}

pub fn nbt_compound_to_json(compound: NbtCompound) -> Map<String, Value> {
    compound
        .0
        .into_iter()
        .map(|(key, value)| (key, nbt_to_json(value)))
        .collect()
}

pub fn nbt_to_json(value: Nbt) -> Value {
    match value {
        Nbt::End => Value::Null,
        Nbt::Byte(value) => Value::Bool(value != 0),
        Nbt::Short(value) => Value::from(value),
        Nbt::Int(value) => Value::from(value),
        Nbt::Long(value) => Value::from(value),
        Nbt::Float(value) => Value::from(value),
        Nbt::Double(value) => Value::from(value),
        Nbt::ByteArray(values) => {
            Value::Array(values.iter().map(|value| Value::from(*value)).collect())
        }
        Nbt::String(value) => Value::String(value),
        Nbt::List(values) => Value::Array(values.into_vec().into_iter().map(nbt_to_json).collect()),
        Nbt::Compound(value) => Value::Object(nbt_compound_to_json(value)),
        Nbt::IntArray(values) => {
            Value::Array(values.iter().map(|value| Value::from(*value)).collect())
        }
        Nbt::LongArray(values) => {
            Value::Array(values.iter().map(|value| Value::from(*value)).collect())
        }
    }
}

fn number_to_nbt(number: &serde_json::Number) -> Option<Nbt> {
    if let Some(value) = number.as_i64() {
        return Some(Nbt::Long(value));
    }
    if let Some(value) = number.as_u64() {
        return i64::try_from(value).ok().map(Nbt::Long);
    }
    number.as_f64().map(Nbt::Double)
}
