pub(crate) struct TagRegistrySpec {
    pub(crate) registry_const: &'static str,
    pub(crate) field_name: &'static str,
    pub(crate) tag_path: &'static str,
}

impl TagRegistrySpec {
    pub(crate) fn table_const(&self) -> String {
        format!("{}_TAGS", self.registry_const)
    }
}

pub(crate) fn tag_registry_specs() -> &'static [TagRegistrySpec] {
    TAG_REGISTRY_SPECS
}

const TAG_REGISTRY_SPECS: &[TagRegistrySpec] = &[
    spec("CHAT_TYPE_REGISTRY", "chat_types", "chat_type"),
    spec("BIOME_REGISTRY", "biomes", "worldgen/biome"),
    spec("DIALOG_REGISTRY", "dialogs", "dialog"),
    spec("DAMAGE_TYPE_REGISTRY", "damage_types", "damage_type"),
    spec("TRIM_MATERIAL_REGISTRY", "trim_materials", "trim_material"),
    spec("TRIM_PATTERN_REGISTRY", "trim_patterns", "trim_pattern"),
    spec(
        "BANNER_PATTERN_REGISTRY",
        "banner_patterns",
        "banner_pattern",
    ),
    spec("ENCHANTMENT_REGISTRY", "enchantments", "enchantment"),
    spec(
        "PAINTING_VARIANT_REGISTRY",
        "painting_variants",
        "painting_variant",
    ),
    spec("JUKEBOX_SONG_REGISTRY", "jukebox_songs", "jukebox_song"),
    spec("INSTRUMENT_REGISTRY", "instruments", "instrument"),
    spec("WOLF_VARIANT_REGISTRY", "wolf_variants", "wolf_variant"),
    spec(
        "WOLF_SOUND_VARIANT_REGISTRY",
        "wolf_sound_variants",
        "wolf_sound_variant",
    ),
    spec("CAT_VARIANT_REGISTRY", "cat_variants", "cat_variant"),
    spec(
        "CHICKEN_VARIANT_REGISTRY",
        "chicken_variants",
        "chicken_variant",
    ),
    spec("COW_VARIANT_REGISTRY", "cow_variants", "cow_variant"),
    spec("FROG_VARIANT_REGISTRY", "frog_variants", "frog_variant"),
    spec("PIG_VARIANT_REGISTRY", "pig_variants", "pig_variant"),
    spec(
        "ZOMBIE_NAUTILUS_VARIANT_REGISTRY",
        "zombie_nautilus_variants",
        "zombie_nautilus_variant",
    ),
    spec("TIMELINE_REGISTRY", "timelines", "timeline"),
    spec(
        "DIMENSION_TYPE_REGISTRY",
        "dimension_types",
        "dimension_type",
    ),
];

const fn spec(
    registry_const: &'static str,
    field_name: &'static str,
    tag_path: &'static str,
) -> TagRegistrySpec {
    TagRegistrySpec {
        registry_const,
        field_name,
        tag_path,
    }
}
