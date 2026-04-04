use std::io::{self, Read, Write};
use uuid::Uuid;

use crate::types::var_int::VarIntWrapper;

pub trait DataType: Sized {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()>;
    fn decode<R: Read>(r: &mut R) -> io::Result<Self>;
}

impl DataType for bool {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&[if *self { 1 } else { 0 }])
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
}

impl DataType for u8 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&[*self])
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl DataType for i8 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(i8::from_be_bytes(buf))
    }
}

impl DataType for u16 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 2];
        r.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
}

impl DataType for i16 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 2];
        r.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }
}

impl DataType for i32 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }
}

impl DataType for i64 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }
}

impl DataType for u64 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
}

impl DataType for f32 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }
}

impl DataType for f64 {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.to_be_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }
}

impl DataType for String {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.len() as i32).encode(w)?;
        w.write_all(self.as_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let len = VarIntWrapper::decode(r)?.0 as usize;
        if len > 32767 * 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "String too long",
            ));
        }
        let mut buf = vec![0u8; len];
        r.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl DataType for Uuid {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(self.as_bytes())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 16];
        r.read_exact(&mut buf)?;
        Ok(Uuid::from_bytes(buf))
    }
}

impl<T: DataType> DataType for Option<T> {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Some(val) => {
                true.encode(w)?;
                val.encode(w)
            }
            None => false.encode(w),
        }
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let is_present = bool::decode(r)?;
        if is_present {
            Ok(Some(T::decode(r)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: DataType> DataType for Vec<T> {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.len() as i32).encode(w)?;
        for item in self {
            item.encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let len = VarIntWrapper::decode(r)?.0 as usize;
        if len > 1_000_000 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Vec length too large",
            ));
        }
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(r)?);
        }
        Ok(vec)
    }
}

impl<A: DataType, B: DataType> DataType for (A, B) {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.encode(w)?;
        self.1.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok((A::decode(r)?, B::decode(r)?))
    }
}

impl<A: DataType, B: DataType, C: DataType> DataType for (A, B, C) {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.encode(w)?;
        self.1.encode(w)?;
        self.2.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok((A::decode(r)?, B::decode(r)?, C::decode(r)?))
    }
}

impl<K: DataType + std::hash::Hash + Eq, V: DataType> DataType for std::collections::HashMap<K, V> {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.len() as i32).encode(w)?;
        for (k, v) in self {
            k.encode(w)?;
            v.encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let len = VarIntWrapper::decode(r)?.0 as usize;
        if len > 1_000_000 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "HashMap length too large",
            ));
        }
        let mut map = std::collections::HashMap::with_capacity(len);
        for _ in 0..len {
            map.insert(K::decode(r)?, V::decode(r)?);
        }
        Ok(map)
    }
}
