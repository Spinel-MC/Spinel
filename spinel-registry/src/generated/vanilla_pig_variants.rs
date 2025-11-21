use crate::pig_variant::{
    BiomeCondition, PigModelType, PigVariant, PigVariantRegistry, SpawnConditionEntry,
};
use crate::types::Identifier;
use std::borrow::Cow;
pub const COLD: &PigVariant = &PigVariant {
    key: Identifier::vanilla_static("cold"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/pig/cold_pig"),
    },
    model: PigModelType::Cold,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:spawns_cold_variant_farm_animals",
        }),
    }],
};
pub const TEMPERATE: &PigVariant = &PigVariant {
    key: Identifier::vanilla_static("temperate"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/pig/temperate_pig"),
    },
    model: PigModelType::Normal,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const WARM: &PigVariant = &PigVariant {
    key: Identifier::vanilla_static("warm"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/pig/warm_pig"),
    },
    model: PigModelType::Normal,
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:spawns_warm_variant_farm_animals",
        }),
    }],
};
pub fn register_pig_variants(registry: &mut PigVariantRegistry) {
    registry.register(COLD);
    registry.register(TEMPERATE);
    registry.register(WARM);
}
