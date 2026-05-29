mod registries;
mod vanilla;

pub use registries::{
    BANNER_PATTERN_REGISTRY, BIOME_REGISTRY, BLOCKS_REGISTRY, CAT_VARIANT_REGISTRY,
    CHAT_TYPE_REGISTRY, CHICKEN_VARIANT_REGISTRY, COW_VARIANT_REGISTRY, DAMAGE_TYPE_REGISTRY,
    DIALOG_REGISTRY, DIMENSION_TYPE_REGISTRY, ENCHANTMENT_REGISTRY, ENTITY_TYPE_REGISTRY,
    FROG_VARIANT_REGISTRY, INSTRUMENT_REGISTRY, ITEM_REGISTRY, JUKEBOX_SONG_REGISTRY,
    PAINTING_VARIANT_REGISTRY, PIG_VARIANT_REGISTRY, Registries, TIMELINE_REGISTRY,
    TRIM_MATERIAL_REGISTRY, TRIM_PATTERN_REGISTRY, WOLF_SOUND_VARIANT_REGISTRY,
    WOLF_VARIANT_REGISTRY, ZOMBIE_NAUTILUS_VARIANT_REGISTRY,
};
pub use vanilla::{
    vanilla_banner_patterns, vanilla_cat_variants, vanilla_chat_types, vanilla_chicken_variants,
    vanilla_cow_variants, vanilla_damage_types, vanilla_dialogs, vanilla_enchantments,
    vanilla_frog_variants, vanilla_instruments, vanilla_jukebox_songs, vanilla_painting_variants,
    vanilla_pig_variants, vanilla_timelines, vanilla_trim_materials, vanilla_trim_patterns,
    vanilla_wolf_sound_variants, vanilla_wolf_variants, vanilla_zombie_nautilus_variants,
};
