use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::container_click::ContainerClickPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_container_click(
    client: &mut Client,
    packet: ContainerClickPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    unsafe { &mut *player }.handle_container_click(&packet, server, client)
}
