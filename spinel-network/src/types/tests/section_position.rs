use crate::data_type::DataType;
use crate::types::SectionPosition;

#[test]
fn section_position_roundtrips() {
    let position = SectionPosition {
        x: -10,
        y: 24,
        z: 300,
    };
    let mut payload = Vec::new();

    position.encode(&mut payload).unwrap();
    let decoded_position = SectionPosition::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_position, position);
}
