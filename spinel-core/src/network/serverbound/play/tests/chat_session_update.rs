use super::super::chat_session_update::ChatSessionUpdatePacket;
use spinel_network::data_type::DataType;
use spinel_network::types::{ProfilePublicKeyData, RemoteChatSessionData};
use uuid::Uuid;

#[test]
fn chat_session_update_packet_roundtrips() {
    let packet = ChatSessionUpdatePacket {
        chat_session: RemoteChatSessionData {
            session_id: Uuid::from_u128(0x00112233445566778899aabbccddeeff),
            profile_public_key: ProfilePublicKeyData {
                expires_at_millis: 1_718_000_000_123,
                encoded_public_key: vec![1, 2, 3],
                key_signature: vec![4, 5, 6, 7],
            },
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = ChatSessionUpdatePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.chat_session, packet.chat_session);
}
