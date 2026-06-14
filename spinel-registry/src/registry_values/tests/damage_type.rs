use super::super::damage_type::{DamageEffects, DamageScaling, DamageType, DeathMessageType};
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

#[test]
fn damage_type_builder_supports_runtime_message_ids_and_minestom_defaults() {
    let message_id = String::from("customRuntimeDamage");
    let damage_type = DamageType::builder()
        .message_id(message_id)
        .scaling(DamageScaling::Never)
        .build()
        .unwrap();

    assert_eq!(damage_type.message_id(), "customRuntimeDamage");
    assert_eq!(damage_type.scaling(), DamageScaling::Never);
    assert_eq!(damage_type.exhaustion(), 0.0);
    assert_eq!(damage_type.effects(), DamageEffects::Hurt);
    assert_eq!(damage_type.death_message_type(), DeathMessageType::Default);
}

#[test]
fn damage_type_builder_rejects_missing_required_fields() {
    assert!(
        DamageType::builder()
            .scaling(DamageScaling::Never)
            .build()
            .is_err()
    );
    assert!(
        DamageType::builder()
            .message_id("missingScaling")
            .build()
            .is_err()
    );
}
