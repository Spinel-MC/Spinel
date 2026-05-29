use crate::entity::PlayerHand;
use crate::events::inventory_open::InventoryOpenEvent;
use crate::events::player_block_interact::{BlockFace, PlayerBlockInteractEvent};
use crate::events::player_block_place::PlayerBlockPlaceEvent;
use crate::events::player_use_item_on_block::PlayerUseItemOnBlockEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{Block, BlockPosition};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::acknowledge_block_change::AcknowledgeBlockChangePacket;
use spinel_core::network::serverbound::play::use_item_on::UseItemOnPacket;
use spinel_macros::packet_listener;

const MIN_BUILD_HEIGHT: i32 = -64;
const MAX_BUILD_HEIGHT: i32 = 319;

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
    let item_stack = player.item_in_hand(event_input.hand);
    let mut item_on_block_event = PlayerUseItemOnBlockEvent::new(
        event_input.player,
        event_input.hand,
        item_stack.clone(),
        event_input.block_position,
        event_input.cursor_position,
        event_input.block_face,
    );
    item_on_block_event.dispatch(server, client);
    if let Some(block) = item_stack.material().block() {
        return place_block(
            event_input.player,
            event_input.block_position,
            event_input.block_face,
            block,
            event_input.cursor_position,
            event_input.hand,
            packet.sequence,
            server,
            client,
        );
    }
    dispatch_open_inventory_event(player, server, client)
        && player.sync_open_inventory(client).is_ok()
}

fn place_block(
    player: *mut crate::entity::Player,
    interacted_position: BlockPosition,
    block_face: BlockFace,
    block: Block,
    cursor_position: (f32, f32, f32),
    hand: PlayerHand,
    sequence: i32,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let player = unsafe { &mut *player };
    if player.game_mode() == GameMode::Spectator || player.game_mode() == GameMode::Adventure {
        return rollback_block_change(interacted_position, sequence, server, client);
    }
    let placement_position = placement_position(interacted_position, block_face, server, client);
    if !block_position_is_buildable(placement_position) {
        return rollback_block_change(interacted_position, sequence, server, client);
    }
    let Some(existing_block) = server.loaded_block_in_world(client, placement_position) else {
        return rollback_block_change(interacted_position, sequence, server, client);
    };
    if existing_block != Block::AIR {
        return rollback_block_change(placement_position, sequence, server, client);
    }
    let mut event = PlayerBlockPlaceEvent::new(
        player as *mut crate::entity::Player,
        block,
        block_face,
        placement_position,
        cursor_position,
        hand,
    );
    event.dispatch(server, client);
    if event.is_cancelled() {
        return rollback_block_change(placement_position, sequence, server, client);
    }
    let block_was_set = server
        .set_block_in_world(client, placement_position, event.block())
        .unwrap_or(false);
    if block_was_set && event.does_consume_block() && player.game_mode() != GameMode::Creative {
        let consumed_item = player.item_in_hand(hand).consume(1);
        player.set_item_in_hand(hand, consumed_item);
        let _ = player.sync_inventory(client);
    }
    let block_change_is_acknowledged = AcknowledgeBlockChangePacket { sequence }
        .dispatch(client)
        .is_ok();
    block_was_set && block_change_is_acknowledged
}

fn placement_position(
    position: BlockPosition,
    block_face: BlockFace,
    server: &MinecraftServer,
    client: &Client,
) -> BlockPosition {
    if server
        .loaded_block_in_world(client, position)
        .is_some_and(|block| block == Block::AIR)
    {
        return position;
    }
    match block_face {
        BlockFace::Bottom => BlockPosition::new(position.x, position.y - 1, position.z),
        BlockFace::Top => BlockPosition::new(position.x, position.y + 1, position.z),
        BlockFace::North => BlockPosition::new(position.x, position.y, position.z - 1),
        BlockFace::South => BlockPosition::new(position.x, position.y, position.z + 1),
        BlockFace::West => BlockPosition::new(position.x - 1, position.y, position.z),
        BlockFace::East => BlockPosition::new(position.x + 1, position.y, position.z),
    }
}

fn block_position_is_buildable(position: BlockPosition) -> bool {
    position.y >= MIN_BUILD_HEIGHT && position.y <= MAX_BUILD_HEIGHT
}

fn rollback_block_change(
    position: BlockPosition,
    sequence: i32,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let block_is_refreshed = server.refresh_block_in_world(client, position).is_ok();
    let block_change_is_acknowledged = AcknowledgeBlockChangePacket { sequence }
        .dispatch(client)
        .is_ok();
    block_is_refreshed && block_change_is_acknowledged
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
    let block = server.loaded_block_in_world(client, block_position)?;
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
