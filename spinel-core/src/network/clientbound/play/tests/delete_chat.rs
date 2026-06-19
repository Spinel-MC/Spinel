use super::super::delete_chat::DeleteChatPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::{MessageSignature, PackedMessageSignature};

#[test]
fn delete_chat_packet_roundtrips() {
    let packet = DeleteChatPacket {
        message_signature: PackedMessageSignature {
            signature_cache_id: -1,
            full_signature: Some(MessageSignature { bytes: [3; 256] }),
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = DeleteChatPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.message_signature, packet.message_signature);
}
