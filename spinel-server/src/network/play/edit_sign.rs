use crate::events::player_edit_sign::PlayerEditSignEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::BlockPosition;
use spinel_core::network::serverbound::play::update_sign::UpdateSignPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_update_sign(
    client: &mut Client,
    packet: UpdateSignPacket,
    server: &mut MinecraftServer,
) -> bool {
    let block_position = BlockPosition::new(
        packet.block_position.x,
        packet.block_position.y,
        packet.block_position.z,
    );
    let Some(block) = server.loaded_block_in_world(client, block_position) else {
        return false;
    };
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let mut event = PlayerEditSignEvent::new(
        player,
        block,
        block_position,
        packet.lines,
        packet.is_front_text,
    );
    event.dispatch(server, client);
    true
}
