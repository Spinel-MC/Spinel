use crate::events::player_debug_subscriptions_request::PlayerDebugSubscriptionsRequestEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::debug_subscription_request::DebugSubscriptionRequestPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_debug_subscription_request(
    client: &mut Client,
    packet: DebugSubscriptionRequestPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let mut event = PlayerDebugSubscriptionsRequestEvent::new(player, packet.subscriptions.clone());
    event.dispatch(server, client);
    unsafe { &mut *player }.set_debug_subscriptions(packet.subscriptions);
    true
}
