use crate::events::player_block_interact::BlockFace;
use crate::events::player_cancel_digging::PlayerCancelDiggingEvent;
use crate::events::player_finish_digging::PlayerFinishDiggingEvent;
use crate::events::player_stab::PlayerStabEvent;
use crate::events::player_start_digging::PlayerStartDiggingEvent;
use crate::network::client::instance::Client;
use crate::network::play::block_break_calculation;
use crate::server::MinecraftServer;
use crate::world::{Block, BlockPosition, BlockState};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::acknowledge_block_change::AcknowledgeBlockChangePacket;
use spinel_core::network::serverbound::play::player_action::PlayerActionPacket;
use spinel_macros::packet_listener;
use spinel_nbt::NbtCompound;
use spinel_network::types::TeleportFlags;
use spinel_registry::data_components::vanilla_components::{CAN_BREAK, PIERCING_WEAPON, TOOL};
use spinel_registry::{BlockPredicates, Registries, Tool};

#[packet_listener]
fn on_player_action(
    client: &mut Client,
    packet: PlayerActionPacket,
    server: &mut MinecraftServer,
) -> bool {
    if packet.status == PlayerActionPacket::START_DESTROY_BLOCK
        || packet.status == PlayerActionPacket::ABORT_DESTROY_BLOCK
    {
        return acknowledge_digging(packet, client, server);
    }
    if packet.status == PlayerActionPacket::STOP_DESTROY_BLOCK {
        return finish_digging(packet, server, client);
    }
    if packet.status == PlayerActionPacket::RELEASE_USE_ITEM {
        return update_item_state(server, client);
    }
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let player = unsafe { &mut *player };
    if packet.status == PlayerActionPacket::STAB {
        if player
            .item_in_hand(crate::entity::PlayerHand::Main)
            .has(PIERCING_WEAPON)
        {
            PlayerStabEvent::new(player).dispatch(server, client);
        }
        return true;
    }
    let action_result = match packet.status {
        PlayerActionPacket::DROP_ITEM_STACK => player.drop_main_hand_item(true, server, client),
        PlayerActionPacket::DROP_ITEM => player.drop_main_hand_item(false, server, client),
        PlayerActionPacket::SWAP_ITEM_HAND => player.swap_item_hands(server, client),
        _ => true,
    };
    if !action_result {
        return false;
    }
    server
        .refresh_player_visible_equipment_in_world(client)
        .is_ok()
}

fn update_item_state(server: &mut MinecraftServer, client: &mut Client) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    if !unsafe { &mut *player }.cancel_item_use(server, client) {
        return false;
    }
    server.refresh_player_metadata_in_world(client).is_ok()
}

fn acknowledge_digging(
    packet: PlayerActionPacket,
    client: &mut Client,
    server: &mut MinecraftServer,
) -> bool {
    let Some(digging_input) = digging_event_input(&packet, client, server) else {
        return true;
    };
    match packet.status {
        PlayerActionPacket::START_DESTROY_BLOCK => {
            start_digging(digging_input, packet.sequence, client, server)
        }
        PlayerActionPacket::ABORT_DESTROY_BLOCK => {
            cancel_digging(digging_input, packet.sequence, client, server)
        }
        _ => acknowledge_block_change(packet.sequence, client),
    }
}

fn finish_digging(
    packet: PlayerActionPacket,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let Some(digging_input) = digging_event_input(&packet, client, server) else {
        return true;
    };
    let player = unsafe { &mut *digging_input.player };
    if should_prevent_breaking(
        player,
        digging_input.block_state,
        digging_input.client_block_entity_nbt.as_ref(),
        &server.registries,
    ) {
        return fail_digging(
            digging_input.block_position,
            packet.sequence,
            server,
            client,
        );
    }
    let player_is_in_water = player_is_in_water(player, server, client);
    let break_ticks = block_break_calculation::break_ticks(
        digging_input.block,
        player,
        &server.registries,
        player_is_in_water,
    );
    if break_ticks == block_break_calculation::UNBREAKABLE {
        let mut event = PlayerCancelDiggingEvent::new(
            digging_input.player,
            digging_input.block,
            digging_input.block_position,
        );
        event.dispatch(server, client);
        return fail_digging(
            digging_input.block_position,
            packet.sequence,
            server,
            client,
        );
    }
    let mut event = PlayerFinishDiggingEvent::new(
        digging_input.player,
        digging_input.block,
        digging_input.block_position,
    );
    event.dispatch(server, client);
    let previous_block = event.block();
    let block_was_broken = attempt_block_break(
        digging_input.player,
        previous_block,
        digging_input.block_position,
        digging_input.block_face,
        server,
        client,
    );
    let block_change_is_acknowledged = acknowledge_block_change(packet.sequence, client);
    if block_was_broken {
        return block_change_is_acknowledged;
    }
    block_change_is_acknowledged
        && refresh_failed_digging_block_entity(digging_input.block_position, server, client)
}

fn start_digging(
    digging_input: DiggingEventInput,
    sequence: i32,
    client: &mut Client,
    server: &mut MinecraftServer,
) -> bool {
    let player = unsafe { &mut *digging_input.player };
    if should_prevent_breaking(
        player,
        digging_input.block_state,
        digging_input.client_block_entity_nbt.as_ref(),
        &server.registries,
    ) {
        return fail_digging(digging_input.block_position, sequence, server, client);
    }
    let player_is_in_water = player_is_in_water(player, server, client);
    let break_ticks = block_break_calculation::break_ticks(
        digging_input.block,
        player,
        &server.registries,
        player_is_in_water,
    );
    if break_ticks == 0 {
        let block_was_broken = attempt_block_break(
            digging_input.player,
            digging_input.block,
            digging_input.block_position,
            digging_input.block_face,
            server,
            client,
        );
        let block_change_is_acknowledged = acknowledge_block_change(sequence, client);
        if block_was_broken {
            return block_change_is_acknowledged;
        }
        return block_change_is_acknowledged
            && refresh_failed_digging_block_entity(digging_input.block_position, server, client);
    }
    let mut event = PlayerStartDiggingEvent::new(
        digging_input.player,
        digging_input.block,
        digging_input.block_position,
        digging_input.block_face,
    );
    event.dispatch(server, client);
    if event.is_cancelled() {
        return fail_digging(digging_input.block_position, sequence, server, client);
    }
    acknowledge_block_change(sequence, client)
}

fn cancel_digging(
    digging_input: DiggingEventInput,
    sequence: i32,
    client: &mut Client,
    server: &mut MinecraftServer,
) -> bool {
    let mut event = PlayerCancelDiggingEvent::new(
        digging_input.player,
        digging_input.block,
        digging_input.block_position,
    );
    event.dispatch(server, client);
    acknowledge_block_change(sequence, client)
}

fn attempt_block_break(
    player: *mut crate::entity::Player,
    previous_block: Block,
    block_position: BlockPosition,
    block_face: BlockFace,
    server: &mut MinecraftServer,
    client: &Client,
) -> bool {
    let player_id = unsafe { &*player }.entity_id();
    let block_was_broken = server
        .break_block_in_world(client, player_id, block_position, block_face, true)
        .unwrap_or(false);
    if block_was_broken {
        return true;
    }
    correct_player_after_failed_digging(player, previous_block, block_position, server, client);
    false
}

fn correct_player_after_failed_digging(
    player: *mut crate::entity::Player,
    previous_block: Block,
    block_position: BlockPosition,
    server: &mut MinecraftServer,
    client: &Client,
) {
    if !previous_block.is_solid() {
        return;
    }
    let player = unsafe { &*player };
    let position = player.position();
    let block_below_player = BlockPosition::new(
        position.x().floor() as i32,
        position.y().floor() as i32 - 1,
        position.z().floor() as i32,
    );
    if block_below_player != block_position {
        return;
    }
    let Some(world_uuid) = server.world_manager.world_uuid_for_client(client) else {
        return;
    };
    let Some(world) = server.world_manager.world_mut(world_uuid) else {
        return;
    };
    let _ = world.teleport_player(
        player.uuid(),
        position,
        None,
        TeleportFlags::absolute(),
        true,
    );
}

fn digging_event_input(
    packet: &PlayerActionPacket,
    client: &Client,
    server: &mut MinecraftServer,
) -> Option<DiggingEventInput> {
    let block_position = BlockPosition::new(
        packet.block_position.x,
        packet.block_position.y,
        packet.block_position.z,
    );
    if !server.block_position_is_loaded_in_world(client, block_position) {
        return None;
    }
    let block_state = server.loaded_block_state_in_world(client, block_position)?;
    let client_block_entity_nbt = server.client_block_entity_nbt_in_world(client, block_position);
    let block = block_state.block();
    let block_face = BlockFace::from_network_direction(packet.block_face.into())?;
    let player = server.world_manager.player_pointer_for_client(client)?;
    Some(DiggingEventInput {
        player,
        block,
        block_state,
        client_block_entity_nbt,
        block_position,
        block_face,
    })
}

fn fail_digging(
    position: BlockPosition,
    sequence: i32,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let block_change_is_acknowledged = acknowledge_block_change(sequence, client);
    let block_entity_is_refreshed = refresh_failed_digging_block_entity(position, server, client);
    block_change_is_acknowledged && block_entity_is_refreshed
}

fn refresh_failed_digging_block_entity(
    position: BlockPosition,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    server
        .refresh_block_entity_in_world(client, position)
        .is_ok()
}

fn acknowledge_block_change(sequence: i32, client: &mut Client) -> bool {
    AcknowledgeBlockChangePacket { sequence }
        .dispatch(client)
        .is_ok()
}

fn player_is_in_water(
    player: &crate::entity::Player,
    server: &mut MinecraftServer,
    client: &Client,
) -> bool {
    let player_position = player.position();
    let eye_position = BlockPosition::new(
        player_position.x().floor() as i32,
        (player_position.y() + player.eye_height()).floor() as i32,
        player_position.z().floor() as i32,
    );
    server
        .loaded_block_in_world(client, eye_position)
        .is_some_and(|block| block == Block::WATER)
}

pub(crate) fn should_prevent_breaking(
    player: &crate::entity::Player,
    block_state: impl Into<BlockState>,
    client_block_entity_nbt: Option<&NbtCompound>,
    registries: &Registries,
) -> bool {
    let block_state = block_state.into();
    let main_hand_item = player.item_in_hand(crate::entity::PlayerHand::Main);
    match player.game_mode() {
        GameMode::Spectator => true,
        GameMode::Adventure => {
            let can_break_block = main_hand_item
                .get_or(CAN_BREAK, BlockPredicates::default())
                .test_state_with_nbt(block_state, client_block_entity_nbt, registries);
            !can_break_block
        }
        GameMode::Creative => main_hand_item
            .get::<Tool>(TOOL)
            .is_some_and(|tool| !tool.can_destroy_blocks_in_creative()),
        _ => false,
    }
}

struct DiggingEventInput {
    player: *mut crate::entity::Player,
    block: Block,
    block_state: BlockState,
    client_block_entity_nbt: Option<NbtCompound>,
    block_position: BlockPosition,
    block_face: BlockFace,
}
