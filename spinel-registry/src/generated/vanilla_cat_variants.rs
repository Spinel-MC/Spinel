use crate::cat_variant::{CatVariant, CatVariantRegistry, SpawnCondition, SpawnConditionEntry};
use crate::types::Identifier;
use std::borrow::Cow;
pub const ALL_BLACK: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("all_black"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/all_black"),
    },
    spawn_conditions: &[
        SpawnConditionEntry {
            priority: 1i32,
            condition: Some(SpawnCondition::Structure {
                structures: "#minecraft:cats_spawn_as_black",
            }),
        },
        SpawnConditionEntry {
            priority: 0i32,
            condition: Some(SpawnCondition::MoonBrightness {
                min: Some(0.9f32),
                max: None,
            }),
        },
    ],
};
pub const BLACK: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("black"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/black"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const BRITISH_SHORTHAIR: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("british_shorthair"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/british_shorthair"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const CALICO: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("calico"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/calico"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const JELLIE: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("jellie"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/jellie"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const PERSIAN: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("persian"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/persian"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const RAGDOLL: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("ragdoll"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/ragdoll"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const RED: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("red"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/red"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const SIAMESE: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("siamese"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/siamese"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const TABBY: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("tabby"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/tabby"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub const WHITE: &CatVariant = &CatVariant {
    key: Identifier::vanilla_static("white"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("entity/cat/white"),
    },
    spawn_conditions: &[SpawnConditionEntry {
        priority: 0i32,
        condition: None,
    }],
};
pub fn register_cat_variants(registry: &mut CatVariantRegistry) {
    registry.register(ALL_BLACK);
    registry.register(BLACK);
    registry.register(BRITISH_SHORTHAIR);
    registry.register(CALICO);
    registry.register(JELLIE);
    registry.register(PERSIAN);
    registry.register(RAGDOLL);
    registry.register(RED);
    registry.register(SIAMESE);
    registry.register(TABBY);
    registry.register(WHITE);
}
