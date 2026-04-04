use crate::instance::MinecraftServer;
use crate::network::client::instance::Client;
use spinel_core::network::serverbound::configuration::client_information::ClientInformationPacket;
use spinel_macros::packet_listener;

#[packet_listener(module: "login")]
fn on_client_information(
    _client: &mut Client,
    packet: ClientInformationPacket,
    _server: &mut MinecraftServer,
) -> bool {
    println!(
        "Received client info (locale: '{}').",
        packet.information.locale
    );

    true
}
