use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::keep_alive::KeepAlivePacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_keep_alive(
    client: &mut Client,
    packet: KeepAlivePacket,
    server: &mut MinecraftServer,
) -> bool {
    if client.handle_keep_alive(packet.id) {
        return true;
    }

    let _ = server.disconnect(
        client,
        spinel_utils::component::Component::text("Timed out"),
    );
    false
}
