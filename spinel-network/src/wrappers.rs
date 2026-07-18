use crate::data_type::DataType;
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct JsonTextComponent(pub TextComponent);

impl DataType for JsonTextComponent {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let json = self.0.to_json_string();
        json.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let json = String::decode(r)?;
        let component = serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(JsonTextComponent(component))
    }
}

#[derive(Debug, Clone)]
pub struct NbtTextComponent(pub TextComponent);

impl DataType for NbtTextComponent {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let nbt = self.0.to_nbt_compound();
        nbt.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let nbt = Nbt::read_unnamed(r)?;
        let json = spinel_nbt::nbt_to_json(nbt);
        let component = serde_json::from_value(json)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        Ok(NbtTextComponent(component))
    }
}

impl DataType for NbtCompound {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        spinel_nbt::to_bytes_unnamed(self, w).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let tag = Nbt::read_unnamed(r)?;
        if let Nbt::Compound(c) = tag {
            Ok(c)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected NBT Compound",
            ))
        }
    }
}

pub type ByteArray = Vec<u8>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawBytes(pub Vec<u8>);

impl DataType for RawBytes {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Ok(Self(bytes))
    }
}

impl Deref for RawBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RawBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<u8>> for RawBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl From<RawBytes> for Vec<u8> {
    fn from(bytes: RawBytes) -> Self {
        bytes.0
    }
}

impl AsRef<[u8]> for RawBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<JsonTextComponent> for TextComponent {
    fn from(w: JsonTextComponent) -> Self {
        w.0
    }
}

impl From<NbtTextComponent> for TextComponent {
    fn from(w: NbtTextComponent) -> Self {
        w.0
    }
}
