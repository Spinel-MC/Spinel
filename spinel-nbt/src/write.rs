use crate::serializer::WriteAdaptor;
use crate::*;
use std::io::{self, Write};

impl Nbt {
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
            Nbt::String(string) => write_string(w, string)?,
            Nbt::List(list) => write_list(w, list)?,
            Nbt::Compound(compound) => compound.serialize_content(w)?,
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

    pub fn write<W: Write>(&self, name: &str, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[self.get_type_id()])?;
        if self.get_type_id() != END_ID {
            write_name(writer, name)?;
            self.write_payload(writer)?;
        }
        Ok(())
    }

    pub fn write_unnamed<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[self.get_type_id()])?;
        if self.get_type_id() != END_ID {
            self.write_payload(writer)?;
        }
        Ok(())
    }

    fn write_payload<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut adaptor = WriteAdaptor::new(writer);
        self.serialize_data(&mut adaptor).map_err(io::Error::other)
    }
}

fn write_string<W: Write>(writer: &mut WriteAdaptor<W>, string: &str) -> Result<(), Error> {
    let java_string = cesu8::to_java_cesu8(string);
    writer.write_u16_be(java_string.len() as u16)?;
    writer.write_slice(&java_string)
}

fn write_list<W: Write>(writer: &mut WriteAdaptor<W>, list: &[Nbt]) -> Result<(), Error> {
    let list_type_id = list.first().map_or(END_ID, Nbt::get_type_id);
    if list.iter().any(|nbt| nbt.get_type_id() != list_type_id) {
        return Err(Error::UnsupportedType("mixed NBT list".to_string()));
    }
    writer.write_u8_be(list_type_id)?;
    writer.write_i32_be(list.len() as i32)?;
    for nbt_tag in list.iter() {
        nbt_tag.serialize_data(writer)?;
    }
    Ok(())
}

fn write_name<W: Write>(writer: &mut W, name: &str) -> io::Result<()> {
    let name_bytes = cesu8::to_java_cesu8(name);
    writer.write_all(&(name_bytes.len() as u16).to_be_bytes())?;
    writer.write_all(&name_bytes)
}
