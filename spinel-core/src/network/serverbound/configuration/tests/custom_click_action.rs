use super::super::custom_click_action::ConfigurationCustomClickActionPacket;
use spinel_nbt::Nbt;
use spinel_network::data_type::DataType;
use spinel_network::types::{CustomClickActionPayload, Identifier};

#[test]
fn configuration_custom_click_action_roundtrips_length_prefixed_optional_nbt_payload() {
    let packet = ConfigurationCustomClickActionPacket {
        click_action_id: Identifier::minecraft("dialog"),
        payload: CustomClickActionPayload::from_tag(Nbt::String("confirm".to_string())).unwrap(),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet =
        ConfigurationCustomClickActionPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(ConfigurationCustomClickActionPacket::get_id_const(), 0x08);
    assert_eq!(
        decoded_packet.click_action_id,
        Identifier::minecraft("dialog")
    );
    assert_eq!(
        decoded_packet.payload.tag().unwrap(),
        Some(Nbt::String("confirm".to_string()))
    );
}
