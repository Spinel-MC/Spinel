use super::super::enchantment::{Enchantment, EnchantmentCost};
use crate::data_components::vanilla_components::CUSTOM_NAME;
use crate::{
    DataComponentMap, EquipmentSlotGroup, Identifier, RegistryCodec, RegistryTagReference,
};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::component::text::TextComponent;

#[test]
fn enchantment_builder_preserves_minestom_registry_payload_fields() {
    let effects = DataComponentMap::new().with(CUSTOM_NAME, TextComponent::literal("effect"));
    let enchantment = Enchantment::builder()
        .description(TextComponent::literal("Test Enchantment"))
        .exclusive_set(RegistryTagReference::backed(Identifier::minecraft(
            "exclusive_set/weapon",
        )))
        .supported_items(RegistryTagReference::backed(Identifier::minecraft(
            "swords",
        )))
        .primary_items(RegistryTagReference::direct(vec![Identifier::minecraft(
            "diamond_sword",
        )]))
        .weight(7)
        .max_level(5)
        .min_cost_values(3, 8)
        .max_cost(EnchantmentCost::new(15, 12))
        .anvil_cost(4)
        .slots(vec![
            EquipmentSlotGroup::MainHand,
            EquipmentSlotGroup::OffHand,
        ])
        .effects(effects.clone())
        .build();

    assert_eq!(
        enchantment.get_description().to_plain_string(),
        "Test Enchantment"
    );
    assert_eq!(
        enchantment.get_exclusive_set(),
        &RegistryTagReference::backed(Identifier::minecraft("exclusive_set/weapon"))
    );
    assert_eq!(
        enchantment.get_supported_items(),
        &RegistryTagReference::backed(Identifier::minecraft("swords"))
    );
    assert_eq!(
        enchantment.get_primary_items(),
        Some(&RegistryTagReference::direct(vec![Identifier::minecraft(
            "diamond_sword"
        )]))
    );
    assert_eq!(enchantment.get_weight(), 7);
    assert_eq!(enchantment.get_max_level(), 5);
    assert_eq!(enchantment.get_min_cost(), EnchantmentCost::new(3, 8));
    assert_eq!(enchantment.get_max_cost(), EnchantmentCost::new(15, 12));
    assert_eq!(enchantment.get_anvil_cost(), 4);
    assert_eq!(
        enchantment.get_slots(),
        &[EquipmentSlotGroup::MainHand, EquipmentSlotGroup::OffHand]
    );
    assert_eq!(enchantment.get_effects(), &effects);
}

#[test]
fn enchantment_registry_nbt_matches_minestom_field_names_and_defaults() {
    let enchantment = Enchantment::builder()
        .description(TextComponent::literal("Sharpness"))
        .supported_items(RegistryTagReference::direct(vec![Identifier::minecraft(
            "diamond_sword",
        )]))
        .primary_items(RegistryTagReference::direct(vec![Identifier::minecraft(
            "netherite_sword",
        )]))
        .weight(10)
        .max_level(5)
        .min_cost_values(1, 11)
        .max_cost_values(21, 11)
        .anvil_cost(1)
        .slots(vec![EquipmentSlotGroup::MainHand])
        .build();
    let registry_nbt = enchantment.registry_nbt();

    assert!(matches!(
        registry_nbt.get("description"),
        Some(Nbt::Compound(_))
    ));
    assert_eq!(
        registry_nbt.get("supported_items"),
        Some(&Nbt::List(
            vec![Nbt::String("minecraft:diamond_sword".to_owned())].into_boxed_slice()
        ))
    );
    assert_eq!(
        registry_nbt.get("primary_items"),
        Some(&Nbt::List(
            vec![Nbt::String("minecraft:netherite_sword".to_owned())].into_boxed_slice()
        ))
    );
    assert_eq!(registry_nbt.get("weight"), Some(&Nbt::Int(10)));
    assert_eq!(registry_nbt.get("max_level"), Some(&Nbt::Int(5)));
    assert_eq!(registry_nbt.get("anvil_cost"), Some(&Nbt::Int(1)));
    assert_eq!(
        registry_nbt.get("slots"),
        Some(&Nbt::List(
            vec![Nbt::String("mainhand".to_owned())].into_boxed_slice()
        ))
    );

    let Some(Nbt::Compound(min_cost)) = registry_nbt.get("min_cost") else {
        panic!("missing min_cost compound");
    };
    assert_eq!(min_cost.get("base"), Some(&Nbt::Int(1)));
    assert_eq!(min_cost.get("per_level_above_first"), Some(&Nbt::Int(11)));

    let default_enchantment = Enchantment::default();
    assert_eq!(default_enchantment.get_weight(), 1);
    assert_eq!(default_enchantment.get_max_level(), 1);
    assert_eq!(
        default_enchantment.get_min_cost(),
        EnchantmentCost::new(1, 1)
    );
    assert_eq!(
        default_enchantment.get_max_cost(),
        EnchantmentCost::new(1, 1)
    );
    assert_eq!(default_enchantment.get_anvil_cost(), 0);
    assert!(default_enchantment.get_slots().is_empty());
    assert!(default_enchantment.get_effects().is_empty());
}

#[test]
fn enchantment_registry_nbt_includes_effect_component_map_when_present() {
    let enchantment = Enchantment::builder()
        .supported_items(RegistryTagReference::empty())
        .effects(DataComponentMap::new().with(CUSTOM_NAME, TextComponent::literal("effect")))
        .build();
    let registry_nbt = enchantment.registry_nbt();

    let Some(Nbt::Compound(effects)) = registry_nbt.get("effects") else {
        panic!("missing effects compound");
    };
    assert!(effects.get("minecraft:custom_name").is_some());
}

#[test]
fn enchantment_raw_registry_nbt_preserves_generated_payload_escape_hatch() {
    let mut raw_payload = NbtCompound::new();
    raw_payload.insert("weight".to_owned(), Nbt::Int(99));

    assert_eq!(
        Enchantment::raw(raw_payload.clone()).registry_nbt(),
        raw_payload
    );
}
