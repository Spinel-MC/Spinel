use crate::data_type::DataType;
use crate::types::{ProfilePublicKeyData, RemoteChatSessionData};
use uuid::Uuid;

#[test]
fn remote_chat_session_data_roundtrips() {
    let chat_session = RemoteChatSessionData {
        session_id: Uuid::from_u128(0x00112233445566778899aabbccddeeff),
        profile_public_key: ProfilePublicKeyData {
            expires_at_millis: 1_718_000_000_123,
            encoded_public_key: vec![1, 2, 3, 4],
            key_signature: vec![5, 6, 7, 8, 9],
        },
    };
    let mut payload = Vec::new();

    chat_session.encode(&mut payload).unwrap();
    let decoded_chat_session = RemoteChatSessionData::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_chat_session, chat_session);
}

#[test]
fn profile_public_key_data_rejects_oversized_public_key() {
    let public_key = ProfilePublicKeyData {
        expires_at_millis: 0,
        encoded_public_key: vec![0; 513],
        key_signature: vec![],
    };

    let error = public_key.encode(&mut Vec::new()).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}
