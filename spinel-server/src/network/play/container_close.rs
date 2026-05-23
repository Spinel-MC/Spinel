use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::container_close::ContainerClosePacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_container_close(
    client: &mut Client,
    packet: ContainerClosePacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    unsafe { &mut *player }.close_inventory_window_with_client(
        true,
        packet.container_id.into(),
        server,
        client,
    )
}
