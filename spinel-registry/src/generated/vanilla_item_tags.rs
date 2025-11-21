use crate::items::ItemRegistry;
use crate::types::Identifier;
pub const ACACIA_LOGS_TAG: &[&str] = &[
    "acacia_log",
    "acacia_wood",
    "stripped_acacia_log",
    "stripped_acacia_wood",
];
pub const ANVIL_TAG: &[&str] = &["anvil", "chipped_anvil", "damaged_anvil"];
pub const ARMADILLO_FOOD_TAG: &[&str] = &["spider_eye"];
pub const ARROWS_TAG: &[&str] = &["arrow", "tipped_arrow", "spectral_arrow"];
pub const AXES_TAG: &[&str] = &[
    "diamond_axe",
    "stone_axe",
    "golden_axe",
    "netherite_axe",
    "wooden_axe",
    "iron_axe",
    "copper_axe",
];
pub const AXOLOTL_FOOD_TAG: &[&str] = &["tropical_fish_bucket"];
pub const BAMBOO_BLOCKS_TAG: &[&str] = &["bamboo_block", "stripped_bamboo_block"];
pub const BANNERS_TAG: &[&str] = &[
    "white_banner",
    "orange_banner",
    "magenta_banner",
    "light_blue_banner",
    "yellow_banner",
    "lime_banner",
    "pink_banner",
    "gray_banner",
    "light_gray_banner",
    "cyan_banner",
    "purple_banner",
    "blue_banner",
    "brown_banner",
    "green_banner",
    "red_banner",
    "black_banner",
];
pub const BARS_TAG: &[&str] = &[
    "iron_bars",
    "copper_bars",
    "waxed_copper_bars",
    "exposed_copper_bars",
    "waxed_exposed_copper_bars",
    "weathered_copper_bars",
    "waxed_weathered_copper_bars",
    "oxidized_copper_bars",
    "waxed_oxidized_copper_bars",
];
pub const BEACON_PAYMENT_ITEMS_TAG: &[&str] = &[
    "netherite_ingot",
    "emerald",
    "diamond",
    "gold_ingot",
    "iron_ingot",
];
pub const BEDS_TAG: &[&str] = &[
    "red_bed",
    "black_bed",
    "blue_bed",
    "brown_bed",
    "cyan_bed",
    "gray_bed",
    "green_bed",
    "light_blue_bed",
    "light_gray_bed",
    "lime_bed",
    "magenta_bed",
    "orange_bed",
    "pink_bed",
    "purple_bed",
    "white_bed",
    "yellow_bed",
];
pub const BEE_FOOD_TAG: &[&str] = &[
    "dandelion",
    "open_eyeblossom",
    "poppy",
    "blue_orchid",
    "allium",
    "azure_bluet",
    "red_tulip",
    "orange_tulip",
    "white_tulip",
    "pink_tulip",
    "oxeye_daisy",
    "cornflower",
    "lily_of_the_valley",
    "wither_rose",
    "torchflower",
    "sunflower",
    "lilac",
    "peony",
    "rose_bush",
    "pitcher_plant",
    "flowering_azalea_leaves",
    "flowering_azalea",
    "mangrove_propagule",
    "cherry_leaves",
    "pink_petals",
    "wildflowers",
    "chorus_flower",
    "spore_blossom",
    "cactus_flower",
];
pub const BIRCH_LOGS_TAG: &[&str] = &[
    "birch_log",
    "birch_wood",
    "stripped_birch_log",
    "stripped_birch_wood",
];
pub const BOATS_TAG: &[&str] = &[
    "oak_boat",
    "spruce_boat",
    "birch_boat",
    "jungle_boat",
    "acacia_boat",
    "dark_oak_boat",
    "pale_oak_boat",
    "mangrove_boat",
    "bamboo_raft",
    "cherry_boat",
    "oak_chest_boat",
    "spruce_chest_boat",
    "birch_chest_boat",
    "jungle_chest_boat",
    "acacia_chest_boat",
    "dark_oak_chest_boat",
    "pale_oak_chest_boat",
    "mangrove_chest_boat",
    "bamboo_chest_raft",
    "cherry_chest_boat",
];
pub const BOOK_CLONING_TARGET_TAG: &[&str] = &["writable_book"];
pub const BOOKSHELF_BOOKS_TAG: &[&str] = &[
    "book",
    "written_book",
    "enchanted_book",
    "writable_book",
    "knowledge_book",
];
pub const BREAKS_DECORATED_POTS_TAG: &[&str] = &[
    "diamond_sword",
    "stone_sword",
    "golden_sword",
    "netherite_sword",
    "wooden_sword",
    "iron_sword",
    "copper_sword",
    "diamond_axe",
    "stone_axe",
    "golden_axe",
    "netherite_axe",
    "wooden_axe",
    "iron_axe",
    "copper_axe",
    "diamond_pickaxe",
    "stone_pickaxe",
    "golden_pickaxe",
    "netherite_pickaxe",
    "wooden_pickaxe",
    "iron_pickaxe",
    "copper_pickaxe",
    "diamond_shovel",
    "stone_shovel",
    "golden_shovel",
    "netherite_shovel",
    "wooden_shovel",
    "iron_shovel",
    "copper_shovel",
    "diamond_hoe",
    "stone_hoe",
    "golden_hoe",
    "netherite_hoe",
    "wooden_hoe",
    "iron_hoe",
    "copper_hoe",
    "trident",
    "mace",
];
pub const BREWING_FUEL_TAG: &[&str] = &["blaze_powder"];
pub const BUNDLES_TAG: &[&str] = &[
    "bundle",
    "black_bundle",
    "blue_bundle",
    "brown_bundle",
    "cyan_bundle",
    "gray_bundle",
    "green_bundle",
    "light_blue_bundle",
    "light_gray_bundle",
    "lime_bundle",
    "magenta_bundle",
    "orange_bundle",
    "pink_bundle",
    "purple_bundle",
    "red_bundle",
    "yellow_bundle",
    "white_bundle",
];
pub const BUTTONS_TAG: &[&str] = &[
    "oak_button",
    "spruce_button",
    "birch_button",
    "jungle_button",
    "acacia_button",
    "dark_oak_button",
    "pale_oak_button",
    "crimson_button",
    "warped_button",
    "mangrove_button",
    "bamboo_button",
    "cherry_button",
    "stone_button",
    "polished_blackstone_button",
];
pub const CAMEL_FOOD_TAG: &[&str] = &["cactus"];
pub const CANDLES_TAG: &[&str] = &[
    "candle",
    "white_candle",
    "orange_candle",
    "magenta_candle",
    "light_blue_candle",
    "yellow_candle",
    "lime_candle",
    "pink_candle",
    "gray_candle",
    "light_gray_candle",
    "cyan_candle",
    "purple_candle",
    "blue_candle",
    "brown_candle",
    "green_candle",
    "red_candle",
    "black_candle",
];
pub const CAT_FOOD_TAG: &[&str] = &["cod", "salmon"];
pub const CHAINS_TAG: &[&str] = &[
    "iron_chain",
    "copper_chain",
    "waxed_copper_chain",
    "exposed_copper_chain",
    "waxed_exposed_copper_chain",
    "weathered_copper_chain",
    "waxed_weathered_copper_chain",
    "oxidized_copper_chain",
    "waxed_oxidized_copper_chain",
];
pub const CHERRY_LOGS_TAG: &[&str] = &[
    "cherry_log",
    "cherry_wood",
    "stripped_cherry_log",
    "stripped_cherry_wood",
];
pub const CHEST_ARMOR_TAG: &[&str] = &[
    "leather_chestplate",
    "copper_chestplate",
    "chainmail_chestplate",
    "golden_chestplate",
    "iron_chestplate",
    "diamond_chestplate",
    "netherite_chestplate",
];
pub const CHEST_BOATS_TAG: &[&str] = &[
    "oak_chest_boat",
    "spruce_chest_boat",
    "birch_chest_boat",
    "jungle_chest_boat",
    "acacia_chest_boat",
    "dark_oak_chest_boat",
    "pale_oak_chest_boat",
    "mangrove_chest_boat",
    "bamboo_chest_raft",
    "cherry_chest_boat",
];
pub const CHICKEN_FOOD_TAG: &[&str] = &[
    "wheat_seeds",
    "melon_seeds",
    "pumpkin_seeds",
    "beetroot_seeds",
    "torchflower_seeds",
    "pitcher_pod",
];
pub const CLUSTER_MAX_HARVESTABLES_TAG: &[&str] = &[
    "diamond_pickaxe",
    "golden_pickaxe",
    "iron_pickaxe",
    "netherite_pickaxe",
    "stone_pickaxe",
    "wooden_pickaxe",
    "copper_pickaxe",
];
pub const COAL_ORES_TAG: &[&str] = &["coal_ore", "deepslate_coal_ore"];
pub const COALS_TAG: &[&str] = &["coal", "charcoal"];
pub const COMPASSES_TAG: &[&str] = &["compass", "recovery_compass"];
pub const COMPLETES_FIND_TREE_TUTORIAL_TAG: &[&str] = &[
    "dark_oak_log",
    "dark_oak_wood",
    "stripped_dark_oak_log",
    "stripped_dark_oak_wood",
    "pale_oak_log",
    "pale_oak_wood",
    "stripped_pale_oak_log",
    "stripped_pale_oak_wood",
    "oak_log",
    "oak_wood",
    "stripped_oak_log",
    "stripped_oak_wood",
    "acacia_log",
    "acacia_wood",
    "stripped_acacia_log",
    "stripped_acacia_wood",
    "birch_log",
    "birch_wood",
    "stripped_birch_log",
    "stripped_birch_wood",
    "jungle_log",
    "jungle_wood",
    "stripped_jungle_log",
    "stripped_jungle_wood",
    "spruce_log",
    "spruce_wood",
    "stripped_spruce_log",
    "stripped_spruce_wood",
    "mangrove_log",
    "mangrove_wood",
    "stripped_mangrove_log",
    "stripped_mangrove_wood",
    "cherry_log",
    "cherry_wood",
    "stripped_cherry_log",
    "stripped_cherry_wood",
    "crimson_stem",
    "stripped_crimson_stem",
    "crimson_hyphae",
    "stripped_crimson_hyphae",
    "warped_stem",
    "stripped_warped_stem",
    "warped_hyphae",
    "stripped_warped_hyphae",
    "jungle_leaves",
    "oak_leaves",
    "spruce_leaves",
    "pale_oak_leaves",
    "dark_oak_leaves",
    "acacia_leaves",
    "birch_leaves",
    "azalea_leaves",
    "flowering_azalea_leaves",
    "mangrove_leaves",
    "cherry_leaves",
    "nether_wart_block",
    "warped_wart_block",
];
pub const COPPER_TAG: &[&str] = &[
    "copper_block",
    "exposed_copper",
    "weathered_copper",
    "oxidized_copper",
    "waxed_copper_block",
    "waxed_exposed_copper",
    "waxed_weathered_copper",
    "waxed_oxidized_copper",
];
pub const COPPER_CHESTS_TAG: &[&str] = &[
    "copper_chest",
    "exposed_copper_chest",
    "weathered_copper_chest",
    "oxidized_copper_chest",
    "waxed_copper_chest",
    "waxed_exposed_copper_chest",
    "waxed_weathered_copper_chest",
    "waxed_oxidized_copper_chest",
];
pub const COPPER_GOLEM_STATUES_TAG: &[&str] = &[
    "copper_golem_statue",
    "exposed_copper_golem_statue",
    "weathered_copper_golem_statue",
    "oxidized_copper_golem_statue",
    "waxed_copper_golem_statue",
    "waxed_exposed_copper_golem_statue",
    "waxed_weathered_copper_golem_statue",
    "waxed_oxidized_copper_golem_statue",
];
pub const COPPER_ORES_TAG: &[&str] = &["copper_ore", "deepslate_copper_ore"];
pub const COPPER_TOOL_MATERIALS_TAG: &[&str] = &["copper_ingot"];
pub const COW_FOOD_TAG: &[&str] = &["wheat"];
pub const CREEPER_DROP_MUSIC_DISCS_TAG: &[&str] = &[
    "music_disc_13",
    "music_disc_cat",
    "music_disc_blocks",
    "music_disc_chirp",
    "music_disc_far",
    "music_disc_mall",
    "music_disc_mellohi",
    "music_disc_stal",
    "music_disc_strad",
    "music_disc_ward",
    "music_disc_11",
    "music_disc_wait",
];
pub const CREEPER_IGNITERS_TAG: &[&str] = &["flint_and_steel", "fire_charge"];
pub const CRIMSON_STEMS_TAG: &[&str] = &[
    "crimson_stem",
    "stripped_crimson_stem",
    "crimson_hyphae",
    "stripped_crimson_hyphae",
];
pub const DAMPENS_VIBRATIONS_TAG: &[&str] = &[
    "white_wool",
    "orange_wool",
    "magenta_wool",
    "light_blue_wool",
    "yellow_wool",
    "lime_wool",
    "pink_wool",
    "gray_wool",
    "light_gray_wool",
    "cyan_wool",
    "purple_wool",
    "blue_wool",
    "brown_wool",
    "green_wool",
    "red_wool",
    "black_wool",
    "white_carpet",
    "orange_carpet",
    "magenta_carpet",
    "light_blue_carpet",
    "yellow_carpet",
    "lime_carpet",
    "pink_carpet",
    "gray_carpet",
    "light_gray_carpet",
    "cyan_carpet",
    "purple_carpet",
    "blue_carpet",
    "brown_carpet",
    "green_carpet",
    "red_carpet",
    "black_carpet",
];
pub const DARK_OAK_LOGS_TAG: &[&str] = &[
    "dark_oak_log",
    "dark_oak_wood",
    "stripped_dark_oak_log",
    "stripped_dark_oak_wood",
];
pub const DECORATED_POT_INGREDIENTS_TAG: &[&str] = &[
    "brick",
    "angler_pottery_sherd",
    "archer_pottery_sherd",
    "arms_up_pottery_sherd",
    "blade_pottery_sherd",
    "brewer_pottery_sherd",
    "burn_pottery_sherd",
    "danger_pottery_sherd",
    "explorer_pottery_sherd",
    "friend_pottery_sherd",
    "heart_pottery_sherd",
    "heartbreak_pottery_sherd",
    "howl_pottery_sherd",
    "miner_pottery_sherd",
    "mourner_pottery_sherd",
    "plenty_pottery_sherd",
    "prize_pottery_sherd",
    "sheaf_pottery_sherd",
    "shelter_pottery_sherd",
    "skull_pottery_sherd",
    "snort_pottery_sherd",
    "flow_pottery_sherd",
    "guster_pottery_sherd",
    "scrape_pottery_sherd",
];
pub const DECORATED_POT_SHERDS_TAG: &[&str] = &[
    "angler_pottery_sherd",
    "archer_pottery_sherd",
    "arms_up_pottery_sherd",
    "blade_pottery_sherd",
    "brewer_pottery_sherd",
    "burn_pottery_sherd",
    "danger_pottery_sherd",
    "explorer_pottery_sherd",
    "friend_pottery_sherd",
    "heart_pottery_sherd",
    "heartbreak_pottery_sherd",
    "howl_pottery_sherd",
    "miner_pottery_sherd",
    "mourner_pottery_sherd",
    "plenty_pottery_sherd",
    "prize_pottery_sherd",
    "sheaf_pottery_sherd",
    "shelter_pottery_sherd",
    "skull_pottery_sherd",
    "snort_pottery_sherd",
    "flow_pottery_sherd",
    "guster_pottery_sherd",
    "scrape_pottery_sherd",
];
pub const DIAMOND_ORES_TAG: &[&str] = &["diamond_ore", "deepslate_diamond_ore"];
pub const DIAMOND_TOOL_MATERIALS_TAG: &[&str] = &["diamond"];
pub const DIRT_TAG: &[&str] = &[
    "dirt",
    "grass_block",
    "podzol",
    "coarse_dirt",
    "mycelium",
    "rooted_dirt",
    "moss_block",
    "pale_moss_block",
    "mud",
    "muddy_mangrove_roots",
];
pub const DOORS_TAG: &[&str] = &[
    "oak_door",
    "spruce_door",
    "birch_door",
    "jungle_door",
    "acacia_door",
    "dark_oak_door",
    "pale_oak_door",
    "crimson_door",
    "warped_door",
    "mangrove_door",
    "bamboo_door",
    "cherry_door",
    "copper_door",
    "exposed_copper_door",
    "weathered_copper_door",
    "oxidized_copper_door",
    "waxed_copper_door",
    "waxed_exposed_copper_door",
    "waxed_weathered_copper_door",
    "waxed_oxidized_copper_door",
    "iron_door",
];
pub const DROWNED_PREFERRED_WEAPONS_TAG: &[&str] = &["trident"];
pub const DUPLICATES_ALLAYS_TAG: &[&str] = &["amethyst_shard"];
pub const DYEABLE_TAG: &[&str] = &[
    "leather_helmet",
    "leather_chestplate",
    "leather_leggings",
    "leather_boots",
    "leather_horse_armor",
    "wolf_armor",
];
pub const EGGS_TAG: &[&str] = &["egg", "blue_egg", "brown_egg"];
pub const EMERALD_ORES_TAG: &[&str] = &["emerald_ore", "deepslate_emerald_ore"];
pub const ENCHANTABLE_ARMOR_TAG: &[&str] = &[
    "leather_boots",
    "copper_boots",
    "chainmail_boots",
    "golden_boots",
    "iron_boots",
    "diamond_boots",
    "netherite_boots",
    "leather_leggings",
    "copper_leggings",
    "chainmail_leggings",
    "golden_leggings",
    "iron_leggings",
    "diamond_leggings",
    "netherite_leggings",
    "leather_chestplate",
    "copper_chestplate",
    "chainmail_chestplate",
    "golden_chestplate",
    "iron_chestplate",
    "diamond_chestplate",
    "netherite_chestplate",
    "leather_helmet",
    "copper_helmet",
    "chainmail_helmet",
    "golden_helmet",
    "iron_helmet",
    "diamond_helmet",
    "netherite_helmet",
    "turtle_helmet",
];
pub const ENCHANTABLE_BOW_TAG: &[&str] = &["bow"];
pub const ENCHANTABLE_CHEST_ARMOR_TAG: &[&str] = &[
    "leather_chestplate",
    "copper_chestplate",
    "chainmail_chestplate",
    "golden_chestplate",
    "iron_chestplate",
    "diamond_chestplate",
    "netherite_chestplate",
];
pub const ENCHANTABLE_CROSSBOW_TAG: &[&str] = &["crossbow"];
pub const ENCHANTABLE_DURABILITY_TAG: &[&str] = &[
    "leather_boots",
    "copper_boots",
    "chainmail_boots",
    "golden_boots",
    "iron_boots",
    "diamond_boots",
    "netherite_boots",
    "leather_leggings",
    "copper_leggings",
    "chainmail_leggings",
    "golden_leggings",
    "iron_leggings",
    "diamond_leggings",
    "netherite_leggings",
    "leather_chestplate",
    "copper_chestplate",
    "chainmail_chestplate",
    "golden_chestplate",
    "iron_chestplate",
    "diamond_chestplate",
    "netherite_chestplate",
    "leather_helmet",
    "copper_helmet",
    "chainmail_helmet",
    "golden_helmet",
    "iron_helmet",
    "diamond_helmet",
    "netherite_helmet",
    "turtle_helmet",
    "elytra",
    "shield",
    "diamond_sword",
    "stone_sword",
    "golden_sword",
    "netherite_sword",
    "wooden_sword",
    "iron_sword",
    "copper_sword",
    "diamond_axe",
    "stone_axe",
    "golden_axe",
    "netherite_axe",
    "wooden_axe",
    "iron_axe",
    "copper_axe",
    "diamond_pickaxe",
    "stone_pickaxe",
    "golden_pickaxe",
    "netherite_pickaxe",
    "wooden_pickaxe",
    "iron_pickaxe",
    "copper_pickaxe",
    "diamond_shovel",
    "stone_shovel",
    "golden_shovel",
    "netherite_shovel",
    "wooden_shovel",
    "iron_shovel",
    "copper_shovel",
    "diamond_hoe",
    "stone_hoe",
    "golden_hoe",
    "netherite_hoe",
    "wooden_hoe",
    "iron_hoe",
    "copper_hoe",
    "bow",
    "crossbow",
    "trident",
    "flint_and_steel",
    "shears",
    "brush",
    "fishing_rod",
    "carrot_on_a_stick",
    "warped_fungus_on_a_stick",
    "mace",
];
pub const ENCHANTABLE_EQUIPPABLE_TAG: &[&str] = &[
    "leather_boots",
    "copper_boots",
    "chainmail_boots",
    "golden_boots",
    "iron_boots",
    "diamond_boots",
    "netherite_boots",
    "leather_leggings",
    "copper_leggings",
    "chainmail_leggings",
    "golden_leggings",
    "iron_leggings",
    "diamond_leggings",
    "netherite_leggings",
    "leather_chestplate",
    "copper_chestplate",
    "chainmail_chestplate",
    "golden_chestplate",
    "iron_chestplate",
    "diamond_chestplate",
    "netherite_chestplate",
    "leather_helmet",
    "copper_helmet",
    "chainmail_helmet",
    "golden_helmet",
    "iron_helmet",
    "diamond_helmet",
    "netherite_helmet",
    "turtle_helmet",
    "elytra",
    "player_head",
    "creeper_head",
    "zombie_head",
    "skeleton_skull",
    "wither_skeleton_skull",
    "dragon_head",
    "piglin_head",
    "carved_pumpkin",
];
pub const ENCHANTABLE_FIRE_ASPECT_TAG: &[&str] = &[
    "diamond_sword",
    "stone_sword",
    "golden_sword",
    "netherite_sword",
    "wooden_sword",
    "iron_sword",
    "copper_sword",
    "mace",
];
pub const ENCHANTABLE_FISHING_TAG: &[&str] = &["fishing_rod"];
pub const ENCHANTABLE_FOOT_ARMOR_TAG: &[&str] = &[
    "leather_boots",
    "copper_boots",
    "chainmail_boots",
    "golden_boots",
    "iron_boots",
    "diamond_boots",
    "netherite_boots",
];
pub const ENCHANTABLE_HEAD_ARMOR_TAG: &[&str] = &[
    "leather_helmet",
    "copper_helmet",
    "chainmail_helmet",
    "golden_helmet",
    "iron_helmet",
    "diamond_helmet",
    "netherite_helmet",
    "turtle_helmet",
];
pub const ENCHANTABLE_LEG_ARMOR_TAG: &[&str] = &[
    "leather_leggings",
    "copper_leggings",
    "chainmail_leggings",
    "golden_leggings",
    "iron_leggings",
    "diamond_leggings",
    "netherite_leggings",
];
pub const ENCHANTABLE_MACE_TAG: &[&str] = &["mace"];
pub const ENCHANTABLE_MINING_TAG: &[&str] = &[
    "diamond_axe",
    "stone_axe",
    "golden_axe",
    "netherite_axe",
    "wooden_axe",
    "iron_axe",
    "copper_axe",
    "diamond_pickaxe",
    "stone_pickaxe",
    "golden_pickaxe",
    "netherite_pickaxe",
    "wooden_pickaxe",
    "iron_pickaxe",
    "copper_pickaxe",
    "diamond_shovel",
    "stone_shovel",
    "golden_shovel",
    "netherite_shovel",
    "wooden_shovel",
    "iron_shovel",
    "copper_shovel",
    "diamond_hoe",
    "stone_hoe",
    "golden_hoe",
    "netherite_hoe",
    "wooden_hoe",
    "iron_hoe",
    "copper_hoe",
    "shears",
];
pub const ENCHANTABLE_MINING_LOOT_TAG: &[&str] = &[
    "diamond_axe",
    "stone_axe",
    "golden_axe",
    "netherite_axe",
    "wooden_axe",
    "iron_axe",
    "copper_axe",
    "diamond_pickaxe",
    "stone_pickaxe",
    "golden_pickaxe",
    "netherite_pickaxe",
    "wooden_pickaxe",
    "iron_pickaxe",
    "copper_pickaxe",
    "diamond_shovel",
    "stone_shovel",
    "golden_shovel",
    "netherite_shovel",
    "wooden_shovel",
    "iron_shovel",
    "copper_shovel",
    "diamond_hoe",
    "stone_hoe",
    "golden_hoe",
    "netherite_hoe",
    "wooden_hoe",
    "iron_hoe",
    "copper_hoe",
];
pub const ENCHANTABLE_SHARP_WEAPON_TAG: &[&str] = &[
    "diamond_sword",
    "stone_sword",
    "golden_sword",
    "netherite_sword",
    "wooden_sword",
    "iron_sword",
    "copper_sword",
    "diamond_axe",
    "stone_axe",
    "golden_axe",
    "netherite_axe",
    "wooden_axe",
    "iron_axe",
    "copper_axe",
];
pub const ENCHANTABLE_SWORD_TAG: &[&str] = &[
    "diamond_sword",
    "stone_sword",
    "golden_sword",
    "netherite_sword",
    "wooden_sword",
    "iron_sword",
    "copper_sword",
];
pub const ENCHANTABLE_TRIDENT_TAG: &[&str] = &["trident"];
pub const ENCHANTABLE_VANISHING_TAG: &[&str] = &[
    "leather_boots",
    "copper_boots",
    "chainmail_boots",
    "golden_boots",
    "iron_boots",
    "diamond_boots",
    "netherite_boots",
    "leather_leggings",
    "copper_leggings",
    "chainmail_leggings",
    "golden_leggings",
    "iron_leggings",
    "diamond_leggings",
    "netherite_leggings",
    "leather_chestplate",
    "copper_chestplate",
    "chainmail_chestplate",
    "golden_chestplate",
    "iron_chestplate",
    "diamond_chestplate",
    "netherite_chestplate",
    "leather_helmet",
    "copper_helmet",
    "chainmail_helmet",
    "golden_helmet",
    "iron_helmet",
    "diamond_helmet",
    "netherite_helmet",
    "turtle_helmet",
    "elytra",
    "shield",
    "diamond_sword",
    "stone_sword",
    "golden_sword",
    "netherite_sword",
    "wooden_sword",
    "iron_sword",
    "copper_sword",
    "diamond_axe",
    "stone_axe",
    "golden_axe",
    "netherite_axe",
    "wooden_axe",
    "iron_axe",
    "copper_axe",
    "diamond_pickaxe",
    "stone_pickaxe",
    "golden_pickaxe",
    "netherite_pickaxe",
    "wooden_pickaxe",
    "iron_pickaxe",
    "copper_pickaxe",
    "diamond_shovel",
    "stone_shovel",
    "golden_shovel",
    "netherite_shovel",
    "wooden_shovel",
    "iron_shovel",
    "copper_shovel",
    "diamond_hoe",
    "stone_hoe",
    "golden_hoe",
    "netherite_hoe",
    "wooden_hoe",
    "iron_hoe",
    "copper_hoe",
    "bow",
    "crossbow",
    "trident",
    "flint_and_steel",
    "shears",
    "brush",
    "fishing_rod",
    "carrot_on_a_stick",
    "warped_fungus_on_a_stick",
    "mace",
    "compass",
    "carved_pumpkin",
    "player_head",
    "creeper_head",
    "zombie_head",
    "skeleton_skull",
    "wither_skeleton_skull",
    "dragon_head",
    "piglin_head",
];
pub const ENCHANTABLE_WEAPON_TAG: &[&str] = &[
    "diamond_sword",
    "stone_sword",
    "golden_sword",
    "netherite_sword",
    "wooden_sword",
    "iron_sword",
    "copper_sword",
    "diamond_axe",
    "stone_axe",
    "golden_axe",
    "netherite_axe",
    "wooden_axe",
    "iron_axe",
    "copper_axe",
    "mace",
];
pub const FENCE_GATES_TAG: &[&str] = &[
    "acacia_fence_gate",
    "birch_fence_gate",
    "dark_oak_fence_gate",
    "pale_oak_fence_gate",
    "jungle_fence_gate",
    "oak_fence_gate",
    "spruce_fence_gate",
    "crimson_fence_gate",
    "warped_fence_gate",
    "mangrove_fence_gate",
    "bamboo_fence_gate",
    "cherry_fence_gate",
];
pub const FENCES_TAG: &[&str] = &[
    "oak_fence",
    "acacia_fence",
    "dark_oak_fence",
    "pale_oak_fence",
    "spruce_fence",
    "birch_fence",
    "jungle_fence",
    "crimson_fence",
    "warped_fence",
    "mangrove_fence",
    "bamboo_fence",
    "cherry_fence",
    "nether_brick_fence",
];
pub const FISHES_TAG: &[&str] = &[
    "cod",
    "cooked_cod",
    "salmon",
    "cooked_salmon",
    "pufferfish",
    "tropical_fish",
];
pub const FLOWERS_TAG: &[&str] = &[
    "dandelion",
    "open_eyeblossom",
    "poppy",
    "blue_orchid",
    "allium",
    "azure_bluet",
    "red_tulip",
    "orange_tulip",
    "white_tulip",
    "pink_tulip",
    "oxeye_daisy",
    "cornflower",
    "lily_of_the_valley",
    "wither_rose",
    "torchflower",
    "closed_eyeblossom",
    "sunflower",
    "lilac",
    "peony",
    "rose_bush",
    "pitcher_plant",
    "flowering_azalea_leaves",
    "flowering_azalea",
    "mangrove_propagule",
    "cherry_leaves",
    "pink_petals",
    "wildflowers",
    "chorus_flower",
    "spore_blossom",
    "cactus_flower",
];
pub const FOOT_ARMOR_TAG: &[&str] = &[
    "leather_boots",
    "copper_boots",
    "chainmail_boots",
    "golden_boots",
    "iron_boots",
    "diamond_boots",
    "netherite_boots",
];
pub const FOX_FOOD_TAG: &[&str] = &["sweet_berries", "glow_berries"];
pub const FREEZE_IMMUNE_WEARABLES_TAG: &[&str] = &[
    "leather_boots",
    "leather_leggings",
    "leather_chestplate",
    "leather_helmet",
    "leather_horse_armor",
];
pub const FROG_FOOD_TAG: &[&str] = &["slime_ball"];
pub const FURNACE_MINECART_FUEL_TAG: &[&str] = &["coal", "charcoal"];
pub const GAZE_DISGUISE_EQUIPMENT_TAG: &[&str] = &["carved_pumpkin"];
pub const GOAT_FOOD_TAG: &[&str] = &["wheat"];
pub const GOLD_ORES_TAG: &[&str] = &["gold_ore", "nether_gold_ore", "deepslate_gold_ore"];
pub const GOLD_TOOL_MATERIALS_TAG: &[&str] = &["gold_ingot"];
pub const HANGING_SIGNS_TAG: &[&str] = &[
    "oak_hanging_sign",
    "spruce_hanging_sign",
    "birch_hanging_sign",
    "acacia_hanging_sign",
    "cherry_hanging_sign",
    "jungle_hanging_sign",
    "dark_oak_hanging_sign",
    "pale_oak_hanging_sign",
    "crimson_hanging_sign",
    "warped_hanging_sign",
    "mangrove_hanging_sign",
    "bamboo_hanging_sign",
];
pub const HAPPY_GHAST_FOOD_TAG: &[&str] = &["snowball"];
pub const HAPPY_GHAST_TEMPT_ITEMS_TAG: &[&str] = &[
    "snowball",
    "white_harness",
    "orange_harness",
    "magenta_harness",
    "light_blue_harness",
    "yellow_harness",
    "lime_harness",
    "pink_harness",
    "gray_harness",
    "light_gray_harness",
    "cyan_harness",
    "purple_harness",
    "blue_harness",
    "brown_harness",
    "green_harness",
    "red_harness",
    "black_harness",
];
pub const HARNESSES_TAG: &[&str] = &[
    "white_harness",
    "orange_harness",
    "magenta_harness",
    "light_blue_harness",
    "yellow_harness",
    "lime_harness",
    "pink_harness",
    "gray_harness",
    "light_gray_harness",
    "cyan_harness",
    "purple_harness",
    "blue_harness",
    "brown_harness",
    "green_harness",
    "red_harness",
    "black_harness",
];
pub const HEAD_ARMOR_TAG: &[&str] = &[
    "leather_helmet",
    "copper_helmet",
    "chainmail_helmet",
    "golden_helmet",
    "iron_helmet",
    "diamond_helmet",
    "netherite_helmet",
    "turtle_helmet",
];
pub const HOES_TAG: &[&str] = &[
    "diamond_hoe",
    "stone_hoe",
    "golden_hoe",
    "netherite_hoe",
    "wooden_hoe",
    "iron_hoe",
    "copper_hoe",
];
pub const HOGLIN_FOOD_TAG: &[&str] = &["crimson_fungus"];
pub const HORSE_FOOD_TAG: &[&str] = &[
    "wheat",
    "sugar",
    "hay_block",
    "apple",
    "carrot",
    "golden_carrot",
    "golden_apple",
    "enchanted_golden_apple",
];
pub const HORSE_TEMPT_ITEMS_TAG: &[&str] =
    &["golden_carrot", "golden_apple", "enchanted_golden_apple"];
pub const IGNORED_BY_PIGLIN_BABIES_TAG: &[&str] = &["leather"];
pub const IRON_ORES_TAG: &[&str] = &["iron_ore", "deepslate_iron_ore"];
pub const IRON_TOOL_MATERIALS_TAG: &[&str] = &["iron_ingot"];
pub const JUNGLE_LOGS_TAG: &[&str] = &[
    "jungle_log",
    "jungle_wood",
    "stripped_jungle_log",
    "stripped_jungle_wood",
];
pub const LANTERNS_TAG: &[&str] = &[
    "lantern",
    "soul_lantern",
    "copper_lantern",
    "waxed_copper_lantern",
    "exposed_copper_lantern",
    "waxed_exposed_copper_lantern",
    "weathered_copper_lantern",
    "waxed_weathered_copper_lantern",
    "oxidized_copper_lantern",
    "waxed_oxidized_copper_lantern",
];
pub const LAPIS_ORES_TAG: &[&str] = &["lapis_ore", "deepslate_lapis_ore"];
pub const LEAVES_TAG: &[&str] = &[
    "jungle_leaves",
    "oak_leaves",
    "spruce_leaves",
    "pale_oak_leaves",
    "dark_oak_leaves",
    "acacia_leaves",
    "birch_leaves",
    "azalea_leaves",
    "flowering_azalea_leaves",
    "mangrove_leaves",
    "cherry_leaves",
];
pub const LECTERN_BOOKS_TAG: &[&str] = &["written_book", "writable_book"];
pub const LEG_ARMOR_TAG: &[&str] = &[
    "leather_leggings",
    "copper_leggings",
    "chainmail_leggings",
    "golden_leggings",
    "iron_leggings",
    "diamond_leggings",
    "netherite_leggings",
];
pub const LIGHTNING_RODS_TAG: &[&str] = &[
    "lightning_rod",
    "exposed_lightning_rod",
    "weathered_lightning_rod",
    "oxidized_lightning_rod",
    "waxed_lightning_rod",
    "waxed_exposed_lightning_rod",
    "waxed_weathered_lightning_rod",
    "waxed_oxidized_lightning_rod",
];
pub const LLAMA_FOOD_TAG: &[&str] = &["wheat", "hay_block"];
pub const LLAMA_TEMPT_ITEMS_TAG: &[&str] = &["hay_block"];
pub const LOGS_TAG: &[&str] = &[
    "dark_oak_log",
    "dark_oak_wood",
    "stripped_dark_oak_log",
    "stripped_dark_oak_wood",
    "pale_oak_log",
    "pale_oak_wood",
    "stripped_pale_oak_log",
    "stripped_pale_oak_wood",
    "oak_log",
    "oak_wood",
    "stripped_oak_log",
    "stripped_oak_wood",
    "acacia_log",
    "acacia_wood",
    "stripped_acacia_log",
    "stripped_acacia_wood",
    "birch_log",
    "birch_wood",
    "stripped_birch_log",
    "stripped_birch_wood",
    "jungle_log",
    "jungle_wood",
    "stripped_jungle_log",
    "stripped_jungle_wood",
    "spruce_log",
    "spruce_wood",
    "stripped_spruce_log",
    "stripped_spruce_wood",
    "mangrove_log",
    "mangrove_wood",
    "stripped_mangrove_log",
    "stripped_mangrove_wood",
    "cherry_log",
    "cherry_wood",
    "stripped_cherry_log",
    "stripped_cherry_wood",
    "crimson_stem",
    "stripped_crimson_stem",
    "crimson_hyphae",
    "stripped_crimson_hyphae",
    "warped_stem",
    "stripped_warped_stem",
    "warped_hyphae",
    "stripped_warped_hyphae",
];
pub const LOGS_THAT_BURN_TAG: &[&str] = &[
    "dark_oak_log",
    "dark_oak_wood",
    "stripped_dark_oak_log",
    "stripped_dark_oak_wood",
    "pale_oak_log",
    "pale_oak_wood",
    "stripped_pale_oak_log",
    "stripped_pale_oak_wood",
    "oak_log",
    "oak_wood",
    "stripped_oak_log",
    "stripped_oak_wood",
    "acacia_log",
    "acacia_wood",
    "stripped_acacia_log",
    "stripped_acacia_wood",
    "birch_log",
    "birch_wood",
    "stripped_birch_log",
    "stripped_birch_wood",
    "jungle_log",
    "jungle_wood",
    "stripped_jungle_log",
    "stripped_jungle_wood",
    "spruce_log",
    "spruce_wood",
    "stripped_spruce_log",
    "stripped_spruce_wood",
    "mangrove_log",
    "mangrove_wood",
    "stripped_mangrove_log",
    "stripped_mangrove_wood",
    "cherry_log",
    "cherry_wood",
    "stripped_cherry_log",
    "stripped_cherry_wood",
];
pub const MANGROVE_LOGS_TAG: &[&str] = &[
    "mangrove_log",
    "mangrove_wood",
    "stripped_mangrove_log",
    "stripped_mangrove_wood",
];
pub const MAP_INVISIBILITY_EQUIPMENT_TAG: &[&str] = &["carved_pumpkin"];
pub const MEAT_TAG: &[&str] = &[
    "beef",
    "chicken",
    "cooked_beef",
    "cooked_chicken",
    "cooked_mutton",
    "cooked_porkchop",
    "cooked_rabbit",
    "mutton",
    "porkchop",
    "rabbit",
    "rotten_flesh",
];
pub const NETHERITE_TOOL_MATERIALS_TAG: &[&str] = &["netherite_ingot"];
pub const NON_FLAMMABLE_WOOD_TAG: &[&str] = &[
    "warped_stem",
    "stripped_warped_stem",
    "warped_hyphae",
    "stripped_warped_hyphae",
    "crimson_stem",
    "stripped_crimson_stem",
    "crimson_hyphae",
    "stripped_crimson_hyphae",
    "crimson_planks",
    "warped_planks",
    "crimson_slab",
    "warped_slab",
    "crimson_pressure_plate",
    "warped_pressure_plate",
    "crimson_fence",
    "warped_fence",
    "crimson_trapdoor",
    "warped_trapdoor",
    "crimson_fence_gate",
    "warped_fence_gate",
    "crimson_stairs",
    "warped_stairs",
    "crimson_button",
    "warped_button",
    "crimson_door",
    "warped_door",
    "crimson_sign",
    "warped_sign",
    "warped_hanging_sign",
    "crimson_hanging_sign",
    "warped_shelf",
    "crimson_shelf",
];
pub const NOTEBLOCK_TOP_INSTRUMENTS_TAG: &[&str] = &[
    "zombie_head",
    "skeleton_skull",
    "creeper_head",
    "dragon_head",
    "wither_skeleton_skull",
    "piglin_head",
    "player_head",
];
pub const OAK_LOGS_TAG: &[&str] = &[
    "oak_log",
    "oak_wood",
    "stripped_oak_log",
    "stripped_oak_wood",
];
pub const OCELOT_FOOD_TAG: &[&str] = &["cod", "salmon"];
pub const PALE_OAK_LOGS_TAG: &[&str] = &[
    "pale_oak_log",
    "pale_oak_wood",
    "stripped_pale_oak_log",
    "stripped_pale_oak_wood",
];
pub const PANDA_EATS_FROM_GROUND_TAG: &[&str] = &["bamboo", "cake"];
pub const PANDA_FOOD_TAG: &[&str] = &["bamboo"];
pub const PARROT_FOOD_TAG: &[&str] = &[
    "wheat_seeds",
    "melon_seeds",
    "pumpkin_seeds",
    "beetroot_seeds",
    "torchflower_seeds",
    "pitcher_pod",
];
pub const PARROT_POISONOUS_FOOD_TAG: &[&str] = &["cookie"];
pub const PICKAXES_TAG: &[&str] = &[
    "diamond_pickaxe",
    "stone_pickaxe",
    "golden_pickaxe",
    "netherite_pickaxe",
    "wooden_pickaxe",
    "iron_pickaxe",
    "copper_pickaxe",
];
pub const PIG_FOOD_TAG: &[&str] = &["carrot", "potato", "beetroot"];
pub const PIGLIN_FOOD_TAG: &[&str] = &["porkchop", "cooked_porkchop"];
pub const PIGLIN_LOVED_TAG: &[&str] = &[
    "gold_ore",
    "nether_gold_ore",
    "deepslate_gold_ore",
    "gold_block",
    "gilded_blackstone",
    "light_weighted_pressure_plate",
    "gold_ingot",
    "bell",
    "clock",
    "golden_carrot",
    "glistering_melon_slice",
    "golden_apple",
    "enchanted_golden_apple",
    "golden_helmet",
    "golden_chestplate",
    "golden_leggings",
    "golden_boots",
    "golden_horse_armor",
    "golden_sword",
    "golden_pickaxe",
    "golden_shovel",
    "golden_axe",
    "golden_hoe",
    "raw_gold",
    "raw_gold_block",
];
pub const PIGLIN_PREFERRED_WEAPONS_TAG: &[&str] = &["crossbow"];
pub const PIGLIN_REPELLENTS_TAG: &[&str] = &["soul_torch", "soul_lantern", "soul_campfire"];
pub const PIGLIN_SAFE_ARMOR_TAG: &[&str] = &[
    "golden_helmet",
    "golden_chestplate",
    "golden_leggings",
    "golden_boots",
];
pub const PILLAGER_PREFERRED_WEAPONS_TAG: &[&str] = &["crossbow"];
pub const PLANKS_TAG: &[&str] = &[
    "oak_planks",
    "spruce_planks",
    "birch_planks",
    "jungle_planks",
    "acacia_planks",
    "dark_oak_planks",
    "pale_oak_planks",
    "crimson_planks",
    "warped_planks",
    "mangrove_planks",
    "bamboo_planks",
    "cherry_planks",
];
pub const RABBIT_FOOD_TAG: &[&str] = &["carrot", "golden_carrot", "dandelion"];
pub const RAILS_TAG: &[&str] = &["rail", "powered_rail", "detector_rail", "activator_rail"];
pub const REDSTONE_ORES_TAG: &[&str] = &["redstone_ore", "deepslate_redstone_ore"];
pub const REPAIRS_CHAIN_ARMOR_TAG: &[&str] = &["iron_ingot"];
pub const REPAIRS_COPPER_ARMOR_TAG: &[&str] = &["copper_ingot"];
pub const REPAIRS_DIAMOND_ARMOR_TAG: &[&str] = &["diamond"];
pub const REPAIRS_GOLD_ARMOR_TAG: &[&str] = &["gold_ingot"];
pub const REPAIRS_IRON_ARMOR_TAG: &[&str] = &["iron_ingot"];
pub const REPAIRS_LEATHER_ARMOR_TAG: &[&str] = &["leather"];
pub const REPAIRS_NETHERITE_ARMOR_TAG: &[&str] = &["netherite_ingot"];
pub const REPAIRS_TURTLE_HELMET_TAG: &[&str] = &["turtle_scute"];
pub const REPAIRS_WOLF_ARMOR_TAG: &[&str] = &["armadillo_scute"];
pub const SAND_TAG: &[&str] = &["sand", "red_sand", "suspicious_sand"];
pub const SAPLINGS_TAG: &[&str] = &[
    "oak_sapling",
    "spruce_sapling",
    "birch_sapling",
    "jungle_sapling",
    "acacia_sapling",
    "dark_oak_sapling",
    "pale_oak_sapling",
    "azalea",
    "flowering_azalea",
    "mangrove_propagule",
    "cherry_sapling",
];
pub const SHEARABLE_FROM_COPPER_GOLEM_TAG: &[&str] = &["poppy"];
pub const SHEEP_FOOD_TAG: &[&str] = &["wheat"];
pub const SHOVELS_TAG: &[&str] = &[
    "diamond_shovel",
    "stone_shovel",
    "golden_shovel",
    "netherite_shovel",
    "wooden_shovel",
    "iron_shovel",
    "copper_shovel",
];
pub const SHULKER_BOXES_TAG: &[&str] = &[
    "shulker_box",
    "black_shulker_box",
    "blue_shulker_box",
    "brown_shulker_box",
    "cyan_shulker_box",
    "gray_shulker_box",
    "green_shulker_box",
    "light_blue_shulker_box",
    "light_gray_shulker_box",
    "lime_shulker_box",
    "magenta_shulker_box",
    "orange_shulker_box",
    "pink_shulker_box",
    "purple_shulker_box",
    "red_shulker_box",
    "white_shulker_box",
    "yellow_shulker_box",
];
pub const SIGNS_TAG: &[&str] = &[
    "oak_sign",
    "spruce_sign",
    "birch_sign",
    "acacia_sign",
    "jungle_sign",
    "dark_oak_sign",
    "pale_oak_sign",
    "crimson_sign",
    "warped_sign",
    "mangrove_sign",
    "bamboo_sign",
    "cherry_sign",
];
pub const SKELETON_PREFERRED_WEAPONS_TAG: &[&str] = &["bow"];
pub const SKULLS_TAG: &[&str] = &[
    "player_head",
    "creeper_head",
    "zombie_head",
    "skeleton_skull",
    "wither_skeleton_skull",
    "dragon_head",
    "piglin_head",
];
pub const SLABS_TAG: &[&str] = &[
    "oak_slab",
    "spruce_slab",
    "birch_slab",
    "jungle_slab",
    "acacia_slab",
    "dark_oak_slab",
    "pale_oak_slab",
    "crimson_slab",
    "warped_slab",
    "mangrove_slab",
    "bamboo_slab",
    "cherry_slab",
    "bamboo_mosaic_slab",
    "stone_slab",
    "smooth_stone_slab",
    "stone_brick_slab",
    "sandstone_slab",
    "purpur_slab",
    "quartz_slab",
    "red_sandstone_slab",
    "brick_slab",
    "cobblestone_slab",
    "nether_brick_slab",
    "petrified_oak_slab",
    "prismarine_slab",
    "prismarine_brick_slab",
    "dark_prismarine_slab",
    "polished_granite_slab",
    "smooth_red_sandstone_slab",
    "mossy_stone_brick_slab",
    "polished_diorite_slab",
    "mossy_cobblestone_slab",
    "end_stone_brick_slab",
    "smooth_sandstone_slab",
    "smooth_quartz_slab",
    "granite_slab",
    "andesite_slab",
    "red_nether_brick_slab",
    "polished_andesite_slab",
    "diorite_slab",
    "cut_sandstone_slab",
    "cut_red_sandstone_slab",
    "blackstone_slab",
    "polished_blackstone_brick_slab",
    "polished_blackstone_slab",
    "cobbled_deepslate_slab",
    "polished_deepslate_slab",
    "deepslate_tile_slab",
    "deepslate_brick_slab",
    "waxed_weathered_cut_copper_slab",
    "waxed_exposed_cut_copper_slab",
    "waxed_cut_copper_slab",
    "oxidized_cut_copper_slab",
    "weathered_cut_copper_slab",
    "exposed_cut_copper_slab",
    "cut_copper_slab",
    "waxed_oxidized_cut_copper_slab",
    "mud_brick_slab",
    "tuff_slab",
    "polished_tuff_slab",
    "tuff_brick_slab",
    "resin_brick_slab",
];
pub const SMALL_FLOWERS_TAG: &[&str] = &[
    "dandelion",
    "open_eyeblossom",
    "poppy",
    "blue_orchid",
    "allium",
    "azure_bluet",
    "red_tulip",
    "orange_tulip",
    "white_tulip",
    "pink_tulip",
    "oxeye_daisy",
    "cornflower",
    "lily_of_the_valley",
    "wither_rose",
    "torchflower",
    "closed_eyeblossom",
];
pub const SMELTS_TO_GLASS_TAG: &[&str] = &["sand", "red_sand"];
pub const SNIFFER_FOOD_TAG: &[&str] = &["torchflower_seeds"];
pub const SOUL_FIRE_BASE_BLOCKS_TAG: &[&str] = &["soul_sand", "soul_soil"];
pub const SPRUCE_LOGS_TAG: &[&str] = &[
    "spruce_log",
    "spruce_wood",
    "stripped_spruce_log",
    "stripped_spruce_wood",
];
pub const STAIRS_TAG: &[&str] = &[
    "oak_stairs",
    "spruce_stairs",
    "birch_stairs",
    "jungle_stairs",
    "acacia_stairs",
    "dark_oak_stairs",
    "pale_oak_stairs",
    "crimson_stairs",
    "warped_stairs",
    "mangrove_stairs",
    "bamboo_stairs",
    "cherry_stairs",
    "bamboo_mosaic_stairs",
    "cobblestone_stairs",
    "sandstone_stairs",
    "nether_brick_stairs",
    "stone_brick_stairs",
    "brick_stairs",
    "purpur_stairs",
    "quartz_stairs",
    "red_sandstone_stairs",
    "prismarine_brick_stairs",
    "prismarine_stairs",
    "dark_prismarine_stairs",
    "polished_granite_stairs",
    "smooth_red_sandstone_stairs",
    "mossy_stone_brick_stairs",
    "polished_diorite_stairs",
    "mossy_cobblestone_stairs",
    "end_stone_brick_stairs",
    "stone_stairs",
    "smooth_sandstone_stairs",
    "smooth_quartz_stairs",
    "granite_stairs",
    "andesite_stairs",
    "red_nether_brick_stairs",
    "polished_andesite_stairs",
    "diorite_stairs",
    "blackstone_stairs",
    "polished_blackstone_brick_stairs",
    "polished_blackstone_stairs",
    "cobbled_deepslate_stairs",
    "polished_deepslate_stairs",
    "deepslate_tile_stairs",
    "deepslate_brick_stairs",
    "oxidized_cut_copper_stairs",
    "weathered_cut_copper_stairs",
    "exposed_cut_copper_stairs",
    "cut_copper_stairs",
    "waxed_weathered_cut_copper_stairs",
    "waxed_exposed_cut_copper_stairs",
    "waxed_cut_copper_stairs",
    "waxed_oxidized_cut_copper_stairs",
    "mud_brick_stairs",
    "tuff_stairs",
    "polished_tuff_stairs",
    "tuff_brick_stairs",
    "resin_brick_stairs",
];
pub const STONE_BRICKS_TAG: &[&str] = &[
    "stone_bricks",
    "mossy_stone_bricks",
    "cracked_stone_bricks",
    "chiseled_stone_bricks",
];
pub const STONE_BUTTONS_TAG: &[&str] = &["stone_button", "polished_blackstone_button"];
pub const STONE_CRAFTING_MATERIALS_TAG: &[&str] =
    &["cobblestone", "blackstone", "cobbled_deepslate"];
pub const STONE_TOOL_MATERIALS_TAG: &[&str] = &["cobblestone", "blackstone", "cobbled_deepslate"];
pub const STRIDER_FOOD_TAG: &[&str] = &["warped_fungus"];
pub const STRIDER_TEMPT_ITEMS_TAG: &[&str] = &["warped_fungus", "warped_fungus_on_a_stick"];
pub const SWORDS_TAG: &[&str] = &[
    "diamond_sword",
    "stone_sword",
    "golden_sword",
    "netherite_sword",
    "wooden_sword",
    "iron_sword",
    "copper_sword",
];
pub const TERRACOTTA_TAG: &[&str] = &[
    "terracotta",
    "white_terracotta",
    "orange_terracotta",
    "magenta_terracotta",
    "light_blue_terracotta",
    "yellow_terracotta",
    "lime_terracotta",
    "pink_terracotta",
    "gray_terracotta",
    "light_gray_terracotta",
    "cyan_terracotta",
    "purple_terracotta",
    "blue_terracotta",
    "brown_terracotta",
    "green_terracotta",
    "red_terracotta",
    "black_terracotta",
];
pub const TRAPDOORS_TAG: &[&str] = &[
    "acacia_trapdoor",
    "birch_trapdoor",
    "dark_oak_trapdoor",
    "pale_oak_trapdoor",
    "jungle_trapdoor",
    "oak_trapdoor",
    "spruce_trapdoor",
    "crimson_trapdoor",
    "warped_trapdoor",
    "mangrove_trapdoor",
    "bamboo_trapdoor",
    "cherry_trapdoor",
    "iron_trapdoor",
    "copper_trapdoor",
    "exposed_copper_trapdoor",
    "weathered_copper_trapdoor",
    "oxidized_copper_trapdoor",
    "waxed_copper_trapdoor",
    "waxed_exposed_copper_trapdoor",
    "waxed_weathered_copper_trapdoor",
    "waxed_oxidized_copper_trapdoor",
];
pub const TRIM_MATERIALS_TAG: &[&str] = &[
    "amethyst_shard",
    "copper_ingot",
    "diamond",
    "emerald",
    "gold_ingot",
    "iron_ingot",
    "lapis_lazuli",
    "netherite_ingot",
    "quartz",
    "redstone",
    "resin_brick",
];
pub const TRIMMABLE_ARMOR_TAG: &[&str] = &[
    "leather_boots",
    "copper_boots",
    "chainmail_boots",
    "golden_boots",
    "iron_boots",
    "diamond_boots",
    "netherite_boots",
    "leather_leggings",
    "copper_leggings",
    "chainmail_leggings",
    "golden_leggings",
    "iron_leggings",
    "diamond_leggings",
    "netherite_leggings",
    "leather_chestplate",
    "copper_chestplate",
    "chainmail_chestplate",
    "golden_chestplate",
    "iron_chestplate",
    "diamond_chestplate",
    "netherite_chestplate",
    "leather_helmet",
    "copper_helmet",
    "chainmail_helmet",
    "golden_helmet",
    "iron_helmet",
    "diamond_helmet",
    "netherite_helmet",
    "turtle_helmet",
];
pub const TURTLE_FOOD_TAG: &[&str] = &["seagrass"];
pub const VILLAGER_PICKS_UP_TAG: &[&str] = &[
    "wheat_seeds",
    "potato",
    "carrot",
    "beetroot_seeds",
    "torchflower_seeds",
    "pitcher_pod",
    "bread",
    "wheat",
    "beetroot",
];
pub const VILLAGER_PLANTABLE_SEEDS_TAG: &[&str] = &[
    "wheat_seeds",
    "potato",
    "carrot",
    "beetroot_seeds",
    "torchflower_seeds",
    "pitcher_pod",
];
pub const WALLS_TAG: &[&str] = &[
    "cobblestone_wall",
    "mossy_cobblestone_wall",
    "brick_wall",
    "prismarine_wall",
    "red_sandstone_wall",
    "mossy_stone_brick_wall",
    "granite_wall",
    "stone_brick_wall",
    "nether_brick_wall",
    "andesite_wall",
    "red_nether_brick_wall",
    "sandstone_wall",
    "end_stone_brick_wall",
    "diorite_wall",
    "blackstone_wall",
    "polished_blackstone_brick_wall",
    "polished_blackstone_wall",
    "cobbled_deepslate_wall",
    "polished_deepslate_wall",
    "deepslate_tile_wall",
    "deepslate_brick_wall",
    "mud_brick_wall",
    "tuff_wall",
    "polished_tuff_wall",
    "tuff_brick_wall",
    "resin_brick_wall",
];
pub const WARPED_STEMS_TAG: &[&str] = &[
    "warped_stem",
    "stripped_warped_stem",
    "warped_hyphae",
    "stripped_warped_hyphae",
];
pub const WART_BLOCKS_TAG: &[&str] = &["nether_wart_block", "warped_wart_block"];
pub const WITHER_SKELETON_DISLIKED_WEAPONS_TAG: &[&str] = &["bow", "crossbow"];
pub const WOLF_FOOD_TAG: &[&str] = &[
    "beef",
    "chicken",
    "cooked_beef",
    "cooked_chicken",
    "cooked_mutton",
    "cooked_porkchop",
    "cooked_rabbit",
    "mutton",
    "porkchop",
    "rabbit",
    "rotten_flesh",
    "cod",
    "cooked_cod",
    "salmon",
    "cooked_salmon",
    "tropical_fish",
    "pufferfish",
    "rabbit_stew",
];
pub const WOODEN_BUTTONS_TAG: &[&str] = &[
    "oak_button",
    "spruce_button",
    "birch_button",
    "jungle_button",
    "acacia_button",
    "dark_oak_button",
    "pale_oak_button",
    "crimson_button",
    "warped_button",
    "mangrove_button",
    "bamboo_button",
    "cherry_button",
];
pub const WOODEN_DOORS_TAG: &[&str] = &[
    "oak_door",
    "spruce_door",
    "birch_door",
    "jungle_door",
    "acacia_door",
    "dark_oak_door",
    "pale_oak_door",
    "crimson_door",
    "warped_door",
    "mangrove_door",
    "bamboo_door",
    "cherry_door",
];
pub const WOODEN_FENCES_TAG: &[&str] = &[
    "oak_fence",
    "acacia_fence",
    "dark_oak_fence",
    "pale_oak_fence",
    "spruce_fence",
    "birch_fence",
    "jungle_fence",
    "crimson_fence",
    "warped_fence",
    "mangrove_fence",
    "bamboo_fence",
    "cherry_fence",
];
pub const WOODEN_PRESSURE_PLATES_TAG: &[&str] = &[
    "oak_pressure_plate",
    "spruce_pressure_plate",
    "birch_pressure_plate",
    "jungle_pressure_plate",
    "acacia_pressure_plate",
    "dark_oak_pressure_plate",
    "pale_oak_pressure_plate",
    "crimson_pressure_plate",
    "warped_pressure_plate",
    "mangrove_pressure_plate",
    "bamboo_pressure_plate",
    "cherry_pressure_plate",
];
pub const WOODEN_SHELVES_TAG: &[&str] = &[
    "acacia_shelf",
    "bamboo_shelf",
    "birch_shelf",
    "cherry_shelf",
    "crimson_shelf",
    "dark_oak_shelf",
    "jungle_shelf",
    "mangrove_shelf",
    "oak_shelf",
    "pale_oak_shelf",
    "spruce_shelf",
    "warped_shelf",
];
pub const WOODEN_SLABS_TAG: &[&str] = &[
    "oak_slab",
    "spruce_slab",
    "birch_slab",
    "jungle_slab",
    "acacia_slab",
    "dark_oak_slab",
    "pale_oak_slab",
    "crimson_slab",
    "warped_slab",
    "mangrove_slab",
    "bamboo_slab",
    "cherry_slab",
];
pub const WOODEN_STAIRS_TAG: &[&str] = &[
    "oak_stairs",
    "spruce_stairs",
    "birch_stairs",
    "jungle_stairs",
    "acacia_stairs",
    "dark_oak_stairs",
    "pale_oak_stairs",
    "crimson_stairs",
    "warped_stairs",
    "mangrove_stairs",
    "bamboo_stairs",
    "cherry_stairs",
];
pub const WOODEN_TOOL_MATERIALS_TAG: &[&str] = &[
    "oak_planks",
    "spruce_planks",
    "birch_planks",
    "jungle_planks",
    "acacia_planks",
    "dark_oak_planks",
    "pale_oak_planks",
    "crimson_planks",
    "warped_planks",
    "mangrove_planks",
    "bamboo_planks",
    "cherry_planks",
];
pub const WOODEN_TRAPDOORS_TAG: &[&str] = &[
    "acacia_trapdoor",
    "birch_trapdoor",
    "dark_oak_trapdoor",
    "pale_oak_trapdoor",
    "jungle_trapdoor",
    "oak_trapdoor",
    "spruce_trapdoor",
    "crimson_trapdoor",
    "warped_trapdoor",
    "mangrove_trapdoor",
    "bamboo_trapdoor",
    "cherry_trapdoor",
];
pub const WOOL_TAG: &[&str] = &[
    "white_wool",
    "orange_wool",
    "magenta_wool",
    "light_blue_wool",
    "yellow_wool",
    "lime_wool",
    "pink_wool",
    "gray_wool",
    "light_gray_wool",
    "cyan_wool",
    "purple_wool",
    "blue_wool",
    "brown_wool",
    "green_wool",
    "red_wool",
    "black_wool",
];
pub const WOOL_CARPETS_TAG: &[&str] = &[
    "white_carpet",
    "orange_carpet",
    "magenta_carpet",
    "light_blue_carpet",
    "yellow_carpet",
    "lime_carpet",
    "pink_carpet",
    "gray_carpet",
    "light_gray_carpet",
    "cyan_carpet",
    "purple_carpet",
    "blue_carpet",
    "brown_carpet",
    "green_carpet",
    "red_carpet",
    "black_carpet",
];
pub fn register_item_tags(registry: &mut ItemRegistry) {
    registry.register_tag(Identifier::vanilla_static("acacia_logs"), ACACIA_LOGS_TAG);
    registry.register_tag(Identifier::vanilla_static("anvil"), ANVIL_TAG);
    registry.register_tag(
        Identifier::vanilla_static("armadillo_food"),
        ARMADILLO_FOOD_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("arrows"), ARROWS_TAG);
    registry.register_tag(Identifier::vanilla_static("axes"), AXES_TAG);
    registry.register_tag(Identifier::vanilla_static("axolotl_food"), AXOLOTL_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("bamboo_blocks"),
        BAMBOO_BLOCKS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("banners"), BANNERS_TAG);
    registry.register_tag(Identifier::vanilla_static("bars"), BARS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("beacon_payment_items"),
        BEACON_PAYMENT_ITEMS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("beds"), BEDS_TAG);
    registry.register_tag(Identifier::vanilla_static("bee_food"), BEE_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("birch_logs"), BIRCH_LOGS_TAG);
    registry.register_tag(Identifier::vanilla_static("boats"), BOATS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("book_cloning_target"),
        BOOK_CLONING_TARGET_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("bookshelf_books"),
        BOOKSHELF_BOOKS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("breaks_decorated_pots"),
        BREAKS_DECORATED_POTS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("brewing_fuel"), BREWING_FUEL_TAG);
    registry.register_tag(Identifier::vanilla_static("bundles"), BUNDLES_TAG);
    registry.register_tag(Identifier::vanilla_static("buttons"), BUTTONS_TAG);
    registry.register_tag(Identifier::vanilla_static("camel_food"), CAMEL_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("candles"), CANDLES_TAG);
    registry.register_tag(Identifier::vanilla_static("cat_food"), CAT_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("chains"), CHAINS_TAG);
    registry.register_tag(Identifier::vanilla_static("cherry_logs"), CHERRY_LOGS_TAG);
    registry.register_tag(Identifier::vanilla_static("chest_armor"), CHEST_ARMOR_TAG);
    registry.register_tag(Identifier::vanilla_static("chest_boats"), CHEST_BOATS_TAG);
    registry.register_tag(Identifier::vanilla_static("chicken_food"), CHICKEN_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("cluster_max_harvestables"),
        CLUSTER_MAX_HARVESTABLES_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("coal_ores"), COAL_ORES_TAG);
    registry.register_tag(Identifier::vanilla_static("coals"), COALS_TAG);
    registry.register_tag(Identifier::vanilla_static("compasses"), COMPASSES_TAG);
    registry.register_tag(
        Identifier::vanilla_static("completes_find_tree_tutorial"),
        COMPLETES_FIND_TREE_TUTORIAL_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("copper"), COPPER_TAG);
    registry.register_tag(
        Identifier::vanilla_static("copper_chests"),
        COPPER_CHESTS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("copper_golem_statues"),
        COPPER_GOLEM_STATUES_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("copper_ores"), COPPER_ORES_TAG);
    registry.register_tag(
        Identifier::vanilla_static("copper_tool_materials"),
        COPPER_TOOL_MATERIALS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("cow_food"), COW_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("creeper_drop_music_discs"),
        CREEPER_DROP_MUSIC_DISCS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("creeper_igniters"),
        CREEPER_IGNITERS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("crimson_stems"),
        CRIMSON_STEMS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("dampens_vibrations"),
        DAMPENS_VIBRATIONS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("dark_oak_logs"),
        DARK_OAK_LOGS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("decorated_pot_ingredients"),
        DECORATED_POT_INGREDIENTS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("decorated_pot_sherds"),
        DECORATED_POT_SHERDS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("diamond_ores"), DIAMOND_ORES_TAG);
    registry.register_tag(
        Identifier::vanilla_static("diamond_tool_materials"),
        DIAMOND_TOOL_MATERIALS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("dirt"), DIRT_TAG);
    registry.register_tag(Identifier::vanilla_static("doors"), DOORS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("drowned_preferred_weapons"),
        DROWNED_PREFERRED_WEAPONS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("duplicates_allays"),
        DUPLICATES_ALLAYS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("dyeable"), DYEABLE_TAG);
    registry.register_tag(Identifier::vanilla_static("eggs"), EGGS_TAG);
    registry.register_tag(Identifier::vanilla_static("emerald_ores"), EMERALD_ORES_TAG);
    registry.register_tag(
        Identifier::vanilla_static("enchantable/armor"),
        ENCHANTABLE_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/bow"),
        ENCHANTABLE_BOW_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/chest_armor"),
        ENCHANTABLE_CHEST_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/crossbow"),
        ENCHANTABLE_CROSSBOW_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/durability"),
        ENCHANTABLE_DURABILITY_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/equippable"),
        ENCHANTABLE_EQUIPPABLE_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/fire_aspect"),
        ENCHANTABLE_FIRE_ASPECT_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/fishing"),
        ENCHANTABLE_FISHING_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/foot_armor"),
        ENCHANTABLE_FOOT_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/head_armor"),
        ENCHANTABLE_HEAD_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/leg_armor"),
        ENCHANTABLE_LEG_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/mace"),
        ENCHANTABLE_MACE_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/mining"),
        ENCHANTABLE_MINING_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/mining_loot"),
        ENCHANTABLE_MINING_LOOT_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/sharp_weapon"),
        ENCHANTABLE_SHARP_WEAPON_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/sword"),
        ENCHANTABLE_SWORD_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/trident"),
        ENCHANTABLE_TRIDENT_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/vanishing"),
        ENCHANTABLE_VANISHING_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("enchantable/weapon"),
        ENCHANTABLE_WEAPON_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("fence_gates"), FENCE_GATES_TAG);
    registry.register_tag(Identifier::vanilla_static("fences"), FENCES_TAG);
    registry.register_tag(Identifier::vanilla_static("fishes"), FISHES_TAG);
    registry.register_tag(Identifier::vanilla_static("flowers"), FLOWERS_TAG);
    registry.register_tag(Identifier::vanilla_static("foot_armor"), FOOT_ARMOR_TAG);
    registry.register_tag(Identifier::vanilla_static("fox_food"), FOX_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("freeze_immune_wearables"),
        FREEZE_IMMUNE_WEARABLES_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("frog_food"), FROG_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("furnace_minecart_fuel"),
        FURNACE_MINECART_FUEL_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("gaze_disguise_equipment"),
        GAZE_DISGUISE_EQUIPMENT_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("goat_food"), GOAT_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("gold_ores"), GOLD_ORES_TAG);
    registry.register_tag(
        Identifier::vanilla_static("gold_tool_materials"),
        GOLD_TOOL_MATERIALS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("hanging_signs"),
        HANGING_SIGNS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("happy_ghast_food"),
        HAPPY_GHAST_FOOD_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("happy_ghast_tempt_items"),
        HAPPY_GHAST_TEMPT_ITEMS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("harnesses"), HARNESSES_TAG);
    registry.register_tag(Identifier::vanilla_static("head_armor"), HEAD_ARMOR_TAG);
    registry.register_tag(Identifier::vanilla_static("hoes"), HOES_TAG);
    registry.register_tag(Identifier::vanilla_static("hoglin_food"), HOGLIN_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("horse_food"), HORSE_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("horse_tempt_items"),
        HORSE_TEMPT_ITEMS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("ignored_by_piglin_babies"),
        IGNORED_BY_PIGLIN_BABIES_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("iron_ores"), IRON_ORES_TAG);
    registry.register_tag(
        Identifier::vanilla_static("iron_tool_materials"),
        IRON_TOOL_MATERIALS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("jungle_logs"), JUNGLE_LOGS_TAG);
    registry.register_tag(Identifier::vanilla_static("lanterns"), LANTERNS_TAG);
    registry.register_tag(Identifier::vanilla_static("lapis_ores"), LAPIS_ORES_TAG);
    registry.register_tag(Identifier::vanilla_static("leaves"), LEAVES_TAG);
    registry.register_tag(
        Identifier::vanilla_static("lectern_books"),
        LECTERN_BOOKS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("leg_armor"), LEG_ARMOR_TAG);
    registry.register_tag(
        Identifier::vanilla_static("lightning_rods"),
        LIGHTNING_RODS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("llama_food"), LLAMA_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("llama_tempt_items"),
        LLAMA_TEMPT_ITEMS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("logs"), LOGS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("logs_that_burn"),
        LOGS_THAT_BURN_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("mangrove_logs"),
        MANGROVE_LOGS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("map_invisibility_equipment"),
        MAP_INVISIBILITY_EQUIPMENT_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("meat"), MEAT_TAG);
    registry.register_tag(
        Identifier::vanilla_static("netherite_tool_materials"),
        NETHERITE_TOOL_MATERIALS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("non_flammable_wood"),
        NON_FLAMMABLE_WOOD_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("noteblock_top_instruments"),
        NOTEBLOCK_TOP_INSTRUMENTS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("oak_logs"), OAK_LOGS_TAG);
    registry.register_tag(Identifier::vanilla_static("ocelot_food"), OCELOT_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("pale_oak_logs"),
        PALE_OAK_LOGS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("panda_eats_from_ground"),
        PANDA_EATS_FROM_GROUND_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("panda_food"), PANDA_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("parrot_food"), PARROT_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("parrot_poisonous_food"),
        PARROT_POISONOUS_FOOD_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("pickaxes"), PICKAXES_TAG);
    registry.register_tag(Identifier::vanilla_static("pig_food"), PIG_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("piglin_food"), PIGLIN_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("piglin_loved"), PIGLIN_LOVED_TAG);
    registry.register_tag(
        Identifier::vanilla_static("piglin_preferred_weapons"),
        PIGLIN_PREFERRED_WEAPONS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("piglin_repellents"),
        PIGLIN_REPELLENTS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("piglin_safe_armor"),
        PIGLIN_SAFE_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("pillager_preferred_weapons"),
        PILLAGER_PREFERRED_WEAPONS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("planks"), PLANKS_TAG);
    registry.register_tag(Identifier::vanilla_static("rabbit_food"), RABBIT_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("rails"), RAILS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("redstone_ores"),
        REDSTONE_ORES_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_chain_armor"),
        REPAIRS_CHAIN_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_copper_armor"),
        REPAIRS_COPPER_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_diamond_armor"),
        REPAIRS_DIAMOND_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_gold_armor"),
        REPAIRS_GOLD_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_iron_armor"),
        REPAIRS_IRON_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_leather_armor"),
        REPAIRS_LEATHER_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_netherite_armor"),
        REPAIRS_NETHERITE_ARMOR_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_turtle_helmet"),
        REPAIRS_TURTLE_HELMET_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("repairs_wolf_armor"),
        REPAIRS_WOLF_ARMOR_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("sand"), SAND_TAG);
    registry.register_tag(Identifier::vanilla_static("saplings"), SAPLINGS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("shearable_from_copper_golem"),
        SHEARABLE_FROM_COPPER_GOLEM_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("sheep_food"), SHEEP_FOOD_TAG);
    registry.register_tag(Identifier::vanilla_static("shovels"), SHOVELS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("shulker_boxes"),
        SHULKER_BOXES_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("signs"), SIGNS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("skeleton_preferred_weapons"),
        SKELETON_PREFERRED_WEAPONS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("skulls"), SKULLS_TAG);
    registry.register_tag(Identifier::vanilla_static("slabs"), SLABS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("small_flowers"),
        SMALL_FLOWERS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("smelts_to_glass"),
        SMELTS_TO_GLASS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("sniffer_food"), SNIFFER_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("soul_fire_base_blocks"),
        SOUL_FIRE_BASE_BLOCKS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("spruce_logs"), SPRUCE_LOGS_TAG);
    registry.register_tag(Identifier::vanilla_static("stairs"), STAIRS_TAG);
    registry.register_tag(Identifier::vanilla_static("stone_bricks"), STONE_BRICKS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("stone_buttons"),
        STONE_BUTTONS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("stone_crafting_materials"),
        STONE_CRAFTING_MATERIALS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("stone_tool_materials"),
        STONE_TOOL_MATERIALS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("strider_food"), STRIDER_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("strider_tempt_items"),
        STRIDER_TEMPT_ITEMS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("swords"), SWORDS_TAG);
    registry.register_tag(Identifier::vanilla_static("terracotta"), TERRACOTTA_TAG);
    registry.register_tag(Identifier::vanilla_static("trapdoors"), TRAPDOORS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("trim_materials"),
        TRIM_MATERIALS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("trimmable_armor"),
        TRIMMABLE_ARMOR_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("turtle_food"), TURTLE_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("villager_picks_up"),
        VILLAGER_PICKS_UP_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("villager_plantable_seeds"),
        VILLAGER_PLANTABLE_SEEDS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("walls"), WALLS_TAG);
    registry.register_tag(Identifier::vanilla_static("warped_stems"), WARPED_STEMS_TAG);
    registry.register_tag(Identifier::vanilla_static("wart_blocks"), WART_BLOCKS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("wither_skeleton_disliked_weapons"),
        WITHER_SKELETON_DISLIKED_WEAPONS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("wolf_food"), WOLF_FOOD_TAG);
    registry.register_tag(
        Identifier::vanilla_static("wooden_buttons"),
        WOODEN_BUTTONS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("wooden_doors"), WOODEN_DOORS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("wooden_fences"),
        WOODEN_FENCES_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("wooden_pressure_plates"),
        WOODEN_PRESSURE_PLATES_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("wooden_shelves"),
        WOODEN_SHELVES_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("wooden_slabs"), WOODEN_SLABS_TAG);
    registry.register_tag(
        Identifier::vanilla_static("wooden_stairs"),
        WOODEN_STAIRS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("wooden_tool_materials"),
        WOODEN_TOOL_MATERIALS_TAG,
    );
    registry.register_tag(
        Identifier::vanilla_static("wooden_trapdoors"),
        WOODEN_TRAPDOORS_TAG,
    );
    registry.register_tag(Identifier::vanilla_static("wool"), WOOL_TAG);
    registry.register_tag(Identifier::vanilla_static("wool_carpets"), WOOL_CARPETS_TAG);
}
