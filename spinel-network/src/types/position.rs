use crate::data_type::DataType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl DataType for Position {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let val = ((self.x as u64 & 0x3FFFFFF) << 38)
            | ((self.z as u64 & 0x3FFFFFF) << 12)
            | (self.y as u64 & 0xFFF);
        w.write_all(&val.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        let val = u64::from_be_bytes(buf);

        let x_raw = (val >> 38) as i64;
        let y_raw = (val & 0xFFF) as i64;
        let z_raw = (val << 26 >> 38) as i64;

        let x = (if x_raw >= 33554432 {
            x_raw - 67108864
        } else {
            x_raw
        }) as i32;
        let y = (if y_raw >= 2048 { y_raw - 4096 } else { y_raw }) as i32;
        let z = (if z_raw >= 33554432 {
            z_raw - 67108864
        } else {
            z_raw
        }) as i32;

        Ok(Position { x, y, z })
    }
}
