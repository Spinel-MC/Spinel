pub mod banner_pattern;
pub mod biome;
pub mod biome_attributes;
pub mod biome_effects;
pub mod block_entity_type;
pub mod blocks;
pub mod cat_variant;
pub mod chat_type;
pub mod chicken_variant;
pub mod cow_variant;
pub mod damage_type;
pub mod dialog;
pub mod dimension_type;
pub mod dynamic_registry;
pub mod enchantment;
pub mod frog_variant;
pub mod instrument;
pub mod items;
pub mod jukebox_song;
mod nbt_builder;
pub mod painting_variant;
pub mod pig_variant;
pub mod registry_codec;
pub mod registry_key;
pub mod registry_tags;
mod simple_registry_value;
pub mod static_registry;
pub mod timeline;
pub mod trim_material;
pub mod trim_pattern;
pub mod types;
pub mod wolf_sound_variant;
pub mod wolf_variant;
pub mod zombie_nautilus_variant;

pub mod data_components;

pub use data_components::{DataComponentMap, DataComponentType};
pub use dynamic_registry::{DynamicRegistry, RegisterError, RegistryEntry, RegistrySource};
pub use registry_codec::RegistryCodec;
pub use registry_key::RegistryKey;
pub use registry_tags::{RegistryTag, RegistryTagError, RegistryTags};
pub use static_registry::{RegisterStaticError, StaticRegistry};
pub use types::{Axis, BlockStateId, Identifier, Todo};
include!("generated_modules.rs");
include!("registry_modules.rs");

pub const BLOCKS_REGISTRY: Identifier = Identifier::vanilla_static("block");
pub const ITEM_REGISTRY: Identifier = Identifier::vanilla_static("item");
pub const BANNER_PATTERN_REGISTRY: Identifier = Identifier::vanilla_static("banner_pattern");
pub const BIOME_REGISTRY: Identifier = Identifier::vanilla_static("worldgen/biome");
pub const CAT_VARIANT_REGISTRY: Identifier = Identifier::vanilla_static("cat_variant");
pub const CHAT_TYPE_REGISTRY: Identifier = Identifier::vanilla_static("chat_type");
pub const CHICKEN_VARIANT_REGISTRY: Identifier = Identifier::vanilla_static("chicken_variant");
pub const COW_VARIANT_REGISTRY: Identifier = Identifier::vanilla_static("cow_variant");
pub const DAMAGE_TYPE_REGISTRY: Identifier = Identifier::vanilla_static("damage_type");
pub const DIALOG_REGISTRY: Identifier = Identifier::vanilla_static("dialog");
pub const DIMENSION_TYPE_REGISTRY: Identifier = Identifier::vanilla_static("dimension_type");
pub const ENCHANTMENT_REGISTRY: Identifier = Identifier::vanilla_static("enchantment");
pub const FROG_VARIANT_REGISTRY: Identifier = Identifier::vanilla_static("frog_variant");
pub const INSTRUMENT_REGISTRY: Identifier = Identifier::vanilla_static("instrument");
pub const JUKEBOX_SONG_REGISTRY: Identifier = Identifier::vanilla_static("jukebox_song");
pub const PAINTING_VARIANT_REGISTRY: Identifier = Identifier::vanilla_static("painting_variant");
pub const PIG_VARIANT_REGISTRY: Identifier = Identifier::vanilla_static("pig_variant");
pub const TIMELINE_REGISTRY: Identifier = Identifier::vanilla_static("timeline");
pub const TRIM_MATERIAL_REGISTRY: Identifier = Identifier::vanilla_static("trim_material");
pub const TRIM_PATTERN_REGISTRY: Identifier = Identifier::vanilla_static("trim_pattern");
pub const WOLF_SOUND_VARIANT_REGISTRY: Identifier =
    Identifier::vanilla_static("wolf_sound_variant");
pub const WOLF_VARIANT_REGISTRY: Identifier = Identifier::vanilla_static("wolf_variant");
pub const ZOMBIE_NAUTILUS_VARIANT_REGISTRY: Identifier =
    Identifier::vanilla_static("zombie_nautilus_variant");

pub struct Registries {
    blocks: StaticRegistry<vanilla_world_blocks::Block>,
    items: StaticRegistry<items::Item>,
    biomes: DynamicRegistry<biome::Biome>,
    dimension_types: DynamicRegistry<dimension_type::DimensionType>,
    chat_types: DynamicRegistry<chat_type::ChatType>,
    damage_types: DynamicRegistry<damage_type::DamageType>,
    banner_patterns: DynamicRegistry<banner_pattern::BannerPattern>,
    cat_variants: DynamicRegistry<cat_variant::CatVariant>,
    chicken_variants: DynamicRegistry<chicken_variant::ChickenVariant>,
    cow_variants: DynamicRegistry<cow_variant::CowVariant>,
    frog_variants: DynamicRegistry<frog_variant::FrogVariant>,
    pig_variants: DynamicRegistry<pig_variant::PigVariant>,
    painting_variants: DynamicRegistry<painting_variant::PaintingVariant>,
    trim_materials: DynamicRegistry<trim_material::TrimMaterial>,
    trim_patterns: DynamicRegistry<trim_pattern::TrimPattern>,
    instruments: DynamicRegistry<instrument::Instrument>,
    jukebox_songs: DynamicRegistry<jukebox_song::JukeboxSong>,
    wolf_variants: DynamicRegistry<wolf_variant::WolfVariant>,
    wolf_sound_variants: DynamicRegistry<wolf_sound_variant::WolfSoundVariant>,
    dialogs: DynamicRegistry<dialog::Dialog>,
    enchantments: DynamicRegistry<enchantment::Enchantment>,
    timelines: DynamicRegistry<timeline::Timeline>,
    zombie_nautilus_variants: DynamicRegistry<zombie_nautilus_variant::ZombieNautilusVariant>,
}

impl Registries {
    #[must_use]
    pub fn new_vanilla() -> Self {
        let mut registries = Self::empty();
        registries.register_vanilla();
        registries
    }

    #[must_use]
    pub fn empty() -> Self {
        Self {
            blocks: StaticRegistry::new(),
            items: StaticRegistry::new(),
            biomes: DynamicRegistry::new(BIOME_REGISTRY),
            dimension_types: DynamicRegistry::new(DIMENSION_TYPE_REGISTRY),
            chat_types: DynamicRegistry::new(CHAT_TYPE_REGISTRY),
            damage_types: DynamicRegistry::new(DAMAGE_TYPE_REGISTRY),
            banner_patterns: DynamicRegistry::new(BANNER_PATTERN_REGISTRY),
            cat_variants: DynamicRegistry::new(CAT_VARIANT_REGISTRY),
            chicken_variants: DynamicRegistry::new(CHICKEN_VARIANT_REGISTRY),
            cow_variants: DynamicRegistry::new(COW_VARIANT_REGISTRY),
            frog_variants: DynamicRegistry::new(FROG_VARIANT_REGISTRY),
            pig_variants: DynamicRegistry::new(PIG_VARIANT_REGISTRY),
            painting_variants: DynamicRegistry::new(PAINTING_VARIANT_REGISTRY),
            trim_materials: DynamicRegistry::new(TRIM_MATERIAL_REGISTRY),
            trim_patterns: DynamicRegistry::new(TRIM_PATTERN_REGISTRY),
            instruments: DynamicRegistry::new(INSTRUMENT_REGISTRY),
            jukebox_songs: DynamicRegistry::new(JUKEBOX_SONG_REGISTRY),
            wolf_variants: DynamicRegistry::new(WOLF_VARIANT_REGISTRY),
            wolf_sound_variants: DynamicRegistry::new(WOLF_SOUND_VARIANT_REGISTRY),
            dialogs: DynamicRegistry::new(DIALOG_REGISTRY),
            enchantments: DynamicRegistry::new(ENCHANTMENT_REGISTRY),
            timelines: DynamicRegistry::new(TIMELINE_REGISTRY),
            zombie_nautilus_variants: DynamicRegistry::new(ZOMBIE_NAUTILUS_VARIANT_REGISTRY),
        }
    }

    pub fn biome(&self) -> &DynamicRegistry<biome::Biome> {
        &self.biomes
    }

    pub fn biome_mut(&mut self) -> &mut DynamicRegistry<biome::Biome> {
        &mut self.biomes
    }

    pub fn freeze(&mut self) {
        self.blocks.freeze();
        self.items.freeze();
        self.biomes.freeze();
        self.dimension_types.freeze();
        self.chat_types.freeze();
        self.damage_types.freeze();
        self.banner_patterns.freeze();
        self.cat_variants.freeze();
        self.chicken_variants.freeze();
        self.cow_variants.freeze();
        self.frog_variants.freeze();
        self.pig_variants.freeze();
        self.painting_variants.freeze();
        self.trim_materials.freeze();
        self.trim_patterns.freeze();
        self.instruments.freeze();
        self.jukebox_songs.freeze();
        self.wolf_variants.freeze();
        self.wolf_sound_variants.freeze();
        self.dialogs.freeze();
        self.enchantments.freeze();
        self.timelines.freeze();
        self.zombie_nautilus_variants.freeze();
    }

    fn register_vanilla(&mut self) {
        vanilla_blocks::register_blocks(&mut self.blocks);
        vanilla_items::register_items(&mut self.items);
        vanilla_biomes::register_biomes(&mut self.biomes);
        vanilla_dimension_types::register_dimension_types(&mut self.dimension_types);
        self.register_dynamic_vanilla();
    }
}

impl Default for Registries {
    fn default() -> Self {
        Self::new_vanilla()
    }
}
