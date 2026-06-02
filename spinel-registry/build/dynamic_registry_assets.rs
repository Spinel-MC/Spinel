pub(crate) struct DynamicRegistryAsset {
    pub(crate) module_name: &'static str,
    pub(crate) type_name: &'static str,
    pub(crate) path: &'static str,
}

pub(crate) const DYNAMIC_REGISTRY_ASSETS: &[DynamicRegistryAsset] = &[
    DynamicRegistryAsset {
        module_name: "vanilla_chat_types.rs",
        type_name: "ChatType",
        path: "chat_type",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_damage_types.rs",
        type_name: "DamageType",
        path: "damage_type",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_banner_patterns.rs",
        type_name: "BannerPattern",
        path: "banner_pattern",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_cat_variants.rs",
        type_name: "CatVariant",
        path: "cat_variant",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_chicken_variants.rs",
        type_name: "ChickenVariant",
        path: "chicken_variant",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_cow_variants.rs",
        type_name: "CowVariant",
        path: "cow_variant",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_frog_variants.rs",
        type_name: "FrogVariant",
        path: "frog_variant",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_pig_variants.rs",
        type_name: "PigVariant",
        path: "pig_variant",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_painting_variants.rs",
        type_name: "PaintingVariant",
        path: "painting_variant",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_trim_materials.rs",
        type_name: "TrimMaterial",
        path: "trim_material",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_trim_patterns.rs",
        type_name: "TrimPattern",
        path: "trim_pattern",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_instruments.rs",
        type_name: "Instrument",
        path: "instrument",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_jukebox_songs.rs",
        type_name: "JukeboxSong",
        path: "jukebox_song",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_wolf_variants.rs",
        type_name: "WolfVariant",
        path: "wolf_variant",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_wolf_sound_variants.rs",
        type_name: "WolfSoundVariant",
        path: "wolf_sound_variant",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_dialogs.rs",
        type_name: "Dialog",
        path: "dialog",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_enchantments.rs",
        type_name: "Enchantment",
        path: "enchantment",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_mob_effects.rs",
        type_name: "MobEffect",
        path: "mob_effect",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_timelines.rs",
        type_name: "Timeline",
        path: "timeline",
    },
    DynamicRegistryAsset {
        module_name: "vanilla_zombie_nautilus_variants.rs",
        type_name: "ZombieNautilusVariant",
        path: "zombie_nautilus_variant",
    },
];
