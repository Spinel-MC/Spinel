use crate::events::player_game_mode_request::PlayerGameModeRequestEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::change_game_mode::ChangeGameModePacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_change_game_mode(
    client: &mut Client,
    packet: ChangeGameModePacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    PlayerGameModeRequestEvent::new(player, packet.game_mode).dispatch(server, client);
    true
}
