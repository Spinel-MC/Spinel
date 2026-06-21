use crate::Identifier;
use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{
    bool_field_or, compound_from_nbt, i32_field_or, string_field,
};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomPotionEffect {
    effect_id: Identifier,
    settings: PotionEffectSettings,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PotionEffectSettings {
    amplifier: i32,
    duration: i32,
    is_ambient: bool,
    show_particles: bool,
    show_icon: bool,
    hidden_effect: Option<Box<PotionEffectSettings>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuspiciousStewEffects {
    effects: Vec<SuspiciousStewEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuspiciousStewEffect {
    effect_id: Identifier,
    duration_ticks: i32,
}

impl CustomPotionEffect {
    #[must_use]
    pub fn new(effect_id: Identifier, settings: PotionEffectSettings) -> Self {
        Self {
            effect_id,
            settings,
        }
    }

    #[must_use]
    pub const fn effect_id(&self) -> &Identifier {
        &self.effect_id
    }

    #[must_use]
    pub const fn get_settings(&self) -> &PotionEffectSettings {
        &self.settings
    }
}

impl PotionEffectSettings {
    #[must_use]
    pub fn new(
        amplifier: i32,
        duration: i32,
        is_ambient: bool,
        show_particles: bool,
        show_icon: bool,
        hidden_effect: Option<PotionEffectSettings>,
    ) -> Self {
        Self {
            amplifier,
            duration,
            is_ambient,
            show_particles,
            show_icon,
            hidden_effect: hidden_effect.map(Box::new),
        }
    }

    #[must_use]
    pub const fn amplifier(&self) -> i32 {
        self.amplifier
    }

    #[must_use]
    pub const fn duration(&self) -> i32 {
        self.duration
    }

    #[must_use]
    pub const fn is_ambient(&self) -> bool {
        self.is_ambient
    }

    #[must_use]
    pub const fn show_particles(&self) -> bool {
        self.show_particles
    }

    #[must_use]
    pub const fn show_icon(&self) -> bool {
        self.show_icon
    }

    #[must_use]
    pub fn hidden_effect(&self) -> Option<&PotionEffectSettings> {
        self.hidden_effect.as_deref()
    }
}

impl SuspiciousStewEffects {
    #[must_use]
    pub fn new(effects: Vec<SuspiciousStewEffect>) -> Self {
        Self { effects }
    }

    #[must_use]
    pub fn effects(&self) -> &[SuspiciousStewEffect] {
        &self.effects
    }

    #[must_use]
    pub fn with(&self, effect: SuspiciousStewEffect) -> Self {
        let mut effects = self.effects.clone();
        effects.push(effect);
        Self { effects }
    }
}

impl SuspiciousStewEffect {
    #[must_use]
    pub fn new(effect_id: Identifier, duration_ticks: i32) -> Self {
        Self {
            effect_id,
            duration_ticks,
        }
    }

    #[must_use]
    pub const fn effect_id(&self) -> &Identifier {
        &self.effect_id
    }

    #[must_use]
    pub const fn duration_ticks(&self) -> i32 {
        self.duration_ticks
    }
}

impl DataComponentValue for CustomPotionEffect {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = match self.settings.to_component_nbt() {
            Nbt::Compound(compound) => compound,
            _ => NbtCompound::new(),
        };
        compound.insert("id".to_string(), Nbt::String(self.effect_id.to_string()));
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            effect_id: string_field(compound, "id")?.parse().ok()?,
            settings: PotionEffectSettings::from_component_nbt(component_nbt)?,
        })
    }
}

impl DataComponentValue for PotionEffectSettings {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.amplifier != 0 {
            compound.insert("amplifier".to_string(), Nbt::Byte(self.amplifier as i8));
        }
        if self.duration != 0 {
            compound.insert("duration".to_string(), Nbt::Int(self.duration));
        }
        if self.is_ambient {
            compound.insert("ambient".to_string(), Nbt::Byte(1));
        }
        if !self.show_particles {
            compound.insert("show_particles".to_string(), Nbt::Byte(0));
        }
        if self.show_icon != self.show_particles {
            compound.insert("show_icon".to_string(), Nbt::Byte(i8::from(self.show_icon)));
        }
        if let Some(hidden_effect) = &self.hidden_effect {
            compound.insert(
                "hidden_effect".to_string(),
                hidden_effect.to_component_nbt(),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let show_particles = bool_field_or(compound, "show_particles", true)?;
        let show_icon = bool_field_or(compound, "show_icon", show_particles)?;
        let hidden_effect = match compound.get("hidden_effect") {
            Some(hidden_effect) => Some(Self::from_component_nbt(hidden_effect)?),
            None => None,
        };
        Some(Self::new(
            i32_field_or(compound, "amplifier", 0)?,
            i32_field_or(compound, "duration", 0)?,
            bool_field_or(compound, "ambient", false)?,
            show_particles,
            show_icon,
            hidden_effect,
        ))
    }
}

impl DataComponentValue for Vec<CustomPotionEffect> {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.iter()
                .map(CustomPotionEffect::to_component_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(effects) => effects
                .iter()
                .map(CustomPotionEffect::from_component_nbt)
                .collect(),
            _ => None,
        }
    }
}

impl DataComponentValue for SuspiciousStewEffects {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.effects
                .iter()
                .map(SuspiciousStewEffect::to_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(effects) => effects
                .iter()
                .map(SuspiciousStewEffect::from_nbt)
                .collect::<Option<Vec<_>>>()
                .map(Self::new),
            _ => None,
        }
    }
}

impl SuspiciousStewEffect {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("id".to_string(), Nbt::String(self.effect_id.to_string()));
        if self.duration_ticks != 160 {
            compound.insert("duration".to_string(), Nbt::Int(self.duration_ticks));
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            effect_id: string_field(compound, "id")?.parse().ok()?,
            duration_ticks: i32_field_or(compound, "duration", 160)?,
        })
    }
}
