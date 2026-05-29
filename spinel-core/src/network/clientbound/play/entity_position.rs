use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "move_entity_pos", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityPositionPacket {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

impl EntityPositionPacket {
    pub fn delta(new_coordinate: f64, old_coordinate: f64) -> i16 {
        ((new_coordinate * 32.0 - old_coordinate * 32.0) * 128.0) as i16
    }
}

#[cfg(test)]
mod tests {
    use super::EntityPositionPacket;
    use spinel_network::DataType;

    #[test]
    fn entity_position_packet_matches_minestom_relative_delta_shape() {
        let packet = EntityPositionPacket {
            entity_id: 7,
            delta_x: EntityPositionPacket::delta(1.5, 1.0),
            delta_y: EntityPositionPacket::delta(2.0, 2.0),
            delta_z: EntityPositionPacket::delta(2.5, 3.0),
            on_ground: true,
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = EntityPositionPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(EntityPositionPacket::get_id(), 0x33);
        assert_eq!(decoded_packet.entity_id, 7);
        assert_eq!(decoded_packet.delta_x, 2048);
        assert_eq!(decoded_packet.delta_y, 0);
        assert_eq!(decoded_packet.delta_z, -2048);
        assert!(decoded_packet.on_ground);
    }
}
