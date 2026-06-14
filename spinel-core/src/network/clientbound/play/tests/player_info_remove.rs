use super::super::player_info_remove::PlayerInfoRemovePacket;
use spinel_network::DataType;
use uuid::Uuid;

#[test]
fn player_info_remove_packet_matches_minestom_uuid_list_shape() {
    let packet = PlayerInfoRemovePacket::new(Uuid::nil());
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerInfoRemovePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(PlayerInfoRemovePacket::get_id(), 0x43);
    assert_eq!(decoded_packet.profile_ids.0, vec![Uuid::nil()]);
}
