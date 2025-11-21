use crate::trim_pattern::{TrimPattern, TrimPatternRegistry};
use crate::types::Identifier;
use spinel_utils::component::text::TextComponent;
use std::borrow::Cow;
pub const BOLT: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("bolt"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("bolt"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.bolt"),
    decal: false,
};
pub const COAST: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("coast"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("coast"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.coast"),
    decal: false,
};
pub const DUNE: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("dune"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("dune"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.dune"),
    decal: false,
};
pub const EYE: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("eye"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("eye"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.eye"),
    decal: false,
};
pub const FLOW: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("flow"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("flow"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.flow"),
    decal: false,
};
pub const HOST: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("host"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("host"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.host"),
    decal: false,
};
pub const RAISER: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("raiser"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("raiser"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.raiser"),
    decal: false,
};
pub const RIB: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("rib"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("rib"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.rib"),
    decal: false,
};
pub const SENTRY: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("sentry"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("sentry"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.sentry"),
    decal: false,
};
pub const SHAPER: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("shaper"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("shaper"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.shaper"),
    decal: false,
};
pub const SILENCE: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("silence"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("silence"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.silence"),
    decal: false,
};
pub const SNOUT: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("snout"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("snout"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.snout"),
    decal: false,
};
pub const SPIRE: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("spire"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("spire"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.spire"),
    decal: false,
};
pub const TIDE: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("tide"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("tide"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.tide"),
    decal: false,
};
pub const VEX: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("vex"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("vex"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.vex"),
    decal: false,
};
pub const WARD: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("ward"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("ward"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.ward"),
    decal: false,
};
pub const WAYFINDER: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("wayfinder"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("wayfinder"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.wayfinder"),
    decal: false,
};
pub const WILD: &TrimPattern = &TrimPattern {
    key: Identifier::vanilla_static("wild"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("wild"),
    },
    description: TextComponent::translatable("trim_pattern.minecraft.wild"),
    decal: false,
};
pub fn register_trim_patterns(registry: &mut TrimPatternRegistry) {
    registry.register(BOLT);
    registry.register(COAST);
    registry.register(DUNE);
    registry.register(EYE);
    registry.register(FLOW);
    registry.register(HOST);
    registry.register(RAISER);
    registry.register(RIB);
    registry.register(SENTRY);
    registry.register(SHAPER);
    registry.register(SILENCE);
    registry.register(SNOUT);
    registry.register(SPIRE);
    registry.register(TIDE);
    registry.register(VEX);
    registry.register(WARD);
    registry.register(WAYFINDER);
    registry.register(WILD);
}
