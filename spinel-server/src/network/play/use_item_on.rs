use crate::entity::PlayerHand;
use crate::events::player_block_interact::{BlockFace, PlayerBlockInteractEvent};
use crate::events::player_block_place::PlayerBlockPlaceEvent;
use crate::events::player_use_item_on_block::PlayerUseItemOnBlockEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{Block, BlockHandlerPlacement, BlockPosition, BlockReplacement, BlockState};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::acknowledge_block_change::AcknowledgeBlockChangePacket;
use spinel_core::network::serverbound::play::use_item_on::UseItemOnPacket;
use spinel_macros::packet_listener;
use spinel_registry::data_components::vanilla_components::{BLOCK_STATE, CAN_PLACE_ON};
use spinel_registry::{BlockPredicates, DataComponentMap, ItemBlockState};

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
    if !event.is_cancelled() {
        let player = unsafe { &mut *event_input.player };
        let handler_allows_item_use = server
            .interact_block_handler_in_world(
                client,
                player.get_entity_id(),
                event_input.hand,
                event_input.block_position,
                event_input.block_face,
                event_input.cursor_position,
            )
            .unwrap_or(true);
        if !handler_allows_item_use {
            event.set_blocking_item_use(true);
        }
    }
    if event.is_blocking_item_use() {
        return acknowledge_block_change(packet.sequence, client);
    }
    let player = unsafe { &mut *event_input.player };
    let item_stack = player.get_item_in_hand(event_input.hand);
    if let Some(block) = item_stack.material().block() {
        let block_state = item_stack.get_or(BLOCK_STATE, ItemBlockState::default());
        let default_block_state = DataComponentMap::default().get_or(
            &item_stack.material().prototype(),
            BLOCK_STATE,
            ItemBlockState::default(),
        );
        return place_block(
            event_input.player,
            event_input.block_position,
            event_input.block_face,
            block_state.apply(block),
            block_state == default_block_state,
            event_input.cursor_position,
            event_input.hand,
            packet.sequence,
            server,
            client,
        );
    }
    let mut item_on_block_event = PlayerUseItemOnBlockEvent::new(
        event_input.player,
        event_input.hand,
        item_stack,
        event_input.block_position,
        event_input.cursor_position,
        event_input.block_face,
    );
    item_on_block_event.dispatch(server, client);
    AcknowledgeBlockChangePacket {
        sequence: packet.sequence,
    }
    .dispatch(client)
    .is_ok()
}

fn place_block(
    player: *mut crate::entity::Player,
    interacted_position: BlockPosition,
    block_face: BlockFace,
    block_state: BlockState,
    should_do_block_updates: bool,
    cursor_position: (f32, f32, f32),
    hand: PlayerHand,
    sequence: i32,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let player = unsafe { &mut *player };
    let mut can_place_block = player.get_game_mode() != GameMode::Spectator;
    if player.get_game_mode() == GameMode::Adventure {
        can_place_block = player
            .get_item_in_hand(hand)
            .get_or(CAN_PLACE_ON, BlockPredicates::default())
            .test_state_with_nbt(
                server
                    .loaded_block_state_in_world(client, interacted_position)
                    .unwrap_or_else(|| Block::AIR.default_state()),
                server
                    .client_block_entity_nbt_in_world(client, interacted_position)
                    .as_ref(),
                &server.registries,
            );
    }
    let placement_position = placement_position(
        interacted_position,
        block_face,
        cursor_position,
        player.get_item_in_hand(hand).material().clone(),
        server,
        client,
    );
    let Some(placement_position) = placement_position else {
        return false;
    };
    if !block_position_is_buildable(placement_position, server, client) {
        return true;
    }
    if !server.block_position_is_inside_world_border(client, placement_position) {
        can_place_block = false;
    }
    if !can_place_block {
        return server
            .refresh_block_in_world(client, placement_position)
            .is_ok();
    }
    if server.chunk_is_read_only_in_world(client, placement_position) {
        return refresh_inventory_and_chunk(player, placement_position, server, client);
    }
    if let Some(collision_entity) =
        server.block_placement_collision_entity(client, placement_position)
    {
        if collision_entity != player.get_entity_id() {
            return refresh_inventory_and_chunk(player, placement_position, server, client);
        }
        return true;
    }
    let Some(existing_block) = server.loaded_block_in_world(client, placement_position) else {
        return false;
    };
    let mut event = PlayerBlockPlaceEvent::new(
        player as *mut crate::entity::Player,
        block_state.block(),
        block_face,
        placement_position,
        cursor_position,
        hand,
    );
    event.consume_block(player.get_game_mode() != GameMode::Creative);
    event.set_do_block_updates(should_do_block_updates);
    event.dispatch(server, client);
    if event.is_cancelled() {
        return refresh_inventory_and_chunk(player, placement_position, server, client);
    }
    let Some(world_uuid) = server.world_uuid_for_client(client) else {
        return false;
    };
    let event_block_state = if event.block() == block_state.block() {
        block_state
    } else {
        event.block().default_state()
    };
    let placement = BlockHandlerPlacement::new(
        event_block_state,
        existing_block,
        world_uuid,
        placement_position,
    )
    .player_placement(player.get_entity_id(), hand, block_face, cursor_position);
    let block_was_set = server
        .place_block_in_world(client, placement, event.should_do_block_updates())
        .unwrap_or(false);
    if !block_was_set {
        return refresh_inventory_and_chunk(player, placement_position, server, client);
    }
    if !synchronize_placed_block_inventory(player, hand, event.does_consume_block(), client) {
        return false;
    }
    let block_change_is_acknowledged = AcknowledgeBlockChangePacket { sequence }
        .dispatch(client)
        .is_ok();
    block_change_is_acknowledged
}

pub(crate) fn synchronize_placed_block_inventory(
    player: &mut crate::entity::Player,
    hand: PlayerHand,
    does_consume_block: bool,
    client: &mut Client,
) -> bool {
    if does_consume_block {
        let consumed_item = player.get_item_in_hand(hand).consume(1);
        return player.set_item_in_hand(hand, consumed_item);
    }
    player.sync_player_inventory_window_contents(client).is_ok()
}
fn placement_position(
    position: BlockPosition,
    block_face: BlockFace,
    cursor_position: (f32, f32, f32),
    material: spinel_registry::Material,
    server: &MinecraftServer,
    client: &Client,
) -> Option<BlockPosition> {
    let interacted_block = server.loaded_block_in_world(client, position)?;
    let interacted_block_is_self_replaceable = server.block_is_self_replaceable_in_world(
        client,
        BlockReplacement::new(
            interacted_block,
            block_face,
            cursor_position,
            false,
            material.clone(),
        ),
    );
    if interacted_block.is_air() || interacted_block_is_self_replaceable {
        return Some(position);
    }
    let placement_position = match block_face {
        BlockFace::Bottom => BlockPosition::new(position.x, position.y - 1, position.z),
        BlockFace::Top => BlockPosition::new(position.x, position.y + 1, position.z),
        BlockFace::North => BlockPosition::new(position.x, position.y, position.z - 1),
        BlockFace::South => BlockPosition::new(position.x, position.y, position.z + 1),
        BlockFace::West => BlockPosition::new(position.x - 1, position.y, position.z),
        BlockFace::East => BlockPosition::new(position.x + 1, position.y, position.z),
    };
    let placement_block = server.loaded_block_in_world(client, placement_position)?;
    let placement_block_is_self_replaceable = server.block_is_self_replaceable_in_world(
        client,
        BlockReplacement::new(placement_block, block_face, cursor_position, true, material),
    );
    if placement_block.is_replaceable() || placement_block_is_self_replaceable {
        Some(placement_position)
    } else {
        None
    }
}

fn block_position_is_buildable(
    position: BlockPosition,
    server: &MinecraftServer,
    client: &Client,
) -> bool {
    let Some(world_uuid) = server.world_uuid_for_client(client) else {
        return false;
    };
    let Some(world) = server.world_manager.world(world_uuid) else {
        return false;
    };
    let dimension = world.cached_dimension_type();
    position.y >= dimension.min_y && position.y < dimension.min_y + dimension.height
}

fn refresh_inventory_and_chunk(
    player: &mut crate::entity::Player,
    position: BlockPosition,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let inventory_is_refreshed = player.sync_player_inventory_window_contents(client).is_ok();
    let chunk_is_refreshed = server.refresh_chunk_in_world(client, position);
    inventory_is_refreshed && chunk_is_refreshed
}

fn acknowledge_block_change(sequence: i32, client: &mut Client) -> bool {
    AcknowledgeBlockChangePacket { sequence }
        .dispatch(client)
        .is_ok()
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
