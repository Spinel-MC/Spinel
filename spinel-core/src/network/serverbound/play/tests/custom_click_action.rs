use super::super::custom_click_action::PlayCustomClickActionPacket;
use spinel_nbt::Nbt;
use spinel_network::data_type::DataType;
use spinel_network::types::{CustomClickActionPayload, Identifier};

#[test]
fn play_custom_click_action_roundtrips_length_prefixed_optional_nbt_payload() {
    let packet = PlayCustomClickActionPacket {
        click_action_id: Identifier::minecraft("dialog"),
        payload: CustomClickActionPayload::from_tag(Nbt::String("accept".to_string())).unwrap(),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayCustomClickActionPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(PlayCustomClickActionPacket::get_id_const(), 0x41);
    assert_eq!(
        decoded_packet.click_action_id,
        Identifier::minecraft("dialog")
    );
    assert_eq!(
        decoded_packet.payload.tag().unwrap(),
        Some(Nbt::String("accept".to_string()))
    );
}
