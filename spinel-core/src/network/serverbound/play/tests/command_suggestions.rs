use super::super::command_suggestions::CommandSuggestionsRequestPacket;
use spinel_network::{DataType, PacketStruct};

#[test]
fn command_suggestions_request_round_trips_minestom_shape() {
    let packet = CommandSuggestionsRequestPacket {
        transaction_id: 6,
        text: "/spawn zo".to_string(),
    };

    let mut payload = Vec::new();
    packet.encode(&mut payload).unwrap();
    let decoded_packet = CommandSuggestionsRequestPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(CommandSuggestionsRequestPacket::get_id(), 0x0f);
    assert_eq!(decoded_packet, packet);
}
