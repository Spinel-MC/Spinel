use super::super::player_chat::PlayerChatPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::{
    BitSet, ChatType, FilterMask, FilterMaskType, MessageSignature, PackedLastSeenMessages,
    PackedMessageSignature, SignedMessageBodyPacked,
};
use spinel_utils::component::text::TextComponent;
use uuid::Uuid;

#[test]
fn player_chat_packet_roundtrips() {
    let packet = PlayerChatPacket {
        global_index: 7,
        sender: Uuid::from_u128(0x00112233445566778899aabbccddeeff),
        index: 3,
        signature: Some(MessageSignature { bytes: [1; 256] }),
        body: SignedMessageBodyPacked {
            content: "hello".to_owned(),
            timestamp_millis: 1_718_000_000_123,
            salt: 44,
            last_seen: PackedLastSeenMessages {
                entries: vec![PackedMessageSignature {
                    signature_cache_id: 5,
                    full_signature: None,
                }],
            },
        },
        unsigned_content: Some(TextComponent::literal("hello")),
        filter_mask: FilterMask {
            filter_mask_type: FilterMaskType::PartiallyFiltered,
            mask: BitSet(vec![0b0000_0001, 0, 0, 0, 0, 0, 0, 0]),
        },
        chat_type: ChatType {
            encoded_bound_chat_type_payload: vec![1, 8, 0, 6, 80, 108, 97, 121, 101, 114, 0],
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerChatPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
