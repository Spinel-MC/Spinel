use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::set_carried_item::SetCarriedItemPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_set_carried_item(
    client: &mut Client,
    packet: SetCarriedItemPacket,
    server: &mut MinecraftServer,
) -> bool {
    server
        .set_player_held_slot_in_world(client, packet.slot as i32)
        .unwrap_or(false)
}
