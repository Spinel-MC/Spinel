use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::move_player_pos::MovePlayerPosPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_move_player_pos(
    client: &mut Client,
    packet: MovePlayerPosPacket,
    server: &mut MinecraftServer,
) -> bool {
    server
        .move_player_in_world(client, packet.x, packet.y, packet.z, packet.on_ground())
        .is_ok()
}
