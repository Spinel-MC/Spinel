use crate::entity::{Damage, Entity, EntityId};
use spinel_macros::event_dispatcher;
use spinel_network::types::sound::SoundEvent;

#[event_dispatcher]
pub struct EntityDamageEvent {
    entity: *mut Entity,
    damage: Damage,
    cancelled: bool,
}

impl EntityDamageEvent {
    pub fn new(entity: *mut Entity, damage: Damage) -> Self {
        Self {
            entity,
            damage,
            cancelled: false,
        }
    }

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }

    pub fn damage_source(&self) -> &str {
        &self.damage.damage_type().key().path
    }

    pub fn damage(&self) -> &Damage {
        &self.damage
    }

    pub fn damage_mut(&mut self) -> &mut Damage {
        &mut self.damage
    }

    pub fn amount(&self) -> f32 {
        self.damage.amount()
    }

    pub fn set_damage(&mut self, damage: f32) {
        self.damage.set_amount(damage);
    }

    pub fn sound(&self) -> Option<SoundEvent> {
        self.damage.sound()
    }

    pub fn set_sound(&mut self, sound: Option<SoundEvent>) {
        self.damage.set_sound(sound);
    }

    pub fn should_animate(&self) -> bool {
        self.damage.should_animate()
    }

    pub fn set_should_animate(&mut self, should_animate: bool) {
        self.damage.set_should_animate(should_animate);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
