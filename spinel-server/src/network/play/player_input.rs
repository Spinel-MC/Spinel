use crate::events::player_input::PlayerInputEvent;
use crate::events::player_start_sneaking::PlayerStartSneakingEvent;
use crate::events::player_stop_sneaking::PlayerStopSneakingEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::player_input::PlayerInputPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_player_input(
    client: &mut Client,
    packet: PlayerInputPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let old_inputs = unsafe { &*player }.get_inputs();
    if server
        .refresh_player_input_in_world(
            client,
            packet.forward(),
            packet.backward(),
            packet.left(),
            packet.right(),
            packet.jump(),
            packet.shift(),
            packet.sprint(),
        )
        .is_err()
    {
        return false;
    }
    let mut event = PlayerInputEvent::new(player, old_inputs);
    event.dispatch(server, client);
    if event.has_pressed_shift_key() {
        PlayerStartSneakingEvent::new(player).dispatch(server, client);
    } else if event.has_released_shift_key() {
        PlayerStopSneakingEvent::new(player).dispatch(server, client);
    }
    true
}
