use spinel_macros::packet;
use spinel_network::data_type::DataType;
use spinel_network::types::{VarInt, Vector3d};
use std::io::{self, Read, Write};

#[packet(
    id: "move_minecart_along_track",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct MoveMinecartPacket {
    pub entity_id: VarInt,
    pub lerp_steps: Vec<MinecartLerpStep>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinecartLerpStep {
    pub position: Vector3d,
    pub velocity: Vector3d,
    pub yaw: f32,
    pub pitch: f32,
    pub weight: f32,
}

impl DataType for MinecartLerpStep {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.position.encode(writer)?;
        self.velocity.encode(writer)?;
        self.yaw.encode(writer)?;
        self.pitch.encode(writer)?;
        self.weight.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            position: Vector3d::decode(reader)?,
            velocity: Vector3d::decode(reader)?,
            yaw: f32::decode(reader)?,
            pitch: f32::decode(reader)?,
            weight: f32::decode(reader)?,
        })
    }
}
