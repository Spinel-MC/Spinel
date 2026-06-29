use crate::events::player_settings_change::PlayerSettingsChangeEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::client_information::ClientInformationPacket;
use spinel_macros::packet_listener;

#[packet_listener()]
fn on_client_information(
    client: &mut Client,
    packet: ClientInformationPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        client.pending_client_settings = packet.settings;
        return true;
    };

    if server
        .refresh_player_settings_in_world(client, packet.settings)
        .is_err()
    {
        return false;
    }
    PlayerSettingsChangeEvent::new(player).dispatch(server, client);
    true
}
