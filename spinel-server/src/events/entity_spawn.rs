use crate::entity::{Entity, EntityId};
use crate::world::World;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntitySpawnEvent {
    world: *mut World,
    entity: *mut Entity,
}

impl EntitySpawnEvent {
    pub fn new(entity: *mut Entity, world: *mut World) -> Self {
        Self { world, entity }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }
}
