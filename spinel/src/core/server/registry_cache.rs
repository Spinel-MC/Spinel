use crate::core::network::clientbound::configuration::registry_data::RegistryDataPacket;
use spinel_registry::{
    Registry, BANNER_PATTERN_REGISTRY, BIOME_REGISTRY, CAT_VARIANT_REGISTRY, CHAT_TYPE_REGISTRY,
    CHICKEN_VARIANT_REGISTRY, COW_VARIANT_REGISTRY, DAMAGE_TYPE_REGISTRY, DIALOG_REGISTRY,
    DIMENSION_TYPE_REGISTRY, FROG_VARIANT_REGISTRY, INSTRUMENT_REGISTRY, JUKEBOX_SONG_REGISTRY,
    PAINTING_VARIANT_REGISTRY, PIG_VARIANT_REGISTRY, TRIM_MATERIAL_REGISTRY, TRIM_PATTERN_REGISTRY,
    WOLF_SOUND_VARIANT_REGISTRY, WOLF_VARIANT_REGISTRY,
};
use std::sync::Arc;

pub struct RegistryCache {
    pub registry_packets: Arc<Vec<RegistryDataPacket>>,
}

impl RegistryCache {
    // Creates a new registry cache with all vanilla registries pre-loaded
    pub fn new(registry: &Registry) -> Self {
        let registry_packets = Self::build_registry_packets(registry);
        Self {
            registry_packets: Arc::new(registry_packets),
        }
    }

    fn build_registry_packets(registry: &Registry) -> Vec<RegistryDataPacket> {
        let mut packets = Vec::new();

        macro_rules! add_registry {
            ($reg_key:expr, $field:ident) => {
                let entries: Vec<String> = registry
                    .$field
                    .iter()
                    .map(|(_, entry)| entry.key.to_string())
                    .collect();
                packets.push(RegistryDataPacket::from_identifiers(
                    $reg_key.to_string(),
                    &entries,
                ));
            };
        }

        // Match SteelMC's order
        add_registry!(BIOME_REGISTRY, biomes);
        add_registry!(CHAT_TYPE_REGISTRY, chat_types);
        add_registry!(TRIM_PATTERN_REGISTRY, trim_patterns);
        add_registry!(TRIM_MATERIAL_REGISTRY, trim_materials);
        add_registry!(WOLF_VARIANT_REGISTRY, wolf_variants);
        add_registry!(WOLF_SOUND_VARIANT_REGISTRY, wolf_sound_variants);
        add_registry!(PIG_VARIANT_REGISTRY, pig_variants);
        add_registry!(FROG_VARIANT_REGISTRY, frog_variants);
        add_registry!(CAT_VARIANT_REGISTRY, cat_variants);
        add_registry!(COW_VARIANT_REGISTRY, cow_variants);
        add_registry!(CHICKEN_VARIANT_REGISTRY, chicken_variants);
        add_registry!(PAINTING_VARIANT_REGISTRY, painting_variants);
        add_registry!(DIMENSION_TYPE_REGISTRY, dimension_types);
        add_registry!(DAMAGE_TYPE_REGISTRY, damage_types);
        add_registry!(BANNER_PATTERN_REGISTRY, banner_patterns);
        add_registry!(JUKEBOX_SONG_REGISTRY, jukebox_songs);
        add_registry!(INSTRUMENT_REGISTRY, instruments);

        let dialog_entries: Vec<String> = registry
            .dialogs
            .iter()
            .map(|(_, entry)| entry.key().to_string())
            .collect();
        packets.push(RegistryDataPacket::from_identifiers(
            DIALOG_REGISTRY.to_string(),
            &dialog_entries,
        ));

        // TODO: Add enchantments when implemented
        // add_registry!(ENCHANTMENT_REGISTRY, enchantments);

        packets
    }

    // Gets the cached registry packets
    pub fn get_packets(&self) -> &[RegistryDataPacket] {
        &self.registry_packets
    }
}
