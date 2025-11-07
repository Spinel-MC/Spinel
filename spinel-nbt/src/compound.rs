use crate::deserializer::NbtReadHelper;
use crate::serializer::WriteAdaptor;
use crate::tag::Nbt;
use crate::{END_ID, Error, get_nbt_string};
use std::collections::BTreeMap;
use std::io::{ErrorKind, Read, Write};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct NbtCompound(pub BTreeMap<String, Nbt>);

impl NbtCompound {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, name: String, value: impl Into<Nbt>) {
        self.0.insert(name, value.into());
    }

    pub fn get(&self, name: &str) -> Option<&Nbt> {
        self.0.get(name)
    }

    pub fn skip_content<R>(reader: &mut NbtReadHelper<R>) -> Result<(), Error>
    where
        R: Read,
    {
        loop {
            let tag_id = match reader.get_u8_be() {
                Ok(id) => id,
                Err(err) => match err {
                    Error::Incomplete(err) => match err.kind() {
                        ErrorKind::UnexpectedEof => break,
                        _ => return Err(Error::Incomplete(err)),
                    },
                    _ => return Err(err),
                },
            };
            if tag_id == END_ID {
                break;
            }

            let len = reader.get_u16_be()?;
            reader.skip_bytes(u64::from(len))?;

            Nbt::skip_data(reader, tag_id)?;
        }
        Ok(())
    }

    pub fn deserialize_content<R>(reader: &mut NbtReadHelper<R>) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut compound = Self::new();
        loop {
            let tag_id = match reader.get_u8_be() {
                Ok(id) => id,
                Err(err) => match err {
                    Error::Incomplete(err) => match err.kind() {
                        ErrorKind::UnexpectedEof => break,
                        _ => return Err(Error::Incomplete(err)),
                    },
                    _ => return Err(err),
                },
            };
            if tag_id == END_ID {
                break;
            }

            let name = get_nbt_string(reader)?;
            let tag = Nbt::deserialize_data(reader, tag_id)?;
            compound.insert(name, tag);
        }
        Ok(compound)
    }

    pub fn serialize_content<W>(&self, w: &mut WriteAdaptor<W>) -> Result<(), Error>
    where
        W: Write,
    {
        for (name, tag) in &self.0 {
            w.write_u8_be(tag.get_type_id())?;
            let name_bytes = cesu8::to_java_cesu8(name);
            w.write_u16_be(name_bytes.len() as u16)?;
            w.write_slice(&name_bytes)?;
            tag.serialize_data(w)?;
        }
        w.write_u8_be(END_ID)?;
        Ok(())
    }
}

impl From<NbtCompound> for Nbt {
    fn from(value: NbtCompound) -> Self {
        Nbt::Compound(value)
    }
}
