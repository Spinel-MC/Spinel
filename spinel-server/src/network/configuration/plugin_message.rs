use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::configuration::plugin_message::CustomPayloadPacket;
use spinel_macros::packet_listener;

#[packet_listener()]
fn on_custom_payload(
    _client: &mut Client,
    _packet: CustomPayloadPacket,
    _server: &mut MinecraftServer,
) -> bool {
    true
}
