use super::super::chat_command_signed::{
    CommandArgumentSignature, SignedCommandArgumentSignature, SignedCommandChatPacket,
};
use spinel_network::DataType;

#[test]
fn signed_command_chat_packet_decodes_minestom_signature_and_ack_shape() {
    let packet = SignedCommandChatPacket {
        command: "spawn foo".to_owned(),
        timestamp: 42,
        salt: 99,
        signatures: vec![CommandArgumentSignature {
            argument_name: "foo".to_owned(),
            signature: SignedCommandArgumentSignature([8; 256]),
        }],
        ack_offset: 2,
        ack_list: [0, 1, 0],
        checksum: 5,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SignedCommandChatPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(SignedCommandChatPacket::get_id(), 0x07);
    assert_eq!(decoded_packet, packet);
}
