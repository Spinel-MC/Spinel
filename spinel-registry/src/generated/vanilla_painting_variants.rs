use crate::painting_variant::{PaintingVariant, PaintingVariantRegistry};
use crate::types::Identifier;
use spinel_utils::component::color::{NamedTextColor, TextColor};
use spinel_utils::component::text::TextComponent;
use std::borrow::Cow;
pub const ALBAN: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("alban"),
    width: 1i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("alban"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.alban.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.alban.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const AZTEC: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("aztec"),
    width: 1i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("aztec"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.aztec.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.aztec.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const AZTEC2: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("aztec2"),
    width: 1i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("aztec2"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.aztec2.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.aztec2.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const BACKYARD: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("backyard"),
    width: 3i32,
    height: 4i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("backyard"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.backyard.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.backyard.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const BAROQUE: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("baroque"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("baroque"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.baroque.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.baroque.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const BOMB: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("bomb"),
    width: 1i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("bomb"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.bomb.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.bomb.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const BOUQUET: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("bouquet"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("bouquet"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.bouquet.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.bouquet.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const BURNING_SKULL: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("burning_skull"),
    width: 4i32,
    height: 4i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("burning_skull"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.burning_skull.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.burning_skull.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const BUST: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("bust"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("bust"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.bust.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.bust.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const CAVEBIRD: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("cavebird"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("cavebird"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.cavebird.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.cavebird.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const CHANGING: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("changing"),
    width: 4i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("changing"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.changing.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.changing.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const COTAN: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("cotan"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("cotan"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.cotan.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.cotan.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const COURBET: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("courbet"),
    width: 2i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("courbet"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.courbet.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.courbet.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const CREEBET: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("creebet"),
    width: 2i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("creebet"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.creebet.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.creebet.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const DENNIS: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("dennis"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("dennis"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.dennis.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.dennis.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const DONKEY_KONG: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("donkey_kong"),
    width: 4i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("donkey_kong"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.donkey_kong.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.donkey_kong.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const EARTH: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("earth"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("earth"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.earth.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: None,
};
pub const ENDBOSS: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("endboss"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("endboss"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.endboss.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.endboss.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const FERN: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("fern"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("fern"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.fern.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.fern.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const FIGHTERS: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("fighters"),
    width: 4i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("fighters"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.fighters.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.fighters.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const FINDING: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("finding"),
    width: 4i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("finding"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.finding.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.finding.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const FIRE: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("fire"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("fire"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.fire.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: None,
};
pub const GRAHAM: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("graham"),
    width: 1i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("graham"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.graham.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.graham.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const HUMBLE: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("humble"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("humble"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.humble.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.humble.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const KEBAB: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("kebab"),
    width: 1i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("kebab"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.kebab.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.kebab.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const LOWMIST: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("lowmist"),
    width: 4i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("lowmist"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.lowmist.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.lowmist.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const MATCH: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("match"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("match"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.match.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.match.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const MEDITATIVE: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("meditative"),
    width: 1i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("meditative"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.meditative.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.meditative.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const ORB: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("orb"),
    width: 4i32,
    height: 4i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("orb"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.orb.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.orb.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const OWLEMONS: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("owlemons"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("owlemons"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.owlemons.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.owlemons.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const PASSAGE: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("passage"),
    width: 4i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("passage"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.passage.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.passage.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const PIGSCENE: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("pigscene"),
    width: 4i32,
    height: 4i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("pigscene"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.pigscene.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.pigscene.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const PLANT: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("plant"),
    width: 1i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("plant"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.plant.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.plant.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const POINTER: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("pointer"),
    width: 4i32,
    height: 4i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("pointer"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.pointer.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.pointer.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const POND: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("pond"),
    width: 3i32,
    height: 4i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("pond"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.pond.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.pond.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const POOL: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("pool"),
    width: 2i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("pool"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.pool.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.pool.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const PRAIRIE_RIDE: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("prairie_ride"),
    width: 1i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("prairie_ride"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.prairie_ride.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.prairie_ride.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const SEA: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("sea"),
    width: 2i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("sea"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.sea.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.sea.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const SKELETON: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("skeleton"),
    width: 4i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("skeleton"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.skeleton.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.skeleton.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const SKULL_AND_ROSES: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("skull_and_roses"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("skull_and_roses"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.skull_and_roses.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.skull_and_roses.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const STAGE: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("stage"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("stage"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.stage.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.stage.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const SUNFLOWERS: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("sunflowers"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("sunflowers"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.sunflowers.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.sunflowers.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const SUNSET: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("sunset"),
    width: 2i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("sunset"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.sunset.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.sunset.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const TIDES: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("tides"),
    width: 3i32,
    height: 3i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("tides"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.tides.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.tides.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const UNPACKED: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("unpacked"),
    width: 4i32,
    height: 4i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("unpacked"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.unpacked.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.unpacked.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const VOID: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("void"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("void"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.void.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.void.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const WANDERER: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("wanderer"),
    width: 1i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("wanderer"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.wanderer.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.wanderer.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const WASTELAND: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("wasteland"),
    width: 1i32,
    height: 1i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("wasteland"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.wasteland.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: Some(TextComponent::translatable_with_color(
        "painting.minecraft.wasteland.author",
        TextColor::Named(NamedTextColor::Gray),
    )),
};
pub const WATER: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("water"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("water"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.water.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: None,
};
pub const WIND: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("wind"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("wind"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.wind.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: None,
};
pub const WITHER: &PaintingVariant = &PaintingVariant {
    key: Identifier::vanilla_static("wither"),
    width: 2i32,
    height: 2i32,
    asset_id: Identifier {
        namespace: Cow::Borrowed("minecraft"),
        path: Cow::Borrowed("wither"),
    },
    title: Some(TextComponent::translatable_with_color(
        "painting.minecraft.wither.title",
        TextColor::Named(NamedTextColor::Yellow),
    )),
    author: None,
};
pub fn register_painting_variants(registry: &mut PaintingVariantRegistry) {
    registry.register(ALBAN);
    registry.register(AZTEC);
    registry.register(AZTEC2);
    registry.register(BACKYARD);
    registry.register(BAROQUE);
    registry.register(BOMB);
    registry.register(BOUQUET);
    registry.register(BURNING_SKULL);
    registry.register(BUST);
    registry.register(CAVEBIRD);
    registry.register(CHANGING);
    registry.register(COTAN);
    registry.register(COURBET);
    registry.register(CREEBET);
    registry.register(DENNIS);
    registry.register(DONKEY_KONG);
    registry.register(EARTH);
    registry.register(ENDBOSS);
    registry.register(FERN);
    registry.register(FIGHTERS);
    registry.register(FINDING);
    registry.register(FIRE);
    registry.register(GRAHAM);
    registry.register(HUMBLE);
    registry.register(KEBAB);
    registry.register(LOWMIST);
    registry.register(MATCH);
    registry.register(MEDITATIVE);
    registry.register(ORB);
    registry.register(OWLEMONS);
    registry.register(PASSAGE);
    registry.register(PIGSCENE);
    registry.register(PLANT);
    registry.register(POINTER);
    registry.register(POND);
    registry.register(POOL);
    registry.register(PRAIRIE_RIDE);
    registry.register(SEA);
    registry.register(SKELETON);
    registry.register(SKULL_AND_ROSES);
    registry.register(STAGE);
    registry.register(SUNFLOWERS);
    registry.register(SUNSET);
    registry.register(TIDES);
    registry.register(UNPACKED);
    registry.register(VOID);
    registry.register(WANDERER);
    registry.register(WASTELAND);
    registry.register(WATER);
    registry.register(WIND);
    registry.register(WITHER);
}
