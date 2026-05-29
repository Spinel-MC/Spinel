use spinel_macros::packet;
use spinel_network::types::{VarInt, Vector3d};

#[packet(id: "teleport_entity", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityTeleportPacket {
    pub entity_id: VarInt,
    pub position: Vector3d,
    pub delta: Vector3d,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i32,
    pub on_ground: bool,
}

#[cfg(test)]
mod tests {
    use super::EntityTeleportPacket;
    use spinel_network::DataType;
    use spinel_network::types::Vector3d;

    #[test]
    fn entity_teleport_packet_matches_minestom_serializer_field_order() {
        let packet = EntityTeleportPacket {
            entity_id: 7,
            position: Vector3d {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            delta: Vector3d {
                x: 0.1,
                y: 0.2,
                z: 0.3,
            },
            yaw: 45.0,
            pitch: 90.0,
            flags: 0,
            on_ground: true,
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = EntityTeleportPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(EntityTeleportPacket::get_id(), 0x7b);
        assert_eq!(decoded_packet.entity_id, 7);
        assert_eq!(decoded_packet.position, packet.position);
        assert_eq!(decoded_packet.delta, packet.delta);
        assert_eq!(decoded_packet.yaw, 45.0);
        assert!(decoded_packet.on_ground);
    }
}
