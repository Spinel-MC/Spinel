use crate::compound::NbtCompound;
use crate::deserializer::NbtReadHelper;
use crate::serializer::WriteAdaptor;
use crate::*;
use std::io::{self, Read, Write};

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

    pub fn serialize_data<W>(&self, w: &mut WriteAdaptor<W>) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            Nbt::End => {}
            Nbt::Byte(byte) => w.write_i8_be(*byte)?,
            Nbt::Short(short) => w.write_i16_be(*short)?,
            Nbt::Int(int) => w.write_i32_be(*int)?,
            Nbt::Long(long) => w.write_i64_be(*long)?,
            Nbt::Float(float) => w.write_f32_be(*float)?,
            Nbt::Double(double) => w.write_f64_be(*double)?,
            Nbt::ByteArray(byte_array) => {
                w.write_i32_be(byte_array.len() as i32)?;
                w.write_slice(byte_array)?;
            }
            Nbt::String(string) => {
                let java_string = cesu8::to_java_cesu8(string);
                w.write_u16_be(java_string.len() as u16)?;
                w.write_slice(&java_string)?;
            }
            Nbt::List(list) => {
                w.write_u8_be(list.first().map_or(END_ID, Nbt::get_type_id))?;
                w.write_i32_be(list.len() as i32)?;
                for nbt_tag in list.iter() {
                    nbt_tag.serialize_data(w)?;
                }
            }
            Nbt::Compound(compound) => {
                compound.serialize_content(w)?;
            }
            Nbt::IntArray(int_array) => {
                w.write_i32_be(int_array.len() as i32)?;
                for int in int_array.iter() {
                    w.write_i32_be(*int)?;
                }
            }
            Nbt::LongArray(long_array) => {
                w.write_i32_be(long_array.len() as i32)?;
                for long in long_array.iter() {
                    w.write_i64_be(*long)?;
                }
            }
        };
        Ok(())
    }

    pub fn skip_data<R: Read>(reader: &mut NbtReadHelper<R>, tag_id: u8) -> Result<(), Error> {
        match tag_id {
            END_ID => Ok(()),
            BYTE_ID => reader.skip_bytes(1),
            SHORT_ID => reader.skip_bytes(2),
            INT_ID | FLOAT_ID => reader.skip_bytes(4),
            LONG_ID | DOUBLE_ID => reader.skip_bytes(8),
            BYTE_ARRAY_ID => {
                let len = reader.get_i32_be()?;
                reader.skip_bytes(len as u64)
            }
            STRING_ID => {
                let len = reader.get_u16_be()?;
                reader.skip_bytes(u64::from(len))
            }
            LIST_ID => {
                let list_type_id = reader.get_u8_be()?;
                let len = reader.get_i32_be()?;
                for _ in 0..len {
                    Self::skip_data(reader, list_type_id)?;
                }
                Ok(())
            }
            COMPOUND_ID => NbtCompound::skip_content(reader),
            INT_ARRAY_ID => {
                let len = reader.get_i32_be()?;
                reader.skip_bytes(len as u64 * 4)
            }
            LONG_ARRAY_ID => {
                let len = reader.get_i32_be()?;
                reader.skip_bytes(len as u64 * 8)
            }
            _ => Err(Error::UnknownTagId(tag_id)),
        }
    }

    pub fn deserialize_data<R: Read>(
        reader: &mut NbtReadHelper<R>,
        tag_id: u8,
    ) -> Result<Nbt, Error> {
        match tag_id {
            END_ID => Ok(Nbt::End),
            BYTE_ID => Ok(Nbt::Byte(reader.get_i8_be()?)),
            SHORT_ID => Ok(Nbt::Short(reader.get_i16_be()?)),
            INT_ID => Ok(Nbt::Int(reader.get_i32_be()?)),
            LONG_ID => Ok(Nbt::Long(reader.get_i64_be()?)),
            FLOAT_ID => Ok(Nbt::Float(reader.get_f32_be()?)),
            DOUBLE_ID => Ok(Nbt::Double(reader.get_f64_be()?)),
            BYTE_ARRAY_ID => {
                let len = reader.get_i32_be()?;
                Ok(Nbt::ByteArray(reader.read_boxed_slice(len as usize)?))
            }
            STRING_ID => Ok(Nbt::String(get_nbt_string(reader)?)),
            LIST_ID => {
                let list_type_id = reader.get_u8_be()?;
                let len = reader.get_i32_be()?;
                let mut list = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    list.push(Nbt::deserialize_data(reader, list_type_id)?);
                }
                Ok(Nbt::List(list.into_boxed_slice()))
            }
            COMPOUND_ID => Ok(Nbt::Compound(NbtCompound::deserialize_content(reader)?)),
            INT_ARRAY_ID => {
                let len = reader.get_i32_be()? as usize;
                let mut int_array = Vec::with_capacity(len);
                for _ in 0..len {
                    int_array.push(reader.get_i32_be()?);
                }
                Ok(Nbt::IntArray(int_array.into_boxed_slice()))
            }
            LONG_ARRAY_ID => {
                let len = reader.get_i32_be()? as usize;
                let mut long_array = Vec::with_capacity(len);
                for _ in 0..len {
                    long_array.push(reader.get_i64_be()?);
                }
                Ok(Nbt::LongArray(long_array.into_boxed_slice()))
            }
            _ => Err(Error::UnknownTagId(tag_id)),
        }
    }

    pub fn read_from_stream<R: Read>(reader: &mut R) -> io::Result<(String, Self)> {
        let mut helper = NbtReadHelper::new(reader);
        let tag_id = helper
            .get_u8_be()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        if tag_id == END_ID {
            return Ok(("".to_string(), Nbt::End));
        }
        let name =
            get_nbt_string(&mut helper).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let tag = Self::deserialize_data(&mut helper, tag_id)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok((name, tag))
    }

    pub fn write<W: Write>(&self, name: &str, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[self.get_type_id()])?;
        if self.get_type_id() != END_ID {
            let name_bytes = cesu8::to_java_cesu8(name);
            writer.write_all(&(name_bytes.len() as u16).to_be_bytes())?;
            writer.write_all(&name_bytes)?;

            let mut data = Vec::new();
            let mut adaptor = WriteAdaptor::new(&mut data);
            self.serialize_data(&mut adaptor).unwrap();
            writer.write_all(&data)?;
        }
        Ok(())
    }

    pub fn write_unnamed<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[self.get_type_id()])?;
        if self.get_type_id() != END_ID {
            let mut data = Vec::new();
            let mut adaptor = WriteAdaptor::new(&mut data);
            self.serialize_data(&mut adaptor).unwrap();
            writer.write_all(&data)?;
        }
        Ok(())
    }
}