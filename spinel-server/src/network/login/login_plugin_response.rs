use crate::network::client::instance::Client;
use crate::network::login::login_start::resume_login_after_plugin_responses;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::login::login_plugin_response::LoginPluginResponsePacket;
use spinel_macros::packet_listener;

#[packet_listener()]
fn on_login_plugin_response(
    client: &mut Client,
    packet: LoginPluginResponsePacket,
    _server: &mut MinecraftServer,
) -> bool {
    if client
        .complete_login_plugin_response(packet.message_id, packet.data)
        .is_err()
    {
        return false;
    }
    resume_login_after_plugin_responses(client)
}
