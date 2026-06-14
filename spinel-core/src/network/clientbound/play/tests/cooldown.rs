use super::super::cooldown::CooldownPacket;
use spinel_network::DataType;

#[test]
fn cooldown_packet_uses_play_packet_id_string_group_and_var_int_ticks() {
    let packet = CooldownPacket::new("minecraft:ender_pearl", 20);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(CooldownPacket::get_id(), 0x16);
    assert_eq!(payload.last().copied(), Some(20));
}
