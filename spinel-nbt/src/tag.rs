use crate::compound::NbtCompound;
use crate::serializer::WriteAdaptor;
use crate::*;
use std::{
    hash::{Hash, Hasher},
    io::Write,
};

#[derive(Clone, Debug)]
pub enum Nbt {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Box<[u8]>),
    String(String),
    List(Box<[Nbt]>),
    Compound(NbtCompound),
    IntArray(Box<[i32]>),
    LongArray(Box<[i64]>),
}

impl PartialEq for Nbt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::End, Self::End) => true,
            (Self::Byte(left), Self::Byte(right)) => left == right,
            (Self::Short(left), Self::Short(right)) => left == right,
            (Self::Int(left), Self::Int(right)) => left == right,
            (Self::Long(left), Self::Long(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => {
                canonical_f32_bits(*left) == canonical_f32_bits(*right)
            }
            (Self::Double(left), Self::Double(right)) => {
                canonical_f64_bits(*left) == canonical_f64_bits(*right)
            }
            (Self::ByteArray(left), Self::ByteArray(right)) => left == right,
            (Self::String(left), Self::String(right)) => left == right,
            (Self::List(left), Self::List(right)) => left == right,
            (Self::Compound(left), Self::Compound(right)) => left == right,
            (Self::IntArray(left), Self::IntArray(right)) => left == right,
            (Self::LongArray(left), Self::LongArray(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for Nbt {}

impl Hash for Nbt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_type_id().hash(state);
        match self {
            Self::End => {}
            Self::Byte(value) => value.hash(state),
            Self::Short(value) => value.hash(state),
            Self::Int(value) => value.hash(state),
            Self::Long(value) => value.hash(state),
            Self::Float(value) => canonical_f32_bits(*value).hash(state),
            Self::Double(value) => canonical_f64_bits(*value).hash(state),
            Self::ByteArray(values) => values.hash(state),
            Self::String(value) => value.hash(state),
            Self::List(values) => values.hash(state),
            Self::Compound(value) => value.hash(state),
            Self::IntArray(values) => values.hash(state),
            Self::LongArray(values) => values.hash(state),
        }
    }
}

fn canonical_f32_bits(value: f32) -> u32 {
    if value.is_nan() {
        f32::NAN.to_bits()
    } else {
        value.to_bits()
    }
}

fn canonical_f64_bits(value: f64) -> u64 {
    if value.is_nan() {
        f64::NAN.to_bits()
    } else {
        value.to_bits()
    }
}

pub trait NbtWritable {
    fn write_unnamed(&self, writer: &mut WriteAdaptor<impl Write>) -> Result<(), Error>;
}

impl NbtWritable for NbtCompound {
    fn write_unnamed(&self, writer: &mut WriteAdaptor<impl Write>) -> Result<(), Error> {
        writer.write_u8_be(COMPOUND_ID)?;
        self.serialize_content(writer)
    }
}

impl Nbt {
    pub const fn get_type_id(&self) -> u8 {
        match self {
            Nbt::End => END_ID,
            Nbt::Byte(_) => BYTE_ID,
            Nbt::Short(_) => SHORT_ID,
            Nbt::Int(_) => INT_ID,
            Nbt::Long(_) => LONG_ID,
            Nbt::Float(_) => FLOAT_ID,
            Nbt::Double(_) => DOUBLE_ID,
            Nbt::ByteArray(_) => BYTE_ARRAY_ID,
            Nbt::String(_) => STRING_ID,
            Nbt::List(_) => LIST_ID,
            Nbt::Compound(_) => COMPOUND_ID,
            Nbt::IntArray(_) => INT_ARRAY_ID,
            Nbt::LongArray(_) => LONG_ARRAY_ID,
        }
    }
}
