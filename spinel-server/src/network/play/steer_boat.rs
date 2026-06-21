use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::steer_boat::SteerBoatPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_steer_boat(
    client: &mut Client,
    packet: SteerBoatPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let Some(vehicle_id) = (unsafe { &mut *player }).get_vehicle() else {
        return true;
    };
    server.steer_client_boat_in_world(
        client,
        vehicle_id,
        packet.left_paddle_turning,
        packet.right_paddle_turning,
    );
    true
}
