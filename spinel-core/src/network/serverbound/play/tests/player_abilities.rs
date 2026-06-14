use super::super::player_abilities::ServerboundPlayerAbilitiesPacket;
use spinel_network::DataType;
use std::io::{Cursor, Read};

#[test]
fn serverbound_player_abilities_packet_decodes_minestom_flying_flag() {
    let mut reader = Cursor::new(vec![0x02]);

    let packet = ServerboundPlayerAbilitiesPacket::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(ServerboundPlayerAbilitiesPacket::get_id(), 0x27);
    assert!(packet.is_flying());
    assert!(remaining.is_empty());
}
