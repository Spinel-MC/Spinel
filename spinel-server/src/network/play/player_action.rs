use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::player_action::PlayerActionPacket;
use spinel_macros::packet_listener;

const DROP_ITEM_STACK: i32 = 3;
const DROP_ITEM: i32 = 4;
const SWAP_ITEM_HAND: i32 = 6;

#[packet_listener]
fn on_player_action(
    client: &mut Client,
    packet: PlayerActionPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let player = unsafe { &mut *player };
    match packet.status {
        DROP_ITEM_STACK => player.drop_main_hand_item(true, server, client),
        DROP_ITEM => player.drop_main_hand_item(false, server, client),
        SWAP_ITEM_HAND => player.swap_item_hands(server, client),
        _ => true,
    }
}
