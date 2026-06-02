use crate::entity::EntityId;
use crate::events::player_pick_block::PlayerPickBlockEvent;
use crate::events::player_pick_entity::PlayerPickEntityEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::BlockPosition;
use spinel_core::network::serverbound::play::pick_item_from_block::PickItemFromBlockPacket;
use spinel_core::network::serverbound::play::pick_item_from_entity::PickItemFromEntityPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_pick_item_from_block(
    client: &mut Client,
    packet: PickItemFromBlockPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let mut event = PlayerPickBlockEvent::new(
        player,
        BlockPosition::new(packet.position.x, packet.position.y, packet.position.z),
        packet.include_data,
    );
    event.dispatch(server, client);
    !event.is_cancelled()
}

#[packet_listener]
fn on_pick_item_from_entity(
    client: &mut Client,
    packet: PickItemFromEntityPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let target = EntityId::from_raw(packet.entity_id);
    if !server
        .world_manager
        .client_world_contains_entity(client, target)
    {
        return false;
    }
    let mut event = PlayerPickEntityEvent::new(player, target, packet.include_data);
    event.dispatch(server, client);
    !event.is_cancelled()
}
