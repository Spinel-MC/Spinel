use super::super::custom_click_action::PlayCustomClickActionPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::Identifier;

#[test]
fn play_custom_click_action_keeps_raw_payload_and_maps_end_tag_to_none() {
    let packet = PlayCustomClickActionPacket {
        key: Identifier::minecraft("dialog"),
        payload: vec![0],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayCustomClickActionPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(PlayCustomClickActionPacket::get_id_const(), 0x41);
    assert_eq!(decoded_packet.key, Identifier::minecraft("dialog"));
    assert_eq!(decoded_packet.payload_without_end_tag(), None);
}
