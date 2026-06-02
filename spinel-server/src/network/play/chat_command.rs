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
    execute_chat_command(client, &packet.command, server)
}

pub(crate) fn execute_chat_command(
    client: &mut Client,
    command: &str,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    if !unsafe { &*player }.can_receive_chat_command() {
        return unsafe { &mut *player }
            .send_chat_rejection_message()
            .is_ok();
    }
    let command_manager = std::mem::take(&mut server.command_manager);
    let command_result = command_manager.execute(server, client, command);
    server.command_manager = command_manager;
    command_result.packet_listener_result()
}
