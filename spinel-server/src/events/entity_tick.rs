use crate::entity::Entity;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityTickEvent {
    entity: *mut Entity,
}

impl EntityTickEvent {
    pub fn new(entity: *mut Entity) -> Self {
        Self { entity }
    }

    pub fn get_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }
}
