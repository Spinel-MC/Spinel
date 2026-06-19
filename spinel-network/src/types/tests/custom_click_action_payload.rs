use crate::data_type::DataType;
use crate::types::CustomClickActionPayload;
use spinel_nbt::Nbt;

#[test]
fn custom_click_action_payload_roundtrips_absent_payload_as_length_prefixed_end_tag() {
    let payload = CustomClickActionPayload::none();
    let mut bytes = Vec::new();

    payload.encode(&mut bytes).unwrap();
    let decoded_payload = CustomClickActionPayload::decode(&mut bytes.as_slice()).unwrap();

    assert_eq!(bytes, [1, 0]);
    assert_eq!(decoded_payload.bytes(), None);
}

#[test]
fn custom_click_action_payload_roundtrips_named_payload_bytes_and_semantic_tag() {
    let payload = CustomClickActionPayload::from_tag(Nbt::String("dialog".to_string())).unwrap();
    let mut bytes = Vec::new();

    payload.encode(&mut bytes).unwrap();
    let decoded_payload = CustomClickActionPayload::decode(&mut bytes.as_slice()).unwrap();

    assert_eq!(
        decoded_payload.tag().unwrap(),
        Some(Nbt::String("dialog".to_string()))
    );
    assert!(decoded_payload.bytes().is_some());
}
