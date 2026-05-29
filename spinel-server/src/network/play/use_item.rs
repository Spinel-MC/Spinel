use crate::entity::PlayerHand;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::acknowledge_block_change::AcknowledgeBlockChangePacket;
use spinel_core::network::serverbound::play::use_item::UseItemPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_use_item(client: &mut Client, packet: UseItemPacket, server: &mut MinecraftServer) -> bool {
    let Some(hand) = PlayerHand::from_protocol_id(packet.hand) else {
        return false;
    };
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let on_ground = unsafe { &*player }.is_on_ground();
    if server
        .look_player_in_world(client, packet.y_rot, packet.x_rot, on_ground)
        .is_err()
    {
        return false;
    }
    let block_change_is_acknowledged = AcknowledgeBlockChangePacket {
        sequence: packet.sequence,
    }
    .dispatch(client)
    .is_ok();
    let item_use_is_allowed = unsafe { &mut *player }
        .use_item(hand, server.current_tick, server, client)
        .unwrap_or(false);
    let metadata_is_refreshed = server.refresh_player_metadata_in_world(client).is_ok();
    let equipment_is_refreshed = server
        .refresh_player_visible_equipment_in_world(client)
        .is_ok();
    block_change_is_acknowledged
        && item_use_is_allowed
        && metadata_is_refreshed
        && equipment_is_refreshed
}
