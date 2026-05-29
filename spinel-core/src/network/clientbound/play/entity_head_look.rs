use crate::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "rotate_head", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityHeadLookPacket {
    pub entity_id: VarInt,
    pub head_yaw: EntityAngle,
}

#[cfg(test)]
mod tests {
    use super::EntityHeadLookPacket;
    use crate::network::clientbound::play::spawn_entity::EntityAngle;
    use spinel_network::DataType;

    #[test]
    fn entity_head_look_packet_matches_minestom_var_int_then_angle_shape() {
        let packet = EntityHeadLookPacket {
            entity_id: 7,
            head_yaw: EntityAngle(90.0),
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(EntityHeadLookPacket::get_id(), 0x51);
        assert_eq!(payload, vec![7, 64]);
    }
}
