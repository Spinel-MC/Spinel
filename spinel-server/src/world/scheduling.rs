impl World {
    pub fn scheduler(&mut self) -> &mut WorldScheduler {
        &mut self.scheduler
    }

    pub fn schedule_next_tick(
        &mut self,
        callback: impl FnMut(&mut World) + Send + 'static,
    ) -> crate::scheduler::Task {
        self.scheduler.schedule_next_tick(callback)
    }

    pub fn schedule_tick_end(
        &mut self,
        callback: impl FnMut(&mut World) + Send + 'static,
    ) -> crate::scheduler::Task {
        self.scheduler.schedule_tick_end(callback)
    }

    fn process_next_tick_scheduler(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick(self);
        self.scheduler = scheduler;
    }

    fn process_tick_end_scheduler(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick_end(self);
        self.scheduler = scheduler;
    }
}
