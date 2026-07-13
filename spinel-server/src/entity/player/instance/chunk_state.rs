use super::constants::*;
use super::state::Player;

impl Player {
    #[cfg(test)]
    pub(crate) fn mark_chunk_sent_to_client(
        &mut self,
        chunk: crate::entity::player::chunks::PlayerChunk,
    ) {
        self.client_sent_chunks.insert(chunk);
    }

    pub fn on_chunk_batch_received(&mut self, desired_chunks_per_tick: f32) {
        self.chunk_batch_lead -= 1;
        self.target_chunks_per_tick = if desired_chunks_per_tick.is_nan() {
            MIN_CHUNKS_PER_TICK
        } else {
            (desired_chunks_per_tick * CHUNKS_PER_TICK_MULTIPLIER)
                .clamp(MIN_CHUNKS_PER_TICK, MAX_CHUNKS_PER_TICK)
        };
        if self.max_chunk_batch_lead == 1 {
            self.max_chunk_batch_lead = 10;
        }
    }

    pub const fn get_chunk_batch_lead(&self) -> i32 {
        self.chunk_batch_lead
    }

    pub const fn get_max_chunk_batch_lead(&self) -> i32 {
        self.max_chunk_batch_lead
    }

    pub const fn get_target_chunks_per_tick(&self) -> f32 {
        self.target_chunks_per_tick
    }

    pub const fn get_pending_chunk_count(&self) -> f32 {
        self.pending_chunk_count
    }

    pub const fn get_client_chunk_view_distance(&self) -> i32 {
        self.client_chunk_view_distance
    }

    pub fn set_client_chunk_view_distance(&mut self, client_chunk_view_distance: i32) {
        self.client_chunk_view_distance = client_chunk_view_distance.max(0);
    }

    pub fn get_effective_chunk_view_distance(&self, world_view_distance: i32) -> i32 {
        self.client_chunk_view_distance
            .min(world_view_distance)
            .max(0)
            + 1
    }
}
