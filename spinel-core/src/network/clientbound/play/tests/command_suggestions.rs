use super::super::command_suggestions::{CommandSuggestionMatch, CommandSuggestionsPacket};
use spinel_network::{DataType, PacketStruct};
use spinel_utils::component::text::TextComponent;

#[test]
fn command_suggestions_round_trips_minestom_shape() {
    let packet = CommandSuggestionsPacket {
        transaction_id: 4,
        start: 1,
        length: 5,
        matches: vec![CommandSuggestionMatch::new(
            "spawn",
            Some(TextComponent::literal("Spawn command")),
        )],
    };

    let mut payload = Vec::new();
    packet.encode(&mut payload).unwrap();
    let decoded_packet = CommandSuggestionsPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(CommandSuggestionsPacket::get_id(), 0x0f);
    assert_eq!(decoded_packet, packet);
}
