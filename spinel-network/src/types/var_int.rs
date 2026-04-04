use crate::data_type::DataType;
use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};

pub type VarInt = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarIntWrapper(pub i32);

impl From<i32> for VarIntWrapper {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl From<VarIntWrapper> for i32 {
    fn from(v: VarIntWrapper) -> Self {
        v.0
    }
}

impl Deref for VarIntWrapper {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VarIntWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataType for VarIntWrapper {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut uvalue = self.0 as u32;
        loop {
            if (uvalue & !0x7F) == 0 {
                w.write_all(&[uvalue as u8])?;
                return Ok(());
            }
            w.write_all(&[((uvalue & 0x7F) | 0x80) as u8])?;
            uvalue >>= 7;
        }
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut num_read = 0;
        let mut result = 0;
        let mut buf = [0u8; 1];
        loop {
            r.read_exact(&mut buf)?;
            let read = buf[0];
            let value = (read & 0x7F) as i32;
            result |= value << (7 * num_read);
            if (read & 0x80) == 0 {
                break;
            }
            num_read += 1;
            if num_read >= 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt is too big",
                ));
            }
        }
        Ok(VarIntWrapper(result))
    }
}
