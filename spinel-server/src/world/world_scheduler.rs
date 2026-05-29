use crate::world::World;

type WorldTickCallback = Box<dyn FnOnce(&mut World) + Send>;

#[derive(Default)]
pub struct WorldScheduler {
    next_tick_callbacks: Vec<WorldTickCallback>,
    tick_end_callbacks: Vec<WorldTickCallback>,
}

impl WorldScheduler {
    pub fn schedule_next_tick(&mut self, callback: impl FnOnce(&mut World) + Send + 'static) {
        self.next_tick_callbacks.push(Box::new(callback));
    }

    pub fn schedule_tick_end(&mut self, callback: impl FnOnce(&mut World) + Send + 'static) {
        self.tick_end_callbacks.push(Box::new(callback));
    }

    pub fn next_tick_callback_count(&self) -> usize {
        self.next_tick_callbacks.len()
    }

    pub fn tick_end_callback_count(&self) -> usize {
        self.tick_end_callbacks.len()
    }

    pub(crate) fn take_next_tick_callbacks(&mut self) -> Vec<WorldTickCallback> {
        std::mem::take(&mut self.next_tick_callbacks)
    }

    pub(crate) fn take_tick_end_callbacks(&mut self) -> Vec<WorldTickCallback> {
        std::mem::take(&mut self.tick_end_callbacks)
    }
}
