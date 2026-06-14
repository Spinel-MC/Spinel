use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::keep_alive::KeepAlivePacket;
use spinel_macros::packet_listener;
use spinel_utils::component::Component;

#[packet_listener]
fn on_keep_alive(
    client: &mut Client,
    packet: KeepAlivePacket,
    server: &mut MinecraftServer,
) -> bool {
    if client.handle_keep_alive(packet.id) {
        return true;
    }

    if server
        .world_manager
        .player_pointer_for_client(client)
        .is_none()
    {
        client.close_connection();
        server.handle_connection_closed_with_client(client.addr, client);
        return false;
    }
    let _ = server.kick(client, Component::text("Timed out"));
    false
}
