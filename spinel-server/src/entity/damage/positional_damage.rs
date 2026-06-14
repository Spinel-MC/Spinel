use super::Damage;
use spinel_network::types::Vector3d;
use spinel_registry::RegistryKey;
use spinel_registry::damage_type::DamageType;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq)]
pub struct PositionalDamage {
    damage: Damage,
}

impl PositionalDamage {
    pub fn new(
        damage_type: RegistryKey<DamageType>,
        source_position: Vector3d,
        amount: f32,
    ) -> Self {
        Self {
            damage: Damage::new(damage_type, amount).with_source_position(source_position),
        }
    }
}

impl Deref for PositionalDamage {
    type Target = Damage;

    fn deref(&self) -> &Self::Target {
        &self.damage
    }
}

impl DerefMut for PositionalDamage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.damage
    }
}

impl From<PositionalDamage> for Damage {
    fn from(positional_damage: PositionalDamage) -> Self {
        positional_damage.damage
    }
}
