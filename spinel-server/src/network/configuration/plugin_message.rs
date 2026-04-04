use crate::instance::MinecraftServer;
use crate::network::client::instance::Client;
use spinel_core::network::serverbound::configuration::plugin_message::PluginMessagePacket;
use spinel_macros::packet_listener;

#[packet_listener(module: "login")]
fn on_plugin_message(
    _client: &mut Client,
    _packet: PluginMessagePacket,
    _server: &mut MinecraftServer,
) -> bool {
    true
}
