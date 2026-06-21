use crate::events::player_chat::PlayerChatEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::chat::ChatPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_chat(client: &mut Client, packet: ChatPacket, server: &mut MinecraftServer) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let sender = unsafe { &*player }.get_uuid();
    if !unsafe { &*player }.can_receive_chat_message() {
        return unsafe { &mut *player }
            .send_chat_rejection_message()
            .is_ok();
    }
    let recipients = server.world_manager.online_player_uuids();
    let mut event = PlayerChatEvent::new(player, recipients, packet.message);
    event.dispatch(server, client);
    if event.is_cancelled() {
        return true;
    }
    let (recipients, message) = event.into_message();
    server
        .world_manager
        .send_chat_message_to_recipients(&recipients, sender, message)
        .is_ok()
}
