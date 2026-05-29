pub mod biome;
pub mod blocks;
pub mod data_components;
pub mod dimension_type;
pub mod entity;
pub mod generated;
pub mod identifier;
pub mod item_stack;
pub mod material;
pub mod registry;
pub mod registry_values;
pub mod sound;

pub use biome::{Biome, BiomeAttribute, BiomeAttributes, BiomeEffects, Color, GrassColorModifier};
pub use data_components::{
    ArmorTrim, AttackRange, AttributeList, AttributeModifierDisplay, AttributeModifierEntry,
    AttributeOperation, BannerPatternLayer, BannerPatterns, Bee, BlockPredicate, BlockPredicates,
    BlocksAttacks, Consumable, ConsumeEffect, CustomModelData, CustomPotionEffect, DamageReduction,
    DamageResistant, DataComponentDescriptor, DataComponentMap, DataComponentMapBuilder,
    DataComponentPatchBuilder, DataComponentPredicates, DataComponentType, DataComponentValue,
    DeathProtection, DebugStickState, DecodeDataComponentMapError, EnchantmentList,
    EquipmentSlotGroup, Equippable, EquippableSlot, FilteredComponent, FilteredString,
    FireworkExplosion, FireworkExplosionShape, FireworkList, Food, GameProfileProperty,
    InstrumentComponent, ItemAnimation, ItemBlockState, ItemDamageFunction, ItemRarity,
    KineticWeapon, KineticWeaponCondition, LodestoneTracker, MapDecorationEntry, MapDecorations,
    MapPostProcessing, PiercingWeapon, PotDecorations, PotionContents, PotionEffectSettings,
    PropertiesPredicate, PropertyValuePredicate, RegistryTagReference, ResolvableProfile,
    SeededContainerLoot, SuspiciousStewEffect, SuspiciousStewEffects, SwingAnimation,
    SwingAnimationType, Tool, ToolRule, TooltipDisplay, TypedCustomData, UnitComponent,
    UseCooldown, UseEffects, Weapon, WorldPosition, WritableBookContent, WrittenBookContent,
    dye_color_from_nbt_name, dye_color_nbt_name, dye_color_protocol_id,
};
pub use entity::{EntityAttachmentOffset, EntityBoundingBox, EntityPacketType, EntityType};
pub use generated::{
    vanilla_biomes, vanilla_blocks, vanilla_dimension_types, vanilla_entity_types, vanilla_items,
    vanilla_materials, vanilla_sound_events, vanilla_world_blocks,
};
pub use identifier::{Axis, BlockStateId, Identifier, Todo};
pub use item_stack::{ItemStack, ItemStackBuilder};
pub use material::Material;
pub use registry::{
    BANNER_PATTERN_REGISTRY, BIOME_REGISTRY, BLOCKS_REGISTRY, CAT_VARIANT_REGISTRY,
    CHAT_TYPE_REGISTRY, CHICKEN_VARIANT_REGISTRY, COW_VARIANT_REGISTRY, DAMAGE_TYPE_REGISTRY,
    DIALOG_REGISTRY, DIMENSION_TYPE_REGISTRY, DynamicRegistry, ENCHANTMENT_REGISTRY,
    FROG_VARIANT_REGISTRY, INSTRUMENT_REGISTRY, ITEM_REGISTRY, JUKEBOX_SONG_REGISTRY,
    PAINTING_VARIANT_REGISTRY, PIG_VARIANT_REGISTRY, RegisterError, RegisterStaticError,
    Registries, RegistryCodec, RegistryEntry, RegistryKey, RegistrySource, RegistryTag,
    RegistryTagError, RegistryTags, StaticRegistry, TIMELINE_REGISTRY, TRIM_MATERIAL_REGISTRY,
    TRIM_PATTERN_REGISTRY, WOLF_SOUND_VARIANT_REGISTRY, WOLF_VARIANT_REGISTRY,
    ZOMBIE_NAUTILUS_VARIANT_REGISTRY, vanilla_banner_patterns, vanilla_cat_variants,
    vanilla_chat_types, vanilla_chicken_variants, vanilla_cow_variants, vanilla_damage_types,
    vanilla_dialogs, vanilla_enchantments, vanilla_frog_variants, vanilla_instruments,
    vanilla_jukebox_songs, vanilla_painting_variants, vanilla_pig_variants, vanilla_timelines,
    vanilla_trim_materials, vanilla_trim_patterns, vanilla_wolf_sound_variants,
    vanilla_wolf_variants, vanilla_zombie_nautilus_variants,
};
pub use sound::BuiltinSoundEvent;

pub mod sound_event {
    pub use crate::sound::BuiltinSoundEvent;
}

pub mod block_entity_type {
    pub use crate::blocks::BlockEntityType;
}

pub mod banner_pattern {
    pub use crate::registry_values::banner_pattern::*;
}

pub mod biome_attributes {
    pub use crate::biome::attributes::*;
}

pub mod biome_effects {
    pub use crate::biome::effects::*;
}

pub mod cat_variant {
    pub use crate::registry_values::cat_variant::*;
}

pub mod chat_type {
    pub use crate::registry_values::chat_type::*;
}

pub mod chicken_variant {
    pub use crate::registry_values::chicken_variant::*;
}

pub mod cow_variant {
    pub use crate::registry_values::cow_variant::*;
}

pub mod damage_type {
    pub use crate::registry_values::damage_type::*;
}

pub mod dialog {
    pub use crate::registry_values::dialog::*;
}

pub mod dynamic_registry {
    pub use crate::registry::{DynamicRegistry, RegisterError, RegistryEntry, RegistrySource};
}

pub mod enchantment {
    pub use crate::registry_values::enchantment::*;
}

pub mod entity_type {
    pub use crate::entity::{
        EntityAttachmentOffset, EntityBoundingBox, EntityPacketType, EntityType,
    };
}

pub mod frog_variant {
    pub use crate::registry_values::frog_variant::*;
}

pub mod instrument {
    pub use crate::registry_values::instrument::*;
}

pub mod items {
    pub use crate::material::Material;
}

pub mod jukebox_song {
    pub use crate::registry_values::jukebox_song::*;
}

pub mod painting_variant {
    pub use crate::registry_values::painting_variant::*;
}

pub mod pig_variant {
    pub use crate::registry_values::pig_variant::*;
}

pub mod registry_codec {
    pub use crate::registry::RegistryCodec;
}

pub mod registry_key {
    pub use crate::registry::RegistryKey;
}

pub mod registry_tags {
    pub(crate) use crate::registry::tags::{dynamic_registry_tags, static_registry_tags};
    pub use crate::registry::{RegistryTag, RegistryTagError, RegistryTags};
}

pub mod registries {
    pub use crate::registry::{
        BANNER_PATTERN_REGISTRY, BIOME_REGISTRY, BLOCKS_REGISTRY, CAT_VARIANT_REGISTRY,
        CHAT_TYPE_REGISTRY, CHICKEN_VARIANT_REGISTRY, COW_VARIANT_REGISTRY, DAMAGE_TYPE_REGISTRY,
        DIALOG_REGISTRY, DIMENSION_TYPE_REGISTRY, ENCHANTMENT_REGISTRY, FROG_VARIANT_REGISTRY,
        INSTRUMENT_REGISTRY, ITEM_REGISTRY, JUKEBOX_SONG_REGISTRY, PAINTING_VARIANT_REGISTRY,
        PIG_VARIANT_REGISTRY, Registries, TIMELINE_REGISTRY, TRIM_MATERIAL_REGISTRY,
        TRIM_PATTERN_REGISTRY, WOLF_SOUND_VARIANT_REGISTRY, WOLF_VARIANT_REGISTRY,
        ZOMBIE_NAUTILUS_VARIANT_REGISTRY, vanilla_banner_patterns, vanilla_cat_variants,
        vanilla_chat_types, vanilla_chicken_variants, vanilla_cow_variants, vanilla_damage_types,
        vanilla_dialogs, vanilla_enchantments, vanilla_frog_variants, vanilla_instruments,
        vanilla_jukebox_songs, vanilla_painting_variants, vanilla_pig_variants, vanilla_timelines,
        vanilla_trim_materials, vanilla_trim_patterns, vanilla_wolf_sound_variants,
        vanilla_wolf_variants, vanilla_zombie_nautilus_variants,
    };
}

pub mod static_registry {
    pub use crate::registry::{RegisterStaticError, StaticRegistry};
}

pub mod timeline {
    pub use crate::registry_values::timeline::*;
}

pub mod trim_material {
    pub use crate::registry_values::trim_material::*;
}

pub mod trim_pattern {
    pub use crate::registry_values::trim_pattern::*;
}

pub mod types {
    pub use crate::identifier::{Axis, BlockStateId, Identifier, Todo};
}

pub mod wolf_sound_variant {
    pub use crate::registry_values::wolf_sound_variant::*;
}

pub mod wolf_variant {
    pub use crate::registry_values::wolf_variant::*;
}

pub mod zombie_nautilus_variant {
    pub use crate::registry_values::zombie_nautilus_variant::*;
}
