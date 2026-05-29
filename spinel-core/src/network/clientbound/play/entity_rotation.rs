use crate::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "move_entity_rot", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityRotationPacket {
    pub entity_id: VarInt,
    pub yaw: EntityAngle,
    pub pitch: EntityAngle,
    pub on_ground: bool,
}

#[cfg(test)]
mod tests {
    use super::EntityRotationPacket;
    use crate::network::clientbound::play::spawn_entity::EntityAngle;
    use spinel_network::DataType;

    #[test]
    fn entity_rotation_packet_matches_minestom_var_int_angles_and_ground_shape() {
        let packet = EntityRotationPacket {
            entity_id: 7,
            yaw: EntityAngle(90.0),
            pitch: EntityAngle(45.0),
            on_ground: true,
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(EntityRotationPacket::get_id(), 0x36);
        assert_eq!(payload, vec![7, 64, 32, 1]);
    }
}
