use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::keep_alive::KeepAlivePacket;
use std::time::{SystemTime, UNIX_EPOCH};

const KEEP_ALIVE_INTERVAL_MILLIS: u64 = 15_000;

impl Client {
    pub(crate) fn enter_play(&mut self) {
        self.alive_time = 0;
        self.alive_pending = false;
        self.alive_id = 0;
        self.latency_millis = 0;
    }

    pub(crate) fn tick(&mut self) -> bool {
        if current_time_millis().saturating_sub(self.alive_time) < KEEP_ALIVE_INTERVAL_MILLIS {
            return true;
        }

        if self.alive_pending {
            return false;
        }

        self.alive_pending = true;
        self.alive_id = current_time_millis();
        self.alive_time = self.alive_id;
        KeepAlivePacket {
            id: self.alive_id as i64,
        }
        .dispatch(self)
        .is_ok()
    }

    pub(crate) fn handle_keep_alive(&mut self, keep_alive_id: i64) -> bool {
        if !self.alive_pending || keep_alive_id as u64 != self.alive_id {
            return false;
        }

        self.alive_pending = false;
        self.latency_millis = (self.latency_millis * 3
            + current_time_millis().saturating_sub(self.alive_time) as u32)
            / 4;
        true
    }
}

fn current_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}
