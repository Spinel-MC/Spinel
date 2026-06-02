use crate::RegistryCodec;
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct DamageType {
    message_id: &'static str,
    scaling: DamageScaling,
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
    pub const fn new(
        message_id: &'static str,
        scaling: DamageScaling,
        exhaustion: f32,
        effects: DamageEffects,
        death_message_type: DeathMessageType,
    ) -> Self {
        Self {
            message_id,
            scaling,
            exhaustion,
            effects,
            death_message_type,
        }
    }

    pub const fn message_id(&self) -> &'static str {
        self.message_id
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

impl RegistryCodec for DamageType {
    fn registry_nbt(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert(
            "message_id".to_owned(),
            Nbt::String(self.message_id.to_owned()),
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

#[cfg(test)]
mod tests {
    use super::{DamageEffects, DamageScaling, DamageType, DeathMessageType};
    use crate::{Registries, RegistryCodec};
    use spinel_nbt::Nbt;

    #[test]
    fn generated_vanilla_damage_types_preserve_datapack_metadata() {
        let registries = Registries::new_vanilla();
        let damage_type = registries
            .damage_type(&DamageType::BAD_RESPAWN_POINT)
            .unwrap();

        assert_eq!(damage_type.message_id(), "badRespawnPoint");
        assert_eq!(damage_type.scaling(), DamageScaling::Always);
        assert_eq!(damage_type.exhaustion(), 0.1);
        assert_eq!(damage_type.effects(), DamageEffects::Hurt);
        assert_eq!(
            damage_type.death_message_type(),
            DeathMessageType::IntentionalGameDesign
        );
        assert_eq!(
            damage_type.registry_nbt().get("death_message_type"),
            Some(&Nbt::String("intentional_game_design".to_owned()))
        );
    }

    #[test]
    fn generated_vanilla_damage_types_preserve_non_default_effect_categories() {
        let registries = Registries::new_vanilla();
        let damage_type = registries.damage_type(&DamageType::ON_FIRE).unwrap();

        assert_eq!(damage_type.message_id(), "onFire");
        assert_eq!(damage_type.effects(), DamageEffects::Burning);
        assert_eq!(
            damage_type.registry_nbt().get("effects"),
            Some(&Nbt::String("burning".to_owned()))
        );
    }
}
