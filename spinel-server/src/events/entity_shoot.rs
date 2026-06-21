use crate::entity::{Entity, EntityId, EntityPosition};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityShootEvent {
    entity: *mut Entity,
    projectile: *mut Entity,
    target: EntityPosition,
    power: f64,
    spread: f64,
    cancelled: bool,
}

impl EntityShootEvent {
    pub fn new(
        entity: *mut Entity,
        projectile: *mut Entity,
        target: EntityPosition,
        power: f64,
        spread: f64,
    ) -> Self {
        Self {
            entity,
            projectile,
            target,
            power,
            spread,
            cancelled: false,
        }
    }

    pub fn get_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn get_shooter_id(&self) -> EntityId {
        unsafe { (&*self.entity).get_entity_id() }
    }

    pub fn get_projectile(&mut self) -> &mut Entity {
        unsafe { &mut *self.projectile }
    }

    pub fn get_projectile_id(&self) -> EntityId {
        unsafe { (&*self.projectile).get_entity_id() }
    }

    pub const fn get_target(&self) -> EntityPosition {
        self.target
    }

    pub const fn get_power(&self) -> f64 {
        self.power
    }

    pub fn set_power(&mut self, power: f64) {
        self.power = power;
    }

    pub const fn get_spread(&self) -> f64 {
        self.spread
    }

    pub fn set_spread(&mut self, spread: f64) {
        self.spread = spread;
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
