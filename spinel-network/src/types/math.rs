use crate::data_type::DataType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl DataType for Vector3f {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.x.encode(w)?;
        self.y.encode(w)?;
        self.z.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Vector3f {
            x: f32::decode(r)?,
            y: f32::decode(r)?,
            z: f32::decode(r)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DataType for Vector3d {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.x.encode(w)?;
        self.y.encode(w)?;
        self.z.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Vector3d {
            x: f64::decode(r)?,
            y: f64::decode(r)?,
            z: f64::decode(r)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternionf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl DataType for Quaternionf {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.x.encode(w)?;
        self.y.encode(w)?;
        self.z.encode(w)?;
        self.w.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Quaternionf {
            x: f32::decode(r)?,
            y: f32::decode(r)?,
            z: f32::decode(r)?,
            w: f32::decode(r)?,
        })
    }
}
