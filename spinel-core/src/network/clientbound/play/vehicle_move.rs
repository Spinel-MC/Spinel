use spinel_macros::packet;
use spinel_network::data_type::DataType;
use std::io::{self, Read, Write};

#[packet(id: "move_vehicle", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct VehicleMovePacket {
    pub position: VehiclePosition,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehiclePosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl DataType for VehiclePosition {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.x.encode(writer)?;
        self.y.encode(writer)?;
        self.z.encode(writer)?;
        self.yaw.encode(writer)?;
        self.pitch.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            x: f64::decode(reader)?,
            y: f64::decode(reader)?,
            z: f64::decode(reader)?,
            yaw: f32::decode(reader)?,
            pitch: f32::decode(reader)?,
        })
    }
}
