use crate::biome::{Biome, Color};
use crate::mob_effect::MobEffect;
use crate::{
    BIOME_REGISTRY, DynamicRegistry, ENCHANTMENT_REGISTRY, Identifier, RegisterError, Registries,
    ZOMBIE_NAUTILUS_VARIANT_REGISTRY,
};
use spinel_nbt::parse_snbt_compound;
use std::{collections::BTreeMap, path::Path};

#[test]
fn custom_biome_packet_entries_include_local_data() {
    let mut biome_registry = DynamicRegistry::new(BIOME_REGISTRY);
    let _plains = biome_registry.register_vanilla(Biome::PLAINS, Biome::default());
    let custom_biome_key = Identifier::minecraft("custom_biome");
    let custom_biome = Biome::builder()
        .water_color(Color::from_rgb_int(0x00ff00))
        .build();
    let _custom_biome = biome_registry.register(custom_biome_key, custom_biome);
    let packet_entries = biome_registry.registry_packet_entries(true);

    assert!(packet_entries[0].1.is_none());
    assert!(packet_entries[1].1.is_some());
}

#[test]
fn biome_builder_supports_dynamic_mutation_and_chaining() {
    let mut builder = Biome::builder();

    builder.temperature(0.2);
    let biome = builder
        .downfall(0.7)
        .water_color(Color::from_rgb(1, 2, 3))
        .build();

    assert_eq!(biome.temperature, 0.2);
    assert_eq!(biome.downfall, 0.7);
    assert_eq!(biome.effects.water_color, Some(Color::from_rgb(1, 2, 3)));
}

#[test]
fn duplicate_keys_fail() {
    let mut biome_registry = DynamicRegistry::new(BIOME_REGISTRY);
    let custom_biome_key = Identifier::minecraft("custom_biome");
    let _first_biome = biome_registry.register(custom_biome_key.clone(), Biome::default());
    let duplicate_biome = biome_registry.register(custom_biome_key, Biome::default());

    assert!(matches!(duplicate_biome, Err(RegisterError::DuplicateKey)));
}

#[test]
fn frozen_registry_rejects_registration() {
    let mut biome_registry = DynamicRegistry::new(BIOME_REGISTRY);
    biome_registry.freeze();
    let custom_biome =
        biome_registry.register(Identifier::minecraft("custom_biome"), Biome::default());

    assert!(matches!(custom_biome, Err(RegisterError::Frozen)));
}

#[test]
fn zombie_nautilus_variant_registry_is_not_empty() {
    let registries = Registries::new_vanilla();
    let registry_entries = registries.dynamic_registry_entries(true);
    let zombie_nautilus_registry = registry_entries
        .iter()
        .find(|(registry_id, _entries)| *registry_id == ZOMBIE_NAUTILUS_VARIANT_REGISTRY);

    assert!(zombie_nautilus_registry.is_some_and(|(_registry_id, entries)| !entries.is_empty()));
}

#[test]
fn enchantment_exclusive_set_tags_are_bound() {
    let registries = Registries::new_vanilla();
    let registry_tags = match registries.dynamic_tag_entries() {
        Ok(registry_tags) => registry_tags,
        Err(error) => panic!("registry tags failed to resolve: {error:?}"),
    };
    let enchantment_tags = registry_tags
        .iter()
        .find(|registry_tags| registry_tags.registry_name == ENCHANTMENT_REGISTRY);
    let required_tags = [
        "minecraft:exclusive_set/armor",
        "minecraft:exclusive_set/boots",
        "minecraft:exclusive_set/bow",
        "minecraft:exclusive_set/crossbow",
        "minecraft:exclusive_set/damage",
        "minecraft:exclusive_set/mining",
        "minecraft:exclusive_set/riptide",
    ];

    assert!(required_tags.iter().all(|required_tag| {
        enchantment_tags.is_some_and(|registry_tags| {
            registry_tags
                .tags
                .iter()
                .any(|tag| tag.tag_name.to_string() == *required_tag && !tag.entries.is_empty())
        })
    }));
}

#[test]
fn vanilla_mob_effect_registry_entries_preserve_extracted_protocol_metadata() {
    let registries = Registries::new_vanilla();
    let speed = registries
        .mob_effect(&MobEffect::SPEED)
        .expect("extracted speed effect should be registered");

    assert_eq!(speed.get_protocol_id(), 0);
    assert_eq!(speed.get_translation_key(), "effect.minecraft.speed");
    assert_eq!(speed.get_color(), 3402751);
    assert!(!speed.is_instantaneous());
}
#[test]
fn vanilla_enchantment_registry_packets_preserve_generated_payloads() {
    let registries = Registries::new_vanilla();
    let registry_entries = registries.dynamic_registry_entries(false);
    let generated_entries: BTreeMap<String, String> =
        serde_json::from_str(include_str!("../../../assets/enchantments.json"))
            .expect("generated enchantment JSON should decode");
    let expected_bane_of_arthropods = parse_snbt_compound(
        generated_entries
            .get("minecraft:bane_of_arthropods")
            .expect("bane of arthropods should be generated"),
    )
    .expect("generated bane of arthropods SNBT should decode");
    let enchantment_entries = registry_entries
        .iter()
        .find(|(registry_id, _entries)| *registry_id == ENCHANTMENT_REGISTRY)
        .map(|(_registry_id, entries)| entries)
        .expect("enchantment registry packet entries should exist");
    let (_, actual_bane_of_arthropods) = enchantment_entries
        .iter()
        .find(|(entry_key, _payload)| *entry_key == Identifier::minecraft("bane_of_arthropods"))
        .expect("bane of arthropods should be present in the registry packet");

    assert_eq!(enchantment_entries.len(), generated_entries.len());
    assert_eq!(
        actual_bane_of_arthropods.as_ref(),
        Some(&expected_bane_of_arthropods)
    );
    assert!(expected_bane_of_arthropods.contains_key("effects"));
}
#[test]
fn registry_build_assets_do_not_depend_on_vanilla_datapack_folder() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert!(
        manifest_dir
            .join("assets/dynamic_registry_keys.json")
            .exists()
    );
    assert!(
        manifest_dir
            .join("assets/dynamic_registry_tags.json")
            .exists()
    );
    assert!(
        !manifest_dir
            .join("assets")
            .join(["data", "packs"].concat())
            .exists()
    );
}
