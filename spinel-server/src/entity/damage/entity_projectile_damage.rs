use super::Damage;
use crate::entity::EntityId;
use spinel_registry::damage_type::DamageType;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq)]
pub struct EntityProjectileDamage {
    damage: Damage,
    projectile: EntityId,
    shooter: Option<EntityId>,
}

impl EntityProjectileDamage {
    pub fn new(shooter: Option<EntityId>, projectile: EntityId, amount: f32) -> Self {
        let damage = Damage::new(DamageType::MOB_PROJECTILE, amount).with_source(projectile);
        let damage = match shooter {
            Some(shooter) => damage.with_attacker(shooter),
            None => damage,
        };
        Self {
            damage,
            projectile,
            shooter,
        }
    }

    pub const fn projectile(&self) -> EntityId {
        self.projectile
    }

    pub const fn shooter(&self) -> Option<EntityId> {
        self.shooter
    }
}

impl Deref for EntityProjectileDamage {
    type Target = Damage;

    fn deref(&self) -> &Self::Target {
        &self.damage
    }
}

impl DerefMut for EntityProjectileDamage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.damage
    }
}

impl From<EntityProjectileDamage> for Damage {
    fn from(projectile_damage: EntityProjectileDamage) -> Self {
        projectile_damage.damage
    }
}
