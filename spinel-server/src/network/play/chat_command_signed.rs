use crate::network::client::instance::Client;
use crate::network::play::chat_command::execute_chat_command;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::chat_command_signed::SignedCommandChatPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_chat_command_signed(
    client: &mut Client,
    packet: SignedCommandChatPacket,
    server: &mut MinecraftServer,
) -> bool {
    execute_chat_command(client, &packet.command, server)
}
