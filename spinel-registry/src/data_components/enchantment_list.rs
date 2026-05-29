use crate::Identifier;
use crate::data_components::DataComponentValue;
use spinel_nbt::{Nbt, NbtCompound};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EnchantmentList {
    enchantments: HashMap<Identifier, i32>,
}

impl EnchantmentList {
    #[must_use]
    pub fn new(enchantments: HashMap<Identifier, i32>) -> Self {
        Self { enchantments }
    }

    #[must_use]
    pub fn enchantments(&self) -> &HashMap<Identifier, i32> {
        &self.enchantments
    }

    #[must_use]
    pub fn has(&self, enchantment: &Identifier) -> bool {
        self.enchantments.contains_key(enchantment)
    }

    #[must_use]
    pub fn level(&self, enchantment: &Identifier) -> i32 {
        self.enchantments.get(enchantment).copied().unwrap_or(0)
    }

    #[must_use]
    pub fn with(&self, enchantment: Identifier, level: i32) -> Self {
        let mut enchantments = self.enchantments.clone();
        enchantments.insert(enchantment, level);
        Self { enchantments }
    }

    #[must_use]
    pub fn remove(&self, enchantment: &Identifier) -> Self {
        let mut enchantments = self.enchantments.clone();
        enchantments.remove(enchantment);
        Self { enchantments }
    }
}

impl DataComponentValue for EnchantmentList {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        for (enchantment, level) in &self.enchantments {
            compound.insert(enchantment.to_string(), Nbt::Int(*level));
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
                Some((enchantment.parse().ok()?, *level))
            })
            .collect::<Option<HashMap<_, _>>>()?;
        Some(Self { enchantments })
    }
}
