use super::super::chat_command::ChatCommandPacket;
use spinel_network::DataType;

#[test]
fn chat_command_packet_matches_minestom_string_shape() {
    let packet = ChatCommandPacket {
        command: "test spawn_zombie".to_string(),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = ChatCommandPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(ChatCommandPacket::get_id(), 0x06);
    assert_eq!(decoded_packet.command, packet.command);
}
