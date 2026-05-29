use super::super::{dynamic::DynamicRegistry, static_registry::StaticRegistry};
use crate::{
    Identifier, Material, RegistryKey, banner_pattern, biome, cat_variant, chat_type,
    chicken_variant, cow_variant, damage_type, dialog, dimension_type, enchantment, frog_variant,
    instrument, jukebox_song, painting_variant, pig_variant, timeline, trim_material, trim_pattern,
    vanilla_biomes, vanilla_blocks, vanilla_dimension_types, vanilla_items, vanilla_world_blocks,
    wolf_sound_variant, wolf_variant, zombie_nautilus_variant,
};

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
pub const ENTITY_TYPE_REGISTRY: Identifier = Identifier::vanilla_static("entity_type");
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
    pub(super) blocks: StaticRegistry<vanilla_world_blocks::Block>,
    pub(super) items: StaticRegistry<Material>,
    pub(super) biomes: DynamicRegistry<biome::Biome>,
    pub(super) dimension_types: DynamicRegistry<dimension_type::DimensionType>,
    pub(super) chat_types: DynamicRegistry<chat_type::ChatType>,
    pub(super) damage_types: DynamicRegistry<damage_type::DamageType>,
    pub(super) banner_patterns: DynamicRegistry<banner_pattern::BannerPattern>,
    pub(super) cat_variants: DynamicRegistry<cat_variant::CatVariant>,
    pub(super) chicken_variants: DynamicRegistry<chicken_variant::ChickenVariant>,
    pub(super) cow_variants: DynamicRegistry<cow_variant::CowVariant>,
    pub(super) frog_variants: DynamicRegistry<frog_variant::FrogVariant>,
    pub(super) pig_variants: DynamicRegistry<pig_variant::PigVariant>,
    pub(super) painting_variants: DynamicRegistry<painting_variant::PaintingVariant>,
    pub(super) trim_materials: DynamicRegistry<trim_material::TrimMaterial>,
    pub(super) trim_patterns: DynamicRegistry<trim_pattern::TrimPattern>,
    pub(super) instruments: DynamicRegistry<instrument::Instrument>,
    pub(super) jukebox_songs: DynamicRegistry<jukebox_song::JukeboxSong>,
    pub(super) wolf_variants: DynamicRegistry<wolf_variant::WolfVariant>,
    pub(super) wolf_sound_variants: DynamicRegistry<wolf_sound_variant::WolfSoundVariant>,
    pub(super) dialogs: DynamicRegistry<dialog::Dialog>,
    pub(super) enchantments: DynamicRegistry<enchantment::Enchantment>,
    pub(super) timelines: DynamicRegistry<timeline::Timeline>,
    pub(super) zombie_nautilus_variants:
        DynamicRegistry<zombie_nautilus_variant::ZombieNautilusVariant>,
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

    pub fn block_id(&self, block: &vanilla_world_blocks::Block) -> Option<i32> {
        self.blocks.get_id(self.blocks.key_for(block)?)
    }

    pub fn block_key(&self, block: &vanilla_world_blocks::Block) -> Option<&Identifier> {
        self.blocks.key_for(block).map(RegistryKey::key)
    }

    pub fn dynamic_registry_id(
        &self,
        registry_name: &Identifier,
        entry_name: &Identifier,
    ) -> Option<i32> {
        if registry_name == &DAMAGE_TYPE_REGISTRY {
            return dynamic_registry_id_for(&self.damage_types, entry_name);
        }
        if registry_name == &BANNER_PATTERN_REGISTRY {
            return dynamic_registry_id_for(&self.banner_patterns, entry_name);
        }
        if registry_name == &CAT_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.cat_variants, entry_name);
        }
        if registry_name == &CHICKEN_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.chicken_variants, entry_name);
        }
        if registry_name == &COW_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.cow_variants, entry_name);
        }
        if registry_name == &FROG_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.frog_variants, entry_name);
        }
        if registry_name == &INSTRUMENT_REGISTRY {
            return dynamic_registry_id_for(&self.instruments, entry_name);
        }
        if registry_name == &JUKEBOX_SONG_REGISTRY {
            return dynamic_registry_id_for(&self.jukebox_songs, entry_name);
        }
        if registry_name == &PAINTING_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.painting_variants, entry_name);
        }
        if registry_name == &PIG_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.pig_variants, entry_name);
        }
        if registry_name == &TRIM_MATERIAL_REGISTRY {
            return dynamic_registry_id_for(&self.trim_materials, entry_name);
        }
        if registry_name == &TRIM_PATTERN_REGISTRY {
            return dynamic_registry_id_for(&self.trim_patterns, entry_name);
        }
        if registry_name == &WOLF_SOUND_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.wolf_sound_variants, entry_name);
        }
        if registry_name == &WOLF_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.wolf_variants, entry_name);
        }
        if registry_name == &ZOMBIE_NAUTILUS_VARIANT_REGISTRY {
            return dynamic_registry_id_for(&self.zombie_nautilus_variants, entry_name);
        }
        None
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

fn dynamic_registry_id_for<T>(
    registry: &DynamicRegistry<T>,
    entry_name: &Identifier,
) -> Option<i32> {
    registry.get_id(&RegistryKey::new(entry_name.clone()))
}
