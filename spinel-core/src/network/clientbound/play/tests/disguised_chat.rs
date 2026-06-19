use super::super::disguised_chat::DisguisedChatPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::ChatType;
use spinel_utils::component::text::TextComponent;

#[test]
fn disguised_chat_packet_roundtrips() {
    let packet = DisguisedChatPacket {
        message: TextComponent::literal("Masked"),
        chat_type: ChatType {
            encoded_bound_chat_type_payload: vec![1, 8, 0, 6, 83, 101, 114, 118, 101, 114, 0],
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = DisguisedChatPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
