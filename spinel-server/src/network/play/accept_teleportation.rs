use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::accept_teleportation::AcceptTeleportationPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_accept_teleportation(
    _client: &mut Client,
    _packet: AcceptTeleportationPacket,
    _server: &mut MinecraftServer,
) -> bool {
    true
}
