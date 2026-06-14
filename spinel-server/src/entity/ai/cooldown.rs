use std::time::Duration;

pub struct AiCooldown {
    duration: Duration,
    duration_ticks: u64,
    last_update: Option<u64>,
}

impl AiCooldown {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            duration_ticks: duration.as_millis().div_ceil(50) as u64,
            last_update: None,
        }
    }

    pub const fn duration(&self) -> Duration {
        self.duration
    }

    pub fn refresh_last_update(&mut self, last_update: u64) {
        self.last_update = Some(last_update);
    }

    pub fn is_ready(&self, time: u64) -> bool {
        self.last_update
            .is_none_or(|last_update| time.saturating_sub(last_update) >= self.duration_ticks)
    }
}
