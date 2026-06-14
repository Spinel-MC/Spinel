use spinel_core::network::clientbound::configuration::registry_data::RegistryDataPacket;
use spinel_core::network::clientbound::configuration::update_tags::{
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
