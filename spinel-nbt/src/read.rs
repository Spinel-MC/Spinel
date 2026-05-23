use crate::compound::NbtCompound;
use crate::deserializer::NbtReadHelper;
use crate::*;
use std::io::{self, Read};

impl Nbt {
    pub fn skip_data<R: Read>(reader: &mut NbtReadHelper<R>, tag_id: u8) -> Result<(), Error> {
        match tag_id {
            END_ID => Ok(()),
            BYTE_ID => reader.skip_bytes(1),
            SHORT_ID => reader.skip_bytes(2),
            INT_ID | FLOAT_ID => reader.skip_bytes(4),
            LONG_ID | DOUBLE_ID => reader.skip_bytes(8),
            BYTE_ARRAY_ID => skip_array(reader, 1),
            STRING_ID => {
                let len = reader.get_u16_be()?;
                reader.skip_bytes(u64::from(len))
            }
            LIST_ID => skip_list(reader),
            COMPOUND_ID => NbtCompound::skip_content(reader),
            INT_ARRAY_ID => skip_array(reader, 4),
            LONG_ARRAY_ID => skip_array(reader, 8),
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
            BYTE_ARRAY_ID => read_byte_array(reader),
            STRING_ID => Ok(Nbt::String(get_nbt_string(reader)?)),
            LIST_ID => read_list(reader),
            COMPOUND_ID => Ok(Nbt::Compound(NbtCompound::deserialize_content(reader)?)),
            INT_ARRAY_ID => read_int_array(reader),
            LONG_ARRAY_ID => read_long_array(reader),
            _ => Err(Error::UnknownTagId(tag_id)),
        }
    }

    pub fn read_from_stream<R: Read>(reader: &mut R) -> io::Result<(String, Self)> {
        let mut helper = NbtReadHelper::new(reader);
        let tag_id = helper.get_u8_be().map_err(io::Error::other)?;
        if tag_id == END_ID {
            return Ok(("".to_string(), Nbt::End));
        }
        let name = get_nbt_string(&mut helper).map_err(io::Error::other)?;
        let tag = Self::deserialize_data(&mut helper, tag_id).map_err(io::Error::other)?;
        Ok((name, tag))
    }

    pub fn read_unnamed<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut helper = NbtReadHelper::new(reader);
        let tag_id = helper.get_u8_be().map_err(io::Error::other)?;
        if tag_id == END_ID {
            return Ok(Nbt::End);
        }
        Self::deserialize_data(&mut helper, tag_id).map_err(io::Error::other)
    }
}

fn checked_len(len: i32) -> Result<usize, Error> {
    if len < 0 {
        return Err(Error::NegativeLength(len));
    }
    Ok(len as usize)
}

fn skip_array<R: Read>(reader: &mut NbtReadHelper<R>, scale: u64) -> Result<(), Error> {
    let len = checked_len(reader.get_i32_be()?)?;
    reader.skip_bytes(len as u64 * scale)
}

fn skip_list<R: Read>(reader: &mut NbtReadHelper<R>) -> Result<(), Error> {
    let list_type_id = reader.get_u8_be()?;
    let len = checked_len(reader.get_i32_be()?)?;
    for _ in 0..len {
        Nbt::skip_data(reader, list_type_id)?;
    }
    Ok(())
}

fn read_byte_array<R: Read>(reader: &mut NbtReadHelper<R>) -> Result<Nbt, Error> {
    let len = checked_len(reader.get_i32_be()?)?;
    Ok(Nbt::ByteArray(reader.read_boxed_slice(len)?))
}

fn read_list<R: Read>(reader: &mut NbtReadHelper<R>) -> Result<Nbt, Error> {
    let list_type_id = reader.get_u8_be()?;
    let len = checked_len(reader.get_i32_be()?)?;
    let mut list = Vec::with_capacity(len);
    for _ in 0..len {
        list.push(Nbt::deserialize_data(reader, list_type_id)?);
    }
    Ok(Nbt::List(list.into_boxed_slice()))
}

fn read_int_array<R: Read>(reader: &mut NbtReadHelper<R>) -> Result<Nbt, Error> {
    let len = checked_len(reader.get_i32_be()?)?;
    let mut int_array = Vec::with_capacity(len);
    for _ in 0..len {
        int_array.push(reader.get_i32_be()?);
    }
    Ok(Nbt::IntArray(int_array.into_boxed_slice()))
}

fn read_long_array<R: Read>(reader: &mut NbtReadHelper<R>) -> Result<Nbt, Error> {
    let len = checked_len(reader.get_i32_be()?)?;
    let mut long_array = Vec::with_capacity(len);
    for _ in 0..len {
        long_array.push(reader.get_i64_be()?);
    }
    Ok(Nbt::LongArray(long_array.into_boxed_slice()))
}
