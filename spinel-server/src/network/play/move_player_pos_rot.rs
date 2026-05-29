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
    server
        .move_player_with_view_in_world(
            client,
            packet.x,
            packet.y,
            packet.z,
            packet.y_rot,
            packet.x_rot,
            packet.on_ground(),
        )
        .is_ok()
}
