use crate::trim_material::{StyledTextComponent, TrimMaterial, TrimMaterialRegistry};
use crate::types::Identifier;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;
pub static AMETHYST: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("amethyst"),
    asset_name: "amethyst".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.amethyst".to_string(),
        color: Some("#9A5CC6".to_string()),
    },
    override_armor_assets: HashMap::from([]),
});
pub static COPPER: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("copper"),
    asset_name: "copper".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.copper".to_string(),
        color: Some("#B4684D".to_string()),
    },
    override_armor_assets: HashMap::from([(
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("copper"),
        },
        "copper_darker".to_string(),
    )]),
});
pub static DIAMOND: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("diamond"),
    asset_name: "diamond".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.diamond".to_string(),
        color: Some("#6EECD2".to_string()),
    },
    override_armor_assets: HashMap::from([(
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("diamond"),
        },
        "diamond_darker".to_string(),
    )]),
});
pub static EMERALD: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("emerald"),
    asset_name: "emerald".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.emerald".to_string(),
        color: Some("#11A036".to_string()),
    },
    override_armor_assets: HashMap::from([]),
});
pub static GOLD: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("gold"),
    asset_name: "gold".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.gold".to_string(),
        color: Some("#DEB12D".to_string()),
    },
    override_armor_assets: HashMap::from([(
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("gold"),
        },
        "gold_darker".to_string(),
    )]),
});
pub static IRON: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("iron"),
    asset_name: "iron".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.iron".to_string(),
        color: Some("#ECECEC".to_string()),
    },
    override_armor_assets: HashMap::from([(
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("iron"),
        },
        "iron_darker".to_string(),
    )]),
});
pub static LAPIS: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("lapis"),
    asset_name: "lapis".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.lapis".to_string(),
        color: Some("#416E97".to_string()),
    },
    override_armor_assets: HashMap::from([]),
});
pub static NETHERITE: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("netherite"),
    asset_name: "netherite".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.netherite".to_string(),
        color: Some("#625859".to_string()),
    },
    override_armor_assets: HashMap::from([(
        Identifier {
            namespace: Cow::Borrowed("minecraft"),
            path: Cow::Borrowed("netherite"),
        },
        "netherite_darker".to_string(),
    )]),
});
pub static QUARTZ: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("quartz"),
    asset_name: "quartz".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.quartz".to_string(),
        color: Some("#E3D4C4".to_string()),
    },
    override_armor_assets: HashMap::from([]),
});
pub static REDSTONE: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("redstone"),
    asset_name: "redstone".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.redstone".to_string(),
        color: Some("#971607".to_string()),
    },
    override_armor_assets: HashMap::from([]),
});
pub static RESIN: LazyLock<TrimMaterial> = LazyLock::new(|| TrimMaterial {
    key: Identifier::vanilla_static("resin"),
    asset_name: "resin".to_string(),
    description: StyledTextComponent {
        translate: "trim_material.minecraft.resin".to_string(),
        color: Some("#FC7812".to_string()),
    },
    override_armor_assets: HashMap::from([]),
});
pub fn register_trim_materials(registry: &mut TrimMaterialRegistry) {
    registry.register(&AMETHYST, AMETHYST.key.clone());
    registry.register(&COPPER, COPPER.key.clone());
    registry.register(&DIAMOND, DIAMOND.key.clone());
    registry.register(&EMERALD, EMERALD.key.clone());
    registry.register(&GOLD, GOLD.key.clone());
    registry.register(&IRON, IRON.key.clone());
    registry.register(&LAPIS, LAPIS.key.clone());
    registry.register(&NETHERITE, NETHERITE.key.clone());
    registry.register(&QUARTZ, QUARTZ.key.clone());
    registry.register(&REDSTONE, REDSTONE.key.clone());
    registry.register(&RESIN, RESIN.key.clone());
}
