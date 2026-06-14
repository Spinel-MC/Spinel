use super::super::player_command::PlayerCommandPacket;
use spinel_network::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use std::io::{Cursor, Read};

#[test]
fn player_command_packet_decodes_minestom_client_entity_action_shape() {
    let mut payload = Vec::new();
    VarIntWrapper(7).encode(&mut payload).unwrap();
    VarIntWrapper(PlayerCommandPacket::START_SPRINTING)
        .encode(&mut payload)
        .unwrap();
    VarIntWrapper(0).encode(&mut payload).unwrap();

    let mut reader = Cursor::new(payload);
    let packet = PlayerCommandPacket::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(PlayerCommandPacket::get_id(), 0x29);
    assert_eq!(packet.entity_id, 7);
    assert_eq!(packet.action, PlayerCommandPacket::START_SPRINTING);
    assert_eq!(packet.data, 0);
    assert!(remaining.is_empty());
}
