use crate::events::player_spectate::PlayerSpectateEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::serverbound::play::teleport_to_entity::TeleportToEntityPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_teleport_to_entity(
    client: &mut Client,
    packet: TeleportToEntityPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return true;
    };
    if unsafe { &*player }.get_game_mode() != GameMode::Spectator {
        return true;
    }
    let Some(world_uuid) = server.world_manager.world_uuid_for_client(client) else {
        return true;
    };
    let Some(target_id) = server
        .world_manager
        .world(world_uuid)
        .and_then(|world| world.entity_by_uuid(packet.target))
        .map(|entity| entity.get_entity_id())
    else {
        return true;
    };
    if unsafe { &*player }.get_entity_id() == target_id {
        return true;
    }
    let mut event = PlayerSpectateEvent::new(player, target_id);
    event.dispatch(server, client);
    if event.is_cancelled() {
        return true;
    }
    unsafe { &mut *player }.spectate(target_id).is_ok()
}
