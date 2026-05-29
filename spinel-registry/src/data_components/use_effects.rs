use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{bool_field_or, compound_from_nbt, f32_field_or};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UseEffects {
    can_sprint: bool,
    interact_vibrations: bool,
    speed_multiplier: f32,
}

impl UseEffects {
    pub const DEFAULT: Self = Self::new(false, true, 0.2);

    #[must_use]
    pub const fn new(can_sprint: bool, interact_vibrations: bool, speed_multiplier: f32) -> Self {
        Self {
            can_sprint,
            interact_vibrations,
            speed_multiplier,
        }
    }

    #[must_use]
    pub const fn can_sprint(&self) -> bool {
        self.can_sprint
    }

    #[must_use]
    pub const fn interact_vibrations(&self) -> bool {
        self.interact_vibrations
    }

    #[must_use]
    pub const fn speed_multiplier(&self) -> f32 {
        self.speed_multiplier
    }
}

impl DataComponentValue for UseEffects {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.can_sprint != Self::DEFAULT.can_sprint {
            compound.insert(
                "can_sprint".to_string(),
                Nbt::Byte(i8::from(self.can_sprint)),
            );
        }
        if self.interact_vibrations != Self::DEFAULT.interact_vibrations {
            compound.insert(
                "interact_vibrations".to_string(),
                Nbt::Byte(i8::from(self.interact_vibrations)),
            );
        }
        if self.speed_multiplier != Self::DEFAULT.speed_multiplier {
            compound.insert(
                "speed_multiplier".to_string(),
                Nbt::Float(self.speed_multiplier),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            can_sprint: bool_field_or(compound, "can_sprint", false)?,
            interact_vibrations: bool_field_or(compound, "interact_vibrations", true)?,
            speed_multiplier: f32_field_or(compound, "speed_multiplier", 0.2)?,
        })
    }
}
