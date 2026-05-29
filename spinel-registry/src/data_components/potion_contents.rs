use crate::Identifier;
use crate::data_components::nbt_reader::{compound_from_nbt, i32_field_or, string_field};
use crate::data_components::{CustomPotionEffect, DataComponentValue};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::color::Color;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PotionContents {
    potion: Option<Identifier>,
    custom_color: Option<Color>,
    custom_effects: Vec<CustomPotionEffect>,
    custom_name: Option<String>,
}

impl PotionContents {
    #[must_use]
    pub fn new(
        potion: Option<Identifier>,
        custom_color: Option<Color>,
        custom_effects: Vec<CustomPotionEffect>,
        custom_name: Option<String>,
    ) -> Self {
        Self {
            potion,
            custom_color,
            custom_effects,
            custom_name,
        }
    }

    #[must_use]
    pub const fn potion(&self) -> Option<&Identifier> {
        self.potion.as_ref()
    }

    #[must_use]
    pub const fn custom_color(&self) -> Option<Color> {
        self.custom_color
    }

    #[must_use]
    pub fn custom_effects(&self) -> &[CustomPotionEffect] {
        &self.custom_effects
    }

    #[must_use]
    pub fn custom_name(&self) -> Option<&str> {
        self.custom_name.as_deref()
    }
}

impl DataComponentValue for PotionContents {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if let Some(potion) = &self.potion {
            compound.insert("potion".to_string(), Nbt::String(potion.to_string()));
        }
        if let Some(custom_color) = self.custom_color {
            compound.insert(
                "custom_color".to_string(),
                Nbt::Int(custom_color.as_rgb() as i32),
            );
        }
        if !self.custom_effects.is_empty() {
            compound.insert(
                "custom_effects".to_string(),
                self.custom_effects.to_component_nbt(),
            );
        }
        if let Some(custom_name) = &self.custom_name {
            compound.insert("custom_name".to_string(), Nbt::String(custom_name.clone()));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let potion = match compound.get("potion") {
            Some(_) => Some(string_field(compound, "potion")?.parse().ok()?),
            None => None,
        };
        let custom_color = match compound.get("custom_color") {
            Some(_) => Some(Color::from_rgb_int(
                i32_field_or(compound, "custom_color", 0)? as u32,
            )),
            None => None,
        };
        let custom_effects = match compound.get("custom_effects") {
            Some(custom_effects) => Vec::<CustomPotionEffect>::from_component_nbt(custom_effects)?,
            None => Vec::new(),
        };
        let custom_name = match compound.get("custom_name") {
            Some(_) => Some(string_field(compound, "custom_name")?),
            None => None,
        };
        Some(Self {
            potion,
            custom_color,
            custom_effects,
            custom_name,
        })
    }
}
