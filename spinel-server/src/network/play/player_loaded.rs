use crate::events::player_loaded::PlayerLoadedEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::player_loaded::PlayerLoadedPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_player_loaded(
    client: &mut Client,
    _packet: PlayerLoadedPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    PlayerLoadedEvent::new(player).dispatch(server, client);
    true
}
