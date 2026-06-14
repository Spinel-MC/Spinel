use crate::Identifier;
use crate::ItemStack;
use crate::data_components::DataComponentType;
use crate::data_components::{
    ArmorTrim, AttackRange, AttributeList, BannerPatterns, Bee, BlockPredicates, BlocksAttacks,
    Consumable, CustomModelData, DamageResistant, DeathProtection, DebugStickState,
    EnchantmentList, Equippable, FireworkExplosion, FireworkList, Food, InstrumentComponent,
    ItemBlockState, ItemRarity, KineticWeapon, LodestoneTracker, MapDecorations, MapPostProcessing,
    PiercingWeapon, PotDecorations, PotionContents, RegistryTagReference, ResolvableProfile,
    SeededContainerLoot, SuspiciousStewEffects, SwingAnimation, Tool, TooltipDisplay,
    TypedCustomData, UnitComponent, UseCooldown, UseEffects, Weapon, WritableBookContent,
    WrittenBookContent,
};
use spinel_nbt::NbtCompound;
use spinel_utils::color::{Color, DyeColor};
use spinel_utils::component::text::TextComponent;

pub const CUSTOM_DATA: DataComponentType<NbtCompound> =
    DataComponentType::new(0, Identifier::vanilla_static("custom_data"));
pub const MAX_STACK_SIZE: DataComponentType<i32> =
    DataComponentType::new(1, Identifier::vanilla_static("max_stack_size"));
pub const MAX_DAMAGE: DataComponentType<i32> =
    DataComponentType::new(2, Identifier::vanilla_static("max_damage"));
pub const DAMAGE: DataComponentType<i32> =
    DataComponentType::new(3, Identifier::vanilla_static("damage"));
pub const UNBREAKABLE: DataComponentType<UnitComponent> =
    DataComponentType::new(4, Identifier::vanilla_static("unbreakable"));
pub const USE_EFFECTS: DataComponentType<UseEffects> =
    DataComponentType::new(5, Identifier::vanilla_static("use_effects"));
pub const CUSTOM_NAME: DataComponentType<TextComponent> =
    DataComponentType::new(6, Identifier::vanilla_static("custom_name"));
pub const MINIMUM_ATTACK_CHARGE: DataComponentType<f32> =
    DataComponentType::new(7, Identifier::vanilla_static("minimum_attack_charge"));
pub const DAMAGE_TYPE: DataComponentType<Identifier> =
    DataComponentType::new(8, Identifier::vanilla_static("damage_type"));
pub const ITEM_NAME: DataComponentType<TextComponent> =
    DataComponentType::new(9, Identifier::vanilla_static("item_name"));
pub const ITEM_MODEL: DataComponentType<String> =
    DataComponentType::new(10, Identifier::vanilla_static("item_model"));
pub const LORE: DataComponentType<Vec<TextComponent>> =
    DataComponentType::new(11, Identifier::vanilla_static("lore"));
pub const RARITY: DataComponentType<ItemRarity> =
    DataComponentType::new(12, Identifier::vanilla_static("rarity"));
pub const ENCHANTMENTS: DataComponentType<EnchantmentList> =
    DataComponentType::new(13, Identifier::vanilla_static("enchantments"));
pub const CAN_PLACE_ON: DataComponentType<BlockPredicates> =
    DataComponentType::new(14, Identifier::vanilla_static("can_place_on"));
pub const CAN_BREAK: DataComponentType<BlockPredicates> =
    DataComponentType::new(15, Identifier::vanilla_static("can_break"));
pub const ATTRIBUTE_MODIFIERS: DataComponentType<AttributeList> =
    DataComponentType::new(16, Identifier::vanilla_static("attribute_modifiers"));
pub const CUSTOM_MODEL_DATA: DataComponentType<CustomModelData> =
    DataComponentType::new(17, Identifier::vanilla_static("custom_model_data"));
pub const TOOLTIP_DISPLAY: DataComponentType<TooltipDisplay> =
    DataComponentType::new(18, Identifier::vanilla_static("tooltip_display"));
pub const REPAIR_COST: DataComponentType<i32> =
    DataComponentType::new(19, Identifier::vanilla_static("repair_cost"));
pub const CREATIVE_SLOT_LOCK: DataComponentType<UnitComponent> =
    DataComponentType::new(20, Identifier::vanilla_static("creative_slot_lock"));
pub const ENCHANTMENT_GLINT_OVERRIDE: DataComponentType<bool> =
    DataComponentType::new(21, Identifier::vanilla_static("enchantment_glint_override"));
pub const INTANGIBLE_PROJECTILE: DataComponentType<UnitComponent> =
    DataComponentType::new(22, Identifier::vanilla_static("intangible_projectile"));
pub const FOOD: DataComponentType<Food> =
    DataComponentType::new(23, Identifier::vanilla_static("food"));
pub const CONSUMABLE: DataComponentType<Consumable> =
    DataComponentType::new(24, Identifier::vanilla_static("consumable"));
pub const USE_REMAINDER: DataComponentType<ItemStack> =
    DataComponentType::new(25, Identifier::vanilla_static("use_remainder"));
pub const USE_COOLDOWN: DataComponentType<UseCooldown> =
    DataComponentType::new(26, Identifier::vanilla_static("use_cooldown"));
pub const DAMAGE_RESISTANT: DataComponentType<DamageResistant> =
    DataComponentType::new(27, Identifier::vanilla_static("damage_resistant"));
pub const TOOL: DataComponentType<Tool> =
    DataComponentType::new(28, Identifier::vanilla_static("tool"));
pub const WEAPON: DataComponentType<Weapon> =
    DataComponentType::new(29, Identifier::vanilla_static("weapon"));
pub const ATTACK_RANGE: DataComponentType<AttackRange> =
    DataComponentType::new(30, Identifier::vanilla_static("attack_range"));
pub const ENCHANTABLE: DataComponentType<i32> =
    DataComponentType::new(31, Identifier::vanilla_static("enchantable"));
pub const EQUIPPABLE: DataComponentType<Equippable> =
    DataComponentType::new(32, Identifier::vanilla_static("equippable"));
pub const REPAIRABLE: DataComponentType<RegistryTagReference> =
    DataComponentType::new(33, Identifier::vanilla_static("repairable"));
pub const GLIDER: DataComponentType<UnitComponent> =
    DataComponentType::new(34, Identifier::vanilla_static("glider"));
pub const TOOLTIP_STYLE: DataComponentType<String> =
    DataComponentType::new(35, Identifier::vanilla_static("tooltip_style"));
pub const DEATH_PROTECTION: DataComponentType<DeathProtection> =
    DataComponentType::new(36, Identifier::vanilla_static("death_protection"));
pub const BLOCKS_ATTACKS: DataComponentType<BlocksAttacks> =
    DataComponentType::new(37, Identifier::vanilla_static("blocks_attacks"));
pub const PIERCING_WEAPON: DataComponentType<PiercingWeapon> =
    DataComponentType::new(38, Identifier::vanilla_static("piercing_weapon"));
pub const KINETIC_WEAPON: DataComponentType<KineticWeapon> =
    DataComponentType::new(39, Identifier::vanilla_static("kinetic_weapon"));
pub const SWING_ANIMATION: DataComponentType<SwingAnimation> =
    DataComponentType::new(40, Identifier::vanilla_static("swing_animation"));
pub const STORED_ENCHANTMENTS: DataComponentType<EnchantmentList> =
    DataComponentType::new(41, Identifier::vanilla_static("stored_enchantments"));
pub const DYED_COLOR: DataComponentType<Color> =
    DataComponentType::new(42, Identifier::vanilla_static("dyed_color"));
pub const MAP_COLOR: DataComponentType<Color> =
    DataComponentType::new(43, Identifier::vanilla_static("map_color"));
pub const MAP_ID: DataComponentType<i32> =
    DataComponentType::new(44, Identifier::vanilla_static("map_id"));
pub const MAP_DECORATIONS: DataComponentType<MapDecorations> =
    DataComponentType::new(45, Identifier::vanilla_static("map_decorations"));
pub const MAP_POST_PROCESSING: DataComponentType<MapPostProcessing> =
    DataComponentType::new(46, Identifier::vanilla_static("map_post_processing"));
pub const CHARGED_PROJECTILES: DataComponentType<Vec<ItemStack>> =
    DataComponentType::new(47, Identifier::vanilla_static("charged_projectiles"));
pub const BUNDLE_CONTENTS: DataComponentType<Vec<ItemStack>> =
    DataComponentType::new(48, Identifier::vanilla_static("bundle_contents"));
pub const POTION_CONTENTS: DataComponentType<PotionContents> =
    DataComponentType::new(49, Identifier::vanilla_static("potion_contents"));
pub const POTION_DURATION_SCALE: DataComponentType<f32> =
    DataComponentType::new(50, Identifier::vanilla_static("potion_duration_scale"));
pub const SUSPICIOUS_STEW_EFFECTS: DataComponentType<SuspiciousStewEffects> =
    DataComponentType::new(51, Identifier::vanilla_static("suspicious_stew_effects"));
pub const WRITABLE_BOOK_CONTENT: DataComponentType<WritableBookContent> =
    DataComponentType::new(52, Identifier::vanilla_static("writable_book_content"));
pub const WRITTEN_BOOK_CONTENT: DataComponentType<WrittenBookContent> =
    DataComponentType::new(53, Identifier::vanilla_static("written_book_content"));
pub const TRIM: DataComponentType<ArmorTrim> =
    DataComponentType::new(54, Identifier::vanilla_static("trim"));
pub const DEBUG_STICK_STATE: DataComponentType<DebugStickState> =
    DataComponentType::new(55, Identifier::vanilla_static("debug_stick_state"));
pub const ENTITY_DATA: DataComponentType<TypedCustomData> =
    DataComponentType::new(56, Identifier::vanilla_static("entity_data"));
pub const BUCKET_ENTITY_DATA: DataComponentType<NbtCompound> =
    DataComponentType::new(57, Identifier::vanilla_static("bucket_entity_data"));
pub const BLOCK_ENTITY_DATA: DataComponentType<TypedCustomData> =
    DataComponentType::new(58, Identifier::vanilla_static("block_entity_data"));
pub const INSTRUMENT: DataComponentType<InstrumentComponent> =
    DataComponentType::new(59, Identifier::vanilla_static("instrument"));
pub const PROVIDES_TRIM_MATERIAL: DataComponentType<Identifier> =
    DataComponentType::new(60, Identifier::vanilla_static("provides_trim_material"));
pub const OMINOUS_BOTTLE_AMPLIFIER: DataComponentType<i32> =
    DataComponentType::new(61, Identifier::vanilla_static("ominous_bottle_amplifier"));
pub const JUKEBOX_PLAYABLE: DataComponentType<Identifier> =
    DataComponentType::new(62, Identifier::vanilla_static("jukebox_playable"));
pub const PROVIDES_BANNER_PATTERNS: DataComponentType<Identifier> =
    DataComponentType::new(63, Identifier::vanilla_static("provides_banner_patterns"));
pub const RECIPES: DataComponentType<Vec<String>> =
    DataComponentType::new(64, Identifier::vanilla_static("recipes"));
pub const LODESTONE_TRACKER: DataComponentType<LodestoneTracker> =
    DataComponentType::new(65, Identifier::vanilla_static("lodestone_tracker"));
pub const FIREWORK_EXPLOSION: DataComponentType<FireworkExplosion> =
    DataComponentType::new(66, Identifier::vanilla_static("firework_explosion"));
pub const FIREWORKS: DataComponentType<FireworkList> =
    DataComponentType::new(67, Identifier::vanilla_static("fireworks"));
pub const PROFILE: DataComponentType<ResolvableProfile> =
    DataComponentType::new(68, Identifier::vanilla_static("profile"));
pub const NOTE_BLOCK_SOUND: DataComponentType<String> =
    DataComponentType::new(69, Identifier::vanilla_static("note_block_sound"));
pub const BANNER_PATTERNS: DataComponentType<BannerPatterns> =
    DataComponentType::new(70, Identifier::vanilla_static("banner_patterns"));
pub const BASE_COLOR: DataComponentType<DyeColor> =
    DataComponentType::new(71, Identifier::vanilla_static("base_color"));
pub const POT_DECORATIONS: DataComponentType<PotDecorations> =
    DataComponentType::new(72, Identifier::vanilla_static("pot_decorations"));
pub const CONTAINER: DataComponentType<Vec<ItemStack>> =
    DataComponentType::new(73, Identifier::vanilla_static("container"));
pub const BLOCK_STATE: DataComponentType<ItemBlockState> =
    DataComponentType::new(74, Identifier::vanilla_static("block_state"));
pub const BEES: DataComponentType<Vec<Bee>> =
    DataComponentType::new(75, Identifier::vanilla_static("bees"));
pub const LOCK: DataComponentType<NbtCompound> =
    DataComponentType::new(76, Identifier::vanilla_static("lock"));
pub const CONTAINER_LOOT: DataComponentType<SeededContainerLoot> =
    DataComponentType::new(77, Identifier::vanilla_static("container_loot"));
pub const BREAK_SOUND: DataComponentType<Identifier> =
    DataComponentType::new(78, Identifier::vanilla_static("break_sound"));
pub const VILLAGER_VARIANT: DataComponentType<crate::VillagerType> =
    DataComponentType::new(79, Identifier::vanilla_static("villager/variant"));
pub const WOLF_VARIANT: DataComponentType<crate::RegistryKey<crate::wolf_variant::WolfVariant>> =
    DataComponentType::new(80, Identifier::vanilla_static("wolf/variant"));
pub const WOLF_SOUND_VARIANT: DataComponentType<
    crate::RegistryKey<crate::wolf_sound_variant::WolfSoundVariant>,
> = DataComponentType::new(81, Identifier::vanilla_static("wolf/sound_variant"));
pub const WOLF_COLLAR: DataComponentType<DyeColor> =
    DataComponentType::new(82, Identifier::vanilla_static("wolf/collar"));
pub const FOX_VARIANT: DataComponentType<crate::FoxVariant> =
    DataComponentType::new(83, Identifier::vanilla_static("fox/variant"));
pub const SALMON_SIZE: DataComponentType<crate::SalmonSize> =
    DataComponentType::new(84, Identifier::vanilla_static("salmon/size"));
pub const PARROT_VARIANT: DataComponentType<crate::ParrotColor> =
    DataComponentType::new(85, Identifier::vanilla_static("parrot/variant"));
pub const TROPICAL_FISH_PATTERN: DataComponentType<crate::TropicalFishPattern> =
    DataComponentType::new(86, Identifier::vanilla_static("tropical_fish/pattern"));
pub const TROPICAL_FISH_BASE_COLOR: DataComponentType<DyeColor> =
    DataComponentType::new(87, Identifier::vanilla_static("tropical_fish/base_color"));
pub const TROPICAL_FISH_PATTERN_COLOR: DataComponentType<DyeColor> = DataComponentType::new(
    88,
    Identifier::vanilla_static("tropical_fish/pattern_color"),
);
pub const MOOSHROOM_VARIANT: DataComponentType<crate::MooshroomVariant> =
    DataComponentType::new(89, Identifier::vanilla_static("mooshroom/variant"));
pub const RABBIT_VARIANT: DataComponentType<crate::RabbitVariant> =
    DataComponentType::new(90, Identifier::vanilla_static("rabbit/variant"));
pub const PIG_VARIANT: DataComponentType<crate::RegistryKey<crate::pig_variant::PigVariant>> =
    DataComponentType::new(91, Identifier::vanilla_static("pig/variant"));
pub const COW_VARIANT: DataComponentType<crate::RegistryKey<crate::cow_variant::CowVariant>> =
    DataComponentType::new(92, Identifier::vanilla_static("cow/variant"));
pub const CHICKEN_VARIANT: DataComponentType<
    crate::RegistryKey<crate::chicken_variant::ChickenVariant>,
> = DataComponentType::new(93, Identifier::vanilla_static("chicken/variant"));
pub const ZOMBIE_NAUTILUS_VARIANT: DataComponentType<
    crate::RegistryKey<crate::zombie_nautilus_variant::ZombieNautilusVariant>,
> = DataComponentType::new(94, Identifier::vanilla_static("zombie_nautilus/variant"));
pub const FROG_VARIANT: DataComponentType<crate::RegistryKey<crate::frog_variant::FrogVariant>> =
    DataComponentType::new(95, Identifier::vanilla_static("frog/variant"));
pub const HORSE_VARIANT: DataComponentType<crate::HorseColor> =
    DataComponentType::new(96, Identifier::vanilla_static("horse/variant"));
pub const PAINTING_VARIANT: DataComponentType<Identifier> =
    DataComponentType::new(97, Identifier::vanilla_static("painting/variant"));
pub const LLAMA_VARIANT: DataComponentType<crate::LlamaVariant> =
    DataComponentType::new(98, Identifier::vanilla_static("llama/variant"));
pub const AXOLOTL_VARIANT: DataComponentType<crate::AxolotlVariant> =
    DataComponentType::new(99, Identifier::vanilla_static("axolotl/variant"));
pub const CAT_VARIANT: DataComponentType<crate::RegistryKey<crate::cat_variant::CatVariant>> =
    DataComponentType::new(100, Identifier::vanilla_static("cat/variant"));
pub const CAT_COLLAR: DataComponentType<DyeColor> =
    DataComponentType::new(101, Identifier::vanilla_static("cat/collar"));
pub const SHEEP_COLOR: DataComponentType<DyeColor> =
    DataComponentType::new(102, Identifier::vanilla_static("sheep/color"));
pub const SHULKER_COLOR: DataComponentType<DyeColor> =
    DataComponentType::new(103, Identifier::vanilla_static("shulker/color"));
