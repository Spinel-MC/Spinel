use super::super::registry_cache::*;
use spinel_registry::Registries;

#[test]
fn tag_packet_includes_enchantment_exclusive_sets() {
    let registries = Registries::new_vanilla();
    let registry_cache = RegistryCache::new(&registries);
    let enchantment_tags = registry_cache
        .tag_packet()
        .registries
        .0
        .iter()
        .find(|registry| registry.registry_name == "minecraft:enchantment");
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
        enchantment_tags.is_some_and(|registry| {
            registry
                .tags
                .0
                .iter()
                .any(|tag| tag.tag_name == *required_tag && !tag.entries.0.is_empty())
        })
    }));
}

#[test]
fn tag_packet_includes_static_item_tags_used_by_enchantments() {
    let registries = Registries::new_vanilla();
    let registry_cache = RegistryCache::new(&registries);
    let item_tags = registry_cache
        .tag_packet()
        .registries
        .0
        .iter()
        .find(|registry| registry.registry_name == "minecraft:item");
    let required_tags = [
        "minecraft:enchantable/weapon",
        "minecraft:swords",
        "minecraft:axes",
    ];

    assert!(required_tags.iter().all(|required_tag| {
        item_tags.is_some_and(|registry| {
            registry
                .tags
                .0
                .iter()
                .any(|tag| tag.tag_name == *required_tag && !tag.entries.0.is_empty())
        })
    }));
}

#[test]
fn tag_packet_includes_static_entity_type_tags_used_by_enchantments() {
    let registries = Registries::new_vanilla();
    let registry_cache = RegistryCache::new(&registries);
    let entity_type_tags = registry_cache
        .tag_packet()
        .registries
        .0
        .iter()
        .find(|registry| registry.registry_name == "minecraft:entity_type");
    let required_tags = [
        "minecraft:arrows",
        "minecraft:sensitive_to_bane_of_arthropods",
        "minecraft:sensitive_to_impaling",
        "minecraft:sensitive_to_smite",
    ];

    assert!(required_tags.iter().all(|required_tag| {
        entity_type_tags.is_some_and(|registry| {
            registry
                .tags
                .0
                .iter()
                .any(|tag| tag.tag_name == *required_tag && !tag.entries.0.is_empty())
        })
    }));
}

#[test]
fn tag_packet_includes_static_block_tags_used_by_registries() {
    let registries = Registries::new_vanilla();
    let registry_cache = RegistryCache::new(&registries);
    let block_tags = registry_cache
        .tag_packet()
        .registries
        .0
        .iter()
        .find(|registry| registry.registry_name == "minecraft:block");
    let required_tags = [
        "minecraft:infiniburn_overworld",
        "minecraft:infiniburn_nether",
        "minecraft:base_stone_overworld",
    ];

    assert!(required_tags.iter().all(|required_tag| {
        block_tags.is_some_and(|registry| {
            registry
                .tags
                .0
                .iter()
                .any(|tag| tag.tag_name == *required_tag && !tag.entries.0.is_empty())
        })
    }));
}
