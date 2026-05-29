use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::accept_teleportation::AcceptTeleportationPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_accept_teleportation(
    client: &mut Client,
    packet: AcceptTeleportationPacket,
    server: &mut MinecraftServer,
) -> bool {
    if let Some(player) = server.world_manager.player_mut_for_client(client) {
        player.set_last_received_teleport_id(packet.id);
    }
    true
}
