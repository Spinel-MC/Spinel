use super::super::use_item::UseItemPacket;
use spinel_network::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use std::io::{Cursor, Read};

#[test]
fn use_item_decodes_minestom_shape() {
    let mut payload = Vec::new();
    VarIntWrapper(0).encode(&mut payload).unwrap();
    VarIntWrapper(300).encode(&mut payload).unwrap();
    90.0f32.encode(&mut payload).unwrap();
    45.0f32.encode(&mut payload).unwrap();

    let mut reader = Cursor::new(payload);
    let packet = UseItemPacket::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(packet.hand, 0);
    assert_eq!(packet.sequence, 300);
    assert_eq!(packet.y_rot, 90.0);
    assert_eq!(packet.x_rot, 45.0);
    assert!(remaining.is_empty());
}
