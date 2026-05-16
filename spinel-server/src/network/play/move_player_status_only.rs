use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::move_player_status_only::MovePlayerStatusOnlyPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_move_player_status_only(
    _client: &mut Client,
    _packet: MovePlayerStatusOnlyPacket,
    _server: &mut MinecraftServer,
) -> bool {
    true
}
