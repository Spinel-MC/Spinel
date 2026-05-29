use spinel_macros::packet;
use spinel_network::types::{Optional, VarInt, Vector3d};

#[packet(id: "damage_event", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct DamageEventPacket {
    pub target_entity_id: VarInt,
    pub damage_type_id: VarInt,
    pub source_entity_id: VarInt,
    pub source_direct_id: VarInt,
    pub source_position: Optional<Vector3d>,
}

#[cfg(test)]
mod tests {
    use super::DamageEventPacket;
    use spinel_network::DataType;

    #[test]
    fn damage_event_packet_matches_minestom_var_int_optional_position_shape() {
        let packet = DamageEventPacket {
            target_entity_id: 7,
            damage_type_id: 1,
            source_entity_id: 0,
            source_direct_id: 0,
            source_position: None,
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(DamageEventPacket::get_id(), 0x19);
        assert_eq!(payload, vec![7, 1, 0, 0, 0]);
    }
}
