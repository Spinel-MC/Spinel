use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, f32_field_or};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AttackRange {
    min_reach: f32,
    max_reach: f32,
    min_creative_reach: f32,
    max_creative_reach: f32,
    hitbox_margin: f32,
    mob_factor: f32,
}

impl AttackRange {
    pub const DEFAULT: Self = Self::new(0.0, 3.0, 0.0, 5.0, 0.3, 1.0);

    #[must_use]
    pub const fn new(
        min_reach: f32,
        max_reach: f32,
        min_creative_reach: f32,
        max_creative_reach: f32,
        hitbox_margin: f32,
        mob_factor: f32,
    ) -> Self {
        Self {
            min_reach,
            max_reach,
            min_creative_reach,
            max_creative_reach,
            hitbox_margin,
            mob_factor,
        }
    }

    #[must_use]
    pub const fn min_reach(&self) -> f32 {
        self.min_reach
    }

    #[must_use]
    pub const fn max_reach(&self) -> f32 {
        self.max_reach
    }

    #[must_use]
    pub const fn min_creative_reach(&self) -> f32 {
        self.min_creative_reach
    }

    #[must_use]
    pub const fn max_creative_reach(&self) -> f32 {
        self.max_creative_reach
    }

    #[must_use]
    pub const fn hitbox_margin(&self) -> f32 {
        self.hitbox_margin
    }

    #[must_use]
    pub const fn mob_factor(&self) -> f32 {
        self.mob_factor
    }
}

impl DataComponentValue for AttackRange {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("min_reach".to_string(), Nbt::Float(self.min_reach));
        compound.insert("max_reach".to_string(), Nbt::Float(self.max_reach));
        compound.insert(
            "min_creative_reach".to_string(),
            Nbt::Float(self.min_creative_reach),
        );
        compound.insert(
            "max_creative_reach".to_string(),
            Nbt::Float(self.max_creative_reach),
        );
        compound.insert("hitbox_margin".to_string(), Nbt::Float(self.hitbox_margin));
        compound.insert("mob_factor".to_string(), Nbt::Float(self.mob_factor));
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            min_reach: f32_field_or(compound, "min_reach", Self::DEFAULT.min_reach)?,
            max_reach: f32_field_or(compound, "max_reach", Self::DEFAULT.max_reach)?,
            min_creative_reach: f32_field_or(
                compound,
                "min_creative_reach",
                Self::DEFAULT.min_creative_reach,
            )?,
            max_creative_reach: f32_field_or(
                compound,
                "max_creative_reach",
                Self::DEFAULT.max_creative_reach,
            )?,
            hitbox_margin: f32_field_or(compound, "hitbox_margin", Self::DEFAULT.hitbox_margin)?,
            mob_factor: f32_field_or(compound, "mob_factor", Self::DEFAULT.mob_factor)?,
        })
    }
}
