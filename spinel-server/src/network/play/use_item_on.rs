use crate::entity::PlayerHand;
use crate::events::inventory_open::InventoryOpenEvent;
use crate::events::player_block_interact::{BlockFace, PlayerBlockInteractEvent};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::BlockPosition;
use spinel_core::network::clientbound::play::acknowledge_block_change::AcknowledgeBlockChangePacket;
use spinel_core::network::serverbound::play::use_item_on::UseItemOnPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_use_item_on(
    client: &mut Client,
    packet: UseItemOnPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(event_input) = player_block_interact_event_input(client, &packet, server) else {
        return false;
    };
    let mut event = PlayerBlockInteractEvent::new(
        event_input.player,
        event_input.hand,
        event_input.block,
        event_input.block_position,
        event_input.cursor_position,
        event_input.block_face,
    );
    event.dispatch(server, client);
    if event.is_blocking_item_use() {
        return finish_blocked_item_use(event_input.player, packet.sequence, server, client);
    }
    let player = unsafe { &mut *event_input.player };
    dispatch_open_inventory_event(player, server, client)
        && player.sync_open_inventory(client).is_ok()
}

fn finish_blocked_item_use(
    player: *mut crate::entity::Player,
    sequence: i32,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let player = unsafe { &mut *player };
    let inventory_is_opened = dispatch_open_inventory_event(player, server, client)
        && player.sync_open_inventory(client).is_ok();
    let block_change_is_acknowledged = AcknowledgeBlockChangePacket { sequence }
        .dispatch(client)
        .is_ok();
    inventory_is_opened && block_change_is_acknowledged
}

fn player_block_interact_event_input(
    client: &Client,
    packet: &UseItemOnPacket,
    server: &mut MinecraftServer,
) -> Option<PlayerBlockInteractEventInput> {
    let hand = player_hand(packet.hand)?;
    let block_face = BlockFace::from_network_direction(packet.block_face)?;
    let block_position = BlockPosition::new(
        packet.block_position.x,
        packet.block_position.y,
        packet.block_position.z,
    );
    let block = server
        .world_manager
        .block_for_client(client, block_position)?;
    let player = server.world_manager.player_pointer_for_client(client)?;
    Some(PlayerBlockInteractEventInput {
        player,
        hand,
        block,
        block_position,
        cursor_position: (
            packet.cursor_position_x,
            packet.cursor_position_y,
            packet.cursor_position_z,
        ),
        block_face,
    })
}

fn player_hand(hand: i32) -> Option<PlayerHand> {
    match hand {
        0 => Some(PlayerHand::Main),
        1 => Some(PlayerHand::Off),
        _ => None,
    }
}

struct PlayerBlockInteractEventInput {
    player: *mut crate::entity::Player,
    hand: PlayerHand,
    block: crate::world::Block,
    block_position: BlockPosition,
    cursor_position: (f32, f32, f32),
    block_face: BlockFace,
}

fn dispatch_open_inventory_event(
    player: &mut crate::entity::Player,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let Some(inventory) = player.opened_inventory().cloned() else {
        return true;
    };
    let mut event = InventoryOpenEvent::new(player as *mut crate::entity::Player, inventory);
    event.dispatch(server, client);
    if !event.is_cancelled() {
        return true;
    }
    player.close_inventory_with_client(false, server, client)
}
