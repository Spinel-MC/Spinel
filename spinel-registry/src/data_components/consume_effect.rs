use crate::Identifier;
use crate::data_components::nbt_reader::{compound_from_nbt, f32_field_or, string_field};
use crate::data_components::{CustomPotionEffect, DataComponentValue, RegistryTagReference};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub enum ConsumeEffect {
    ApplyEffects {
        effects: Vec<CustomPotionEffect>,
        probability: f32,
    },
    RemoveEffects {
        effects: RegistryTagReference,
    },
    ClearAllEffects,
    TeleportRandomly {
        diameter: f32,
    },
    PlaySound {
        sound: Identifier,
    },
}

impl ConsumeEffect {
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::ApplyEffects { .. } => "apply_effects",
            Self::RemoveEffects { .. } => "remove_effects",
            Self::ClearAllEffects => "clear_all_effects",
            Self::TeleportRandomly { .. } => "teleport_randomly",
            Self::PlaySound { .. } => "play_sound",
        }
    }
}

impl DataComponentValue for ConsumeEffect {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "type".to_string(),
            Nbt::String(self.type_name().to_string()),
        );
        match self {
            Self::ApplyEffects {
                effects,
                probability,
            } => {
                compound.insert("effects".to_string(), effects.to_component_nbt());
                if *probability != 1.0 {
                    compound.insert("probability".to_string(), Nbt::Float(*probability));
                }
            }
            Self::RemoveEffects { effects } => {
                compound.insert("effects".to_string(), effects.to_nbt());
            }
            Self::ClearAllEffects => {}
            Self::TeleportRandomly { diameter } => {
                if *diameter != 16.0 {
                    compound.insert("diameter".to_string(), Nbt::Float(*diameter));
                }
            }
            Self::PlaySound { sound } => {
                compound.insert("sound".to_string(), Nbt::String(sound.to_string()));
            }
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        match string_field(compound, "type")?.as_str() {
            "apply_effects" => Some(Self::ApplyEffects {
                effects: Vec::<CustomPotionEffect>::from_component_nbt(compound.get("effects")?)?,
                probability: f32_field_or(compound, "probability", 1.0)?,
            }),
            "remove_effects" => Some(Self::RemoveEffects {
                effects: RegistryTagReference::from_nbt(compound.get("effects")?)?,
            }),
            "clear_all_effects" => Some(Self::ClearAllEffects),
            "teleport_randomly" => Some(Self::TeleportRandomly {
                diameter: f32_field_or(compound, "diameter", 16.0)?,
            }),
            "play_sound" => Some(Self::PlaySound {
                sound: string_field(compound, "sound")?.parse().ok()?,
            }),
            _ => None,
        }
    }
}

impl DataComponentValue for Vec<ConsumeEffect> {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.iter()
                .map(ConsumeEffect::to_component_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(effects) => effects
                .iter()
                .map(ConsumeEffect::from_component_nbt)
                .collect(),
            _ => None,
        }
    }
}
