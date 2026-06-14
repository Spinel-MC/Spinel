use super::super::player_loaded::PlayerLoadedPacket;
use spinel_network::DataType;
use std::io::{Cursor, Read};

#[test]
fn player_loaded_packet_decodes_minestom_empty_shape() {
    let mut reader = Cursor::new(Vec::new());
    let _packet = PlayerLoadedPacket::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(PlayerLoadedPacket::get_id(), 0x2b);
    assert!(remaining.is_empty());
}
