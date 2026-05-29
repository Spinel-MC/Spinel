use crate::Identifier;
use crate::data_components::nbt_reader::{
    bool_field_or, compound_from_nbt, f32_field_or, string_field,
};
use crate::data_components::{ConsumeEffect, DataComponentValue};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct Consumable {
    consume_seconds: f32,
    animation: ItemAnimation,
    sound: Identifier,
    has_consume_particles: bool,
    effects: Vec<ConsumeEffect>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemAnimation {
    None,
    Eat,
    Drink,
    Block,
    Bow,
    Spear,
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
    Bundle,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeathProtection {
    death_effects: Vec<ConsumeEffect>,
}

impl Consumable {
    #[must_use]
    pub fn new(
        consume_seconds: f32,
        animation: ItemAnimation,
        sound: Identifier,
        has_consume_particles: bool,
        effects: Vec<ConsumeEffect>,
    ) -> Self {
        Self {
            consume_seconds,
            animation,
            sound,
            has_consume_particles,
            effects,
        }
    }

    #[must_use]
    pub const fn consume_seconds(&self) -> f32 {
        self.consume_seconds
    }

    #[must_use]
    pub const fn animation(&self) -> ItemAnimation {
        self.animation
    }

    #[must_use]
    pub const fn sound(&self) -> &Identifier {
        &self.sound
    }

    #[must_use]
    pub const fn has_consume_particles(&self) -> bool {
        self.has_consume_particles
    }

    #[must_use]
    pub fn effects(&self) -> &[ConsumeEffect] {
        &self.effects
    }

    #[must_use]
    pub fn consume_ticks(&self) -> i32 {
        (self.consume_seconds * 20.0) as i32
    }
}

impl DeathProtection {
    #[must_use]
    pub fn new(death_effects: Vec<ConsumeEffect>) -> Self {
        Self { death_effects }
    }

    #[must_use]
    pub fn death_effects(&self) -> &[ConsumeEffect] {
        &self.death_effects
    }
}

impl ItemAnimation {
    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Eat => "eat",
            Self::Drink => "drink",
            Self::Block => "block",
            Self::Bow => "bow",
            Self::Spear => "spear",
            Self::Crossbow => "crossbow",
            Self::Spyglass => "spyglass",
            Self::TootHorn => "toot_horn",
            Self::Brush => "brush",
            Self::Bundle => "bundle",
        }
    }

    #[must_use]
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Eat => 1,
            Self::Drink => 2,
            Self::Block => 3,
            Self::Bow => 4,
            Self::Spear => 5,
            Self::Crossbow => 6,
            Self::Spyglass => 7,
            Self::TootHorn => 8,
            Self::Brush => 9,
            Self::Bundle => 10,
        }
    }

    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name {
            "none" => Some(Self::None),
            "eat" => Some(Self::Eat),
            "drink" => Some(Self::Drink),
            "block" => Some(Self::Block),
            "bow" => Some(Self::Bow),
            "spear" => Some(Self::Spear),
            "crossbow" => Some(Self::Crossbow),
            "spyglass" => Some(Self::Spyglass),
            "toot_horn" => Some(Self::TootHorn),
            "brush" => Some(Self::Brush),
            "bundle" => Some(Self::Bundle),
            _ => None,
        }
    }
}

impl DataComponentValue for Consumable {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.consume_seconds != 1.6 {
            compound.insert(
                "consume_seconds".to_string(),
                Nbt::Float(self.consume_seconds),
            );
        }
        if self.animation != ItemAnimation::Eat {
            compound.insert(
                "animation".to_string(),
                Nbt::String(self.animation.nbt_name().to_string()),
            );
        }
        compound.insert("sound".to_string(), Nbt::String(self.sound.to_string()));
        if !self.has_consume_particles {
            compound.insert("has_consume_particles".to_string(), Nbt::Byte(0));
        }
        if !self.effects.is_empty() {
            compound.insert(
                "on_consume_effects".to_string(),
                self.effects.to_component_nbt(),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let animation = match compound.get("animation") {
            Some(_) => ItemAnimation::from_nbt_name(&string_field(compound, "animation")?)?,
            None => ItemAnimation::Eat,
        };
        let effects = match compound.get("on_consume_effects") {
            Some(effects) => Vec::<ConsumeEffect>::from_component_nbt(effects)?,
            None => Vec::new(),
        };
        Some(Self {
            consume_seconds: f32_field_or(compound, "consume_seconds", 1.6)?,
            animation,
            sound: string_field(compound, "sound")
                .and_then(|sound| sound.parse().ok())
                .unwrap_or_else(|| Identifier::vanilla_static("entity.generic.eat")),
            has_consume_particles: bool_field_or(compound, "has_consume_particles", true)?,
            effects,
        })
    }
}

impl DataComponentValue for DeathProtection {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if !self.death_effects.is_empty() {
            compound.insert(
                "death_effects".to_string(),
                self.death_effects.to_component_nbt(),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let death_effects = match compound.get("death_effects") {
            Some(death_effects) => Vec::<ConsumeEffect>::from_component_nbt(death_effects)?,
            None => Vec::new(),
        };
        Some(Self { death_effects })
    }
}
