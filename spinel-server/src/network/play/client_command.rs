use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::client_command::ClientCommandPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_client_command(
    client: &mut Client,
    packet: ClientCommandPacket,
    server: &mut MinecraftServer,
) -> bool {
    if packet.action != ClientCommandPacket::PERFORM_RESPAWN {
        return true;
    }

    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };

    unsafe { &mut *player }.respawn().is_ok()
}
