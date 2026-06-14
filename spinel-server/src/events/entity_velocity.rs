use crate::entity::{Entity, EntityId};
use spinel_macros::event_dispatcher;
use spinel_network::types::Velocity;

#[event_dispatcher]
pub struct EntityVelocityEvent {
    entity: *mut Entity,
    velocity: Velocity,
    cancelled: bool,
}

impl EntityVelocityEvent {
    pub fn new(entity: *mut Entity, velocity: Velocity) -> Self {
        Self {
            entity,
            velocity,
            cancelled: false,
        }
    }

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }

    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
