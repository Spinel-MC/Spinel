use crate::types::Identifier;
use crate::wolf_variant::{
    BiomeCondition, SpawnConditionEntry, WolfAssetInfo, WolfVariant, WolfVariantRegistry,
};
use std::borrow::Cow;
pub const ASHEN: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("ashen"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_ashen"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_ashen_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_ashen_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "minecraft:snowy_taiga",
        }),
    }],
};
pub const BLACK: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("black"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_black"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_black_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_black_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "minecraft:old_growth_pine_taiga",
        }),
    }],
};
pub const CHESTNUT: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("chestnut"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_chestnut"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_chestnut_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_chestnut_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "minecraft:old_growth_spruce_taiga",
        }),
    }],
};
pub const PALE: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("pale"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const RUSTY: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("rusty"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_rusty"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_rusty_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_rusty_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:is_jungle",
        }),
    }],
};
pub const SNOWY: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("snowy"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_snowy"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_snowy_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_snowy_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "minecraft:grove",
        }),
    }],
};
pub const SPOTTED: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("spotted"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_spotted"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_spotted_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_spotted_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:is_savanna",
        }),
    }],
};
pub const STRIPED: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("striped"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_striped"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_striped_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_striped_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "#minecraft:is_badlands",
        }),
    }],
};
pub const WOODS: &WolfVariant = &WolfVariant {
    key: Identifier::vanilla_static("woods"),
    assets: WolfAssetInfo {
        wild: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_woods"),
        },
        tame: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_woods_tame"),
        },
        angry: Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("entity/wolf/wolf_woods_angry"),
        },
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 1i32,
        condition: Some(BiomeCondition {
            condition_type: "minecraft:biome",
            biomes: "minecraft:forest",
        }),
    }],
};
pub fn register_wolf_variants(registry: &mut WolfVariantRegistry) {
    registry.register(ASHEN);
    registry.register(BLACK);
    registry.register(CHESTNUT);
    registry.register(PALE);
    registry.register(RUSTY);
    registry.register(SNOWY);
    registry.register(SPOTTED);
    registry.register(STRIPED);
    registry.register(WOODS);
}
