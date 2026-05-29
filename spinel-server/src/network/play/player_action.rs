use crate::events::player_block_interact::BlockFace;
use crate::events::player_cancel_digging::PlayerCancelDiggingEvent;
use crate::events::player_finish_digging::PlayerFinishDiggingEvent;
use crate::events::player_start_digging::PlayerStartDiggingEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{Block, BlockPosition};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::acknowledge_block_change::AcknowledgeBlockChangePacket;
use spinel_core::network::serverbound::play::player_action::PlayerActionPacket;
use spinel_macros::packet_listener;

const STARTED_DIGGING: i32 = 0;
const CANCELLED_DIGGING: i32 = 1;
const FINISHED_DIGGING: i32 = 2;
const DROP_ITEM_STACK: i32 = 3;
const DROP_ITEM: i32 = 4;
const UPDATE_ITEM_STATE: i32 = 5;
const SWAP_ITEM_HAND: i32 = 6;

#[packet_listener]
fn on_player_action(
    client: &mut Client,
    packet: PlayerActionPacket,
    server: &mut MinecraftServer,
) -> bool {
    if packet.status == STARTED_DIGGING || packet.status == CANCELLED_DIGGING {
        return acknowledge_digging(packet, client, server);
    }
    if packet.status == FINISHED_DIGGING {
        return finish_digging(packet, server, client);
    }
    if packet.status == UPDATE_ITEM_STATE {
        return update_item_state(server, client);
    }
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let player = unsafe { &mut *player };
    let action_result = match packet.status {
        DROP_ITEM_STACK => player.drop_main_hand_item(true, server, client),
        DROP_ITEM => player.drop_main_hand_item(false, server, client),
        SWAP_ITEM_HAND => player.swap_item_hands(server, client),
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
        return acknowledge_block_change(packet.sequence, client);
    };
    match packet.status {
        STARTED_DIGGING => start_digging(digging_input, packet.sequence, client, server),
        CANCELLED_DIGGING => cancel_digging(digging_input, packet.sequence, client, server),
        _ => acknowledge_block_change(packet.sequence, client),
    }
}

fn finish_digging(
    packet: PlayerActionPacket,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let Some(digging_input) = digging_event_input(&packet, client, server) else {
        return acknowledge_block_change(packet.sequence, client);
    };
    let player = unsafe { &mut *digging_input.player };
    if player.game_mode() == GameMode::Spectator || player.game_mode() == GameMode::Adventure {
        return rollback_digging(
            digging_input.block_position,
            packet.sequence,
            server,
            client,
        );
    }
    let mut event = PlayerFinishDiggingEvent::new(
        digging_input.player,
        Block::AIR,
        digging_input.block_position,
    );
    event.dispatch(server, client);
    let block_was_broken = server
        .set_block_in_world(client, digging_input.block_position, event.block())
        .unwrap_or(false);
    let block_change_is_acknowledged = acknowledge_block_change(packet.sequence, client);
    block_was_broken && block_change_is_acknowledged
}

fn start_digging(
    digging_input: DiggingEventInput,
    sequence: i32,
    client: &mut Client,
    server: &mut MinecraftServer,
) -> bool {
    let player = unsafe { &mut *digging_input.player };
    if player.game_mode() == GameMode::Spectator || player.game_mode() == GameMode::Adventure {
        return rollback_digging(digging_input.block_position, sequence, server, client);
    }
    let mut event = PlayerStartDiggingEvent::new(
        digging_input.player,
        digging_input.block,
        digging_input.block_position,
        digging_input.block_face,
    );
    event.dispatch(server, client);
    if event.is_cancelled() {
        return rollback_digging(digging_input.block_position, sequence, server, client);
    }
    if player.has_instant_break() {
        let mut finish_event = PlayerFinishDiggingEvent::new(
            digging_input.player,
            Block::AIR,
            digging_input.block_position,
        );
        finish_event.dispatch(server, client);
        let block_was_broken = server
            .set_block_in_world(client, digging_input.block_position, finish_event.block())
            .unwrap_or(false);
        return block_was_broken && acknowledge_block_change(sequence, client);
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
    let block = server.loaded_block_in_world(client, block_position)?;
    let block_face = BlockFace::from_network_direction(packet.block_face.into())?;
    let player = server.world_manager.player_pointer_for_client(client)?;
    Some(DiggingEventInput {
        player,
        block,
        block_position,
        block_face,
    })
}

fn rollback_digging(
    position: BlockPosition,
    sequence: i32,
    server: &mut MinecraftServer,
    client: &mut Client,
) -> bool {
    let block_is_refreshed = server.refresh_block_in_world(client, position).is_ok();
    block_is_refreshed && acknowledge_block_change(sequence, client)
}

fn acknowledge_block_change(sequence: i32, client: &mut Client) -> bool {
    AcknowledgeBlockChangePacket { sequence }
        .dispatch(client)
        .is_ok()
}

struct DiggingEventInput {
    player: *mut crate::entity::Player,
    block: Block,
    block_position: BlockPosition,
    block_face: BlockFace,
}
