use crate::data_type::DataType;
use crate::types::math::Vector3d;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vector3d);

impl DataType for Velocity {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let x = (self.0.x * 8000.0) as i16;
        let y = (self.0.y * 8000.0) as i16;
        let z = (self.0.z * 8000.0) as i16;
        x.encode(w)?;
        y.encode(w)?;
        z.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let x = i16::decode(r)? as f64 / 8000.0;
        let y = i16::decode(r)? as f64 / 8000.0;
        let z = i16::decode(r)? as f64 / 8000.0;
        Ok(Velocity(Vector3d { x, y, z }))
    }
}
