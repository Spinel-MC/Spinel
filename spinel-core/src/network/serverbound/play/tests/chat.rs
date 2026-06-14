use super::super::chat::{ChatPacket, ChatSignature};
use spinel_network::DataType;

#[test]
fn chat_packet_matches_minestom_signed_body_shape() {
    let packet = ChatPacket {
        message: "hello".to_owned(),
        timestamp: 42,
        salt: 99,
        signature: Some(ChatSignature([7; 256])),
        ack_offset: 3,
        ack_list: [1, 2, 3],
        checksum: -4,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = ChatPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(ChatPacket::get_id(), 0x08);
    assert_eq!(decoded_packet, packet);
}
