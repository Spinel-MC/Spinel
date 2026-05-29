use spinel_macros::packet;
use spinel_network::types::{VarInt, Velocity};

#[packet(id: "set_entity_motion", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityVelocityPacket {
    pub entity_id: VarInt,
    pub velocity: Velocity,
}

#[cfg(test)]
mod tests {
    use super::EntityVelocityPacket;
    use spinel_network::DataType;
    use spinel_network::types::{Vector3d, Velocity};

    #[test]
    fn entity_velocity_packet_matches_minestom_var_int_then_lpv3_shape() {
        let packet = EntityVelocityPacket {
            entity_id: 7,
            velocity: Velocity(Vector3d {
                x: 1.0,
                y: -1.0,
                z: 0.5,
            }),
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = EntityVelocityPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(EntityVelocityPacket::get_id(), 0x63);
        assert_eq!(decoded_packet.entity_id, 7);
        assert!((decoded_packet.velocity.0.x - 1.0).abs() < 0.001);
        assert!((decoded_packet.velocity.0.y + 1.0).abs() < 0.001);
        assert!((decoded_packet.velocity.0.z - 0.5).abs() < 0.001);
    }

    #[test]
    fn entity_velocity_packet_encodes_zero_velocity_as_single_lpv3_byte() {
        let packet = EntityVelocityPacket {
            entity_id: 7,
            velocity: Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(payload, vec![7, 0]);
    }
}
