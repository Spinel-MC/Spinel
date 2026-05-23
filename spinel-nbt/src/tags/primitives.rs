use crate::tags::serializer::TagSerializer;
use crate::{Nbt, NbtCompound, Tag};
use uuid::Uuid;

impl Tag<i8> {
    pub fn byte(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(read_byte, |value| Some(Nbt::Byte(value))),
        )
    }
}

impl Tag<bool> {
    pub fn boolean(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(read_boolean, |value| Some(Nbt::Byte(i8::from(value)))),
        )
    }
}

impl Tag<i16> {
    pub fn short(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(read_short, |value| Some(Nbt::Short(value))),
        )
    }
}

impl Tag<i32> {
    pub fn integer(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(read_int, |value| Some(Nbt::Int(value))),
        )
    }
}

impl Tag<i64> {
    pub fn long(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(read_long, |value| Some(Nbt::Long(value))),
        )
    }
}

impl Tag<f32> {
    pub fn float(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(read_float, |value| Some(Nbt::Float(value))),
        )
    }
}

impl Tag<f64> {
    pub fn double(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(read_double, |value| Some(Nbt::Double(value))),
        )
    }
}

impl Tag<String> {
    pub fn string(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(read_string, |value| Some(Nbt::String(value))),
        )
    }
}

impl Tag<Uuid> {
    pub fn uuid(key: impl Into<String>) -> Self {
        Tag::new(key, TagSerializer::cloned(read_uuid, write_uuid))
    }
}

impl Tag<Nbt> {
    pub fn nbt(key: impl Into<String>) -> Self {
        Tag::new(
            key,
            TagSerializer::cloned(|value| Some(value.clone()), Some),
        )
    }
}

impl Tag<NbtCompound> {
    pub fn structure(key: impl Into<String>, serializer: TagSerializer<NbtCompound>) -> Self {
        Tag::new(key, serializer)
    }

    pub fn view(serializer: TagSerializer<NbtCompound>) -> Self {
        Tag::new("", serializer)
    }
}

fn read_byte(nbt: &Nbt) -> Option<i8> {
    match nbt {
        Nbt::Byte(value) => Some(*value),
        _ => None,
    }
}

fn read_boolean(nbt: &Nbt) -> Option<bool> {
    read_byte(nbt).map(|value| value != 0)
}

fn read_short(nbt: &Nbt) -> Option<i16> {
    match nbt {
        Nbt::Short(value) => Some(*value),
        _ => None,
    }
}

fn read_int(nbt: &Nbt) -> Option<i32> {
    match nbt {
        Nbt::Int(value) => Some(*value),
        _ => None,
    }
}

fn read_long(nbt: &Nbt) -> Option<i64> {
    match nbt {
        Nbt::Long(value) => Some(*value),
        _ => None,
    }
}

fn read_float(nbt: &Nbt) -> Option<f32> {
    match nbt {
        Nbt::Float(value) => Some(*value),
        _ => None,
    }
}

fn read_double(nbt: &Nbt) -> Option<f64> {
    match nbt {
        Nbt::Double(value) => Some(*value),
        _ => None,
    }
}

fn read_string(nbt: &Nbt) -> Option<String> {
    match nbt {
        Nbt::String(value) => Some(value.clone()),
        _ => None,
    }
}

fn read_uuid(nbt: &Nbt) -> Option<Uuid> {
    let Nbt::IntArray(values) = nbt else {
        return None;
    };
    let values: &[i32] = values;
    if values.len() != 4 {
        return None;
    }
    let mut bytes = [0u8; 16];
    values.iter().enumerate().for_each(|(index, value)| {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_be_bytes());
    });
    Some(Uuid::from_bytes(bytes))
}

fn write_uuid(value: Uuid) -> Option<Nbt> {
    let bytes = value.as_bytes();
    Some(Nbt::IntArray(
        bytes
            .chunks_exact(4)
            .map(|chunk| i32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    ))
}
