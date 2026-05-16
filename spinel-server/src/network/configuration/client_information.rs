use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::configuration::client_information::ClientInformationPacket;
use spinel_macros::packet_listener;

#[packet_listener(module: "login")]
fn on_client_information(
    _client: &mut Client,
    _packet: ClientInformationPacket,
    _server: &mut MinecraftServer,
) -> bool {
    true
}
