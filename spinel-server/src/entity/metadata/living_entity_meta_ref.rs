use crate::entity::PlayerHand;
use crate::entity::metadata::EntityMetaRef;
use spinel_network::types::{Particle, Position};
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct LivingEntityMetaRef<'entity> {
    entity_meta: EntityMetaRef<'entity>,
}

impl<'entity> LivingEntityMetaRef<'entity> {
    pub(crate) const fn from_entity_meta(entity_meta: EntityMetaRef<'entity>) -> Self {
        Self { entity_meta }
    }

    pub fn is_hand_active(&self) -> bool {
        self.get_entity().is_hand_active()
    }

    pub fn get_active_hand(&self) -> PlayerHand {
        self.get_entity().get_active_hand()
    }

    pub fn get_health(&self) -> f32 {
        self.get_entity().get_health()
    }

    pub fn get_effect_particles(&self) -> Vec<Particle> {
        self.get_entity().get_effect_particles()
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        self.get_entity().is_potion_effect_ambient()
    }

    pub fn get_arrow_count(&self) -> i32 {
        self.get_entity().get_arrow_count()
    }

    pub fn get_bee_stinger_count(&self) -> i32 {
        self.get_entity().get_bee_stinger_count()
    }

    pub fn get_bed_in_which_sleeping_position(&self) -> Option<Position> {
        self.get_entity().get_bed_in_which_sleeping_position()
    }
}

impl<'entity> Deref for LivingEntityMetaRef<'entity> {
    type Target = EntityMetaRef<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}
