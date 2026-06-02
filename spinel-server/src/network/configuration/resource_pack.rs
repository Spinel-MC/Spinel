use crate::events::player_resource_pack_status::PlayerResourcePackStatusEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::configuration::resource_pack::ResourcePackStatusPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_resource_pack(
    client: &mut Client,
    packet: ResourcePackStatusPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let mut event = PlayerResourcePackStatusEvent::new(player, packet.id, packet.status);
    event.dispatch(server, client);
    unsafe { &mut *player }
        .on_resource_pack_status(packet.id, packet.status)
        .is_ok()
}
