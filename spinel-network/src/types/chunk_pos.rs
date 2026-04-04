use crate::data_type::DataType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl DataType for ChunkPos {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let val = (self.x as i64 & 0xFFFFFFFF) | ((self.z as i64 & 0xFFFFFFFF) << 32);
        w.write_all(&val.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        let val = i64::from_be_bytes(buf);
        Ok(ChunkPos {
            x: (val & 0xFFFFFFFF) as i32,
            z: (val >> 32) as i32,
        })
    }
}
