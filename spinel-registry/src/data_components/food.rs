use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{
    bool_field_or, compound_from_nbt, f32_field_or, i32_field_or,
};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct Food {
    nutrition: i32,
    saturation_modifier: f32,
    can_always_eat: bool,
}

impl Food {
    #[must_use]
    pub const fn new(nutrition: i32, saturation_modifier: f32, can_always_eat: bool) -> Self {
        Self {
            nutrition,
            saturation_modifier,
            can_always_eat,
        }
    }

    #[must_use]
    pub const fn nutrition(&self) -> i32 {
        self.nutrition
    }

    #[must_use]
    pub const fn saturation_modifier(&self) -> f32 {
        self.saturation_modifier
    }

    #[must_use]
    pub const fn can_always_eat(&self) -> bool {
        self.can_always_eat
    }
}

impl DataComponentValue for Food {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("nutrition".to_string(), Nbt::Int(self.nutrition));
        compound.insert(
            "saturation".to_string(),
            Nbt::Float(self.saturation_modifier),
        );
        if self.can_always_eat {
            compound.insert("can_always_eat".to_string(), Nbt::Byte(1));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            nutrition: i32_field_or(compound, "nutrition", 0)?,
            saturation_modifier: f32_field_or(compound, "saturation", 0.0)?,
            can_always_eat: bool_field_or(compound, "can_always_eat", false)?,
        })
    }
}
