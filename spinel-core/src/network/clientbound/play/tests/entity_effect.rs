use super::super::entity_effect::EntityEffectPacket;
use spinel_network::DataType;

#[test]
fn entity_effect_packet_matches_minestom_potion_network_shape() {
    let packet = EntityEffectPacket {
        entity_id: 7,
        effect_id: 1,
        amplifier: 2,
        duration_ticks: 40,
        flags: 6,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(EntityEffectPacket::get_id(), 0x82);
    assert_eq!(payload, vec![7, 1, 2, 40, 6]);
}
