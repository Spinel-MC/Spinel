use crate::*;
use std::io::{self, Read};

#[derive(Debug)]
pub struct NbtReadHelper<R: Read> {
    pub reader: R,
}

impl<R: Read> NbtReadHelper<R> {
    pub fn new(r: R) -> Self {
        Self { reader: r }
    }
}

impl<R: Read> NbtReadHelper<R> {
    pub fn skip_bytes(&mut self, count: u64) -> Result<(), Error> {
        io::copy(&mut self.reader.by_ref().take(count), &mut io::sink())
            .map_err(Error::Incomplete)?;
        Ok(())
    }

    pub fn get_u8_be(&mut self) -> Result<u8, Error> {
        let mut buf = [0u8];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(u8::from_be_bytes(buf))
    }

    pub fn get_i8_be(&mut self) -> Result<i8, Error> {
        let mut buf = [0u8];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(i8::from_be_bytes(buf))
    }

    pub fn get_i16_be(&mut self) -> Result<i16, Error> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn get_u16_be(&mut self) -> Result<u16, Error> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn get_i32_be(&mut self) -> Result<i32, Error> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(i32::from_be_bytes(buf))
    }

    pub fn get_i64_be(&mut self) -> Result<i64, Error> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(i64::from_be_bytes(buf))
    }

    pub fn get_f32_be(&mut self) -> Result<f32, Error> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(f32::from_be_bytes(buf))
    }

    pub fn get_f64_be(&mut self) -> Result<f64, Error> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(f64::from_be_bytes(buf))
    }

    pub fn read_boxed_slice(&mut self, count: usize) -> Result<Box<[u8]>, Error> {
        let mut buf = vec![0u8; count];
        self.reader.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(buf.into())
    }
}