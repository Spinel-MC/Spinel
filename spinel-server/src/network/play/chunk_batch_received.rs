use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::chunk_batch_received::ChunkBatchReceivedPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_chunk_batch_received(
    client: &mut Client,
    packet: ChunkBatchReceivedPacket,
    server: &mut MinecraftServer,
) -> bool {
    if let Some(player) = server.world_manager.player_mut_for_client(client) {
        player.on_chunk_batch_received(packet.desired_chunks_per_tick);
    }
    true
}
