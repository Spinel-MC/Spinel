use super::super::player_input::PlayerInputPacket;
use spinel_network::DataType;
use std::io::{Cursor, Read};

#[test]
fn player_input_packet_decodes_minestom_client_input_flags() {
    let mut reader = Cursor::new(vec![0x61]);

    let packet = PlayerInputPacket::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(PlayerInputPacket::get_id(), 0x2a);
    assert!(packet.forward());
    assert!(!packet.backward());
    assert!(packet.shift());
    assert!(packet.sprint());
    assert!(remaining.is_empty());
}
