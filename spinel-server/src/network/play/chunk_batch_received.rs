use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::chunk_batch_received::ChunkBatchReceivedPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_chunk_batch_received(
    _client: &mut Client,
    _packet: ChunkBatchReceivedPacket,
    _server: &mut MinecraftServer,
) -> bool {
    true
}
