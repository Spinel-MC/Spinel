use crate::entity::{Entity, EntityId};
use crate::world::World;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct AddEntityToInstanceEvent {
    instance: *mut World,
    entity: *mut Entity,
    cancelled: bool,
}

impl AddEntityToInstanceEvent {
    pub fn new(instance: *mut World, entity: *mut Entity) -> Self {
        Self {
            instance,
            entity,
            cancelled: false,
        }
    }

    pub fn get_instance(&mut self) -> &mut World {
        unsafe { &mut *self.instance }
    }

    pub fn get_world(&mut self) -> &mut World {
        self.get_instance()
    }

    pub fn get_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn get_entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).get_entity_id() }
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
