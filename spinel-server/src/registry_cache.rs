use ::spinel_core::network::clientbound::configuration::registry_data::RegistryDataPacket;
use ::spinel_core::network::clientbound::configuration::update_tags::{
    Tag, TagRegistry, UpdateTagsPacket,
};
use spinel_network::types::Array;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_registry::{EntityType, Identifier, Registries, RegistryTag, RegistryTags};
use std::sync::Arc;

pub struct RegistryCache {
    pub registry_packets: Arc<Vec<RegistryDataPacket>>,
    tag_packet: Arc<UpdateTagsPacket>,
}

impl RegistryCache {
    pub fn new(registries: &Registries) -> Self {
        Self {
            registry_packets: Arc::new(Self::registry_packets(registries)),
            tag_packet: Arc::new(Self::build_tag_packet(registries)),
        }
    }

    pub fn packets(&self) -> &[RegistryDataPacket] {
        &self.registry_packets
    }

    pub fn tag_packet(&self) -> &UpdateTagsPacket {
        &self.tag_packet
    }

    fn registry_packets(registries: &Registries) -> Vec<RegistryDataPacket> {
        registries
            .dynamic_registry_entries(true)
            .into_iter()
            .map(|(registry_id, entries)| RegistryDataPacket::new(registry_id, Array(entries)))
            .collect()
    }

    fn build_tag_packet(registries: &Registries) -> UpdateTagsPacket {
        let mut registry_tags = match registries.static_tag_entries() {
            Ok(registry_tags) => registry_tags,
            Err(error) => panic!("failed to build registry tags: {error:?}"),
        };
        registry_tags.extend(match registries.dynamic_tag_entries() {
            Ok(registry_tags) => registry_tags,
            Err(error) => panic!("failed to build registry tags: {error:?}"),
        });
        registry_tags.push(Self::entity_type_tags());
        UpdateTagsPacket::new(registry_tags.into_iter().map(Self::tag_registry).collect())
    }

    fn entity_type_tags() -> RegistryTags {
        RegistryTags::new(
            Identifier::vanilla_static("entity_type"),
            vec![
                Self::entity_type_tag(
                    Identifier::vanilla_static("arrows"),
                    &["minecraft:arrow", "minecraft:spectral_arrow"],
                ),
                Self::entity_type_tag(
                    Identifier::vanilla_static("sensitive_to_bane_of_arthropods"),
                    &[
                        "minecraft:bee",
                        "minecraft:cave_spider",
                        "minecraft:endermite",
                        "minecraft:silverfish",
                        "minecraft:spider",
                    ],
                ),
                Self::entity_type_tag(
                    Identifier::vanilla_static("sensitive_to_impaling"),
                    &[
                        "minecraft:axolotl",
                        "minecraft:cod",
                        "minecraft:dolphin",
                        "minecraft:drowned",
                        "minecraft:elder_guardian",
                        "minecraft:glow_squid",
                        "minecraft:guardian",
                        "minecraft:pufferfish",
                        "minecraft:salmon",
                        "minecraft:squid",
                        "minecraft:tropical_fish",
                        "minecraft:turtle",
                    ],
                ),
                Self::entity_type_tag(
                    Identifier::vanilla_static("sensitive_to_smite"),
                    &[
                        "minecraft:bogged",
                        "minecraft:drowned",
                        "minecraft:husk",
                        "minecraft:phantom",
                        "minecraft:skeleton",
                        "minecraft:skeleton_horse",
                        "minecraft:stray",
                        "minecraft:wither",
                        "minecraft:wither_skeleton",
                        "minecraft:zoglin",
                        "minecraft:zombie",
                        "minecraft:zombie_horse",
                        "minecraft:zombie_villager",
                        "minecraft:zombified_piglin",
                    ],
                ),
            ],
        )
    }

    fn entity_type_tag(tag_name: Identifier, entity_type_names: &[&str]) -> RegistryTag {
        RegistryTag::new(
            tag_name,
            entity_type_names
                .iter()
                .filter_map(|entity_type_name| EntityType::from_key(entity_type_name))
                .map(|entity_type| entity_type.id())
                .collect(),
        )
    }

    fn tag_registry(registry_tags: RegistryTags) -> TagRegistry {
        TagRegistry {
            registry_name: registry_tags.registry_name.to_string(),
            tags: Array(registry_tags.tags.into_iter().map(Self::tag).collect()),
        }
    }

    fn tag(tag: spinel_registry::RegistryTag) -> Tag {
        Tag {
            tag_name: tag.tag_name.to_string(),
            entries: Array(tag.entries.into_iter().map(VarIntWrapper).collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
