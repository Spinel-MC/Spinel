use super::super::consume_effect::ConsumeEffect;
use super::super::potion_effect::{CustomPotionEffect, PotionEffectSettings};
use crate::{DataComponentValue, Identifier, RegistryTagReference};
use spinel_nbt::{Nbt, NbtCompound};

#[test]
fn consume_effect_apply_effects_preserves_custom_effects_and_probability_default() {
    let effect = CustomPotionEffect::new(
        Identifier::minecraft("haste"),
        PotionEffectSettings::new(2, 80, true, false, true, None),
    );
    let consume_effect = ConsumeEffect::ApplyEffects {
        effects: vec![effect.clone()],
        probability: 1.0,
    };

    let encoded = consume_effect.to_component_nbt();
    let Nbt::Compound(compound) = &encoded else {
        panic!("expected consume effect compound");
    };

    assert!(!compound.contains_key("probability"));
    assert_eq!(
        ConsumeEffect::from_component_nbt(&encoded),
        Some(consume_effect)
    );

    let explicit_probability = ConsumeEffect::ApplyEffects {
        effects: vec![effect],
        probability: 0.25,
    };
    assert_eq!(
        ConsumeEffect::from_component_nbt(&explicit_probability.to_component_nbt()),
        Some(explicit_probability)
    );
}

#[test]
fn consume_effect_remove_clear_sound_and_teleport_round_trip() {
    let effects = RegistryTagReference::direct(vec![Identifier::minecraft("poison")]);
    let remove_effects = ConsumeEffect::RemoveEffects {
        effects: effects.clone(),
    };
    assert_eq!(
        ConsumeEffect::from_component_nbt(&remove_effects.to_component_nbt()),
        Some(remove_effects)
    );

    assert_eq!(
        ConsumeEffect::from_component_nbt(&ConsumeEffect::ClearAllEffects.to_component_nbt()),
        Some(ConsumeEffect::ClearAllEffects)
    );

    let teleport = ConsumeEffect::TeleportRandomly { diameter: 16.0 };
    let Nbt::Compound(teleport_compound) = teleport.to_component_nbt() else {
        panic!("expected teleport consume effect compound");
    };
    assert!(!teleport_compound.contains_key("diameter"));
    assert_eq!(
        ConsumeEffect::from_component_nbt(&Nbt::Compound(teleport_compound)),
        Some(teleport)
    );

    let sound = ConsumeEffect::PlaySound {
        sound: Identifier::minecraft("entity.generic.drink"),
    };
    assert_eq!(
        ConsumeEffect::from_component_nbt(&sound.to_component_nbt()),
        Some(sound)
    );
}

#[test]
fn consume_effect_rejects_unknown_type_and_missing_required_payloads() {
    let mut unknown = NbtCompound::new();
    unknown.insert("type".to_string(), Nbt::String("explode".to_string()));
    assert_eq!(
        ConsumeEffect::from_component_nbt(&Nbt::Compound(unknown)),
        None
    );

    let mut missing_effects = NbtCompound::new();
    missing_effects.insert("type".to_string(), Nbt::String("apply_effects".to_string()));
    assert_eq!(
        ConsumeEffect::from_component_nbt(&Nbt::Compound(missing_effects)),
        None
    );

    let mut missing_sound = NbtCompound::new();
    missing_sound.insert("type".to_string(), Nbt::String("play_sound".to_string()));
    assert_eq!(
        ConsumeEffect::from_component_nbt(&Nbt::Compound(missing_sound)),
        None
    );
}
