use crate::entity::{Entity, EntityId};
use crate::world::World;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct RemoveEntityFromInstanceEvent {
    world: *mut World,
    entity: *mut Entity,
}

impl RemoveEntityFromInstanceEvent {
    pub fn new(world: *mut World, entity: *mut Entity) -> Self {
        Self { world, entity }
    }

    pub fn get_world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }

    pub fn get_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn get_entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).get_entity_id() }
    }
}
