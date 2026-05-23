use crate::biome::{BiomeAttributes, BiomeEffects, Color};
use crate::registry::nbt_builder::{bool_tag, put_optional};
use crate::{Identifier, RegistryCodec};
use serde_json::Value;
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Debug, Clone)]
pub struct Biome {
    pub has_precipitation: bool,
    pub temperature: f32,
    pub downfall: f32,
    pub temperature_modifier: TemperatureModifier,
    pub attributes: BiomeAttributes,
    pub effects: BiomeEffects,
}

impl Biome {
    #[must_use]
    pub fn builder() -> BiomeBuilder {
        BiomeBuilder::new()
    }
}

impl RegistryCodec for Biome {
    fn registry_nbt(&self) -> NbtCompound {
        let mut biome_nbt = NbtCompound::new();
        biome_nbt.insert(
            "has_precipitation".to_string(),
            bool_tag(self.has_precipitation),
        );
        biome_nbt.insert("temperature".to_string(), Nbt::Float(self.temperature));
        put_optional(
            &mut biome_nbt,
            "temperature_modifier",
            self.temperature_modifier.nbt(),
        );
        biome_nbt.insert("downfall".to_string(), Nbt::Float(self.downfall));
        put_optional(&mut biome_nbt, "attributes", self.attributes.nbt());
        biome_nbt.insert("effects".to_string(), self.effects.nbt());
        biome_nbt
    }
}

pub struct BiomeBuilder {
    biome: Biome,
}

impl BiomeBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            biome: Biome::default(),
        }
    }

    pub fn precipitation(&mut self, has_precipitation: bool) -> &mut Self {
        self.biome.has_precipitation = has_precipitation;
        self
    }

    pub fn temperature(&mut self, temperature: f32) -> &mut Self {
        self.biome.temperature = temperature;
        self
    }

    pub fn downfall(&mut self, downfall: f32) -> &mut Self {
        self.biome.downfall = downfall;
        self
    }

    pub fn temperature_modifier(&mut self, temperature_modifier: TemperatureModifier) -> &mut Self {
        self.biome.temperature_modifier = temperature_modifier;
        self
    }

    pub fn attribute(&mut self, key: Identifier, attribute: Value) -> &mut Self {
        self.biome.attributes.push(key, attribute);
        self
    }

    pub fn effects(&mut self, effects: BiomeEffects) -> &mut Self {
        self.biome.effects = effects;
        self
    }

    pub fn water_color(&mut self, water_color: Color) -> &mut Self {
        self.biome.effects.water_color = Some(water_color);
        self
    }

    #[must_use]
    pub fn build(&self) -> Biome {
        self.biome.clone()
    }
}

impl Default for BiomeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Biome {
    fn default() -> Self {
        Self {
            has_precipitation: true,
            temperature: 0.8,
            downfall: 0.4,
            temperature_modifier: TemperatureModifier::None,
            attributes: BiomeAttributes::default(),
            effects: BiomeEffects::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureModifier {
    #[default]
    None,
    Frozen,
}

impl TemperatureModifier {
    fn nbt(self) -> Option<Nbt> {
        match self {
            Self::None => None,
            Self::Frozen => Some(Nbt::String("frozen".to_string())),
        }
    }
}
