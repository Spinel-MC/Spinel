use crate::Identifier;
use crate::data_components::nbt_reader::{compound_from_nbt, string_field};
use crate::data_components::{DataComponentValue, dye_color_from_nbt_name, dye_color_nbt_name};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::color::DyeColor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArmorTrim {
    material: Identifier,
    pattern: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstrumentComponent {
    instrument: Identifier,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BannerPatterns {
    layers: Vec<BannerPatternLayer>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BannerPatternLayer {
    pattern: Identifier,
    color: DyeColor,
}

impl ArmorTrim {
    #[must_use]
    pub const fn new(material: Identifier, pattern: Identifier) -> Self {
        Self { material, pattern }
    }

    #[must_use]
    pub const fn material(&self) -> &Identifier {
        &self.material
    }

    #[must_use]
    pub const fn pattern(&self) -> &Identifier {
        &self.pattern
    }
}

impl InstrumentComponent {
    #[must_use]
    pub const fn new(instrument: Identifier) -> Self {
        Self { instrument }
    }

    #[must_use]
    pub const fn instrument(&self) -> &Identifier {
        &self.instrument
    }
}

impl BannerPatterns {
    #[must_use]
    pub fn new(layers: Vec<BannerPatternLayer>) -> Self {
        Self { layers }
    }

    #[must_use]
    pub fn layers(&self) -> &[BannerPatternLayer] {
        &self.layers
    }

    #[must_use]
    pub fn with(&self, layer: BannerPatternLayer) -> Self {
        let mut layers = self.layers.clone();
        layers.push(layer);
        Self { layers }
    }
}

impl BannerPatternLayer {
    #[must_use]
    pub const fn new(pattern: Identifier, color: DyeColor) -> Self {
        Self { pattern, color }
    }

    #[must_use]
    pub const fn pattern(&self) -> &Identifier {
        &self.pattern
    }

    #[must_use]
    pub const fn color(&self) -> DyeColor {
        self.color
    }
}

impl DataComponentValue for ArmorTrim {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "material".to_string(),
            Nbt::String(self.material.to_string()),
        );
        compound.insert("pattern".to_string(), Nbt::String(self.pattern.to_string()));
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            material: string_field(compound, "material")?.parse().ok()?,
            pattern: string_field(compound, "pattern")?.parse().ok()?,
        })
    }
}

impl DataComponentValue for InstrumentComponent {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.instrument.to_string())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(instrument) => Some(Self {
                instrument: instrument.parse().ok()?,
            }),
            Nbt::Compound(compound) => Some(Self {
                instrument: string_field(compound, "id")?.parse().ok()?,
            }),
            _ => None,
        }
    }
}

impl DataComponentValue for BannerPatterns {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.layers
                .iter()
                .map(BannerPatternLayer::to_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(layers) => layers
                .iter()
                .map(BannerPatternLayer::from_nbt)
                .collect::<Option<Vec<_>>>()
                .map(Self::new),
            _ => None,
        }
    }
}

impl BannerPatternLayer {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("pattern".to_string(), Nbt::String(self.pattern.to_string()));
        compound.insert(
            "color".to_string(),
            Nbt::String(dye_color_nbt_name(self.color).to_string()),
        );
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            pattern: string_field(compound, "pattern")?.parse().ok()?,
            color: dye_color_from_nbt_name(&string_field(compound, "color")?)?,
        })
    }
}
