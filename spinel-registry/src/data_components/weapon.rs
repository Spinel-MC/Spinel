use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, f32_field_or, i32_field_or};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon {
    item_damage_per_attack: i32,
    disable_blocking_for_seconds: f32,
}

impl Weapon {
    pub const DEFAULT: Self = Self::new(1, 0.0);

    #[must_use]
    pub const fn new(item_damage_per_attack: i32, disable_blocking_for_seconds: f32) -> Self {
        Self {
            item_damage_per_attack,
            disable_blocking_for_seconds,
        }
    }

    #[must_use]
    pub const fn item_damage_per_attack(&self) -> i32 {
        self.item_damage_per_attack
    }

    #[must_use]
    pub const fn disable_blocking_for_seconds(&self) -> f32 {
        self.disable_blocking_for_seconds
    }

    #[must_use]
    pub const fn with_item_damage_per_attack(&self, item_damage_per_attack: i32) -> Self {
        Self {
            item_damage_per_attack,
            disable_blocking_for_seconds: self.disable_blocking_for_seconds,
        }
    }

    #[must_use]
    pub const fn with_disable_blocking_for_seconds(
        &self,
        disable_blocking_for_seconds: f32,
    ) -> Self {
        Self {
            item_damage_per_attack: self.item_damage_per_attack,
            disable_blocking_for_seconds,
        }
    }
}

impl DataComponentValue for Weapon {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.item_damage_per_attack != Self::DEFAULT.item_damage_per_attack {
            compound.insert(
                "item_damage_per_attack".to_string(),
                Nbt::Int(self.item_damage_per_attack),
            );
        }
        if self.disable_blocking_for_seconds != Self::DEFAULT.disable_blocking_for_seconds {
            compound.insert(
                "disable_blocking_for_seconds".to_string(),
                Nbt::Float(self.disable_blocking_for_seconds),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            item_damage_per_attack: i32_field_or(compound, "item_damage_per_attack", 1)?,
            disable_blocking_for_seconds: f32_field_or(
                compound,
                "disable_blocking_for_seconds",
                0.0,
            )?,
        })
    }
}
