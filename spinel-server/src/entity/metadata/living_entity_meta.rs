use crate::entity::PlayerHand;
use crate::entity::metadata::EntityMeta;
use spinel_network::types::{Particle, Position};
use std::ops::{Deref, DerefMut};

pub struct LivingEntityMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> LivingEntityMeta<'entity> {
    pub(crate) fn new(entity_meta: EntityMeta<'entity>) -> Self {
        Self { entity_meta }
    }

    pub fn is_hand_active(&self) -> bool {
        self.entity().is_hand_active()
    }

    pub fn set_hand_active(&mut self, is_hand_active: bool) {
        self.entity_mut().set_hand_active(is_hand_active);
    }

    pub fn active_hand(&self) -> PlayerHand {
        self.entity().active_hand()
    }

    pub fn set_active_hand(&mut self, active_hand: PlayerHand) {
        self.entity_mut().set_active_hand(active_hand);
    }

    pub fn is_in_riptide_spin_attack(&self) -> bool {
        self.entity().is_in_riptide_spin_attack()
    }

    pub fn set_in_riptide_spin_attack(&mut self, is_in_riptide_spin_attack: bool) {
        self.entity_mut()
            .set_in_riptide_spin_attack(is_in_riptide_spin_attack);
    }

    pub fn health(&self) -> f32 {
        self.entity().health()
    }

    pub fn set_health(&mut self, health: f32) {
        self.entity_mut().set_health(health);
    }

    pub fn effect_particles(&self) -> Vec<Particle> {
        self.entity().effect_particles()
    }

    pub fn set_effect_particles(&mut self, effect_particles: Vec<Particle>) {
        self.entity_mut().set_effect_particles(effect_particles);
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        self.entity().is_potion_effect_ambient()
    }

    pub fn set_potion_effect_ambient(&mut self, is_potion_effect_ambient: bool) {
        self.entity_mut()
            .set_potion_effect_ambient(is_potion_effect_ambient);
    }

    pub fn arrow_count(&self) -> i32 {
        self.entity().arrow_count()
    }

    pub fn set_arrow_count(&mut self, arrow_count: i32) {
        self.entity_mut().set_arrow_count(arrow_count);
    }

    pub fn bee_stinger_count(&self) -> i32 {
        self.entity().bee_stinger_count()
    }

    pub fn set_bee_stinger_count(&mut self, bee_stinger_count: i32) {
        self.entity_mut().set_bee_stinger_count(bee_stinger_count);
    }

    pub fn bed_in_which_sleeping_position(&self) -> Option<Position> {
        self.entity().bed_in_which_sleeping_position()
    }

    pub fn set_bed_in_which_sleeping_position(&mut self, bed_position: Option<Position>) {
        self.entity_mut()
            .set_bed_in_which_sleeping_position(bed_position);
    }
}

impl<'entity> Deref for LivingEntityMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for LivingEntityMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
