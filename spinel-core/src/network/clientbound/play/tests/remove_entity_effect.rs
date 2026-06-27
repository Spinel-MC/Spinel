use super::super::remove_entity_effect::RemoveEntityEffectPacket;
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

    let decoded_packet = RemoveEntityEffectPacket::decode(&mut payload.as_slice()).unwrap();
    assert_eq!(decoded_packet.entity_id, packet.entity_id);
    assert_eq!(decoded_packet.effect_id, packet.effect_id);
}
