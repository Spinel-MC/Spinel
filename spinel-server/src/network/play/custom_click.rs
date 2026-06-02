use crate::events::player_custom_click::PlayerCustomClickEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::custom_click_action::PlayCustomClickActionPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_custom_click_action(
    client: &mut Client,
    packet: PlayCustomClickActionPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let payload = packet.payload_without_end_tag().map(ToOwned::to_owned);
    PlayerCustomClickEvent::new(player, packet.key, payload).dispatch(server, client);
    true
}
