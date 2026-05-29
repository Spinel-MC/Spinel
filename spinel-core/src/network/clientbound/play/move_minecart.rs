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

#[cfg(test)]
mod tests {
    use super::{MinecartLerpStep, MoveMinecartPacket};
    use spinel_network::DataType;
    use spinel_network::types::Vector3d;

    #[test]
    fn move_minecart_packet_matches_minestom_lerp_step_shape() {
        let packet = MoveMinecartPacket {
            entity_id: 7,
            lerp_steps: vec![MinecartLerpStep {
                position: Vector3d {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                },
                velocity: Vector3d {
                    x: 0.1,
                    y: 0.2,
                    z: 0.3,
                },
                yaw: 90.0,
                pitch: 45.0,
                weight: 1.0,
            }],
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = MoveMinecartPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(MoveMinecartPacket::get_id(), 0x35);
        assert_eq!(decoded_packet.entity_id, packet.entity_id);
        assert_eq!(decoded_packet.lerp_steps, packet.lerp_steps);
    }
}
