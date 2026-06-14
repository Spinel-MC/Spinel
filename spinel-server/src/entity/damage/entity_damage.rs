use super::Damage;
use crate::entity::EntityId;
use spinel_registry::damage_type::DamageType;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq)]
pub struct EntityDamage {
    damage: Damage,
    source: EntityId,
}

impl EntityDamage {
    pub fn new(source: EntityId, amount: f32) -> Self {
        Self {
            damage: Damage::new(DamageType::MOB_ATTACK, amount)
                .with_source(source)
                .with_attacker(source),
            source,
        }
    }

    pub const fn source(&self) -> EntityId {
        self.source
    }

    pub const fn attacker(&self) -> EntityId {
        self.source()
    }
}

impl Deref for EntityDamage {
    type Target = Damage;

    fn deref(&self) -> &Self::Target {
        &self.damage
    }
}

impl DerefMut for EntityDamage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.damage
    }
}

impl From<EntityDamage> for Damage {
    fn from(entity_damage: EntityDamage) -> Self {
        entity_damage.damage
    }
}
