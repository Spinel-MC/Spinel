use super::super::client_command::ClientCommandPacket;
use spinel_network::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use std::io::{Cursor, Read};

#[test]
fn client_command_packet_matches_vanilla_client_status_shape() {
    let packet = ClientCommandPacket {
        action: ClientCommandPacket::PERFORM_RESPAWN,
    };
    let mut encoded = Vec::new();
    packet.encode(&mut encoded).unwrap();

    let mut reader = Cursor::new(encoded);
    let action = VarIntWrapper::decode(&mut reader).unwrap().0;
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(ClientCommandPacket::get_id(), 0x0b);
    assert_eq!(action, 0);
    assert!(remaining.is_empty());
}
