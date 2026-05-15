use crate::server::MinecraftServer;
use crate::network::client::instance::Client;
use spinel_core::network::serverbound::configuration::plugin_message::CustomPayloadPacket;
use spinel_macros::packet_listener;

#[packet_listener(module: "login")]
fn on_custom_payload(
    _client: &mut Client,
    _packet: CustomPayloadPacket,
    _server: &mut MinecraftServer,
) -> bool {
    true
}
