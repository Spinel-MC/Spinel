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
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    unsafe { &mut *player }.set_held_slot(packet.slot as i32)
}
