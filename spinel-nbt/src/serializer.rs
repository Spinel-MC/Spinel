// File: spinel-nbt/src/serializer.rs
use std::io::Write;

use crate::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct WriteAdaptor<W: Write> {
    pub writer: W,
}

impl<W: Write> WriteAdaptor<W> {
    pub fn new(w: W) -> Self {
        Self { writer: w }
    }
}

impl<W: Write> WriteAdaptor<W> {
    pub fn write_u8_be(&mut self, value: u8) -> Result<()> {
        self.writer
            .write_all(&value.to_be_bytes())
            .map_err(Error::Incomplete)
    }

    pub fn write_i8_be(&mut self, value: i8) -> Result<()> {
        self.writer
            .write_all(&value.to_be_bytes())
            .map_err(Error::Incomplete)
    }

    pub fn write_u16_be(&mut self, value: u16) -> Result<()> {
        self.writer
            .write_all(&value.to_be_bytes())
            .map_err(Error::Incomplete)
    }

    pub fn write_i16_be(&mut self, value: i16) -> Result<()> {
        self.writer
            .write_all(&value.to_be_bytes())
            .map_err(Error::Incomplete)
    }

    pub fn write_i32_be(&mut self, value: i32) -> Result<()> {
        self.writer
            .write_all(&value.to_be_bytes())
            .map_err(Error::Incomplete)
    }

    pub fn write_i64_be(&mut self, value: i64) -> Result<()> {
        self.writer
            .write_all(&value.to_be_bytes())
            .map_err(Error::Incomplete)
    }

    pub fn write_f32_be(&mut self, value: f32) -> Result<()> {
        self.writer
            .write_all(&value.to_be_bytes())
            .map_err(Error::Incomplete)
    }

    pub fn write_f64_be(&mut self, value: f64) -> Result<()> {
        self.writer
            .write_all(&value.to_be_bytes())
            .map_err(Error::Incomplete)
    }

    pub fn write_slice(&mut self, value: &[u8]) -> Result<()> {
        self.writer.write_all(value).map_err(Error::Incomplete)
    }
}

pub fn to_bytes_unnamed<T: crate::tag::NbtWritable>(value: &T, w: impl Write) -> Result<()> {
    let mut adaptor = WriteAdaptor::new(w);
    value.write_unnamed(&mut adaptor)
}
