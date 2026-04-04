use crate::data_type::DataType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy)]
pub struct TeleportFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub y_rot: bool,
    pub x_rot: bool,
}

impl DataType for TeleportFlags {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut byte = 0;
        if self.x {
            byte |= 0x01;
        }
        if self.y {
            byte |= 0x02;
        }
        if self.z {
            byte |= 0x04;
        }
        if self.y_rot {
            byte |= 0x08;
        }
        if self.x_rot {
            byte |= 0x10;
        }
        w.write_all(&[byte])
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        let byte = buf[0];

        Ok(Self {
            x: (byte & 0x01) != 0,
            y: (byte & 0x02) != 0,
            z: (byte & 0x04) != 0,
            y_rot: (byte & 0x08) != 0,
            x_rot: (byte & 0x10) != 0,
        })
    }
}
