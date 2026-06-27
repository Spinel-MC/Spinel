use super::super::potion_contents::PotionContents;
use super::super::potion_effect::{
    CustomPotionEffect, PotionEffectSettings, SuspiciousStewEffect, SuspiciousStewEffects,
};
use crate::{DataComponentValue, Identifier};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::color::Color;

#[test]
fn potion_effect_settings_preserve_minestom_defaults_and_hidden_effects() {
    let default_settings = PotionEffectSettings::new(0, 0, false, true, true, None);
    let Nbt::Compound(default_compound) = default_settings.to_component_nbt() else {
        panic!("expected settings compound");
    };
    assert!(default_compound.is_empty());
    assert_eq!(
        PotionEffectSettings::from_component_nbt(&Nbt::Compound(default_compound)),
        Some(default_settings)
    );

    let hidden_effect = PotionEffectSettings::new(1, 40, true, false, false, None);
    let settings = PotionEffectSettings::new(2, 80, true, false, true, Some(hidden_effect));
    let decoded = PotionEffectSettings::from_component_nbt(&settings.to_component_nbt()).unwrap();
    assert_eq!(decoded, settings);
    assert!(decoded.hidden_effect().is_some());
}

#[test]
fn custom_potion_effect_round_trips_identifier_and_settings() {
    let effect = CustomPotionEffect::new(
        Identifier::minecraft("speed"),
        PotionEffectSettings::new(1, 200, true, true, false, None),
    );

    assert_eq!(
        CustomPotionEffect::from_component_nbt(&effect.to_component_nbt()),
        Some(effect)
    );

    let mut missing_id = NbtCompound::new();
    missing_id.insert("duration".to_string(), Nbt::Int(20));
    assert_eq!(
        CustomPotionEffect::from_component_nbt(&Nbt::Compound(missing_id)),
        None
    );
}

#[test]
fn potion_contents_preserves_custom_effects_color_name_and_empty_defaults() {
    let empty = PotionContents::new(None, None, Vec::new(), None);
    let Nbt::Compound(empty_compound) = empty.to_component_nbt() else {
        panic!("expected potion contents compound");
    };
    assert!(empty_compound.is_empty());
    assert_eq!(
        PotionContents::from_component_nbt(&Nbt::Compound(empty_compound)),
        Some(empty)
    );

    let effect = CustomPotionEffect::new(
        Identifier::minecraft("night_vision"),
        PotionEffectSettings::new(0, 600, false, true, true, None),
    );
    let contents = PotionContents::new(
        Some(Identifier::minecraft("strong_healing")),
        Some(Color::from_rgb_int(0x3366ff)),
        vec![effect],
        Some("healing".to_string()),
    );

    assert_eq!(
        PotionContents::from_component_nbt(&contents.to_component_nbt()),
        Some(contents)
    );
}

#[test]
fn suspicious_stew_effects_preserve_default_duration_and_immutable_with() {
    let blindness = SuspiciousStewEffect::new(Identifier::minecraft("blindness"), 160);
    let base_effects = SuspiciousStewEffects::new(vec![blindness.clone()]);
    let Nbt::List(encoded_effects) = base_effects.to_component_nbt() else {
        panic!("expected stew effect list");
    };
    let Nbt::Compound(blindness_compound) = &encoded_effects[0] else {
        panic!("expected stew effect compound");
    };
    assert!(!blindness_compound.contains_key("duration"));
    assert_eq!(
        SuspiciousStewEffects::from_component_nbt(&Nbt::List(encoded_effects)),
        Some(base_effects.clone())
    );

    let with_speed = base_effects.with(SuspiciousStewEffect::new(
        Identifier::minecraft("speed"),
        100,
    ));
    assert_eq!(base_effects.effects().len(), 1);
    assert_eq!(with_speed.effects().len(), 2);
    assert_eq!(
        SuspiciousStewEffects::from_component_nbt(&with_speed.to_component_nbt()),
        Some(with_speed)
    );
}
