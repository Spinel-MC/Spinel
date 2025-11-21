use crate::cow_variant::{
    BiomeCondition, CowModelType, CowVariant, CowVariantRegistry, SpawnConditionEntry,
};
use crate::types::Identifier;
use std::borrow::Cow;
pub const COLD: &CowVariant = &CowVariant {
    key: Identifier::vanilla_static("cold"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cow/cold_cow"),
    },
    model: CowModelType::Cold,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:spawns_cold_variant_farm_animals",
        }),
    }],
};
pub const TEMPERATE: &CowVariant = &CowVariant {
    key: Identifier::vanilla_static("temperate"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cow/temperate_cow"),
    },
    model: CowModelType::Normal,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const WARM: &CowVariant = &CowVariant {
    key: Identifier::vanilla_static("warm"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cow/warm_cow"),
    },
    model: CowModelType::Warm,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:spawns_warm_variant_farm_animals",
        }),
    }],
};
pub fn register_cow_variants(registry: &mut CowVariantRegistry) {
    registry.register(COLD);
    registry.register(TEMPERATE);
    registry.register(WARM);
}
