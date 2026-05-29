use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::chat_command::ChatCommandPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_chat_command(
    client: &mut Client,
    packet: ChatCommandPacket,
    server: &mut MinecraftServer,
) -> bool {
    let command_manager = std::mem::take(&mut server.command_manager);
    let command_result = command_manager.execute(server, client, &packet.command);
    server.command_manager = command_manager;
    command_result.packet_listener_result()
}
