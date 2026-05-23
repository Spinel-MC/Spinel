use crate::compound::NbtCompound;
use crate::serializer::WriteAdaptor;
use crate::*;
use std::io::Write;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
