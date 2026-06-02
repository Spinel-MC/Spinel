use crate::events::player_config_custom_click::PlayerConfigCustomClickEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::configuration::custom_click_action::ConfigurationCustomClickActionPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_configuration_custom_click_action(
    client: &mut Client,
    packet: ConfigurationCustomClickActionPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let payload = packet.payload_without_end_tag().map(ToOwned::to_owned);
    PlayerConfigCustomClickEvent::new(player, packet.key, payload).dispatch(server, client);
    true
}
