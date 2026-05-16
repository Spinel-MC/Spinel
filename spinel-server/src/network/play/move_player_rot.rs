use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::move_player_rot::MovePlayerRotPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_move_player_rot(
    client: &mut Client,
    packet: MovePlayerRotPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_mut_for_client(client) else {
        return false;
    };

    player.look(packet.y_rot, packet.x_rot);
    true
}
