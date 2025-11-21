use crate::banner_pattern::{BannerPattern, BannerPatternRegistry};
use crate::types::Identifier;
use std::borrow::Cow;
pub const BASE: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("base"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("base"),
    },
    translation_key: "block.minecraft.banner.base",
};
pub const BORDER: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("border"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("border"),
    },
    translation_key: "block.minecraft.banner.border",
};
pub const BRICKS: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("bricks"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("bricks"),
    },
    translation_key: "block.minecraft.banner.bricks",
};
pub const CIRCLE: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("circle"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("circle"),
    },
    translation_key: "block.minecraft.banner.circle",
};
pub const CREEPER: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("creeper"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("creeper"),
    },
    translation_key: "block.minecraft.banner.creeper",
};
pub const CROSS: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("cross"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("cross"),
    },
    translation_key: "block.minecraft.banner.cross",
};
pub const CURLY_BORDER: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("curly_border"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("curly_border"),
    },
    translation_key: "block.minecraft.banner.curly_border",
};
pub const DIAGONAL_LEFT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("diagonal_left"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("diagonal_left"),
    },
    translation_key: "block.minecraft.banner.diagonal_left",
};
pub const DIAGONAL_RIGHT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("diagonal_right"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("diagonal_right"),
    },
    translation_key: "block.minecraft.banner.diagonal_right",
};
pub const DIAGONAL_UP_LEFT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("diagonal_up_left"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("diagonal_up_left"),
    },
    translation_key: "block.minecraft.banner.diagonal_up_left",
};
pub const DIAGONAL_UP_RIGHT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("diagonal_up_right"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("diagonal_up_right"),
    },
    translation_key: "block.minecraft.banner.diagonal_up_right",
};
pub const FLOW: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("flow"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("flow"),
    },
    translation_key: "block.minecraft.banner.flow",
};
pub const FLOWER: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("flower"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("flower"),
    },
    translation_key: "block.minecraft.banner.flower",
};
pub const GLOBE: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("globe"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("globe"),
    },
    translation_key: "block.minecraft.banner.globe",
};
pub const GRADIENT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("gradient"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("gradient"),
    },
    translation_key: "block.minecraft.banner.gradient",
};
pub const GRADIENT_UP: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("gradient_up"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("gradient_up"),
    },
    translation_key: "block.minecraft.banner.gradient_up",
};
pub const GUSTER: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("guster"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("guster"),
    },
    translation_key: "block.minecraft.banner.guster",
};
pub const HALF_HORIZONTAL: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("half_horizontal"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("half_horizontal"),
    },
    translation_key: "block.minecraft.banner.half_horizontal",
};
pub const HALF_HORIZONTAL_BOTTOM: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("half_horizontal_bottom"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("half_horizontal_bottom"),
    },
    translation_key: "block.minecraft.banner.half_horizontal_bottom",
};
pub const HALF_VERTICAL: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("half_vertical"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("half_vertical"),
    },
    translation_key: "block.minecraft.banner.half_vertical",
};
pub const HALF_VERTICAL_RIGHT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("half_vertical_right"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("half_vertical_right"),
    },
    translation_key: "block.minecraft.banner.half_vertical_right",
};
pub const MOJANG: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("mojang"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("mojang"),
    },
    translation_key: "block.minecraft.banner.mojang",
};
pub const PIGLIN: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("piglin"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("piglin"),
    },
    translation_key: "block.minecraft.banner.piglin",
};
pub const RHOMBUS: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("rhombus"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("rhombus"),
    },
    translation_key: "block.minecraft.banner.rhombus",
};
pub const SKULL: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("skull"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("skull"),
    },
    translation_key: "block.minecraft.banner.skull",
};
pub const SMALL_STRIPES: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("small_stripes"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("small_stripes"),
    },
    translation_key: "block.minecraft.banner.small_stripes",
};
pub const SQUARE_BOTTOM_LEFT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("square_bottom_left"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("square_bottom_left"),
    },
    translation_key: "block.minecraft.banner.square_bottom_left",
};
pub const SQUARE_BOTTOM_RIGHT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("square_bottom_right"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("square_bottom_right"),
    },
    translation_key: "block.minecraft.banner.square_bottom_right",
};
pub const SQUARE_TOP_LEFT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("square_top_left"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("square_top_left"),
    },
    translation_key: "block.minecraft.banner.square_top_left",
};
pub const SQUARE_TOP_RIGHT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("square_top_right"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("square_top_right"),
    },
    translation_key: "block.minecraft.banner.square_top_right",
};
pub const STRAIGHT_CROSS: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("straight_cross"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("straight_cross"),
    },
    translation_key: "block.minecraft.banner.straight_cross",
};
pub const STRIPE_BOTTOM: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("stripe_bottom"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stripe_bottom"),
    },
    translation_key: "block.minecraft.banner.stripe_bottom",
};
pub const STRIPE_CENTER: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("stripe_center"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stripe_center"),
    },
    translation_key: "block.minecraft.banner.stripe_center",
};
pub const STRIPE_DOWNLEFT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("stripe_downleft"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stripe_downleft"),
    },
    translation_key: "block.minecraft.banner.stripe_downleft",
};
pub const STRIPE_DOWNRIGHT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("stripe_downright"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stripe_downright"),
    },
    translation_key: "block.minecraft.banner.stripe_downright",
};
pub const STRIPE_LEFT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("stripe_left"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stripe_left"),
    },
    translation_key: "block.minecraft.banner.stripe_left",
};
pub const STRIPE_MIDDLE: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("stripe_middle"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stripe_middle"),
    },
    translation_key: "block.minecraft.banner.stripe_middle",
};
pub const STRIPE_RIGHT: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("stripe_right"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stripe_right"),
    },
    translation_key: "block.minecraft.banner.stripe_right",
};
pub const STRIPE_TOP: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("stripe_top"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stripe_top"),
    },
    translation_key: "block.minecraft.banner.stripe_top",
};
pub const TRIANGLE_BOTTOM: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("triangle_bottom"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("triangle_bottom"),
    },
    translation_key: "block.minecraft.banner.triangle_bottom",
};
pub const TRIANGLE_TOP: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("triangle_top"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("triangle_top"),
    },
    translation_key: "block.minecraft.banner.triangle_top",
};
pub const TRIANGLES_BOTTOM: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("triangles_bottom"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("triangles_bottom"),
    },
    translation_key: "block.minecraft.banner.triangles_bottom",
};
pub const TRIANGLES_TOP: &BannerPattern = &BannerPattern {
    key: Identifier::vanilla_static("triangles_top"),
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("triangles_top"),
    },
    translation_key: "block.minecraft.banner.triangles_top",
};
pub fn register_banner_patterns(registry: &mut BannerPatternRegistry) {
    registry.register(BASE);
    registry.register(BORDER);
    registry.register(BRICKS);
    registry.register(CIRCLE);
    registry.register(CREEPER);
    registry.register(CROSS);
    registry.register(CURLY_BORDER);
    registry.register(DIAGONAL_LEFT);
    registry.register(DIAGONAL_RIGHT);
    registry.register(DIAGONAL_UP_LEFT);
    registry.register(DIAGONAL_UP_RIGHT);
    registry.register(FLOW);
    registry.register(FLOWER);
    registry.register(GLOBE);
    registry.register(GRADIENT);
    registry.register(GRADIENT_UP);
    registry.register(GUSTER);
    registry.register(HALF_HORIZONTAL);
    registry.register(HALF_HORIZONTAL_BOTTOM);
    registry.register(HALF_VERTICAL);
    registry.register(HALF_VERTICAL_RIGHT);
    registry.register(MOJANG);
    registry.register(PIGLIN);
    registry.register(RHOMBUS);
    registry.register(SKULL);
    registry.register(SMALL_STRIPES);
    registry.register(SQUARE_BOTTOM_LEFT);
    registry.register(SQUARE_BOTTOM_RIGHT);
    registry.register(SQUARE_TOP_LEFT);
    registry.register(SQUARE_TOP_RIGHT);
    registry.register(STRAIGHT_CROSS);
    registry.register(STRIPE_BOTTOM);
    registry.register(STRIPE_CENTER);
    registry.register(STRIPE_DOWNLEFT);
    registry.register(STRIPE_DOWNRIGHT);
    registry.register(STRIPE_LEFT);
    registry.register(STRIPE_MIDDLE);
    registry.register(STRIPE_RIGHT);
    registry.register(STRIPE_TOP);
    registry.register(TRIANGLE_BOTTOM);
    registry.register(TRIANGLE_TOP);
    registry.register(TRIANGLES_BOTTOM);
    registry.register(TRIANGLES_TOP);
}
