use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::play::chunk_batch_finished::ChunkBatchFinishedPacket;
use spinel_core::network::clientbound::play::chunk_batch_start::ChunkBatchStartPacket;
use spinel_core::network::serverbound::play::chunk_batch_received::ChunkBatchReceivedPacket;
use spinel_macros::packet_listener;
use std::time::Instant;

const INITIAL_NANOS_PER_CHUNK: f64 = 2_000_000.0;
const TARGET_NANOS_PER_TICK: f64 = 7_000_000.0;
const MAX_SAMPLE_SIZE: u32 = 49;

pub(crate) struct ChunkBatchSizeCalculator {
    average_nanos_per_chunk: f64,
    sample_size: u32,
    started_at: Instant,
}

impl ChunkBatchSizeCalculator {
    pub(crate) fn new() -> Self {
        Self {
            average_nanos_per_chunk: INITIAL_NANOS_PER_CHUNK,
            sample_size: 1,
            started_at: Instant::now(),
        }
    }

    pub(crate) fn start_batch(&mut self) {
        self.started_at = Instant::now();
    }

    pub(crate) fn finish_batch(&mut self, batch_size: i32) -> f32 {
        if batch_size > 0 {
            let elapsed_nanos = self.started_at.elapsed().as_nanos() as f64;
            let nanos_per_chunk = elapsed_nanos / f64::from(batch_size);
            let bounded_nanos_per_chunk = nanos_per_chunk.clamp(
                self.average_nanos_per_chunk / 3.0,
                self.average_nanos_per_chunk * 3.0,
            );
            self.average_nanos_per_chunk = (self.average_nanos_per_chunk
                * f64::from(self.sample_size)
                + bounded_nanos_per_chunk)
                / f64::from(self.sample_size + 1);
            self.sample_size = (self.sample_size + 1).min(MAX_SAMPLE_SIZE);
        }

        (TARGET_NANOS_PER_TICK / self.average_nanos_per_chunk) as f32
    }
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_chunk_batch_start(
    _server: &mut Server,
    _packet: ChunkBatchStartPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.start_chunk_batch();
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_chunk_batch_finished(
    server: &mut Server,
    packet: ChunkBatchFinishedPacket,
    client: &mut MinecraftClient,
) -> bool {
    let desired_chunks_per_tick = client.finish_chunk_batch(packet.batch_size);
    ChunkBatchReceivedPacket {
        desired_chunks_per_tick,
    }
    .dispatch(server)
    .is_ok()
}
