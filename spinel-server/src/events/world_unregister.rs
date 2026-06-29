use crate::world::World;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct WorldUnregisterEvent {
    world: *mut World,
}

impl WorldUnregisterEvent {
    pub fn new(world: *mut World) -> Self {
        Self { world }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }
}
