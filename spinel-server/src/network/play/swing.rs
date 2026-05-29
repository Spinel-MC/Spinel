use crate::entity::PlayerHand;
use crate::events::player_hand_animation::PlayerHandAnimationEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::swing::SwingPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_swing(client: &mut Client, packet: SwingPacket, server: &mut MinecraftServer) -> bool {
    let Some(hand) = PlayerHand::from_protocol_id(packet.hand) else {
        return false;
    };
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let mut event = PlayerHandAnimationEvent::new(player, hand);
    event.dispatch(server, client);
    if event.is_cancelled() {
        return true;
    }
    server.animate_player_hand_in_world(client, hand).is_ok()
}
