use crate::data_type::DataType;
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

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
        let _nbt = NbtCompound::decode(r)?;
        Ok(NbtTextComponent(TextComponent::literal(
            "NBT Component (Not Implemented)",
        )))
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
