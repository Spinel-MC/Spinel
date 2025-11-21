use crate::chicken_variant::{
    BiomeCondition, ChickenModelType, ChickenVariant, ChickenVariantRegistry, SpawnConditionEntry,
};
use crate::types::Identifier;
use std::borrow::Cow;
pub const COLD: &ChickenVariant = &ChickenVariant {
    key: Identifier::vanilla_static("cold"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/chicken/cold_chicken"),
    },
    model: ChickenModelType::Cold,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:spawns_cold_variant_farm_animals",
        }),
    }],
};
pub const TEMPERATE: &ChickenVariant = &ChickenVariant {
    key: Identifier::vanilla_static("temperate"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/chicken/temperate_chicken"),
    },
    model: ChickenModelType::Normal,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const WARM: &ChickenVariant = &ChickenVariant {
    key: Identifier::vanilla_static("warm"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/chicken/warm_chicken"),
    },
    model: ChickenModelType::Normal,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:spawns_warm_variant_farm_animals",
        }),
    }],
};
pub fn register_chicken_variants(registry: &mut ChickenVariantRegistry) {
    registry.register(COLD);
    registry.register(TEMPERATE);
    registry.register(WARM);
}
