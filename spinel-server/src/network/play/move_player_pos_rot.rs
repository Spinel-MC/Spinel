use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::move_player_pos_rot::MovePlayerPosRotPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_move_player_pos_rot(
    client: &mut Client,
    packet: MovePlayerPosRotPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_mut_for_client(client) else {
        return false;
    };

    player.look(packet.y_rot, packet.x_rot);
    server
        .world_manager
        .move_player(client, packet.x, packet.y, packet.z)
        .is_ok()
}
