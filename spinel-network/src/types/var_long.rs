use crate::data_type::DataType;
use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};

pub type VarLong = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarLongWrapper(pub i64);

impl From<i64> for VarLongWrapper {
    fn from(v: i64) -> Self {
        Self(v)
    }
}

impl From<VarLongWrapper> for i64 {
    fn from(v: VarLongWrapper) -> Self {
        v.0
    }
}

impl Deref for VarLongWrapper {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VarLongWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataType for VarLongWrapper {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut uvalue = self.0 as u64;
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
        let mut result: i64 = 0;
        let mut buf = [0u8; 1];
        loop {
            r.read_exact(&mut buf)?;
            let read = buf[0];
            let value = (read & 0x7F) as i64;
            result |= value << (7 * num_read);
            if (read & 0x80) == 0 {
                break;
            }
            num_read += 1;
            if num_read >= 10 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarLong is too big",
                ));
            }
        }
        Ok(VarLongWrapper(result))
    }
}
