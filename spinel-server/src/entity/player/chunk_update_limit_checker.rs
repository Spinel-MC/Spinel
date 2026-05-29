use crate::entity::player::chunks::PlayerChunk;
use std::collections::VecDeque;

pub(crate) struct ChunkUpdateLimitChecker {
    chunk_history: VecDeque<PlayerChunk>,
    history_size: usize,
}

impl ChunkUpdateLimitChecker {
    pub(crate) fn new(history_size: usize) -> Self {
        Self {
            chunk_history: VecDeque::with_capacity(history_size),
            history_size,
        }
    }

    pub(crate) fn add_to_history(&mut self, chunk: PlayerChunk) -> bool {
        if self.history_size == 0 {
            return true;
        }
        let chunk_was_already_updated = self.chunk_history.contains(&chunk);
        if self.chunk_history.len() == self.history_size {
            self.chunk_history.pop_front();
        }
        self.chunk_history.push_back(chunk);
        !chunk_was_already_updated
    }

    pub(crate) fn clear_history(&mut self) {
        self.chunk_history.clear();
    }
}
