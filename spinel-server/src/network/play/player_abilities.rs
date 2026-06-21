use crate::events::player_start_flying::PlayerStartFlyingEvent;
use crate::events::player_stop_flying::PlayerStopFlyingEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::serverbound::play::player_abilities::ServerboundPlayerAbilitiesPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_player_abilities(
    client: &mut Client,
    packet: ServerboundPlayerAbilitiesPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let player_ref = unsafe { &mut *player };
    let can_fly = player_ref.can_fly() || player_ref.get_game_mode() == GameMode::Creative;
    if !can_fly {
        return true;
    }
    let is_flying = packet.is_flying();
    player_ref.refresh_flying(is_flying);
    if is_flying {
        PlayerStartFlyingEvent::new(player).dispatch(server, client);
    } else {
        PlayerStopFlyingEvent::new(player).dispatch(server, client);
    }
    true
}
