use crate::biome::{
    AdditionsSound, Biome, BiomeEffects, BiomeRegistry, GrassColorModifier, MoodSound, Music,
    Particle, ParticleOptions, SpawnCost, SpawnerData, TemperatureModifier, WeightedMusic,
};
use crate::types::Identifier;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;
pub static BADLANDS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("badlands"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: Some(10387789i32),
        grass_color: Some(9470285i32),
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.badlands"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0.03f32,
    spawners: HashMap::from([
        ("water_ambient".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("armadillo"),
                    },
                    weight: 6i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_extra"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dry_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dead_bush_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_cactus_decorated"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static BAMBOO_JUNGLE: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("bamboo_jungle"),
    has_precipitation: true,
    temperature: 0.95f32,
    downfall: 0.9f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7842047i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.bamboo_jungle"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("axolotls".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("parrot"),
                    },
                    weight: 40i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("panda"),
                    },
                    weight: 80i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("ocelot"),
                    },
                    weight: 2i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("bamboo"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("bamboo_vegetation"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_warm"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_jungle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("vines"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_melon"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static BASALT_DELTAS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("basalt_deltas"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 6840176i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.nether.basalt_deltas"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: Some(Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ambient.basalt_deltas.loop"),
        }),
        additions_sound: Some(AdditionsSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.basalt_deltas.additions"),
            },
            tick_chance: 0.0111f64,
        }),
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.basalt_deltas.mood"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: Some(Particle {
            options: ParticleOptions {
                particle_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("white_ash"),
                },
            },
            probability: 0.118093334f32,
        }),
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_ambient".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("strider"),
                },
                weight: 60i32,
                min_count: 1i32,
                max_count: 2i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("ghast"),
                    },
                    weight: 40i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("magma_cube"),
                    },
                    weight: 100i32,
                    min_count: 2i32,
                    max_count: 5i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        ("underground_water_creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("nether_cave"),
    }],
    features: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("delta"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("small_basalt_columns"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("large_basalt_columns"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("basalt_blobs"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("blackstone_blobs"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_delta"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_soul_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone_extra"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_closed_double"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_deltas"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_quartz_deltas"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_ancient_debris_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_debris_small"),
            },
        ],
    ],
});
pub static BEACH: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("beach"),
    has_precipitation: true,
    temperature: 0.8f32,
    downfall: 0.4f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7907327i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("turtle"),
                },
                weight: 5i32,
                min_count: 2i32,
                max_count: 5i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static BIRCH_FOREST: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("birch_forest"),
    has_precipitation: true,
    temperature: 0.6f32,
    downfall: 0.6f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8037887i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.forest"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("forest_flowers"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("wildflowers_birch_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_birch"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static CHERRY_GROVE: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("cherry_grove"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 6141935i32,
        water_fog_color: 6141935i32,
        foliage_color: Some(11983713i32),
        grass_color: Some(11983713i32),
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.cherry_grove"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 2i32,
                    min_count: 2i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 2i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_plain"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_cherry"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_cherry"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static COLD_OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("cold_ocean"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4020182i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("squid"),
                },
                weight: 3i32,
                min_count: 1i32,
                max_count: 4i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "water_ambient".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cod"),
                    },
                    weight: 15i32,
                    min_count: 3i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("salmon"),
                    },
                    weight: 15i32,
                    min_count: 1i32,
                    max_count: 5i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_cold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("kelp_cold"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static CRIMSON_FOREST: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("crimson_forest"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 3343107i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.nether.crimson_forest"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: Some(Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ambient.crimson_forest.loop"),
        }),
        additions_sound: Some(AdditionsSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.crimson_forest.additions"),
            },
            tick_chance: 0.0111f64,
        }),
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.crimson_forest.mood"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: Some(Particle {
            options: ParticleOptions {
                particle_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("crimson_spore"),
                },
            },
            probability: 0.025f32,
        }),
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("strider"),
                },
                weight: 60i32,
                min_count: 1i32,
                max_count: 2i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombified_piglin"),
                    },
                    weight: 1i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("hoglin"),
                    },
                    weight: 9i32,
                    min_count: 3i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("piglin"),
                    },
                    weight: 5i32,
                    min_count: 3i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("water_ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("underground_water_creature".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("nether_cave"),
    }],
    features: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_open"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone_extra"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_closed"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_blackstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_quartz_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_ancient_debris_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_debris_small"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("weeping_vines"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("crimson_fungi"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("crimson_forest_vegetation"),
            },
        ],
    ],
});
pub static DARK_FOREST: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("dark_forest"),
    has_precipitation: true,
    temperature: 0.7f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7972607i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::DarkForest,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.forest"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("dark_forest_vegetation"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("forest_flowers"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_leaf_litter"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static DEEP_COLD_OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("deep_cold_ocean"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4020182i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "water_ambient".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cod"),
                    },
                    weight: 15i32,
                    min_count: 3i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("salmon"),
                    },
                    weight: 15i32,
                    min_count: 1i32,
                    max_count: 5i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("squid"),
                },
                weight: 3i32,
                min_count: 1i32,
                max_count: 4i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_deep_cold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("kelp_cold"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static DEEP_DARK: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("deep_dark"),
    has_precipitation: true,
    temperature: 0.8f32,
    downfall: 0.4f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7907327i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.deep_dark"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("ambient".to_string(), vec![]),
        ("underground_water_creature".to_string(), vec![]),
        ("creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("monster".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("sculk_vein"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("sculk_patch_deep_dark"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_plains"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_plains"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_plain"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static DEEP_FROZEN_OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("deep_frozen_ocean"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::Frozen,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 3750089i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("polar_bear"),
                },
                weight: 1i32,
                min_count: 1i32,
                max_count: 2i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("squid"),
                },
                weight: 1i32,
                min_count: 1i32,
                max_count: 4i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "water_ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("salmon"),
                },
                weight: 15i32,
                min_count: 1i32,
                max_count: 5i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("iceberg_packed"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("iceberg_blue"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("amethyst_geode"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("blue_ice"),
        }],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static DEEP_LUKEWARM_OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("deep_lukewarm_ocean"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4566514i32,
        water_fog_color: 267827i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "water_creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("squid"),
                    },
                    weight: 8i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("dolphin"),
                    },
                    weight: 2i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("creature".to_string(), vec![]),
        (
            "water_ambient".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cod"),
                    },
                    weight: 8i32,
                    min_count: 3i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pufferfish"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("tropical_fish"),
                    },
                    weight: 25i32,
                    min_count: 8i32,
                    max_count: 8i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_deep_warm"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("kelp_warm"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static DEEP_OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("deep_ocean"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("creature".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "water_creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("squid"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("dolphin"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        (
            "water_ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("cod"),
                },
                weight: 10i32,
                min_count: 3i32,
                max_count: 6i32,
            }],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_deep"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("kelp_cold"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static DESERT: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("desert"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.desert"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 19i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("husk"),
                    },
                    weight: 80i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("water_ambient".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 12i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("camel"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("fossil_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("fossil_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("desert_well"),
        }],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dry_grass_desert"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dead_bush_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane_desert"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_cactus_desert"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static DRIPSTONE_CAVES: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("dripstone_caves"),
    has_precipitation: true,
    temperature: 0.8f32,
    downfall: 0.4f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7907327i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.dripstone_caves"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("amethyst_geode"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("large_dripstone"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("dripstone_cluster"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("pointed_dripstone"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_plains"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_plains"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_plain"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static END_BARRENS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("end_barrens"),
    has_precipitation: false,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 10518688i32,
        sky_color: 0i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_ambient".to_string(), vec![]),
        ("creature".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("underground_water_creature".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("enderman"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 4i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![],
    features: vec![],
});
pub static END_HIGHLANDS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("end_highlands"),
    has_precipitation: false,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 10518688i32,
        sky_color: 0i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("underground_water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("enderman"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 4i32,
            }],
        ),
        ("creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![],
    features: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("end_gateway_return"),
        }],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("chorus_plant"),
        }],
    ],
});
pub static END_MIDLANDS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("end_midlands"),
    has_precipitation: false,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 10518688i32,
        sky_color: 0i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("underground_water_creature".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("creature".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("enderman"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 4i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![],
    features: vec![],
});
pub static ERODED_BADLANDS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("eroded_badlands"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: Some(10387789i32),
        grass_color: Some(9470285i32),
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.badlands"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0.03f32,
    spawners: HashMap::from([
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("armadillo"),
                    },
                    weight: 6i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_extra"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dry_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dead_bush_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_cactus_decorated"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static FLOWER_FOREST: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("flower_forest"),
    has_precipitation: true,
    temperature: 0.7f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7972607i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.flower_forest"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 4i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_forest_flowers"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_flower_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_flower_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static FOREST: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("forest"),
    has_precipitation: true,
    temperature: 0.7f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7972607i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.forest"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 5i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("forest_flowers"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_birch_and_oak_leaf_litter"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static FROZEN_OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("frozen_ocean"),
    has_precipitation: true,
    temperature: 0f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::Frozen,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8364543i32,
        water_color: 3750089i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "water_ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("salmon"),
                },
                weight: 15i32,
                min_count: 1i32,
                max_count: 5i32,
            }],
        ),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("polar_bear"),
                },
                weight: 1i32,
                min_count: 1i32,
                max_count: 2i32,
            }],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("squid"),
                },
                weight: 1i32,
                min_count: 1i32,
                max_count: 4i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("iceberg_packed"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("iceberg_blue"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("amethyst_geode"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("blue_ice"),
        }],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static FROZEN_PEAKS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("frozen_peaks"),
    has_precipitation: true,
    temperature: -0.7f32,
    downfall: 0.9f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8756735i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.frozen_peaks"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("goat"),
                },
                weight: 5i32,
                min_count: 1i32,
                max_count: 3i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava_frozen"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("glow_lichen"),
        }],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static FROZEN_RIVER: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("frozen_river"),
    has_precipitation: true,
    temperature: 0f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8364543i32,
        water_color: 3750089i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("squid"),
                },
                weight: 2i32,
                min_count: 1i32,
                max_count: 4i32,
            }],
        ),
        (
            "water_ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("salmon"),
                },
                weight: 5i32,
                min_count: 1i32,
                max_count: 5i32,
            }],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static GROVE: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("grove"),
    has_precipitation: true,
    temperature: -0.2f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8495359i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.grove"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 8i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("fox"),
                    },
                    weight: 4i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava_frozen"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_grove"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static ICE_SPIKES: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("ice_spikes"),
    has_precipitation: true,
    temperature: 0f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8364543i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0.07f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 20i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("stray"),
                    },
                    weight: 80i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 10i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("polar_bear"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ice_spike"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ice_patch"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_snowy"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static JAGGED_PEAKS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("jagged_peaks"),
    has_precipitation: true,
    temperature: -0.7f32,
    downfall: 0.9f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8756735i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.jagged_peaks"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("goat"),
                },
                weight: 5i32,
                min_count: 1i32,
                max_count: 3i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava_frozen"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("glow_lichen"),
        }],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static JUNGLE: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("jungle"),
    has_precipitation: true,
    temperature: 0.95f32,
    downfall: 0.9f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7842047i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.jungle"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("parrot"),
                    },
                    weight: 40i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("panda"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("ocelot"),
                    },
                    weight: 2i32,
                    min_count: 1i32,
                    max_count: 3i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("bamboo_light"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_jungle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_warm"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_jungle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("vines"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_melon"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static LUKEWARM_OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("lukewarm_ocean"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4566514i32,
        water_fog_color: 267827i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("creature".to_string(), vec![]),
        (
            "water_creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("squid"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("dolphin"),
                    },
                    weight: 2i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "water_ambient".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cod"),
                    },
                    weight: 15i32,
                    min_count: 3i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pufferfish"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("tropical_fish"),
                    },
                    weight: 25i32,
                    min_count: 8i32,
                    max_count: 8i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_warm"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("kelp_warm"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static LUSH_CAVES: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("lush_caves"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.lush_caves"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "water_ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("tropical_fish"),
                },
                weight: 25i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "axolotls".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("axolotl"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lush_caves_ceiling_vegetation"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("cave_vines"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lush_caves_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lush_caves_vegetation"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("rooted_azalea_tree"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spore_blossom"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("classic_vines_cave_feature"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static MANGROVE_SWAMP: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("mangrove_swamp"),
    has_precipitation: true,
    temperature: 0.8f32,
    downfall: 0.9f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7907327i32,
        water_color: 3832426i32,
        water_fog_color: 5077600i32,
        foliage_color: Some(9285927i32),
        grass_color: None,
        grass_color_modifier: GrassColorModifier::Swamp,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.swamp"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("frog"),
                },
                weight: 10i32,
                min_count: 2i32,
                max_count: 5i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "water_ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("tropical_fish"),
                },
                weight: 25i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 70i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("bogged"),
                    },
                    weight: 30i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("fossil_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("fossil_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_grass"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_mangrove"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dead_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_waterlily"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_swamp"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static MEADOW: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("meadow"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 937679i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.meadow"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("donkey"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 2i32,
                    min_count: 2i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 2i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_meadow"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_meadow"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_meadow"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("wildflowers_meadow"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static MUSHROOM_FIELDS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("mushroom_fields"),
    has_precipitation: true,
    temperature: 0.9f32,
    downfall: 1f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7842047i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("monster".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("mooshroom"),
                },
                weight: 8i32,
                min_count: 4i32,
                max_count: 8i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("mushroom_island_vegetation"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static NETHER_WASTES: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("nether_wastes"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 3344392i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.nether.nether_wastes"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: Some(Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ambient.nether_wastes.loop"),
        }),
        additions_sound: Some(AdditionsSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.nether_wastes.additions"),
            },
            tick_chance: 0.0111f64,
        }),
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.nether_wastes.mood"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("underground_water_creature".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("strider"),
                },
                weight: 60i32,
                min_count: 1i32,
                max_count: 2i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("ghast"),
                    },
                    weight: 50i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombified_piglin"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("magma_cube"),
                    },
                    weight: 2i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 1i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("piglin"),
                    },
                    weight: 15i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("nether_cave"),
    }],
    features: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_open"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_soul_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone_extra"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_closed"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_blackstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_quartz_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_ancient_debris_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_debris_small"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
        ],
    ],
});
pub static OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("ocean"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "water_creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("squid"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("dolphin"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "water_ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("cod"),
                },
                weight: 10i32,
                min_count: 3i32,
                max_count: 6i32,
            }],
        ),
        ("creature".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("kelp_cold"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static OLD_GROWTH_BIRCH_FOREST: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("old_growth_birch_forest"),
    has_precipitation: true,
    temperature: 0.6f32,
    downfall: 0.6f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8037887i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.forest"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("forest_flowers"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("wildflowers_birch_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("birch_tall"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static OLD_GROWTH_PINE_TAIGA: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("old_growth_pine_taiga"),
    has_precipitation: true,
    temperature: 0.3f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8168447i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.old_growth_taiga"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 4i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("fox"),
                    },
                    weight: 8i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 25i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("amethyst_geode"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("forest_rock"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_large_fern"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_old_growth_pine_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dead_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_old_growth"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_old_growth"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_berry_common"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static OLD_GROWTH_SPRUCE_TAIGA: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("old_growth_spruce_taiga"),
    has_precipitation: true,
    temperature: 0.25f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8233983i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.old_growth_taiga"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 4i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("fox"),
                    },
                    weight: 8i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("amethyst_geode"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("forest_rock"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_large_fern"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_old_growth_spruce_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dead_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_old_growth"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_old_growth"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_berry_common"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static PALE_GARDEN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("pale_garden"),
    has_precipitation: true,
    temperature: 0.7f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 8484720i32,
        sky_color: 12171705i32,
        water_color: 7768221i32,
        water_fog_color: 5597568i32,
        foliage_color: Some(8883574i32),
        grass_color: Some(7832178i32),
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("creature".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("pale_garden_vegetation"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("pale_moss_patch"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("pale_garden_flowers"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_pale_garden"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static PLAINS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("plains"),
    has_precipitation: true,
    temperature: 0.8f32,
    downfall: 0.4f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7907327i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("horse"),
                    },
                    weight: 5i32,
                    min_count: 2i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("donkey"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 3i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_plains"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_plains"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_plain"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static RIVER: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("river"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("squid"),
                },
                weight: 2i32,
                min_count: 1i32,
                max_count: 4i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 100i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "water_ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("salmon"),
                },
                weight: 5i32,
                min_count: 1i32,
                max_count: 5i32,
            }],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_river"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SAVANNA: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("savanna"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("horse"),
                    },
                    weight: 1i32,
                    min_count: 2i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("donkey"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("armadillo"),
                    },
                    weight: 10i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_savanna"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_warm"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_savanna"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SAVANNA_PLATEAU: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("savanna_plateau"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("horse"),
                    },
                    weight: 1i32,
                    min_count: 2i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("donkey"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("armadillo"),
                    },
                    weight: 10i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("llama"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 8i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_savanna"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_warm"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_savanna"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SMALL_END_ISLANDS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("small_end_islands"),
    has_precipitation: false,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 10518688i32,
        sky_color: 0i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("enderman"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 4i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        ("underground_water_creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("creature".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![],
    features: vec![vec![Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("end_island_decorated"),
    }]],
});
pub static SNOWY_BEACH: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("snowy_beach"),
    has_precipitation: true,
    temperature: 0.05f32,
    downfall: 0.3f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8364543i32,
        water_color: 4020182i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SNOWY_PLAINS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("snowy_plains"),
    has_precipitation: true,
    temperature: 0f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8364543i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0.07f32,
    spawners: HashMap::from([
        ("water_ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 10i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("polar_bear"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 20i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("stray"),
                    },
                    weight: 80i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_snowy"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SNOWY_SLOPES: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("snowy_slopes"),
    has_precipitation: true,
    temperature: -0.3f32,
    downfall: 0.9f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8560639i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.snowy_slopes"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 4i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("goat"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 3i32,
                },
            ],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava_frozen"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SNOWY_TAIGA: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("snowy_taiga"),
    has_precipitation: true,
    temperature: -0.5f32,
    downfall: 0.4f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8625919i32,
        water_color: 4020182i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 4i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("fox"),
                    },
                    weight: 8i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_large_fern"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_taiga_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_berry_rare"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SOUL_SAND_VALLEY: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("soul_sand_valley"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 1787717i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.nether.soul_sand_valley"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: Some(Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ambient.soul_sand_valley.loop"),
        }),
        additions_sound: Some(AdditionsSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.soul_sand_valley.additions"),
            },
            tick_chance: 0.0111f64,
        }),
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.soul_sand_valley.mood"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: Some(Particle {
            options: ParticleOptions {
                particle_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("ash"),
                },
            },
            probability: 0.00625f32,
        }),
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_ambient".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("strider"),
                },
                weight: 60i32,
                min_count: 1i32,
                max_count: 2i32,
            }],
        ),
        ("underground_water_creature".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 20i32,
                    min_count: 5i32,
                    max_count: 5i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("ghast"),
                    },
                    weight: 50i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 1i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([
        (
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("enderman"),
            },
            SpawnCost {
                energy_budget: 0.15f64,
                charge: 0.7f64,
            },
        ),
        (
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("strider"),
            },
            SpawnCost {
                energy_budget: 0.15f64,
                charge: 0.7f64,
            },
        ),
        (
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("skeleton"),
            },
            SpawnCost {
                energy_budget: 0.15f64,
                charge: 0.7f64,
            },
        ),
        (
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ghast"),
            },
            SpawnCost {
                energy_budget: 0.15f64,
                charge: 0.7f64,
            },
        ),
    ]),
    carvers: vec![Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("nether_cave"),
    }],
    features: vec![
        vec![],
        vec![],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("basalt_pillar"),
        }],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_open"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_soul_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone_extra"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_crimson_roots"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_closed"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_soul_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_blackstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_quartz_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_ancient_debris_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_debris_small"),
            },
        ],
        vec![],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("spring_lava"),
        }],
    ],
});
pub static SPARSE_JUNGLE: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("sparse_jungle"),
    has_precipitation: true,
    temperature: 0.95f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7842047i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.sparse_jungle"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 8i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("water_ambient".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_sparse_jungle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_warm"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_jungle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("vines"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_melon_sparse"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static STONY_PEAKS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("stony_peaks"),
    has_precipitation: true,
    temperature: 1f32,
    downfall: 0.3f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7776511i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.stony_peaks"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("creature".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("glow_lichen"),
        }],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static STONY_SHORE: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("stony_shore"),
    has_precipitation: true,
    temperature: 0.2f32,
    downfall: 0.3f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8233727i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("misc".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        ("creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SUNFLOWER_PLAINS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("sunflower_plains"),
    has_precipitation: true,
    temperature: 0.8f32,
    downfall: 0.4f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7907327i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("horse"),
                    },
                    weight: 5i32,
                    min_count: 2i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("donkey"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 3i32,
                },
            ],
        ),
        ("water_creature".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_tall_grass_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sunflower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_plains"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_plains"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_plain"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static SWAMP: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("swamp"),
    has_precipitation: true,
    temperature: 0.8f32,
    downfall: 0.9f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7907327i32,
        water_color: 6388580i32,
        water_fog_color: 2302743i32,
        foliage_color: Some(6975545i32),
        grass_color: None,
        grass_color_modifier: GrassColorModifier::Swamp,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.swamp"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("frog"),
                    },
                    weight: 10i32,
                    min_count: 2i32,
                    max_count: 5i32,
                },
            ],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 70i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("bogged"),
                    },
                    weight: 30i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("water_ambient".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("fossil_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("fossil_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_swamp"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_swamp"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dead_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_waterlily"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_swamp"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_swamp"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane_swamp"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_swamp"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water_swamp"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_swamp"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static TAIGA: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("taiga"),
    has_precipitation: true,
    temperature: 0.25f32,
    downfall: 0.8f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8233983i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("water_ambient".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("rabbit"),
                    },
                    weight: 4i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("fox"),
                    },
                    weight: 8i32,
                    min_count: 2i32,
                    max_count: 4i32,
                },
            ],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_large_fern"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_taiga_2"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_taiga"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_berry_common"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static THE_END: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("the_end"),
    has_precipitation: false,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 10518688i32,
        sky_color: 0i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_ambient".to_string(), vec![]),
        ("ambient".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("enderman"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 4i32,
            }],
        ),
        ("underground_water_creature".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![],
    features: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("end_spike"),
        }],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("end_platform"),
        }],
    ],
});
pub static THE_VOID: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("the_void"),
    has_precipitation: false,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("underground_water_creature".to_string(), vec![]),
        ("creature".to_string(), vec![]),
        ("monster".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![],
    features: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("void_start_platform"),
        }],
    ],
});
pub static WARM_OCEAN: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("warm_ocean"),
    has_precipitation: true,
    temperature: 0.5f32,
    downfall: 0.5f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8103167i32,
        water_color: 4445678i32,
        water_fog_color: 270131i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "water_ambient".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pufferfish"),
                    },
                    weight: 15i32,
                    min_count: 1i32,
                    max_count: 3i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("tropical_fish"),
                    },
                    weight: 25i32,
                    min_count: 8i32,
                    max_count: 8i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        ("creature".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("drowned"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("misc".to_string(), vec![]),
        (
            "water_creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("squid"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("dolphin"),
                    },
                    weight: 2i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
            ],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("warm_ocean_vegetation"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("seagrass_warm"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("sea_pickle"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static WARPED_FOREST: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("warped_forest"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 1705242i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.nether.warped_forest"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: Some(Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ambient.warped_forest.loop"),
        }),
        additions_sound: Some(AdditionsSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.warped_forest.additions"),
            },
            tick_chance: 0.0111f64,
        }),
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.warped_forest.mood"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: Some(Particle {
            options: ParticleOptions {
                particle_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("warped_spore"),
                },
            },
            probability: 0.01428f32,
        }),
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("axolotls".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("strider"),
                },
                weight: 60i32,
                min_count: 1i32,
                max_count: 2i32,
            }],
        ),
        ("ambient".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("enderman"),
                },
                weight: 1i32,
                min_count: 4i32,
                max_count: 4i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("underground_water_creature".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([(
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("enderman"),
        },
        SpawnCost {
            energy_budget: 0.12f64,
            charge: 1f64,
        },
    )]),
    carvers: vec![Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("nether_cave"),
    }],
    features: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_open"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_soul_fire"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone_extra"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glowstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_closed"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_blackstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_quartz_nether"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_ancient_debris_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_debris_small"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("warped_fungi"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("warped_forest_vegetation"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("nether_sprouts"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("twisting_vines"),
            },
        ],
    ],
});
pub static WINDSWEPT_FOREST: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("windswept_forest"),
    has_precipitation: true,
    temperature: 0.2f32,
    downfall: 0.3f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8233727i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        ("water_ambient".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        ("axolotls".to_string(), vec![]),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("llama"),
                    },
                    weight: 5i32,
                    min_count: 4i32,
                    max_count: 6i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_windswept_forest"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static WINDSWEPT_GRAVELLY_HILLS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("windswept_gravelly_hills"),
    has_precipitation: true,
    temperature: 0.2f32,
    downfall: 0.3f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8233727i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("misc".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("llama"),
                    },
                    weight: 5i32,
                    min_count: 4i32,
                    max_count: 6i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_windswept_hills"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static WINDSWEPT_HILLS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("windswept_hills"),
    has_precipitation: true,
    temperature: 0.2f32,
    downfall: 0.3f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 8233727i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        ("water_creature".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("llama"),
                    },
                    weight: 5i32,
                    min_count: 4i32,
                    max_count: 6i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        ("water_ambient".to_string(), vec![]),
        ("misc".to_string(), vec![]),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_emerald"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("ore_infested"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_windswept_hills"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_bush"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static WINDSWEPT_SAVANNA: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("windswept_savanna"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: None,
        grass_color: None,
        grass_color_modifier: GrassColorModifier::None,
        music: None,
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0f32,
    spawners: HashMap::from([
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("horse"),
                    },
                    weight: 1i32,
                    min_count: 2i32,
                    max_count: 6i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("donkey"),
                    },
                    weight: 1i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("armadillo"),
                    },
                    weight: 10i32,
                    min_count: 2i32,
                    max_count: 3i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        ("water_creature".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_windswept_savanna"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("flower_default"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub static WOODED_BADLANDS: LazyLock<Biome> = LazyLock::new(|| Biome {
    key: Identifier::vanilla_static("wooded_badlands"),
    has_precipitation: false,
    temperature: 2f32,
    downfall: 0f32,
    temperature_modifier: TemperatureModifier::None,
    effects: BiomeEffects {
        fog_color: 12638463i32,
        sky_color: 7254527i32,
        water_color: 4159204i32,
        water_fog_color: 329011i32,
        foliage_color: Some(10387789i32),
        grass_color: Some(9470285i32),
        grass_color_modifier: GrassColorModifier::None,
        music: Some(vec![WeightedMusic {
            data: Music {
                replace_current_music: false,
                max_delay: 24000i32,
                min_delay: 12000i32,
                sound: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("music.overworld.badlands"),
                },
            },
            weight: 1i32,
        }]),
        ambient_sound: None,
        additions_sound: None,
        mood_sound: Some(MoodSound {
            sound: Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ambient.cave"),
            },
            tick_delay: 6000i32,
            block_search_extent: 8i32,
            offset: 2f64,
        }),
        particle: None,
    },
    creature_spawn_probability: 0.04f32,
    spawners: HashMap::from([
        (
            "creature".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("sheep"),
                    },
                    weight: 12i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("pig"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("chicken"),
                    },
                    weight: 10i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("cow"),
                    },
                    weight: 8i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("armadillo"),
                    },
                    weight: 6i32,
                    min_count: 1i32,
                    max_count: 2i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("wolf"),
                    },
                    weight: 2i32,
                    min_count: 4i32,
                    max_count: 8i32,
                },
            ],
        ),
        ("axolotls".to_string(), vec![]),
        ("misc".to_string(), vec![]),
        (
            "underground_water_creature".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("glow_squid"),
                },
                weight: 10i32,
                min_count: 4i32,
                max_count: 6i32,
            }],
        ),
        ("water_creature".to_string(), vec![]),
        (
            "ambient".to_string(),
            vec![SpawnerData {
                entity_type: Identifier {
                    namespace: Cow::Borrowed("minecraft"),
                    path: Cow::Borrowed("bat"),
                },
                weight: 10i32,
                min_count: 8i32,
                max_count: 8i32,
            }],
        ),
        ("water_ambient".to_string(), vec![]),
        (
            "monster".to_string(),
            vec![
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("spider"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie"),
                    },
                    weight: 95i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("zombie_villager"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("skeleton"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("creeper"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("slime"),
                    },
                    weight: 100i32,
                    min_count: 4i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("enderman"),
                    },
                    weight: 10i32,
                    min_count: 1i32,
                    max_count: 4i32,
                },
                SpawnerData {
                    entity_type: Identifier {
                        namespace: Cow::Borrowed("minecraft"),
                        path: Cow::Borrowed("witch"),
                    },
                    weight: 5i32,
                    min_count: 1i32,
                    max_count: 1i32,
                },
            ],
        ),
    ]),
    spawn_costs: HashMap::from([]),
    carvers: vec![
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("cave_extra_underground"),
        },
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("canyon"),
        },
    ],
    features: vec![
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_underground"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("lake_lava_surface"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("amethyst_geode"),
        }],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("monster_room_deep"),
            },
        ],
        vec![],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_dirt"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gravel"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_granite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diorite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_andesite_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_tuff"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_coal_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_upper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_middle"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_iron_small"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_redstone_lower"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_medium"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_large"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_diamond_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_lapis_buried"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_copper"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("underwater_magma"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("ore_gold_extra"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_sand"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_clay"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("disk_gravel"),
            },
        ],
        vec![],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_water"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("spring_lava"),
            },
        ],
        vec![
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("glow_lichen"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("trees_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dry_grass_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_dead_bush_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("brown_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("red_mushroom_normal"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_sugar_cane_badlands"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_pumpkin"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_cactus_decorated"),
            },
            Identifier {
                namespace: Cow::Borrowed("minecraft"),
                path: Cow::Borrowed("patch_firefly_bush_near_water"),
            },
        ],
        vec![Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("freeze_top_layer"),
        }],
    ],
});
pub fn register_biomes(registry: &mut BiomeRegistry) {
    registry.register(&BADLANDS, BADLANDS.key.clone());
    registry.register(&BAMBOO_JUNGLE, BAMBOO_JUNGLE.key.clone());
    registry.register(&BASALT_DELTAS, BASALT_DELTAS.key.clone());
    registry.register(&BEACH, BEACH.key.clone());
    registry.register(&BIRCH_FOREST, BIRCH_FOREST.key.clone());
    registry.register(&CHERRY_GROVE, CHERRY_GROVE.key.clone());
    registry.register(&COLD_OCEAN, COLD_OCEAN.key.clone());
    registry.register(&CRIMSON_FOREST, CRIMSON_FOREST.key.clone());
    registry.register(&DARK_FOREST, DARK_FOREST.key.clone());
    registry.register(&DEEP_COLD_OCEAN, DEEP_COLD_OCEAN.key.clone());
    registry.register(&DEEP_DARK, DEEP_DARK.key.clone());
    registry.register(&DEEP_FROZEN_OCEAN, DEEP_FROZEN_OCEAN.key.clone());
    registry.register(&DEEP_LUKEWARM_OCEAN, DEEP_LUKEWARM_OCEAN.key.clone());
    registry.register(&DEEP_OCEAN, DEEP_OCEAN.key.clone());
    registry.register(&DESERT, DESERT.key.clone());
    registry.register(&DRIPSTONE_CAVES, DRIPSTONE_CAVES.key.clone());
    registry.register(&END_BARRENS, END_BARRENS.key.clone());
    registry.register(&END_HIGHLANDS, END_HIGHLANDS.key.clone());
    registry.register(&END_MIDLANDS, END_MIDLANDS.key.clone());
    registry.register(&ERODED_BADLANDS, ERODED_BADLANDS.key.clone());
    registry.register(&FLOWER_FOREST, FLOWER_FOREST.key.clone());
    registry.register(&FOREST, FOREST.key.clone());
    registry.register(&FROZEN_OCEAN, FROZEN_OCEAN.key.clone());
    registry.register(&FROZEN_PEAKS, FROZEN_PEAKS.key.clone());
    registry.register(&FROZEN_RIVER, FROZEN_RIVER.key.clone());
    registry.register(&GROVE, GROVE.key.clone());
    registry.register(&ICE_SPIKES, ICE_SPIKES.key.clone());
    registry.register(&JAGGED_PEAKS, JAGGED_PEAKS.key.clone());
    registry.register(&JUNGLE, JUNGLE.key.clone());
    registry.register(&LUKEWARM_OCEAN, LUKEWARM_OCEAN.key.clone());
    registry.register(&LUSH_CAVES, LUSH_CAVES.key.clone());
    registry.register(&MANGROVE_SWAMP, MANGROVE_SWAMP.key.clone());
    registry.register(&MEADOW, MEADOW.key.clone());
    registry.register(&MUSHROOM_FIELDS, MUSHROOM_FIELDS.key.clone());
    registry.register(&NETHER_WASTES, NETHER_WASTES.key.clone());
    registry.register(&OCEAN, OCEAN.key.clone());
    registry.register(
        &OLD_GROWTH_BIRCH_FOREST,
        OLD_GROWTH_BIRCH_FOREST.key.clone(),
    );
    registry.register(&OLD_GROWTH_PINE_TAIGA, OLD_GROWTH_PINE_TAIGA.key.clone());
    registry.register(
        &OLD_GROWTH_SPRUCE_TAIGA,
        OLD_GROWTH_SPRUCE_TAIGA.key.clone(),
    );
    registry.register(&PALE_GARDEN, PALE_GARDEN.key.clone());
    registry.register(&PLAINS, PLAINS.key.clone());
    registry.register(&RIVER, RIVER.key.clone());
    registry.register(&SAVANNA, SAVANNA.key.clone());
    registry.register(&SAVANNA_PLATEAU, SAVANNA_PLATEAU.key.clone());
    registry.register(&SMALL_END_ISLANDS, SMALL_END_ISLANDS.key.clone());
    registry.register(&SNOWY_BEACH, SNOWY_BEACH.key.clone());
    registry.register(&SNOWY_PLAINS, SNOWY_PLAINS.key.clone());
    registry.register(&SNOWY_SLOPES, SNOWY_SLOPES.key.clone());
    registry.register(&SNOWY_TAIGA, SNOWY_TAIGA.key.clone());
    registry.register(&SOUL_SAND_VALLEY, SOUL_SAND_VALLEY.key.clone());
    registry.register(&SPARSE_JUNGLE, SPARSE_JUNGLE.key.clone());
    registry.register(&STONY_PEAKS, STONY_PEAKS.key.clone());
    registry.register(&STONY_SHORE, STONY_SHORE.key.clone());
    registry.register(&SUNFLOWER_PLAINS, SUNFLOWER_PLAINS.key.clone());
    registry.register(&SWAMP, SWAMP.key.clone());
    registry.register(&TAIGA, TAIGA.key.clone());
    registry.register(&THE_END, THE_END.key.clone());
    registry.register(&THE_VOID, THE_VOID.key.clone());
    registry.register(&WARM_OCEAN, WARM_OCEAN.key.clone());
    registry.register(&WARPED_FOREST, WARPED_FOREST.key.clone());
    registry.register(&WINDSWEPT_FOREST, WINDSWEPT_FOREST.key.clone());
    registry.register(
        &WINDSWEPT_GRAVELLY_HILLS,
        WINDSWEPT_GRAVELLY_HILLS.key.clone(),
    );
    registry.register(&WINDSWEPT_HILLS, WINDSWEPT_HILLS.key.clone());
    registry.register(&WINDSWEPT_SAVANNA, WINDSWEPT_SAVANNA.key.clone());
    registry.register(&WOODED_BADLANDS, WOODED_BADLANDS.key.clone());
}
