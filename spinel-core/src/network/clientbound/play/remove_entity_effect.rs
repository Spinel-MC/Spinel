use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "remove_mob_effect", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct RemoveEntityEffectPacket {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
}

#[cfg(test)]
mod tests {
    use super::RemoveEntityEffectPacket;
    use spinel_network::DataType;

    #[test]
    fn remove_entity_effect_packet_matches_minestom_var_int_shape() {
        let packet = RemoveEntityEffectPacket {
            entity_id: 7,
            effect_id: 1,
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(RemoveEntityEffectPacket::get_id(), 0x4c);
        assert_eq!(payload, vec![7, 1]);
    }
}
