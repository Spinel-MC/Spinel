use crate::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "move_entity_pos_rot", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityPositionAndRotationPacket {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: EntityAngle,
    pub pitch: EntityAngle,
    pub on_ground: bool,
}

#[cfg(test)]
mod tests {
    use super::EntityPositionAndRotationPacket;
    use crate::network::clientbound::play::entity_position::EntityPositionPacket;
    use crate::network::clientbound::play::spawn_entity::EntityAngle;
    use spinel_network::DataType;

    #[test]
    fn entity_position_and_rotation_packet_matches_minestom_shape() {
        let packet = EntityPositionAndRotationPacket {
            entity_id: 7,
            delta_x: EntityPositionPacket::delta(1.5, 1.0),
            delta_y: 0,
            delta_z: 0,
            yaw: EntityAngle(90.0),
            pitch: EntityAngle(45.0),
            on_ground: false,
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet =
            EntityPositionAndRotationPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(EntityPositionAndRotationPacket::get_id(), 0x34);
        assert_eq!(decoded_packet.entity_id, 7);
        assert_eq!(decoded_packet.delta_x, 2048);
        assert_eq!(decoded_packet.yaw.0, 90.0);
        assert!(!decoded_packet.on_ground);
    }
}
