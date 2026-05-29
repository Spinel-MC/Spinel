use crate::registry::{
    BANNER_PATTERN_REGISTRY, BIOME_REGISTRY, BLOCKS_REGISTRY, CAT_VARIANT_REGISTRY,
    CHAT_TYPE_REGISTRY, CHICKEN_VARIANT_REGISTRY, COW_VARIANT_REGISTRY, DAMAGE_TYPE_REGISTRY,
    DIALOG_REGISTRY, DIMENSION_TYPE_REGISTRY, ENCHANTMENT_REGISTRY, FROG_VARIANT_REGISTRY,
    INSTRUMENT_REGISTRY, ITEM_REGISTRY, JUKEBOX_SONG_REGISTRY, PAINTING_VARIANT_REGISTRY,
    PIG_VARIANT_REGISTRY, Registries, TIMELINE_REGISTRY, TRIM_MATERIAL_REGISTRY,
    TRIM_PATTERN_REGISTRY, WOLF_SOUND_VARIANT_REGISTRY, WOLF_VARIANT_REGISTRY,
    ZOMBIE_NAUTILUS_VARIANT_REGISTRY,
};
use crate::{Identifier, RegistryTagError, RegistryTags};

#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_chat_types.rs"]
pub mod vanilla_chat_types;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_damage_types.rs"]
pub mod vanilla_damage_types;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_banner_patterns.rs"]
pub mod vanilla_banner_patterns;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_cat_variants.rs"]
pub mod vanilla_cat_variants;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_chicken_variants.rs"]
pub mod vanilla_chicken_variants;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_cow_variants.rs"]
pub mod vanilla_cow_variants;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_frog_variants.rs"]
pub mod vanilla_frog_variants;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_pig_variants.rs"]
pub mod vanilla_pig_variants;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_painting_variants.rs"]
pub mod vanilla_painting_variants;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_trim_materials.rs"]
pub mod vanilla_trim_materials;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_trim_patterns.rs"]
pub mod vanilla_trim_patterns;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_instruments.rs"]
pub mod vanilla_instruments;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_jukebox_songs.rs"]
pub mod vanilla_jukebox_songs;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_wolf_variants.rs"]
pub mod vanilla_wolf_variants;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_wolf_sound_variants.rs"]
pub mod vanilla_wolf_sound_variants;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_dialogs.rs"]
pub mod vanilla_dialogs;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_enchantments.rs"]
pub mod vanilla_enchantments;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_timelines.rs"]
pub mod vanilla_timelines;
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../../generated/vanilla_zombie_nautilus_variants.rs"]
pub mod vanilla_zombie_nautilus_variants;

include!("../../generated/vanilla_dynamic_tags.rs");

impl Registries {
    pub fn block_tag_contains(
        &self,
        tag_name: &Identifier,
        block: &crate::vanilla_world_blocks::Block,
    ) -> bool {
        let Some(block_id) = self.block_id(block) else {
            return false;
        };
        self.static_tag_entries().is_ok_and(|registry_tags| {
            registry_tags.into_iter().any(|registry_tags| {
                registry_tags.registry_name == BLOCKS_REGISTRY
                    && registry_tags.tags.into_iter().any(|registry_tag| {
                        registry_tag.tag_name == *tag_name
                            && registry_tag.entries.contains(&block_id)
                    })
            })
        })
    }

    pub fn dynamic_registry_entries(
        &self,
        exclude_vanilla: bool,
    ) -> Vec<(
        Identifier,
        Vec<(Identifier, Option<spinel_nbt::NbtCompound>)>,
    )> {
        vec![
            (
                BIOME_REGISTRY,
                self.biomes.registry_packet_entries(exclude_vanilla),
            ),
            (
                CHAT_TYPE_REGISTRY,
                self.chat_types.registry_packet_entries(exclude_vanilla),
            ),
            (
                TRIM_PATTERN_REGISTRY,
                self.trim_patterns.registry_packet_entries(exclude_vanilla),
            ),
            (
                TRIM_MATERIAL_REGISTRY,
                self.trim_materials.registry_packet_entries(exclude_vanilla),
            ),
            (
                WOLF_VARIANT_REGISTRY,
                self.wolf_variants.registry_packet_entries(exclude_vanilla),
            ),
            (
                WOLF_SOUND_VARIANT_REGISTRY,
                self.wolf_sound_variants
                    .registry_packet_entries(exclude_vanilla),
            ),
            (
                PIG_VARIANT_REGISTRY,
                self.pig_variants.registry_packet_entries(exclude_vanilla),
            ),
            (
                FROG_VARIANT_REGISTRY,
                self.frog_variants.registry_packet_entries(exclude_vanilla),
            ),
            (
                CAT_VARIANT_REGISTRY,
                self.cat_variants.registry_packet_entries(exclude_vanilla),
            ),
            (
                COW_VARIANT_REGISTRY,
                self.cow_variants.registry_packet_entries(exclude_vanilla),
            ),
            (
                CHICKEN_VARIANT_REGISTRY,
                self.chicken_variants
                    .registry_packet_entries(exclude_vanilla),
            ),
            (
                PAINTING_VARIANT_REGISTRY,
                self.painting_variants
                    .registry_packet_entries(exclude_vanilla),
            ),
            (
                DIMENSION_TYPE_REGISTRY,
                self.dimension_types
                    .registry_packet_entries(exclude_vanilla),
            ),
            (
                DAMAGE_TYPE_REGISTRY,
                self.damage_types.registry_packet_entries(exclude_vanilla),
            ),
            (
                BANNER_PATTERN_REGISTRY,
                self.banner_patterns
                    .registry_packet_entries(exclude_vanilla),
            ),
            (
                JUKEBOX_SONG_REGISTRY,
                self.jukebox_songs.registry_packet_entries(exclude_vanilla),
            ),
            (
                INSTRUMENT_REGISTRY,
                self.instruments.registry_packet_entries(exclude_vanilla),
            ),
            (
                DIALOG_REGISTRY,
                self.dialogs.registry_packet_entries(exclude_vanilla),
            ),
            (
                ZOMBIE_NAUTILUS_VARIANT_REGISTRY,
                self.zombie_nautilus_variants
                    .registry_packet_entries(exclude_vanilla),
            ),
            (
                TIMELINE_REGISTRY,
                self.timelines.registry_packet_entries(exclude_vanilla),
            ),
            (
                ENCHANTMENT_REGISTRY,
                self.enchantments.registry_packet_entries(exclude_vanilla),
            ),
        ]
    }

    pub fn register_dynamic_vanilla(&mut self) {
        vanilla_chat_types::register_chat_types(&mut self.chat_types);
        vanilla_damage_types::register_damage_types(&mut self.damage_types);
        vanilla_banner_patterns::register_banner_patterns(&mut self.banner_patterns);
        vanilla_cat_variants::register_cat_variants(&mut self.cat_variants);
        vanilla_chicken_variants::register_chicken_variants(&mut self.chicken_variants);
        vanilla_cow_variants::register_cow_variants(&mut self.cow_variants);
        vanilla_frog_variants::register_frog_variants(&mut self.frog_variants);
        vanilla_pig_variants::register_pig_variants(&mut self.pig_variants);
        vanilla_painting_variants::register_painting_variants(&mut self.painting_variants);
        vanilla_trim_materials::register_trim_materials(&mut self.trim_materials);
        vanilla_trim_patterns::register_trim_patterns(&mut self.trim_patterns);
        vanilla_instruments::register_instruments(&mut self.instruments);
        vanilla_jukebox_songs::register_jukebox_songs(&mut self.jukebox_songs);
        vanilla_wolf_variants::register_wolf_variants(&mut self.wolf_variants);
        vanilla_wolf_sound_variants::register_wolf_sound_variants(&mut self.wolf_sound_variants);
        vanilla_dialogs::register_dialogs(&mut self.dialogs);
        vanilla_enchantments::register_enchantments(&mut self.enchantments);
        vanilla_timelines::register_timelines(&mut self.timelines);
        vanilla_zombie_nautilus_variants::register_zombie_nautilus_variants(
            &mut self.zombie_nautilus_variants,
        );
    }
}
