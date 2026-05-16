use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::player_loaded::PlayerLoadedPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_player_loaded(
    _client: &mut Client,
    _packet: PlayerLoadedPacket,
    _server: &mut MinecraftServer,
) -> bool {
    true
}
