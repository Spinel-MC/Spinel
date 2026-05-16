use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::client_tick_end::ClientTickEndPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_client_tick_end(
    client: &mut Client,
    _packet: ClientTickEndPacket,
    server: &mut MinecraftServer,
) -> bool {
    if let Some(player) = server.world_manager.player_mut_for_client(client) {
        player.finish_client_tick(server.current_tick);
    }
    true
}
