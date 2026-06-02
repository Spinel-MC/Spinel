use crate::entity::EntityPosition;
use crate::events::player_vehicle_move::PlayerVehicleMoveEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::move_vehicle::ServerboundVehicleMovePacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_move_vehicle(
    client: &mut Client,
    packet: ServerboundVehicleMovePacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let Some(vehicle) = (unsafe { &mut *player }).vehicle() else {
        return true;
    };
    let position = EntityPosition::new(packet.x, packet.y, packet.z, packet.yaw, packet.pitch);
    let mut event = PlayerVehicleMoveEvent::new(player, vehicle, position, packet.on_ground);
    event.dispatch(server, client);
    if event.is_cancelled() {
        return false;
    }
    server
        .world_manager
        .move_client_world_entity(client, vehicle, event.new_position())
}
