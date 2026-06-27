use crate::data_components::DataComponentValue;
use crate::enchantment::Enchantment;
use crate::{Identifier, RegistryKey};
use spinel_nbt::{Nbt, NbtCompound};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EnchantmentList {
    enchantments: HashMap<RegistryKey<Enchantment>, i32>,
}

impl EnchantmentList {
    #[must_use]
    pub fn new(enchantments: HashMap<RegistryKey<Enchantment>, i32>) -> Self {
        Self { enchantments }
    }

    #[must_use]
    pub fn from_enchantment(enchantment: RegistryKey<Enchantment>, level: i32) -> Self {
        Self {
            enchantments: HashMap::from([(enchantment, level)]),
        }
    }

    #[must_use]
    pub fn get_enchantments(&self) -> &HashMap<RegistryKey<Enchantment>, i32> {
        &self.enchantments
    }

    #[must_use]
    pub fn has(&self, enchantment: &RegistryKey<Enchantment>) -> bool {
        self.enchantments.contains_key(enchantment)
    }

    #[must_use]
    pub fn level(&self, enchantment: &RegistryKey<Enchantment>) -> i32 {
        self.enchantments.get(enchantment).copied().unwrap_or(0)
    }

    #[must_use]
    pub fn with(&self, enchantment: RegistryKey<Enchantment>, level: i32) -> Self {
        let mut enchantments = self.enchantments.clone();
        enchantments.insert(enchantment, level);
        Self { enchantments }
    }

    #[must_use]
    pub fn remove(&self, enchantment: &RegistryKey<Enchantment>) -> Self {
        let mut enchantments = self.enchantments.clone();
        enchantments.remove(enchantment);
        Self { enchantments }
    }
}

impl DataComponentValue for EnchantmentList {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        for (enchantment, level) in &self.enchantments {
            compound.insert(enchantment.key().to_string(), Nbt::Int(*level));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let Nbt::Compound(compound) = component_nbt else {
            return None;
        };
        let enchantments = compound
            .0
            .iter()
            .map(|(enchantment, level)| {
                let Nbt::Int(level) = level else {
                    return None;
                };
                let enchantment_identifier: Identifier = enchantment.parse().ok()?;
                Some((RegistryKey::new(enchantment_identifier), *level))
            })
            .collect::<Option<HashMap<_, _>>>()?;
        Some(Self { enchantments })
    }
}
