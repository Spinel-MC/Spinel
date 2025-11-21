use crate::frog_variant::{BiomeCondition, FrogVariant, FrogVariantRegistry, SpawnConditionEntry};
use crate::types::Identifier;
use std::borrow::Cow;
pub const COLD: &FrogVariant = &FrogVariant {
    key: Identifier::vanilla_static("cold"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/frog/cold_frog"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:spawns_cold_variant_frogs",
        }),
    }],
};
pub const TEMPERATE: &FrogVariant = &FrogVariant {
    key: Identifier::vanilla_static("temperate"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/frog/temperate_frog"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const WARM: &FrogVariant = &FrogVariant {
    key: Identifier::vanilla_static("warm"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/frog/warm_frog"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:spawns_warm_variant_frogs",
        }),
    }],
};
pub fn register_frog_variants(registry: &mut FrogVariantRegistry) {
    registry.register(COLD);
    registry.register(TEMPERATE);
    registry.register(WARM);
}
