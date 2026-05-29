use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::move_player_status_only::MovePlayerStatusOnlyPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_move_player_status_only(
    client: &mut Client,
    packet: MovePlayerStatusOnlyPacket,
    server: &mut MinecraftServer,
) -> bool {
    server
        .refresh_player_status_in_world(client, packet.on_ground())
        .is_ok()
}
