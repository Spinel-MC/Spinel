use super::super::custom_click_action::ConfigurationCustomClickActionPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::Identifier;

#[test]
fn configuration_custom_click_action_keeps_raw_payload_and_maps_end_tag_to_none() {
    let packet = ConfigurationCustomClickActionPacket {
        key: Identifier::minecraft("dialog"),
        payload: vec![0],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet =
        ConfigurationCustomClickActionPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(ConfigurationCustomClickActionPacket::get_id_const(), 0x08);
    assert_eq!(decoded_packet.key, Identifier::minecraft("dialog"));
    assert_eq!(decoded_packet.payload_without_end_tag(), None);
}
