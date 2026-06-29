use crate::world::World;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct WorldTickEvent {
    world: *mut World,
    world_age: i64,
}

impl WorldTickEvent {
    pub fn new(world: *mut World, world_age: i64) -> Self {
        Self { world, world_age }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }

    pub const fn world_age(&self) -> i64 {
        self.world_age
    }
}
