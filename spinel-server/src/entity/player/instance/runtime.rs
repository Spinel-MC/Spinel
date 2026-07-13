use crate::network::client::instance::Client;
use crate::scheduler::{ContextScheduler, Task, TaskSchedule};
use spinel_core::network::clientbound::play::ticking_state::TickingStatePacket;
use spinel_core::network::clientbound::play::ticking_step::TickingStepPacket;
use std::io;

use super::state::Player;

impl Player {
    pub fn get_scheduler(&mut self) -> &mut ContextScheduler<Player> {
        &mut self.scheduler
    }

    pub fn schedule_next_tick(
        &mut self,
        callback: impl FnMut(&mut Player) -> TaskSchedule + Send + 'static,
    ) -> Task {
        self.scheduler.schedule_next_tick(callback)
    }

    pub const fn get_alive_ticks(&self) -> u64 {
        self.alive_ticks
    }

    pub fn is_removed(&self) -> bool {
        self.is_dead()
    }

    pub(crate) fn tick(&mut self) -> Option<crate::entity::player::PlayerItemUseCompletion> {
        self.process_scheduler_tick_start();
        let _ = self.sync_dirty_player_inventory_slots();
        let current_tick = self.last_completed_client_tick;
        self.item_cooldowns
            .retain(|_, cooldown_expires_at| *cooldown_expires_at > current_tick);
        let item_use_completion = self.tick_item_use();
        if self
            .delayed_remove_ticks
            .is_some_and(|remove_tick| remove_tick <= self.alive_ticks)
        {
            self.living.kill();
        }
        self.process_scheduler_tick_end();
        item_use_completion
    }

    pub(crate) fn get_experience_pickup_is_ready(&self, current_tick: i64) -> bool {
        self.last_experience_pickup_tick
            .is_none_or(|last_pickup_tick| current_tick - last_pickup_tick >= 10)
    }

    pub(crate) fn refresh_experience_pickup_cooldown(&mut self, current_tick: i64) {
        self.last_experience_pickup_tick = Some(current_tick);
    }

    pub(super) fn process_scheduler_tick_start(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick(self);
        self.scheduler = scheduler;
    }

    pub(super) fn process_scheduler_tick_end(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick_end(self);
        self.scheduler = scheduler;
    }

    pub(in crate::entity::player) fn send_tick_rate(
        &self,
        client: &mut Client,
        ticks_per_second: u32,
    ) -> io::Result<()> {
        TickingStatePacket {
            tick_rate: ticks_per_second as f32,
            is_frozen: false,
        }
        .dispatch(client)?;
        TickingStepPacket::new(0).dispatch(client)
    }

    pub(crate) fn finish_client_tick(&mut self, server_tick: u64) {
        self.last_completed_client_tick = server_tick;
    }
}
