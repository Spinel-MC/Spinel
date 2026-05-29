use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::player_input::PlayerInputPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_player_input(
    client: &mut Client,
    packet: PlayerInputPacket,
    server: &mut MinecraftServer,
) -> bool {
    server
        .refresh_player_input_in_world(
            client,
            packet.forward(),
            packet.backward(),
            packet.left(),
            packet.right(),
            packet.jump(),
            packet.shift(),
            packet.sprint(),
        )
        .is_ok()
}
