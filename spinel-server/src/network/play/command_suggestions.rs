use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::command_suggestions::{
    CommandSuggestionMatch, CommandSuggestionsPacket,
};
use spinel_core::network::serverbound::play::command_suggestions::CommandSuggestionsRequestPacket;
use spinel_macros::packet_listener;
use spinel_network::{DataType, PacketSender, PacketStruct};
use spinel_utils::component::text::TextComponent;

#[packet_listener]
fn on_command_suggestions(
    client: &mut Client,
    packet: CommandSuggestionsRequestPacket,
    server: &mut MinecraftServer,
) -> bool {
    let sender_kind = crate::command::CommandSender::Player(client).kind();
    let suggestion = server.command_manager.suggest(sender_kind, &packet.text);
    let response = CommandSuggestionsPacket {
        transaction_id: packet.transaction_id,
        start: suggestion.start() as i32,
        length: suggestion.length() as i32,
        matches: suggestion
            .entries()
            .iter()
            .map(|entry| {
                CommandSuggestionMatch::new(
                    entry.entry(),
                    entry.tooltip().map(TextComponent::literal),
                )
            })
            .collect(),
    };
    let mut payload = Vec::new();
    response
        .encode(&mut payload)
        .and_then(|_| client.send_packet(CommandSuggestionsPacket::get_id(), &payload))
        .is_ok()
}
