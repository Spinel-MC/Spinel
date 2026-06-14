use crate::RegistryCodec;
use spinel_nbt::{Nbt, NbtCompound};
use std::borrow::Cow;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct DamageType {
    message_id: Cow<'static, str>,
    scaling: DamageScaling,
    exhaustion: f32,
    effects: DamageEffects,
    death_message_type: DeathMessageType,
}

#[derive(Default)]
pub struct DamageTypeBuilder {
    message_id: Option<Cow<'static, str>>,
    scaling: Option<DamageScaling>,
    exhaustion: f32,
    effects: DamageEffects,
    death_message_type: DeathMessageType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageScaling {
    Always,
    WhenCausedByLivingNonPlayer,
    Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageEffects {
    Hurt,
    Thorns,
    Drowning,
    Burning,
    Poking,
    Freezing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeathMessageType {
    Default,
    FallVariants,
    IntentionalGameDesign,
}

impl DamageType {
    pub fn new(
        message_id: impl Into<Cow<'static, str>>,
        scaling: DamageScaling,
        exhaustion: f32,
        effects: DamageEffects,
        death_message_type: DeathMessageType,
    ) -> Self {
        Self {
            message_id: message_id.into(),
            scaling,
            exhaustion,
            effects,
            death_message_type,
        }
    }

    pub fn builder() -> DamageTypeBuilder {
        DamageTypeBuilder::default()
    }

    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    pub const fn scaling(&self) -> DamageScaling {
        self.scaling
    }

    pub const fn exhaustion(&self) -> f32 {
        self.exhaustion
    }

    pub const fn effects(&self) -> DamageEffects {
        self.effects
    }

    pub const fn death_message_type(&self) -> DeathMessageType {
        self.death_message_type
    }
}

impl DamageTypeBuilder {
    pub fn message_id(mut self, message_id: impl Into<Cow<'static, str>>) -> Self {
        self.message_id = Some(message_id.into());
        self
    }

    pub const fn scaling(mut self, scaling: DamageScaling) -> Self {
        self.scaling = Some(scaling);
        self
    }

    pub const fn exhaustion(mut self, exhaustion: f32) -> Self {
        self.exhaustion = exhaustion;
        self
    }

    pub const fn effects(mut self, effects: DamageEffects) -> Self {
        self.effects = effects;
        self
    }

    pub const fn death_message_type(mut self, death_message_type: DeathMessageType) -> Self {
        self.death_message_type = death_message_type;
        self
    }

    pub fn build(self) -> Result<DamageType> {
        let message_id = self
            .message_id
            .filter(|message_id| !message_id.is_empty())
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "missing message id"))?;
        let scaling = self
            .scaling
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "missing scaling"))?;
        Ok(DamageType::new(
            message_id,
            scaling,
            self.exhaustion,
            self.effects,
            self.death_message_type,
        ))
    }
}

impl Default for DamageEffects {
    fn default() -> Self {
        Self::Hurt
    }
}

impl Default for DeathMessageType {
    fn default() -> Self {
        Self::Default
    }
}

impl RegistryCodec for DamageType {
    fn registry_nbt(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert(
            "message_id".to_owned(),
            Nbt::String(self.message_id.to_string()),
        );
        compound.insert(
            "scaling".to_owned(),
            Nbt::String(self.scaling.nbt_name().to_owned()),
        );
        compound.insert("exhaustion".to_owned(), Nbt::Float(self.exhaustion));
        if self.effects != DamageEffects::Hurt {
            compound.insert(
                "effects".to_owned(),
                Nbt::String(self.effects.nbt_name().to_owned()),
            );
        }
        if self.death_message_type != DeathMessageType::Default {
            compound.insert(
                "death_message_type".to_owned(),
                Nbt::String(self.death_message_type.nbt_name().to_owned()),
            );
        }
        compound
    }
}

impl DamageScaling {
    const fn nbt_name(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::WhenCausedByLivingNonPlayer => "when_caused_by_living_non_player",
            Self::Never => "never",
        }
    }
}

impl DamageEffects {
    const fn nbt_name(self) -> &'static str {
        match self {
            Self::Hurt => "hurt",
            Self::Thorns => "thorns",
            Self::Drowning => "drowning",
            Self::Burning => "burning",
            Self::Poking => "poking",
            Self::Freezing => "freezing",
        }
    }
}

impl DeathMessageType {
    const fn nbt_name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::FallVariants => "fall_variants",
            Self::IntentionalGameDesign => "intentional_game_design",
        }
    }
}
