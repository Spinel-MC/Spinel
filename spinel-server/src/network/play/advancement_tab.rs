use crate::events::advancement_tab::AdvancementTabEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::seen_advancements::SeenAdvancementsPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_seen_advancements(
    client: &mut Client,
    packet: SeenAdvancementsPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let tab_identifier = match packet.action {
        SeenAdvancementsPacket::OPENED_TAB => packet.tab_identifier,
        SeenAdvancementsPacket::CLOSED_SCREEN => None,
        _ => return false,
    };
    AdvancementTabEvent::new(player, tab_identifier).dispatch(server, client);
    true
}
