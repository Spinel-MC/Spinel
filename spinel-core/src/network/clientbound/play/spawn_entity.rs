use spinel_macros::packet;
use spinel_network::data_type::DataType;
use spinel_network::types::{VarInt, Velocity};
use std::io::{self, Read, Write};
use uuid::Uuid;

#[packet(id: "add_entity", state: ConnectionState::Play, recipient: Recipient::Client)]
#[derive(Clone)]
pub struct SpawnEntityPacket {
    pub entity_id: VarInt,
    pub uuid: Uuid,
    pub entity_type: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity: Velocity,
    pub pitch: EntityAngle,
    pub yaw: EntityAngle,
    pub head_yaw: EntityAngle,
    pub data: VarInt,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EntityAngle(pub f32);

impl DataType for EntityAngle {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        ((self.0 * 256.0 / 360.0) as i32 as u8).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self(u8::decode(reader)? as f32 * 360.0 / 256.0))
    }
}

impl SpawnEntityPacket {
    pub fn new(entity_id: i32, uuid: Uuid, entity_type: i32, x: f64, y: f64, z: f64) -> Self {
        Self {
            entity_id,
            uuid,
            entity_type,
            x,
            y,
            z,
            velocity: Velocity(spinel_network::types::Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            pitch: EntityAngle(0.0),
            yaw: EntityAngle(0.0),
            head_yaw: EntityAngle(0.0),
            data: 0,
        }
    }
}
