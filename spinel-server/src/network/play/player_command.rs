use crate::events::player_leave_bed::PlayerLeaveBedEvent;
use crate::events::player_start_flying_with_elytra::PlayerStartFlyingWithElytraEvent;
use crate::events::player_start_sprinting::PlayerStartSprintingEvent;
use crate::events::player_stop_sprinting::PlayerStopSprintingEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::player_command::PlayerCommandPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_player_command(
    client: &mut Client,
    packet: PlayerCommandPacket,
    server: &mut MinecraftServer,
) -> bool {
    match packet.action {
        PlayerCommandPacket::START_SPRINTING => set_player_sprinting(client, server, true),
        PlayerCommandPacket::STOP_SPRINTING => set_player_sprinting(client, server, false),
        PlayerCommandPacket::START_FLYING_ELYTRA => start_player_flying_with_elytra(client, server),
        PlayerCommandPacket::LEAVE_BED => leave_bed(client, server),
        PlayerCommandPacket::START_JUMP_HORSE
        | PlayerCommandPacket::STOP_JUMP_HORSE
        | PlayerCommandPacket::OPEN_HORSE_INVENTORY => true,
        _ => false,
    }
}

fn set_player_sprinting(
    client: &mut Client,
    server: &mut MinecraftServer,
    sprinting: bool,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let old_sprinting = unsafe { &*player }.is_sprinting();
    if server
        .set_player_sprinting_in_world(client, sprinting)
        .is_err()
    {
        return false;
    }
    if old_sprinting == sprinting {
        return true;
    }
    if sprinting {
        PlayerStartSprintingEvent::new(player).dispatch(server, client);
    } else {
        PlayerStopSprintingEvent::new(player).dispatch(server, client);
    }
    true
}

fn start_player_flying_with_elytra(client: &mut Client, server: &mut MinecraftServer) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    if server
        .start_player_flying_with_elytra_in_world(client)
        .is_err()
    {
        return false;
    }
    PlayerStartFlyingWithElytraEvent::new(player).dispatch(server, client);
    true
}

fn leave_bed(client: &mut Client, server: &mut MinecraftServer) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let mut event = PlayerLeaveBedEvent::new(player);
    event.dispatch(server, client);
    if event.is_cancelled() {
        return true;
    }
    unsafe { &mut *player }.leave_bed();
    server.refresh_player_metadata_in_world(client).is_ok()
}
