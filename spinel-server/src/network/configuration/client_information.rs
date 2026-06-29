use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::configuration::client_information::ClientInformationPacket;
use spinel_macros::packet_listener;

#[packet_listener()]
fn on_client_information(
    client: &mut Client,
    packet: ClientInformationPacket,
    _server: &mut MinecraftServer,
) -> bool {
    client.pending_client_settings = packet.settings;
    true
}
