use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use crate::wrappers::JsonTextComponent;
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapId {
    pub id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapDecoration {
    pub map_decoration_type_id: i32,
    pub x: i8,
    pub y: i8,
    pub rotation: i8,
    pub name: Option<TextComponent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapColorPatch {
    pub start_x: u8,
    pub start_y: u8,
    pub width: u8,
    pub height: u8,
    pub map_colors: Vec<u8>,
}

impl DataType for MapId {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.id).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            id: VarIntWrapper::decode(reader)?.0,
        })
    }
}

impl DataType for MapDecoration {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.map_decoration_type_id).encode(writer)?;
        self.x.encode(writer)?;
        self.y.encode(writer)?;
        (self.rotation & 15).encode(writer)?;
        self.name.clone().map(JsonTextComponent).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            map_decoration_type_id: VarIntWrapper::decode(reader)?.0,
            x: i8::decode(reader)?,
            y: i8::decode(reader)?,
            rotation: i8::decode(reader)? & 15,
            name: Option::<JsonTextComponent>::decode(reader)?.map(|name| name.0),
        })
    }
}

impl MapColorPatch {
    pub fn encode_optional<W: Write>(patch: &Option<Self>, writer: &mut W) -> io::Result<()> {
        match patch {
            Some(patch) => {
                patch.width.encode(writer)?;
                patch.height.encode(writer)?;
                patch.start_x.encode(writer)?;
                patch.start_y.encode(writer)?;
                VarIntWrapper(patch.map_colors.len() as i32).encode(writer)?;
                writer.write_all(&patch.map_colors)
            }
            None => 0u8.encode(writer),
        }
    }

    pub fn decode_optional<R: Read>(reader: &mut R) -> io::Result<Option<Self>> {
        let width = u8::decode(reader)?;
        if width == 0 {
            return Ok(None);
        }

        let height = u8::decode(reader)?;
        let start_x = u8::decode(reader)?;
        let start_y = u8::decode(reader)?;
        let color_count = VarIntWrapper::decode(reader)?.0;
        if color_count < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "map color patch length cannot be negative",
            ));
        }

        let mut map_colors = vec![0; color_count as usize];
        reader.read_exact(&mut map_colors)?;
        Ok(Some(Self {
            start_x,
            start_y,
            width,
            height,
            map_colors,
        }))
    }
}
