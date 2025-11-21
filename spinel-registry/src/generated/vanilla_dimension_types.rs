use crate::dimension_type::{DimensionType, DimensionTypeRegistry, MonsterSpawnLightLevel};
use crate::types::Identifier;
use std::borrow::Cow;
pub const OVERWORLD: &DimensionType = &DimensionType {
    key: Identifier::vanilla_static("overworld"),
    fixed_time: None,
    has_skylight: true,
    has_ceiling: false,
    ultrawarm: false,
    natural: true,
    coordinate_scale: 1f64,
    bed_works: true,
    respawn_anchor_works: false,
    min_y: -64i32,
    height: 384i32,
    logical_height: 384i32,
    infiniburn: "#minecraft:infiniburn_overworld",
    effects: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("overworld"),
    },
    ambient_light: 0f32,
    cloud_height: Some(192i32),
    piglin_safe: false,
    has_raids: true,
    monster_spawn_light_level: MonsterSpawnLightLevel::Complex {
        distribution_type: "minecraft:uniform",
        min_inclusive: 0i32,
        max_inclusive: 7i32,
    },
    monster_spawn_block_light_limit: 0i32,
};
pub const OVERWORLD_CAVES: &DimensionType = &DimensionType {
    key: Identifier::vanilla_static("overworld_caves"),
    fixed_time: None,
    has_skylight: true,
    has_ceiling: true,
    ultrawarm: false,
    natural: true,
    coordinate_scale: 1f64,
    bed_works: true,
    respawn_anchor_works: false,
    min_y: -64i32,
    height: 384i32,
    logical_height: 384i32,
    infiniburn: "#minecraft:infiniburn_overworld",
    effects: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("overworld"),
    },
    ambient_light: 0f32,
    cloud_height: Some(192i32),
    piglin_safe: false,
    has_raids: true,
    monster_spawn_light_level: MonsterSpawnLightLevel::Complex {
        distribution_type: "minecraft:uniform",
        min_inclusive: 0i32,
        max_inclusive: 7i32,
    },
    monster_spawn_block_light_limit: 0i32,
};
pub const THE_END: &DimensionType = &DimensionType {
    key: Identifier::vanilla_static("the_end"),
    fixed_time: Some(6000i64),
    has_skylight: true,
    has_ceiling: false,
    ultrawarm: false,
    natural: false,
    coordinate_scale: 1f64,
    bed_works: false,
    respawn_anchor_works: false,
    min_y: 0i32,
    height: 256i32,
    logical_height: 256i32,
    infiniburn: "#minecraft:infiniburn_end",
    effects: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("the_end"),
    },
    ambient_light: 0.25f32,
    cloud_height: None,
    piglin_safe: false,
    has_raids: true,
    monster_spawn_light_level: MonsterSpawnLightLevel::Simple(15i32),
    monster_spawn_block_light_limit: 0i32,
};
pub const THE_NETHER: &DimensionType = &DimensionType {
    key: Identifier::vanilla_static("the_nether"),
    fixed_time: Some(18000i64),
    has_skylight: false,
    has_ceiling: true,
    ultrawarm: true,
    natural: false,
    coordinate_scale: 8f64,
    bed_works: false,
    respawn_anchor_works: true,
    min_y: 0i32,
    height: 256i32,
    logical_height: 128i32,
    infiniburn: "#minecraft:infiniburn_nether",
    effects: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("the_nether"),
    },
    ambient_light: 0.1f32,
    cloud_height: None,
    piglin_safe: true,
    has_raids: false,
    monster_spawn_light_level: MonsterSpawnLightLevel::Simple(7i32),
    monster_spawn_block_light_limit: 15i32,
};
pub fn register_dimension_types(registry: &mut DimensionTypeRegistry) {
    registry.register(OVERWORLD);
    registry.register(OVERWORLD_CAVES);
    registry.register(THE_END);
    registry.register(THE_NETHER);
}
