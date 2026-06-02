use crate::events::player_plugin_message::PlayerPluginMessageEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::plugin_message::ServerboundPlayCustomPayloadPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_plugin_message(
    client: &mut Client,
    packet: ServerboundPlayCustomPayloadPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    PlayerPluginMessageEvent::new(player, packet.channel, packet.data).dispatch(server, client);
    true
}
