use crate::data_type::DataType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl SectionPosition {
    pub const fn packed(self) -> i64 {
        (((self.x as i64) & 4_194_303) << 42)
            | (((self.z as i64) & 4_194_303) << 20)
            | ((self.y as i64) & 1_048_575)
    }

    pub const fn from_packed(packed: i64) -> Self {
        Self {
            x: (packed << 0 >> 42) as i32,
            y: (packed << 44 >> 44) as i32,
            z: (packed << 22 >> 42) as i32,
        }
    }
}

impl DataType for SectionPosition {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.packed().encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self::from_packed(i64::decode(reader)?))
    }
}
