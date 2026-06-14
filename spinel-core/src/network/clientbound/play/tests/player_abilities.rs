use super::super::player_abilities::PlayerAbilitiesPacket;
use spinel_network::DataType;

#[test]
fn player_abilities_packet_matches_minestom_shape() {
    let packet = PlayerAbilitiesPacket::new(
        PlayerAbilitiesPacket::INVULNERABLE
            | PlayerAbilitiesPacket::ALLOW_FLYING
            | PlayerAbilitiesPacket::INSTANT_BREAK,
        0.05,
        0.1,
    );
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerAbilitiesPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(PlayerAbilitiesPacket::get_id(), 0x3e);
    assert_eq!(payload.len(), 9);
    assert_eq!(decoded_packet.flags, 0x0d);
    assert_eq!(decoded_packet.flying_speed, 0.05);
    assert_eq!(decoded_packet.field_view_modifier, 0.1);
}
