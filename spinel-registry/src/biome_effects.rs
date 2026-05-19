use crate::nbt_builder::{color_tag, put_optional};
use spinel_nbt::{Nbt, NbtCompound};
pub use spinel_utils::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct BiomeEffects {
    pub water_color: Option<Color>,
    pub foliage_color: Option<Color>,
    pub dry_foliage_color: Option<Color>,
    pub grass_color: Option<Color>,
    pub grass_color_modifier: GrassColorModifier,
}

impl BiomeEffects {
    pub(crate) fn nbt(&self) -> NbtCompound {
        let mut effects_nbt = NbtCompound::new();
        let water_color = self.water_color.unwrap_or(Color::from_rgb_int(0x3f76e4));
        effects_nbt.insert("water_color".to_string(), color_tag(water_color.as_rgb()));
        put_optional(
            &mut effects_nbt,
            "foliage_color",
            color_nbt(self.foliage_color),
        );
        put_optional(
            &mut effects_nbt,
            "dry_foliage_color",
            color_nbt(self.dry_foliage_color),
        );
        put_optional(&mut effects_nbt, "grass_color", color_nbt(self.grass_color));
        put_optional(
            &mut effects_nbt,
            "grass_color_modifier",
            self.grass_color_modifier.nbt(),
        );
        effects_nbt
    }
}

fn color_nbt(color: Option<Color>) -> Option<Nbt> {
    color.map(|biome_color| color_tag(biome_color.as_rgb()))
}

impl Default for BiomeEffects {
    fn default() -> Self {
        Self {
            water_color: Some(Color::from_rgb_int(0x3f76e4)),
            foliage_color: None,
            dry_foliage_color: None,
            grass_color: None,
            grass_color_modifier: GrassColorModifier::None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GrassColorModifier {
    #[default]
    None,
    DarkForest,
    Swamp,
}

impl GrassColorModifier {
    fn nbt(self) -> Option<Nbt> {
        match self {
            Self::None => None,
            Self::DarkForest => Some(Nbt::String("dark_forest".to_string())),
            Self::Swamp => Some(Nbt::String("swamp".to_string())),
        }
    }
}
