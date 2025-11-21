use crate::types::Identifier;
use crate::{
    data_components::{vanilla_components, DataComponentMap},
    items::{Item, ItemRegistry},
    vanilla_blocks,
};
use std::sync::LazyLock;
pub static ITEMS: LazyLock<Items> = LazyLock::new(Items::init);
pub struct Items {
    pub air: Item,
    pub stone: Item,
    pub granite: Item,
    pub polished_granite: Item,
    pub diorite: Item,
    pub polished_diorite: Item,
    pub andesite: Item,
    pub polished_andesite: Item,
    pub deepslate: Item,
    pub cobbled_deepslate: Item,
    pub polished_deepslate: Item,
    pub calcite: Item,
    pub tuff: Item,
    pub tuff_slab: Item,
    pub tuff_stairs: Item,
    pub tuff_wall: Item,
    pub chiseled_tuff: Item,
    pub polished_tuff: Item,
    pub polished_tuff_slab: Item,
    pub polished_tuff_stairs: Item,
    pub polished_tuff_wall: Item,
    pub tuff_bricks: Item,
    pub tuff_brick_slab: Item,
    pub tuff_brick_stairs: Item,
    pub tuff_brick_wall: Item,
    pub chiseled_tuff_bricks: Item,
    pub dripstone_block: Item,
    pub grass_block: Item,
    pub dirt: Item,
    pub coarse_dirt: Item,
    pub podzol: Item,
    pub rooted_dirt: Item,
    pub mud: Item,
    pub crimson_nylium: Item,
    pub warped_nylium: Item,
    pub cobblestone: Item,
    pub oak_planks: Item,
    pub spruce_planks: Item,
    pub birch_planks: Item,
    pub jungle_planks: Item,
    pub acacia_planks: Item,
    pub cherry_planks: Item,
    pub dark_oak_planks: Item,
    pub pale_oak_planks: Item,
    pub mangrove_planks: Item,
    pub bamboo_planks: Item,
    pub crimson_planks: Item,
    pub warped_planks: Item,
    pub bamboo_mosaic: Item,
    pub oak_sapling: Item,
    pub spruce_sapling: Item,
    pub birch_sapling: Item,
    pub jungle_sapling: Item,
    pub acacia_sapling: Item,
    pub cherry_sapling: Item,
    pub dark_oak_sapling: Item,
    pub pale_oak_sapling: Item,
    pub mangrove_propagule: Item,
    pub bedrock: Item,
    pub sand: Item,
    pub suspicious_sand: Item,
    pub suspicious_gravel: Item,
    pub red_sand: Item,
    pub gravel: Item,
    pub coal_ore: Item,
    pub deepslate_coal_ore: Item,
    pub iron_ore: Item,
    pub deepslate_iron_ore: Item,
    pub copper_ore: Item,
    pub deepslate_copper_ore: Item,
    pub gold_ore: Item,
    pub deepslate_gold_ore: Item,
    pub redstone_ore: Item,
    pub deepslate_redstone_ore: Item,
    pub emerald_ore: Item,
    pub deepslate_emerald_ore: Item,
    pub lapis_ore: Item,
    pub deepslate_lapis_ore: Item,
    pub diamond_ore: Item,
    pub deepslate_diamond_ore: Item,
    pub nether_gold_ore: Item,
    pub nether_quartz_ore: Item,
    pub ancient_debris: Item,
    pub coal_block: Item,
    pub raw_iron_block: Item,
    pub raw_copper_block: Item,
    pub raw_gold_block: Item,
    pub heavy_core: Item,
    pub amethyst_block: Item,
    pub budding_amethyst: Item,
    pub iron_block: Item,
    pub copper_block: Item,
    pub gold_block: Item,
    pub diamond_block: Item,
    pub netherite_block: Item,
    pub exposed_copper: Item,
    pub weathered_copper: Item,
    pub oxidized_copper: Item,
    pub chiseled_copper: Item,
    pub exposed_chiseled_copper: Item,
    pub weathered_chiseled_copper: Item,
    pub oxidized_chiseled_copper: Item,
    pub cut_copper: Item,
    pub exposed_cut_copper: Item,
    pub weathered_cut_copper: Item,
    pub oxidized_cut_copper: Item,
    pub cut_copper_stairs: Item,
    pub exposed_cut_copper_stairs: Item,
    pub weathered_cut_copper_stairs: Item,
    pub oxidized_cut_copper_stairs: Item,
    pub cut_copper_slab: Item,
    pub exposed_cut_copper_slab: Item,
    pub weathered_cut_copper_slab: Item,
    pub oxidized_cut_copper_slab: Item,
    pub waxed_copper_block: Item,
    pub waxed_exposed_copper: Item,
    pub waxed_weathered_copper: Item,
    pub waxed_oxidized_copper: Item,
    pub waxed_chiseled_copper: Item,
    pub waxed_exposed_chiseled_copper: Item,
    pub waxed_weathered_chiseled_copper: Item,
    pub waxed_oxidized_chiseled_copper: Item,
    pub waxed_cut_copper: Item,
    pub waxed_exposed_cut_copper: Item,
    pub waxed_weathered_cut_copper: Item,
    pub waxed_oxidized_cut_copper: Item,
    pub waxed_cut_copper_stairs: Item,
    pub waxed_exposed_cut_copper_stairs: Item,
    pub waxed_weathered_cut_copper_stairs: Item,
    pub waxed_oxidized_cut_copper_stairs: Item,
    pub waxed_cut_copper_slab: Item,
    pub waxed_exposed_cut_copper_slab: Item,
    pub waxed_weathered_cut_copper_slab: Item,
    pub waxed_oxidized_cut_copper_slab: Item,
    pub oak_log: Item,
    pub spruce_log: Item,
    pub birch_log: Item,
    pub jungle_log: Item,
    pub acacia_log: Item,
    pub cherry_log: Item,
    pub pale_oak_log: Item,
    pub dark_oak_log: Item,
    pub mangrove_log: Item,
    pub mangrove_roots: Item,
    pub muddy_mangrove_roots: Item,
    pub crimson_stem: Item,
    pub warped_stem: Item,
    pub bamboo_block: Item,
    pub stripped_oak_log: Item,
    pub stripped_spruce_log: Item,
    pub stripped_birch_log: Item,
    pub stripped_jungle_log: Item,
    pub stripped_acacia_log: Item,
    pub stripped_cherry_log: Item,
    pub stripped_dark_oak_log: Item,
    pub stripped_pale_oak_log: Item,
    pub stripped_mangrove_log: Item,
    pub stripped_crimson_stem: Item,
    pub stripped_warped_stem: Item,
    pub stripped_oak_wood: Item,
    pub stripped_spruce_wood: Item,
    pub stripped_birch_wood: Item,
    pub stripped_jungle_wood: Item,
    pub stripped_acacia_wood: Item,
    pub stripped_cherry_wood: Item,
    pub stripped_dark_oak_wood: Item,
    pub stripped_pale_oak_wood: Item,
    pub stripped_mangrove_wood: Item,
    pub stripped_crimson_hyphae: Item,
    pub stripped_warped_hyphae: Item,
    pub stripped_bamboo_block: Item,
    pub oak_wood: Item,
    pub spruce_wood: Item,
    pub birch_wood: Item,
    pub jungle_wood: Item,
    pub acacia_wood: Item,
    pub cherry_wood: Item,
    pub pale_oak_wood: Item,
    pub dark_oak_wood: Item,
    pub mangrove_wood: Item,
    pub crimson_hyphae: Item,
    pub warped_hyphae: Item,
    pub oak_leaves: Item,
    pub spruce_leaves: Item,
    pub birch_leaves: Item,
    pub jungle_leaves: Item,
    pub acacia_leaves: Item,
    pub cherry_leaves: Item,
    pub dark_oak_leaves: Item,
    pub pale_oak_leaves: Item,
    pub mangrove_leaves: Item,
    pub azalea_leaves: Item,
    pub flowering_azalea_leaves: Item,
    pub sponge: Item,
    pub wet_sponge: Item,
    pub glass: Item,
    pub tinted_glass: Item,
    pub lapis_block: Item,
    pub sandstone: Item,
    pub chiseled_sandstone: Item,
    pub cut_sandstone: Item,
    pub cobweb: Item,
    pub short_grass: Item,
    pub fern: Item,
    pub bush: Item,
    pub azalea: Item,
    pub flowering_azalea: Item,
    pub dead_bush: Item,
    pub firefly_bush: Item,
    pub short_dry_grass: Item,
    pub tall_dry_grass: Item,
    pub seagrass: Item,
    pub sea_pickle: Item,
    pub white_wool: Item,
    pub orange_wool: Item,
    pub magenta_wool: Item,
    pub light_blue_wool: Item,
    pub yellow_wool: Item,
    pub lime_wool: Item,
    pub pink_wool: Item,
    pub gray_wool: Item,
    pub light_gray_wool: Item,
    pub cyan_wool: Item,
    pub purple_wool: Item,
    pub blue_wool: Item,
    pub brown_wool: Item,
    pub green_wool: Item,
    pub red_wool: Item,
    pub black_wool: Item,
    pub dandelion: Item,
    pub open_eyeblossom: Item,
    pub closed_eyeblossom: Item,
    pub poppy: Item,
    pub blue_orchid: Item,
    pub allium: Item,
    pub azure_bluet: Item,
    pub red_tulip: Item,
    pub orange_tulip: Item,
    pub white_tulip: Item,
    pub pink_tulip: Item,
    pub oxeye_daisy: Item,
    pub cornflower: Item,
    pub lily_of_the_valley: Item,
    pub wither_rose: Item,
    pub torchflower: Item,
    pub pitcher_plant: Item,
    pub spore_blossom: Item,
    pub brown_mushroom: Item,
    pub red_mushroom: Item,
    pub crimson_fungus: Item,
    pub warped_fungus: Item,
    pub crimson_roots: Item,
    pub warped_roots: Item,
    pub nether_sprouts: Item,
    pub weeping_vines: Item,
    pub twisting_vines: Item,
    pub sugar_cane: Item,
    pub kelp: Item,
    pub pink_petals: Item,
    pub wildflowers: Item,
    pub leaf_litter: Item,
    pub moss_carpet: Item,
    pub moss_block: Item,
    pub pale_moss_carpet: Item,
    pub pale_hanging_moss: Item,
    pub pale_moss_block: Item,
    pub hanging_roots: Item,
    pub big_dripleaf: Item,
    pub small_dripleaf: Item,
    pub bamboo: Item,
    pub oak_slab: Item,
    pub spruce_slab: Item,
    pub birch_slab: Item,
    pub jungle_slab: Item,
    pub acacia_slab: Item,
    pub cherry_slab: Item,
    pub dark_oak_slab: Item,
    pub pale_oak_slab: Item,
    pub mangrove_slab: Item,
    pub bamboo_slab: Item,
    pub bamboo_mosaic_slab: Item,
    pub crimson_slab: Item,
    pub warped_slab: Item,
    pub stone_slab: Item,
    pub smooth_stone_slab: Item,
    pub sandstone_slab: Item,
    pub cut_sandstone_slab: Item,
    pub petrified_oak_slab: Item,
    pub cobblestone_slab: Item,
    pub brick_slab: Item,
    pub stone_brick_slab: Item,
    pub mud_brick_slab: Item,
    pub nether_brick_slab: Item,
    pub quartz_slab: Item,
    pub red_sandstone_slab: Item,
    pub cut_red_sandstone_slab: Item,
    pub purpur_slab: Item,
    pub prismarine_slab: Item,
    pub prismarine_brick_slab: Item,
    pub dark_prismarine_slab: Item,
    pub smooth_quartz: Item,
    pub smooth_red_sandstone: Item,
    pub smooth_sandstone: Item,
    pub smooth_stone: Item,
    pub bricks: Item,
    pub acacia_shelf: Item,
    pub bamboo_shelf: Item,
    pub birch_shelf: Item,
    pub cherry_shelf: Item,
    pub crimson_shelf: Item,
    pub dark_oak_shelf: Item,
    pub jungle_shelf: Item,
    pub mangrove_shelf: Item,
    pub oak_shelf: Item,
    pub pale_oak_shelf: Item,
    pub spruce_shelf: Item,
    pub warped_shelf: Item,
    pub bookshelf: Item,
    pub chiseled_bookshelf: Item,
    pub decorated_pot: Item,
    pub mossy_cobblestone: Item,
    pub obsidian: Item,
    pub torch: Item,
    pub end_rod: Item,
    pub chorus_plant: Item,
    pub chorus_flower: Item,
    pub purpur_block: Item,
    pub purpur_pillar: Item,
    pub purpur_stairs: Item,
    pub spawner: Item,
    pub creaking_heart: Item,
    pub chest: Item,
    pub crafting_table: Item,
    pub farmland: Item,
    pub furnace: Item,
    pub ladder: Item,
    pub cobblestone_stairs: Item,
    pub snow: Item,
    pub ice: Item,
    pub snow_block: Item,
    pub cactus: Item,
    pub cactus_flower: Item,
    pub clay: Item,
    pub jukebox: Item,
    pub oak_fence: Item,
    pub spruce_fence: Item,
    pub birch_fence: Item,
    pub jungle_fence: Item,
    pub acacia_fence: Item,
    pub cherry_fence: Item,
    pub dark_oak_fence: Item,
    pub pale_oak_fence: Item,
    pub mangrove_fence: Item,
    pub bamboo_fence: Item,
    pub crimson_fence: Item,
    pub warped_fence: Item,
    pub pumpkin: Item,
    pub carved_pumpkin: Item,
    pub jack_o_lantern: Item,
    pub netherrack: Item,
    pub soul_sand: Item,
    pub soul_soil: Item,
    pub basalt: Item,
    pub polished_basalt: Item,
    pub smooth_basalt: Item,
    pub soul_torch: Item,
    pub copper_torch: Item,
    pub glowstone: Item,
    pub infested_stone: Item,
    pub infested_cobblestone: Item,
    pub infested_stone_bricks: Item,
    pub infested_mossy_stone_bricks: Item,
    pub infested_cracked_stone_bricks: Item,
    pub infested_chiseled_stone_bricks: Item,
    pub infested_deepslate: Item,
    pub stone_bricks: Item,
    pub mossy_stone_bricks: Item,
    pub cracked_stone_bricks: Item,
    pub chiseled_stone_bricks: Item,
    pub packed_mud: Item,
    pub mud_bricks: Item,
    pub deepslate_bricks: Item,
    pub cracked_deepslate_bricks: Item,
    pub deepslate_tiles: Item,
    pub cracked_deepslate_tiles: Item,
    pub chiseled_deepslate: Item,
    pub reinforced_deepslate: Item,
    pub brown_mushroom_block: Item,
    pub red_mushroom_block: Item,
    pub mushroom_stem: Item,
    pub iron_bars: Item,
    pub copper_bars: Item,
    pub exposed_copper_bars: Item,
    pub weathered_copper_bars: Item,
    pub oxidized_copper_bars: Item,
    pub waxed_copper_bars: Item,
    pub waxed_exposed_copper_bars: Item,
    pub waxed_weathered_copper_bars: Item,
    pub waxed_oxidized_copper_bars: Item,
    pub iron_chain: Item,
    pub copper_chain: Item,
    pub exposed_copper_chain: Item,
    pub weathered_copper_chain: Item,
    pub oxidized_copper_chain: Item,
    pub waxed_copper_chain: Item,
    pub waxed_exposed_copper_chain: Item,
    pub waxed_weathered_copper_chain: Item,
    pub waxed_oxidized_copper_chain: Item,
    pub glass_pane: Item,
    pub melon: Item,
    pub vine: Item,
    pub glow_lichen: Item,
    pub resin_clump: Item,
    pub resin_block: Item,
    pub resin_bricks: Item,
    pub resin_brick_stairs: Item,
    pub resin_brick_slab: Item,
    pub resin_brick_wall: Item,
    pub chiseled_resin_bricks: Item,
    pub brick_stairs: Item,
    pub stone_brick_stairs: Item,
    pub mud_brick_stairs: Item,
    pub mycelium: Item,
    pub lily_pad: Item,
    pub nether_bricks: Item,
    pub cracked_nether_bricks: Item,
    pub chiseled_nether_bricks: Item,
    pub nether_brick_fence: Item,
    pub nether_brick_stairs: Item,
    pub sculk: Item,
    pub sculk_vein: Item,
    pub sculk_catalyst: Item,
    pub sculk_shrieker: Item,
    pub enchanting_table: Item,
    pub end_portal_frame: Item,
    pub end_stone: Item,
    pub end_stone_bricks: Item,
    pub dragon_egg: Item,
    pub sandstone_stairs: Item,
    pub ender_chest: Item,
    pub emerald_block: Item,
    pub oak_stairs: Item,
    pub spruce_stairs: Item,
    pub birch_stairs: Item,
    pub jungle_stairs: Item,
    pub acacia_stairs: Item,
    pub cherry_stairs: Item,
    pub dark_oak_stairs: Item,
    pub pale_oak_stairs: Item,
    pub mangrove_stairs: Item,
    pub bamboo_stairs: Item,
    pub bamboo_mosaic_stairs: Item,
    pub crimson_stairs: Item,
    pub warped_stairs: Item,
    pub command_block: Item,
    pub beacon: Item,
    pub cobblestone_wall: Item,
    pub mossy_cobblestone_wall: Item,
    pub brick_wall: Item,
    pub prismarine_wall: Item,
    pub red_sandstone_wall: Item,
    pub mossy_stone_brick_wall: Item,
    pub granite_wall: Item,
    pub stone_brick_wall: Item,
    pub mud_brick_wall: Item,
    pub nether_brick_wall: Item,
    pub andesite_wall: Item,
    pub red_nether_brick_wall: Item,
    pub sandstone_wall: Item,
    pub end_stone_brick_wall: Item,
    pub diorite_wall: Item,
    pub blackstone_wall: Item,
    pub polished_blackstone_wall: Item,
    pub polished_blackstone_brick_wall: Item,
    pub cobbled_deepslate_wall: Item,
    pub polished_deepslate_wall: Item,
    pub deepslate_brick_wall: Item,
    pub deepslate_tile_wall: Item,
    pub anvil: Item,
    pub chipped_anvil: Item,
    pub damaged_anvil: Item,
    pub chiseled_quartz_block: Item,
    pub quartz_block: Item,
    pub quartz_bricks: Item,
    pub quartz_pillar: Item,
    pub quartz_stairs: Item,
    pub white_terracotta: Item,
    pub orange_terracotta: Item,
    pub magenta_terracotta: Item,
    pub light_blue_terracotta: Item,
    pub yellow_terracotta: Item,
    pub lime_terracotta: Item,
    pub pink_terracotta: Item,
    pub gray_terracotta: Item,
    pub light_gray_terracotta: Item,
    pub cyan_terracotta: Item,
    pub purple_terracotta: Item,
    pub blue_terracotta: Item,
    pub brown_terracotta: Item,
    pub green_terracotta: Item,
    pub red_terracotta: Item,
    pub black_terracotta: Item,
    pub barrier: Item,
    pub light: Item,
    pub hay_block: Item,
    pub white_carpet: Item,
    pub orange_carpet: Item,
    pub magenta_carpet: Item,
    pub light_blue_carpet: Item,
    pub yellow_carpet: Item,
    pub lime_carpet: Item,
    pub pink_carpet: Item,
    pub gray_carpet: Item,
    pub light_gray_carpet: Item,
    pub cyan_carpet: Item,
    pub purple_carpet: Item,
    pub blue_carpet: Item,
    pub brown_carpet: Item,
    pub green_carpet: Item,
    pub red_carpet: Item,
    pub black_carpet: Item,
    pub terracotta: Item,
    pub packed_ice: Item,
    pub dirt_path: Item,
    pub sunflower: Item,
    pub lilac: Item,
    pub rose_bush: Item,
    pub peony: Item,
    pub tall_grass: Item,
    pub large_fern: Item,
    pub white_stained_glass: Item,
    pub orange_stained_glass: Item,
    pub magenta_stained_glass: Item,
    pub light_blue_stained_glass: Item,
    pub yellow_stained_glass: Item,
    pub lime_stained_glass: Item,
    pub pink_stained_glass: Item,
    pub gray_stained_glass: Item,
    pub light_gray_stained_glass: Item,
    pub cyan_stained_glass: Item,
    pub purple_stained_glass: Item,
    pub blue_stained_glass: Item,
    pub brown_stained_glass: Item,
    pub green_stained_glass: Item,
    pub red_stained_glass: Item,
    pub black_stained_glass: Item,
    pub white_stained_glass_pane: Item,
    pub orange_stained_glass_pane: Item,
    pub magenta_stained_glass_pane: Item,
    pub light_blue_stained_glass_pane: Item,
    pub yellow_stained_glass_pane: Item,
    pub lime_stained_glass_pane: Item,
    pub pink_stained_glass_pane: Item,
    pub gray_stained_glass_pane: Item,
    pub light_gray_stained_glass_pane: Item,
    pub cyan_stained_glass_pane: Item,
    pub purple_stained_glass_pane: Item,
    pub blue_stained_glass_pane: Item,
    pub brown_stained_glass_pane: Item,
    pub green_stained_glass_pane: Item,
    pub red_stained_glass_pane: Item,
    pub black_stained_glass_pane: Item,
    pub prismarine: Item,
    pub prismarine_bricks: Item,
    pub dark_prismarine: Item,
    pub prismarine_stairs: Item,
    pub prismarine_brick_stairs: Item,
    pub dark_prismarine_stairs: Item,
    pub sea_lantern: Item,
    pub red_sandstone: Item,
    pub chiseled_red_sandstone: Item,
    pub cut_red_sandstone: Item,
    pub red_sandstone_stairs: Item,
    pub repeating_command_block: Item,
    pub chain_command_block: Item,
    pub magma_block: Item,
    pub nether_wart_block: Item,
    pub warped_wart_block: Item,
    pub red_nether_bricks: Item,
    pub bone_block: Item,
    pub structure_void: Item,
    pub shulker_box: Item,
    pub white_shulker_box: Item,
    pub orange_shulker_box: Item,
    pub magenta_shulker_box: Item,
    pub light_blue_shulker_box: Item,
    pub yellow_shulker_box: Item,
    pub lime_shulker_box: Item,
    pub pink_shulker_box: Item,
    pub gray_shulker_box: Item,
    pub light_gray_shulker_box: Item,
    pub cyan_shulker_box: Item,
    pub purple_shulker_box: Item,
    pub blue_shulker_box: Item,
    pub brown_shulker_box: Item,
    pub green_shulker_box: Item,
    pub red_shulker_box: Item,
    pub black_shulker_box: Item,
    pub white_glazed_terracotta: Item,
    pub orange_glazed_terracotta: Item,
    pub magenta_glazed_terracotta: Item,
    pub light_blue_glazed_terracotta: Item,
    pub yellow_glazed_terracotta: Item,
    pub lime_glazed_terracotta: Item,
    pub pink_glazed_terracotta: Item,
    pub gray_glazed_terracotta: Item,
    pub light_gray_glazed_terracotta: Item,
    pub cyan_glazed_terracotta: Item,
    pub purple_glazed_terracotta: Item,
    pub blue_glazed_terracotta: Item,
    pub brown_glazed_terracotta: Item,
    pub green_glazed_terracotta: Item,
    pub red_glazed_terracotta: Item,
    pub black_glazed_terracotta: Item,
    pub white_concrete: Item,
    pub orange_concrete: Item,
    pub magenta_concrete: Item,
    pub light_blue_concrete: Item,
    pub yellow_concrete: Item,
    pub lime_concrete: Item,
    pub pink_concrete: Item,
    pub gray_concrete: Item,
    pub light_gray_concrete: Item,
    pub cyan_concrete: Item,
    pub purple_concrete: Item,
    pub blue_concrete: Item,
    pub brown_concrete: Item,
    pub green_concrete: Item,
    pub red_concrete: Item,
    pub black_concrete: Item,
    pub white_concrete_powder: Item,
    pub orange_concrete_powder: Item,
    pub magenta_concrete_powder: Item,
    pub light_blue_concrete_powder: Item,
    pub yellow_concrete_powder: Item,
    pub lime_concrete_powder: Item,
    pub pink_concrete_powder: Item,
    pub gray_concrete_powder: Item,
    pub light_gray_concrete_powder: Item,
    pub cyan_concrete_powder: Item,
    pub purple_concrete_powder: Item,
    pub blue_concrete_powder: Item,
    pub brown_concrete_powder: Item,
    pub green_concrete_powder: Item,
    pub red_concrete_powder: Item,
    pub black_concrete_powder: Item,
    pub turtle_egg: Item,
    pub sniffer_egg: Item,
    pub dried_ghast: Item,
    pub dead_tube_coral_block: Item,
    pub dead_brain_coral_block: Item,
    pub dead_bubble_coral_block: Item,
    pub dead_fire_coral_block: Item,
    pub dead_horn_coral_block: Item,
    pub tube_coral_block: Item,
    pub brain_coral_block: Item,
    pub bubble_coral_block: Item,
    pub fire_coral_block: Item,
    pub horn_coral_block: Item,
    pub tube_coral: Item,
    pub brain_coral: Item,
    pub bubble_coral: Item,
    pub fire_coral: Item,
    pub horn_coral: Item,
    pub dead_brain_coral: Item,
    pub dead_bubble_coral: Item,
    pub dead_fire_coral: Item,
    pub dead_horn_coral: Item,
    pub dead_tube_coral: Item,
    pub tube_coral_fan: Item,
    pub brain_coral_fan: Item,
    pub bubble_coral_fan: Item,
    pub fire_coral_fan: Item,
    pub horn_coral_fan: Item,
    pub dead_tube_coral_fan: Item,
    pub dead_brain_coral_fan: Item,
    pub dead_bubble_coral_fan: Item,
    pub dead_fire_coral_fan: Item,
    pub dead_horn_coral_fan: Item,
    pub blue_ice: Item,
    pub conduit: Item,
    pub polished_granite_stairs: Item,
    pub smooth_red_sandstone_stairs: Item,
    pub mossy_stone_brick_stairs: Item,
    pub polished_diorite_stairs: Item,
    pub mossy_cobblestone_stairs: Item,
    pub end_stone_brick_stairs: Item,
    pub stone_stairs: Item,
    pub smooth_sandstone_stairs: Item,
    pub smooth_quartz_stairs: Item,
    pub granite_stairs: Item,
    pub andesite_stairs: Item,
    pub red_nether_brick_stairs: Item,
    pub polished_andesite_stairs: Item,
    pub diorite_stairs: Item,
    pub cobbled_deepslate_stairs: Item,
    pub polished_deepslate_stairs: Item,
    pub deepslate_brick_stairs: Item,
    pub deepslate_tile_stairs: Item,
    pub polished_granite_slab: Item,
    pub smooth_red_sandstone_slab: Item,
    pub mossy_stone_brick_slab: Item,
    pub polished_diorite_slab: Item,
    pub mossy_cobblestone_slab: Item,
    pub end_stone_brick_slab: Item,
    pub smooth_sandstone_slab: Item,
    pub smooth_quartz_slab: Item,
    pub granite_slab: Item,
    pub andesite_slab: Item,
    pub red_nether_brick_slab: Item,
    pub polished_andesite_slab: Item,
    pub diorite_slab: Item,
    pub cobbled_deepslate_slab: Item,
    pub polished_deepslate_slab: Item,
    pub deepslate_brick_slab: Item,
    pub deepslate_tile_slab: Item,
    pub scaffolding: Item,
    pub redstone: Item,
    pub redstone_torch: Item,
    pub redstone_block: Item,
    pub repeater: Item,
    pub comparator: Item,
    pub piston: Item,
    pub sticky_piston: Item,
    pub slime_block: Item,
    pub honey_block: Item,
    pub observer: Item,
    pub hopper: Item,
    pub dispenser: Item,
    pub dropper: Item,
    pub lectern: Item,
    pub target: Item,
    pub lever: Item,
    pub lightning_rod: Item,
    pub exposed_lightning_rod: Item,
    pub weathered_lightning_rod: Item,
    pub oxidized_lightning_rod: Item,
    pub waxed_lightning_rod: Item,
    pub waxed_exposed_lightning_rod: Item,
    pub waxed_weathered_lightning_rod: Item,
    pub waxed_oxidized_lightning_rod: Item,
    pub daylight_detector: Item,
    pub sculk_sensor: Item,
    pub calibrated_sculk_sensor: Item,
    pub tripwire_hook: Item,
    pub trapped_chest: Item,
    pub tnt: Item,
    pub redstone_lamp: Item,
    pub note_block: Item,
    pub stone_button: Item,
    pub polished_blackstone_button: Item,
    pub oak_button: Item,
    pub spruce_button: Item,
    pub birch_button: Item,
    pub jungle_button: Item,
    pub acacia_button: Item,
    pub cherry_button: Item,
    pub dark_oak_button: Item,
    pub pale_oak_button: Item,
    pub mangrove_button: Item,
    pub bamboo_button: Item,
    pub crimson_button: Item,
    pub warped_button: Item,
    pub stone_pressure_plate: Item,
    pub polished_blackstone_pressure_plate: Item,
    pub light_weighted_pressure_plate: Item,
    pub heavy_weighted_pressure_plate: Item,
    pub oak_pressure_plate: Item,
    pub spruce_pressure_plate: Item,
    pub birch_pressure_plate: Item,
    pub jungle_pressure_plate: Item,
    pub acacia_pressure_plate: Item,
    pub cherry_pressure_plate: Item,
    pub dark_oak_pressure_plate: Item,
    pub pale_oak_pressure_plate: Item,
    pub mangrove_pressure_plate: Item,
    pub bamboo_pressure_plate: Item,
    pub crimson_pressure_plate: Item,
    pub warped_pressure_plate: Item,
    pub iron_door: Item,
    pub oak_door: Item,
    pub spruce_door: Item,
    pub birch_door: Item,
    pub jungle_door: Item,
    pub acacia_door: Item,
    pub cherry_door: Item,
    pub dark_oak_door: Item,
    pub pale_oak_door: Item,
    pub mangrove_door: Item,
    pub bamboo_door: Item,
    pub crimson_door: Item,
    pub warped_door: Item,
    pub copper_door: Item,
    pub exposed_copper_door: Item,
    pub weathered_copper_door: Item,
    pub oxidized_copper_door: Item,
    pub waxed_copper_door: Item,
    pub waxed_exposed_copper_door: Item,
    pub waxed_weathered_copper_door: Item,
    pub waxed_oxidized_copper_door: Item,
    pub iron_trapdoor: Item,
    pub oak_trapdoor: Item,
    pub spruce_trapdoor: Item,
    pub birch_trapdoor: Item,
    pub jungle_trapdoor: Item,
    pub acacia_trapdoor: Item,
    pub cherry_trapdoor: Item,
    pub dark_oak_trapdoor: Item,
    pub pale_oak_trapdoor: Item,
    pub mangrove_trapdoor: Item,
    pub bamboo_trapdoor: Item,
    pub crimson_trapdoor: Item,
    pub warped_trapdoor: Item,
    pub copper_trapdoor: Item,
    pub exposed_copper_trapdoor: Item,
    pub weathered_copper_trapdoor: Item,
    pub oxidized_copper_trapdoor: Item,
    pub waxed_copper_trapdoor: Item,
    pub waxed_exposed_copper_trapdoor: Item,
    pub waxed_weathered_copper_trapdoor: Item,
    pub waxed_oxidized_copper_trapdoor: Item,
    pub oak_fence_gate: Item,
    pub spruce_fence_gate: Item,
    pub birch_fence_gate: Item,
    pub jungle_fence_gate: Item,
    pub acacia_fence_gate: Item,
    pub cherry_fence_gate: Item,
    pub dark_oak_fence_gate: Item,
    pub pale_oak_fence_gate: Item,
    pub mangrove_fence_gate: Item,
    pub bamboo_fence_gate: Item,
    pub crimson_fence_gate: Item,
    pub warped_fence_gate: Item,
    pub powered_rail: Item,
    pub detector_rail: Item,
    pub rail: Item,
    pub activator_rail: Item,
    pub saddle: Item,
    pub white_harness: Item,
    pub orange_harness: Item,
    pub magenta_harness: Item,
    pub light_blue_harness: Item,
    pub yellow_harness: Item,
    pub lime_harness: Item,
    pub pink_harness: Item,
    pub gray_harness: Item,
    pub light_gray_harness: Item,
    pub cyan_harness: Item,
    pub purple_harness: Item,
    pub blue_harness: Item,
    pub brown_harness: Item,
    pub green_harness: Item,
    pub red_harness: Item,
    pub black_harness: Item,
    pub minecart: Item,
    pub chest_minecart: Item,
    pub furnace_minecart: Item,
    pub tnt_minecart: Item,
    pub hopper_minecart: Item,
    pub carrot_on_a_stick: Item,
    pub warped_fungus_on_a_stick: Item,
    pub phantom_membrane: Item,
    pub elytra: Item,
    pub oak_boat: Item,
    pub oak_chest_boat: Item,
    pub spruce_boat: Item,
    pub spruce_chest_boat: Item,
    pub birch_boat: Item,
    pub birch_chest_boat: Item,
    pub jungle_boat: Item,
    pub jungle_chest_boat: Item,
    pub acacia_boat: Item,
    pub acacia_chest_boat: Item,
    pub cherry_boat: Item,
    pub cherry_chest_boat: Item,
    pub dark_oak_boat: Item,
    pub dark_oak_chest_boat: Item,
    pub pale_oak_boat: Item,
    pub pale_oak_chest_boat: Item,
    pub mangrove_boat: Item,
    pub mangrove_chest_boat: Item,
    pub bamboo_raft: Item,
    pub bamboo_chest_raft: Item,
    pub structure_block: Item,
    pub jigsaw: Item,
    pub test_block: Item,
    pub test_instance_block: Item,
    pub turtle_helmet: Item,
    pub turtle_scute: Item,
    pub armadillo_scute: Item,
    pub wolf_armor: Item,
    pub flint_and_steel: Item,
    pub bowl: Item,
    pub apple: Item,
    pub bow: Item,
    pub arrow: Item,
    pub coal: Item,
    pub charcoal: Item,
    pub diamond: Item,
    pub emerald: Item,
    pub lapis_lazuli: Item,
    pub quartz: Item,
    pub amethyst_shard: Item,
    pub raw_iron: Item,
    pub iron_ingot: Item,
    pub raw_copper: Item,
    pub copper_ingot: Item,
    pub raw_gold: Item,
    pub gold_ingot: Item,
    pub netherite_ingot: Item,
    pub netherite_scrap: Item,
    pub wooden_sword: Item,
    pub wooden_shovel: Item,
    pub wooden_pickaxe: Item,
    pub wooden_axe: Item,
    pub wooden_hoe: Item,
    pub copper_sword: Item,
    pub copper_shovel: Item,
    pub copper_pickaxe: Item,
    pub copper_axe: Item,
    pub copper_hoe: Item,
    pub stone_sword: Item,
    pub stone_shovel: Item,
    pub stone_pickaxe: Item,
    pub stone_axe: Item,
    pub stone_hoe: Item,
    pub golden_sword: Item,
    pub golden_shovel: Item,
    pub golden_pickaxe: Item,
    pub golden_axe: Item,
    pub golden_hoe: Item,
    pub iron_sword: Item,
    pub iron_shovel: Item,
    pub iron_pickaxe: Item,
    pub iron_axe: Item,
    pub iron_hoe: Item,
    pub diamond_sword: Item,
    pub diamond_shovel: Item,
    pub diamond_pickaxe: Item,
    pub diamond_axe: Item,
    pub diamond_hoe: Item,
    pub netherite_sword: Item,
    pub netherite_shovel: Item,
    pub netherite_pickaxe: Item,
    pub netherite_axe: Item,
    pub netherite_hoe: Item,
    pub stick: Item,
    pub mushroom_stew: Item,
    pub string: Item,
    pub feather: Item,
    pub gunpowder: Item,
    pub wheat_seeds: Item,
    pub wheat: Item,
    pub bread: Item,
    pub leather_helmet: Item,
    pub leather_chestplate: Item,
    pub leather_leggings: Item,
    pub leather_boots: Item,
    pub copper_helmet: Item,
    pub copper_chestplate: Item,
    pub copper_leggings: Item,
    pub copper_boots: Item,
    pub chainmail_helmet: Item,
    pub chainmail_chestplate: Item,
    pub chainmail_leggings: Item,
    pub chainmail_boots: Item,
    pub iron_helmet: Item,
    pub iron_chestplate: Item,
    pub iron_leggings: Item,
    pub iron_boots: Item,
    pub diamond_helmet: Item,
    pub diamond_chestplate: Item,
    pub diamond_leggings: Item,
    pub diamond_boots: Item,
    pub golden_helmet: Item,
    pub golden_chestplate: Item,
    pub golden_leggings: Item,
    pub golden_boots: Item,
    pub netherite_helmet: Item,
    pub netherite_chestplate: Item,
    pub netherite_leggings: Item,
    pub netherite_boots: Item,
    pub flint: Item,
    pub porkchop: Item,
    pub cooked_porkchop: Item,
    pub painting: Item,
    pub golden_apple: Item,
    pub enchanted_golden_apple: Item,
    pub oak_sign: Item,
    pub spruce_sign: Item,
    pub birch_sign: Item,
    pub jungle_sign: Item,
    pub acacia_sign: Item,
    pub cherry_sign: Item,
    pub dark_oak_sign: Item,
    pub pale_oak_sign: Item,
    pub mangrove_sign: Item,
    pub bamboo_sign: Item,
    pub crimson_sign: Item,
    pub warped_sign: Item,
    pub oak_hanging_sign: Item,
    pub spruce_hanging_sign: Item,
    pub birch_hanging_sign: Item,
    pub jungle_hanging_sign: Item,
    pub acacia_hanging_sign: Item,
    pub cherry_hanging_sign: Item,
    pub dark_oak_hanging_sign: Item,
    pub pale_oak_hanging_sign: Item,
    pub mangrove_hanging_sign: Item,
    pub bamboo_hanging_sign: Item,
    pub crimson_hanging_sign: Item,
    pub warped_hanging_sign: Item,
    pub bucket: Item,
    pub water_bucket: Item,
    pub lava_bucket: Item,
    pub powder_snow_bucket: Item,
    pub snowball: Item,
    pub leather: Item,
    pub milk_bucket: Item,
    pub pufferfish_bucket: Item,
    pub salmon_bucket: Item,
    pub cod_bucket: Item,
    pub tropical_fish_bucket: Item,
    pub axolotl_bucket: Item,
    pub tadpole_bucket: Item,
    pub brick: Item,
    pub clay_ball: Item,
    pub dried_kelp_block: Item,
    pub paper: Item,
    pub book: Item,
    pub slime_ball: Item,
    pub egg: Item,
    pub blue_egg: Item,
    pub brown_egg: Item,
    pub compass: Item,
    pub recovery_compass: Item,
    pub bundle: Item,
    pub white_bundle: Item,
    pub orange_bundle: Item,
    pub magenta_bundle: Item,
    pub light_blue_bundle: Item,
    pub yellow_bundle: Item,
    pub lime_bundle: Item,
    pub pink_bundle: Item,
    pub gray_bundle: Item,
    pub light_gray_bundle: Item,
    pub cyan_bundle: Item,
    pub purple_bundle: Item,
    pub blue_bundle: Item,
    pub brown_bundle: Item,
    pub green_bundle: Item,
    pub red_bundle: Item,
    pub black_bundle: Item,
    pub fishing_rod: Item,
    pub clock: Item,
    pub spyglass: Item,
    pub glowstone_dust: Item,
    pub cod: Item,
    pub salmon: Item,
    pub tropical_fish: Item,
    pub pufferfish: Item,
    pub cooked_cod: Item,
    pub cooked_salmon: Item,
    pub ink_sac: Item,
    pub glow_ink_sac: Item,
    pub cocoa_beans: Item,
    pub white_dye: Item,
    pub orange_dye: Item,
    pub magenta_dye: Item,
    pub light_blue_dye: Item,
    pub yellow_dye: Item,
    pub lime_dye: Item,
    pub pink_dye: Item,
    pub gray_dye: Item,
    pub light_gray_dye: Item,
    pub cyan_dye: Item,
    pub purple_dye: Item,
    pub blue_dye: Item,
    pub brown_dye: Item,
    pub green_dye: Item,
    pub red_dye: Item,
    pub black_dye: Item,
    pub bone_meal: Item,
    pub bone: Item,
    pub sugar: Item,
    pub cake: Item,
    pub white_bed: Item,
    pub orange_bed: Item,
    pub magenta_bed: Item,
    pub light_blue_bed: Item,
    pub yellow_bed: Item,
    pub lime_bed: Item,
    pub pink_bed: Item,
    pub gray_bed: Item,
    pub light_gray_bed: Item,
    pub cyan_bed: Item,
    pub purple_bed: Item,
    pub blue_bed: Item,
    pub brown_bed: Item,
    pub green_bed: Item,
    pub red_bed: Item,
    pub black_bed: Item,
    pub cookie: Item,
    pub crafter: Item,
    pub filled_map: Item,
    pub shears: Item,
    pub melon_slice: Item,
    pub dried_kelp: Item,
    pub pumpkin_seeds: Item,
    pub melon_seeds: Item,
    pub beef: Item,
    pub cooked_beef: Item,
    pub chicken: Item,
    pub cooked_chicken: Item,
    pub rotten_flesh: Item,
    pub ender_pearl: Item,
    pub blaze_rod: Item,
    pub ghast_tear: Item,
    pub gold_nugget: Item,
    pub nether_wart: Item,
    pub glass_bottle: Item,
    pub potion: Item,
    pub spider_eye: Item,
    pub fermented_spider_eye: Item,
    pub blaze_powder: Item,
    pub magma_cream: Item,
    pub brewing_stand: Item,
    pub cauldron: Item,
    pub ender_eye: Item,
    pub glistering_melon_slice: Item,
    pub armadillo_spawn_egg: Item,
    pub allay_spawn_egg: Item,
    pub axolotl_spawn_egg: Item,
    pub bat_spawn_egg: Item,
    pub bee_spawn_egg: Item,
    pub blaze_spawn_egg: Item,
    pub bogged_spawn_egg: Item,
    pub breeze_spawn_egg: Item,
    pub cat_spawn_egg: Item,
    pub camel_spawn_egg: Item,
    pub cave_spider_spawn_egg: Item,
    pub chicken_spawn_egg: Item,
    pub cod_spawn_egg: Item,
    pub copper_golem_spawn_egg: Item,
    pub cow_spawn_egg: Item,
    pub creeper_spawn_egg: Item,
    pub dolphin_spawn_egg: Item,
    pub donkey_spawn_egg: Item,
    pub drowned_spawn_egg: Item,
    pub elder_guardian_spawn_egg: Item,
    pub ender_dragon_spawn_egg: Item,
    pub enderman_spawn_egg: Item,
    pub endermite_spawn_egg: Item,
    pub evoker_spawn_egg: Item,
    pub fox_spawn_egg: Item,
    pub frog_spawn_egg: Item,
    pub ghast_spawn_egg: Item,
    pub happy_ghast_spawn_egg: Item,
    pub glow_squid_spawn_egg: Item,
    pub goat_spawn_egg: Item,
    pub guardian_spawn_egg: Item,
    pub hoglin_spawn_egg: Item,
    pub horse_spawn_egg: Item,
    pub husk_spawn_egg: Item,
    pub iron_golem_spawn_egg: Item,
    pub llama_spawn_egg: Item,
    pub magma_cube_spawn_egg: Item,
    pub mooshroom_spawn_egg: Item,
    pub mule_spawn_egg: Item,
    pub ocelot_spawn_egg: Item,
    pub panda_spawn_egg: Item,
    pub parrot_spawn_egg: Item,
    pub phantom_spawn_egg: Item,
    pub pig_spawn_egg: Item,
    pub piglin_spawn_egg: Item,
    pub piglin_brute_spawn_egg: Item,
    pub pillager_spawn_egg: Item,
    pub polar_bear_spawn_egg: Item,
    pub pufferfish_spawn_egg: Item,
    pub rabbit_spawn_egg: Item,
    pub ravager_spawn_egg: Item,
    pub salmon_spawn_egg: Item,
    pub sheep_spawn_egg: Item,
    pub shulker_spawn_egg: Item,
    pub silverfish_spawn_egg: Item,
    pub skeleton_spawn_egg: Item,
    pub skeleton_horse_spawn_egg: Item,
    pub slime_spawn_egg: Item,
    pub sniffer_spawn_egg: Item,
    pub snow_golem_spawn_egg: Item,
    pub spider_spawn_egg: Item,
    pub squid_spawn_egg: Item,
    pub stray_spawn_egg: Item,
    pub strider_spawn_egg: Item,
    pub tadpole_spawn_egg: Item,
    pub trader_llama_spawn_egg: Item,
    pub tropical_fish_spawn_egg: Item,
    pub turtle_spawn_egg: Item,
    pub vex_spawn_egg: Item,
    pub villager_spawn_egg: Item,
    pub vindicator_spawn_egg: Item,
    pub wandering_trader_spawn_egg: Item,
    pub warden_spawn_egg: Item,
    pub witch_spawn_egg: Item,
    pub wither_spawn_egg: Item,
    pub wither_skeleton_spawn_egg: Item,
    pub wolf_spawn_egg: Item,
    pub zoglin_spawn_egg: Item,
    pub creaking_spawn_egg: Item,
    pub zombie_spawn_egg: Item,
    pub zombie_horse_spawn_egg: Item,
    pub zombie_villager_spawn_egg: Item,
    pub zombified_piglin_spawn_egg: Item,
    pub experience_bottle: Item,
    pub fire_charge: Item,
    pub wind_charge: Item,
    pub writable_book: Item,
    pub written_book: Item,
    pub breeze_rod: Item,
    pub mace: Item,
    pub item_frame: Item,
    pub glow_item_frame: Item,
    pub flower_pot: Item,
    pub carrot: Item,
    pub potato: Item,
    pub baked_potato: Item,
    pub poisonous_potato: Item,
    pub map: Item,
    pub golden_carrot: Item,
    pub skeleton_skull: Item,
    pub wither_skeleton_skull: Item,
    pub player_head: Item,
    pub zombie_head: Item,
    pub creeper_head: Item,
    pub dragon_head: Item,
    pub piglin_head: Item,
    pub nether_star: Item,
    pub pumpkin_pie: Item,
    pub firework_rocket: Item,
    pub firework_star: Item,
    pub enchanted_book: Item,
    pub nether_brick: Item,
    pub resin_brick: Item,
    pub prismarine_shard: Item,
    pub prismarine_crystals: Item,
    pub rabbit: Item,
    pub cooked_rabbit: Item,
    pub rabbit_stew: Item,
    pub rabbit_foot: Item,
    pub rabbit_hide: Item,
    pub armor_stand: Item,
    pub copper_horse_armor: Item,
    pub iron_horse_armor: Item,
    pub golden_horse_armor: Item,
    pub diamond_horse_armor: Item,
    pub leather_horse_armor: Item,
    pub lead: Item,
    pub name_tag: Item,
    pub command_block_minecart: Item,
    pub mutton: Item,
    pub cooked_mutton: Item,
    pub white_banner: Item,
    pub orange_banner: Item,
    pub magenta_banner: Item,
    pub light_blue_banner: Item,
    pub yellow_banner: Item,
    pub lime_banner: Item,
    pub pink_banner: Item,
    pub gray_banner: Item,
    pub light_gray_banner: Item,
    pub cyan_banner: Item,
    pub purple_banner: Item,
    pub blue_banner: Item,
    pub brown_banner: Item,
    pub green_banner: Item,
    pub red_banner: Item,
    pub black_banner: Item,
    pub end_crystal: Item,
    pub chorus_fruit: Item,
    pub popped_chorus_fruit: Item,
    pub torchflower_seeds: Item,
    pub pitcher_pod: Item,
    pub beetroot: Item,
    pub beetroot_seeds: Item,
    pub beetroot_soup: Item,
    pub dragon_breath: Item,
    pub splash_potion: Item,
    pub spectral_arrow: Item,
    pub tipped_arrow: Item,
    pub lingering_potion: Item,
    pub shield: Item,
    pub totem_of_undying: Item,
    pub shulker_shell: Item,
    pub iron_nugget: Item,
    pub copper_nugget: Item,
    pub knowledge_book: Item,
    pub debug_stick: Item,
    pub music_disc_13: Item,
    pub music_disc_cat: Item,
    pub music_disc_blocks: Item,
    pub music_disc_chirp: Item,
    pub music_disc_creator: Item,
    pub music_disc_creator_music_box: Item,
    pub music_disc_far: Item,
    pub music_disc_lava_chicken: Item,
    pub music_disc_mall: Item,
    pub music_disc_mellohi: Item,
    pub music_disc_stal: Item,
    pub music_disc_strad: Item,
    pub music_disc_ward: Item,
    pub music_disc_11: Item,
    pub music_disc_wait: Item,
    pub music_disc_otherside: Item,
    pub music_disc_relic: Item,
    pub music_disc_5: Item,
    pub music_disc_pigstep: Item,
    pub music_disc_precipice: Item,
    pub music_disc_tears: Item,
    pub disc_fragment_5: Item,
    pub trident: Item,
    pub nautilus_shell: Item,
    pub heart_of_the_sea: Item,
    pub crossbow: Item,
    pub suspicious_stew: Item,
    pub loom: Item,
    pub flower_banner_pattern: Item,
    pub creeper_banner_pattern: Item,
    pub skull_banner_pattern: Item,
    pub mojang_banner_pattern: Item,
    pub globe_banner_pattern: Item,
    pub piglin_banner_pattern: Item,
    pub flow_banner_pattern: Item,
    pub guster_banner_pattern: Item,
    pub field_masoned_banner_pattern: Item,
    pub bordure_indented_banner_pattern: Item,
    pub goat_horn: Item,
    pub composter: Item,
    pub barrel: Item,
    pub smoker: Item,
    pub blast_furnace: Item,
    pub cartography_table: Item,
    pub fletching_table: Item,
    pub grindstone: Item,
    pub smithing_table: Item,
    pub stonecutter: Item,
    pub bell: Item,
    pub lantern: Item,
    pub soul_lantern: Item,
    pub copper_lantern: Item,
    pub exposed_copper_lantern: Item,
    pub weathered_copper_lantern: Item,
    pub oxidized_copper_lantern: Item,
    pub waxed_copper_lantern: Item,
    pub waxed_exposed_copper_lantern: Item,
    pub waxed_weathered_copper_lantern: Item,
    pub waxed_oxidized_copper_lantern: Item,
    pub sweet_berries: Item,
    pub glow_berries: Item,
    pub campfire: Item,
    pub soul_campfire: Item,
    pub shroomlight: Item,
    pub honeycomb: Item,
    pub bee_nest: Item,
    pub beehive: Item,
    pub honey_bottle: Item,
    pub honeycomb_block: Item,
    pub lodestone: Item,
    pub crying_obsidian: Item,
    pub blackstone: Item,
    pub blackstone_slab: Item,
    pub blackstone_stairs: Item,
    pub gilded_blackstone: Item,
    pub polished_blackstone: Item,
    pub polished_blackstone_slab: Item,
    pub polished_blackstone_stairs: Item,
    pub chiseled_polished_blackstone: Item,
    pub polished_blackstone_bricks: Item,
    pub polished_blackstone_brick_slab: Item,
    pub polished_blackstone_brick_stairs: Item,
    pub cracked_polished_blackstone_bricks: Item,
    pub respawn_anchor: Item,
    pub candle: Item,
    pub white_candle: Item,
    pub orange_candle: Item,
    pub magenta_candle: Item,
    pub light_blue_candle: Item,
    pub yellow_candle: Item,
    pub lime_candle: Item,
    pub pink_candle: Item,
    pub gray_candle: Item,
    pub light_gray_candle: Item,
    pub cyan_candle: Item,
    pub purple_candle: Item,
    pub blue_candle: Item,
    pub brown_candle: Item,
    pub green_candle: Item,
    pub red_candle: Item,
    pub black_candle: Item,
    pub small_amethyst_bud: Item,
    pub medium_amethyst_bud: Item,
    pub large_amethyst_bud: Item,
    pub amethyst_cluster: Item,
    pub pointed_dripstone: Item,
    pub ochre_froglight: Item,
    pub verdant_froglight: Item,
    pub pearlescent_froglight: Item,
    pub frogspawn: Item,
    pub echo_shard: Item,
    pub brush: Item,
    pub netherite_upgrade_smithing_template: Item,
    pub sentry_armor_trim_smithing_template: Item,
    pub dune_armor_trim_smithing_template: Item,
    pub coast_armor_trim_smithing_template: Item,
    pub wild_armor_trim_smithing_template: Item,
    pub ward_armor_trim_smithing_template: Item,
    pub eye_armor_trim_smithing_template: Item,
    pub vex_armor_trim_smithing_template: Item,
    pub tide_armor_trim_smithing_template: Item,
    pub snout_armor_trim_smithing_template: Item,
    pub rib_armor_trim_smithing_template: Item,
    pub spire_armor_trim_smithing_template: Item,
    pub wayfinder_armor_trim_smithing_template: Item,
    pub shaper_armor_trim_smithing_template: Item,
    pub silence_armor_trim_smithing_template: Item,
    pub raiser_armor_trim_smithing_template: Item,
    pub host_armor_trim_smithing_template: Item,
    pub flow_armor_trim_smithing_template: Item,
    pub bolt_armor_trim_smithing_template: Item,
    pub angler_pottery_sherd: Item,
    pub archer_pottery_sherd: Item,
    pub arms_up_pottery_sherd: Item,
    pub blade_pottery_sherd: Item,
    pub brewer_pottery_sherd: Item,
    pub burn_pottery_sherd: Item,
    pub danger_pottery_sherd: Item,
    pub explorer_pottery_sherd: Item,
    pub flow_pottery_sherd: Item,
    pub friend_pottery_sherd: Item,
    pub guster_pottery_sherd: Item,
    pub heart_pottery_sherd: Item,
    pub heartbreak_pottery_sherd: Item,
    pub howl_pottery_sherd: Item,
    pub miner_pottery_sherd: Item,
    pub mourner_pottery_sherd: Item,
    pub plenty_pottery_sherd: Item,
    pub prize_pottery_sherd: Item,
    pub scrape_pottery_sherd: Item,
    pub sheaf_pottery_sherd: Item,
    pub shelter_pottery_sherd: Item,
    pub skull_pottery_sherd: Item,
    pub snort_pottery_sherd: Item,
    pub copper_grate: Item,
    pub exposed_copper_grate: Item,
    pub weathered_copper_grate: Item,
    pub oxidized_copper_grate: Item,
    pub waxed_copper_grate: Item,
    pub waxed_exposed_copper_grate: Item,
    pub waxed_weathered_copper_grate: Item,
    pub waxed_oxidized_copper_grate: Item,
    pub copper_bulb: Item,
    pub exposed_copper_bulb: Item,
    pub weathered_copper_bulb: Item,
    pub oxidized_copper_bulb: Item,
    pub waxed_copper_bulb: Item,
    pub waxed_exposed_copper_bulb: Item,
    pub waxed_weathered_copper_bulb: Item,
    pub waxed_oxidized_copper_bulb: Item,
    pub copper_chest: Item,
    pub exposed_copper_chest: Item,
    pub weathered_copper_chest: Item,
    pub oxidized_copper_chest: Item,
    pub waxed_copper_chest: Item,
    pub waxed_exposed_copper_chest: Item,
    pub waxed_weathered_copper_chest: Item,
    pub waxed_oxidized_copper_chest: Item,
    pub copper_golem_statue: Item,
    pub exposed_copper_golem_statue: Item,
    pub weathered_copper_golem_statue: Item,
    pub oxidized_copper_golem_statue: Item,
    pub waxed_copper_golem_statue: Item,
    pub waxed_exposed_copper_golem_statue: Item,
    pub waxed_weathered_copper_golem_statue: Item,
    pub waxed_oxidized_copper_golem_statue: Item,
    pub trial_spawner: Item,
    pub trial_key: Item,
    pub ominous_trial_key: Item,
    pub vault: Item,
    pub ominous_bottle: Item,
}
impl Items {
    fn init() -> Self {
        Self {
            air: Item {
                key: Identifier::vanilla_static("air"),
                components: DataComponentMap::common_item_components(),
            },
            stone: Item::from_block(&*vanilla_blocks::STONE),
            granite: Item::from_block(&*vanilla_blocks::GRANITE),
            polished_granite: Item::from_block(&*vanilla_blocks::POLISHED_GRANITE),
            diorite: Item::from_block(&*vanilla_blocks::DIORITE),
            polished_diorite: Item::from_block(&*vanilla_blocks::POLISHED_DIORITE),
            andesite: Item::from_block(&*vanilla_blocks::ANDESITE),
            polished_andesite: Item::from_block(&*vanilla_blocks::POLISHED_ANDESITE),
            deepslate: Item::from_block(&*vanilla_blocks::DEEPSLATE),
            cobbled_deepslate: Item::from_block(&*vanilla_blocks::COBBLED_DEEPSLATE),
            polished_deepslate: Item::from_block(&*vanilla_blocks::POLISHED_DEEPSLATE),
            calcite: Item::from_block(&*vanilla_blocks::CALCITE),
            tuff: Item::from_block(&*vanilla_blocks::TUFF),
            tuff_slab: Item::from_block(&*vanilla_blocks::TUFF_SLAB),
            tuff_stairs: Item::from_block(&*vanilla_blocks::TUFF_STAIRS),
            tuff_wall: Item::from_block(&*vanilla_blocks::TUFF_WALL),
            chiseled_tuff: Item::from_block(&*vanilla_blocks::CHISELED_TUFF),
            polished_tuff: Item::from_block(&*vanilla_blocks::POLISHED_TUFF),
            polished_tuff_slab: Item::from_block(&*vanilla_blocks::POLISHED_TUFF_SLAB),
            polished_tuff_stairs: Item::from_block(&*vanilla_blocks::POLISHED_TUFF_STAIRS),
            polished_tuff_wall: Item::from_block(&*vanilla_blocks::POLISHED_TUFF_WALL),
            tuff_bricks: Item::from_block(&*vanilla_blocks::TUFF_BRICKS),
            tuff_brick_slab: Item::from_block(&*vanilla_blocks::TUFF_BRICK_SLAB),
            tuff_brick_stairs: Item::from_block(&*vanilla_blocks::TUFF_BRICK_STAIRS),
            tuff_brick_wall: Item::from_block(&*vanilla_blocks::TUFF_BRICK_WALL),
            chiseled_tuff_bricks: Item::from_block(&*vanilla_blocks::CHISELED_TUFF_BRICKS),
            dripstone_block: Item::from_block(&*vanilla_blocks::DRIPSTONE_BLOCK),
            grass_block: Item::from_block(&*vanilla_blocks::GRASS_BLOCK),
            dirt: Item::from_block(&*vanilla_blocks::DIRT),
            coarse_dirt: Item::from_block(&*vanilla_blocks::COARSE_DIRT),
            podzol: Item::from_block(&*vanilla_blocks::PODZOL),
            rooted_dirt: Item::from_block(&*vanilla_blocks::ROOTED_DIRT),
            mud: Item::from_block(&*vanilla_blocks::MUD),
            crimson_nylium: Item::from_block(&*vanilla_blocks::CRIMSON_NYLIUM),
            warped_nylium: Item::from_block(&*vanilla_blocks::WARPED_NYLIUM),
            cobblestone: Item::from_block(&*vanilla_blocks::COBBLESTONE),
            oak_planks: Item::from_block(&*vanilla_blocks::OAK_PLANKS),
            spruce_planks: Item::from_block(&*vanilla_blocks::SPRUCE_PLANKS),
            birch_planks: Item::from_block(&*vanilla_blocks::BIRCH_PLANKS),
            jungle_planks: Item::from_block(&*vanilla_blocks::JUNGLE_PLANKS),
            acacia_planks: Item::from_block(&*vanilla_blocks::ACACIA_PLANKS),
            cherry_planks: Item::from_block(&*vanilla_blocks::CHERRY_PLANKS),
            dark_oak_planks: Item::from_block(&*vanilla_blocks::DARK_OAK_PLANKS),
            pale_oak_planks: Item::from_block(&*vanilla_blocks::PALE_OAK_PLANKS),
            mangrove_planks: Item::from_block(&*vanilla_blocks::MANGROVE_PLANKS),
            bamboo_planks: Item::from_block(&*vanilla_blocks::BAMBOO_PLANKS),
            crimson_planks: Item::from_block(&*vanilla_blocks::CRIMSON_PLANKS),
            warped_planks: Item::from_block(&*vanilla_blocks::WARPED_PLANKS),
            bamboo_mosaic: Item::from_block(&*vanilla_blocks::BAMBOO_MOSAIC),
            oak_sapling: Item::from_block(&*vanilla_blocks::OAK_SAPLING),
            spruce_sapling: Item::from_block(&*vanilla_blocks::SPRUCE_SAPLING),
            birch_sapling: Item::from_block(&*vanilla_blocks::BIRCH_SAPLING),
            jungle_sapling: Item::from_block(&*vanilla_blocks::JUNGLE_SAPLING),
            acacia_sapling: Item::from_block(&*vanilla_blocks::ACACIA_SAPLING),
            cherry_sapling: Item::from_block(&*vanilla_blocks::CHERRY_SAPLING),
            dark_oak_sapling: Item::from_block(&*vanilla_blocks::DARK_OAK_SAPLING),
            pale_oak_sapling: Item::from_block(&*vanilla_blocks::PALE_OAK_SAPLING),
            mangrove_propagule: Item::from_block(&*vanilla_blocks::MANGROVE_PROPAGULE),
            bedrock: Item::from_block(&*vanilla_blocks::BEDROCK),
            sand: Item::from_block(&*vanilla_blocks::SAND),
            suspicious_sand: Item::from_block(&*vanilla_blocks::SUSPICIOUS_SAND),
            suspicious_gravel: Item::from_block(&*vanilla_blocks::SUSPICIOUS_GRAVEL),
            red_sand: Item::from_block(&*vanilla_blocks::RED_SAND),
            gravel: Item::from_block(&*vanilla_blocks::GRAVEL),
            coal_ore: Item::from_block(&*vanilla_blocks::COAL_ORE),
            deepslate_coal_ore: Item::from_block(&*vanilla_blocks::DEEPSLATE_COAL_ORE),
            iron_ore: Item::from_block(&*vanilla_blocks::IRON_ORE),
            deepslate_iron_ore: Item::from_block(&*vanilla_blocks::DEEPSLATE_IRON_ORE),
            copper_ore: Item::from_block(&*vanilla_blocks::COPPER_ORE),
            deepslate_copper_ore: Item::from_block(&*vanilla_blocks::DEEPSLATE_COPPER_ORE),
            gold_ore: Item::from_block(&*vanilla_blocks::GOLD_ORE),
            deepslate_gold_ore: Item::from_block(&*vanilla_blocks::DEEPSLATE_GOLD_ORE),
            redstone_ore: Item::from_block(&*vanilla_blocks::REDSTONE_ORE),
            deepslate_redstone_ore: Item::from_block(&*vanilla_blocks::DEEPSLATE_REDSTONE_ORE),
            emerald_ore: Item::from_block(&*vanilla_blocks::EMERALD_ORE),
            deepslate_emerald_ore: Item::from_block(&*vanilla_blocks::DEEPSLATE_EMERALD_ORE),
            lapis_ore: Item::from_block(&*vanilla_blocks::LAPIS_ORE),
            deepslate_lapis_ore: Item::from_block(&*vanilla_blocks::DEEPSLATE_LAPIS_ORE),
            diamond_ore: Item::from_block(&*vanilla_blocks::DIAMOND_ORE),
            deepslate_diamond_ore: Item::from_block(&*vanilla_blocks::DEEPSLATE_DIAMOND_ORE),
            nether_gold_ore: Item::from_block(&*vanilla_blocks::NETHER_GOLD_ORE),
            nether_quartz_ore: Item::from_block(&*vanilla_blocks::NETHER_QUARTZ_ORE),
            ancient_debris: Item::from_block(&*vanilla_blocks::ANCIENT_DEBRIS),
            coal_block: Item::from_block(&*vanilla_blocks::COAL_BLOCK),
            raw_iron_block: Item::from_block(&*vanilla_blocks::RAW_IRON_BLOCK),
            raw_copper_block: Item::from_block(&*vanilla_blocks::RAW_COPPER_BLOCK),
            raw_gold_block: Item::from_block(&*vanilla_blocks::RAW_GOLD_BLOCK),
            heavy_core: Item::from_block(&*vanilla_blocks::HEAVY_CORE),
            amethyst_block: Item::from_block(&*vanilla_blocks::AMETHYST_BLOCK),
            budding_amethyst: Item::from_block(&*vanilla_blocks::BUDDING_AMETHYST),
            iron_block: Item::from_block(&*vanilla_blocks::IRON_BLOCK),
            copper_block: Item::from_block(&*vanilla_blocks::COPPER_BLOCK),
            gold_block: Item::from_block(&*vanilla_blocks::GOLD_BLOCK),
            diamond_block: Item::from_block(&*vanilla_blocks::DIAMOND_BLOCK),
            netherite_block: Item::from_block(&*vanilla_blocks::NETHERITE_BLOCK),
            exposed_copper: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER),
            weathered_copper: Item::from_block(&*vanilla_blocks::WEATHERED_COPPER),
            oxidized_copper: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER),
            chiseled_copper: Item::from_block(&*vanilla_blocks::CHISELED_COPPER),
            exposed_chiseled_copper: Item::from_block(&*vanilla_blocks::EXPOSED_CHISELED_COPPER),
            weathered_chiseled_copper: Item::from_block(
                &*vanilla_blocks::WEATHERED_CHISELED_COPPER,
            ),
            oxidized_chiseled_copper: Item::from_block(&*vanilla_blocks::OXIDIZED_CHISELED_COPPER),
            cut_copper: Item::from_block(&*vanilla_blocks::CUT_COPPER),
            exposed_cut_copper: Item::from_block(&*vanilla_blocks::EXPOSED_CUT_COPPER),
            weathered_cut_copper: Item::from_block(&*vanilla_blocks::WEATHERED_CUT_COPPER),
            oxidized_cut_copper: Item::from_block(&*vanilla_blocks::OXIDIZED_CUT_COPPER),
            cut_copper_stairs: Item::from_block(&*vanilla_blocks::CUT_COPPER_STAIRS),
            exposed_cut_copper_stairs: Item::from_block(
                &*vanilla_blocks::EXPOSED_CUT_COPPER_STAIRS,
            ),
            weathered_cut_copper_stairs: Item::from_block(
                &*vanilla_blocks::WEATHERED_CUT_COPPER_STAIRS,
            ),
            oxidized_cut_copper_stairs: Item::from_block(
                &*vanilla_blocks::OXIDIZED_CUT_COPPER_STAIRS,
            ),
            cut_copper_slab: Item::from_block(&*vanilla_blocks::CUT_COPPER_SLAB),
            exposed_cut_copper_slab: Item::from_block(&*vanilla_blocks::EXPOSED_CUT_COPPER_SLAB),
            weathered_cut_copper_slab: Item::from_block(
                &*vanilla_blocks::WEATHERED_CUT_COPPER_SLAB,
            ),
            oxidized_cut_copper_slab: Item::from_block(&*vanilla_blocks::OXIDIZED_CUT_COPPER_SLAB),
            waxed_copper_block: Item::from_block(&*vanilla_blocks::WAXED_COPPER_BLOCK),
            waxed_exposed_copper: Item::from_block(&*vanilla_blocks::WAXED_EXPOSED_COPPER),
            waxed_weathered_copper: Item::from_block(&*vanilla_blocks::WAXED_WEATHERED_COPPER),
            waxed_oxidized_copper: Item::from_block(&*vanilla_blocks::WAXED_OXIDIZED_COPPER),
            waxed_chiseled_copper: Item::from_block(&*vanilla_blocks::WAXED_CHISELED_COPPER),
            waxed_exposed_chiseled_copper: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_CHISELED_COPPER,
            ),
            waxed_weathered_chiseled_copper: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_CHISELED_COPPER,
            ),
            waxed_oxidized_chiseled_copper: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_CHISELED_COPPER,
            ),
            waxed_cut_copper: Item::from_block(&*vanilla_blocks::WAXED_CUT_COPPER),
            waxed_exposed_cut_copper: Item::from_block(&*vanilla_blocks::WAXED_EXPOSED_CUT_COPPER),
            waxed_weathered_cut_copper: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_CUT_COPPER,
            ),
            waxed_oxidized_cut_copper: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_CUT_COPPER,
            ),
            waxed_cut_copper_stairs: Item::from_block(&*vanilla_blocks::WAXED_CUT_COPPER_STAIRS),
            waxed_exposed_cut_copper_stairs: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_CUT_COPPER_STAIRS,
            ),
            waxed_weathered_cut_copper_stairs: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_CUT_COPPER_STAIRS,
            ),
            waxed_oxidized_cut_copper_stairs: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_CUT_COPPER_STAIRS,
            ),
            waxed_cut_copper_slab: Item::from_block(&*vanilla_blocks::WAXED_CUT_COPPER_SLAB),
            waxed_exposed_cut_copper_slab: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_CUT_COPPER_SLAB,
            ),
            waxed_weathered_cut_copper_slab: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_CUT_COPPER_SLAB,
            ),
            waxed_oxidized_cut_copper_slab: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_CUT_COPPER_SLAB,
            ),
            oak_log: Item::from_block(&*vanilla_blocks::OAK_LOG),
            spruce_log: Item::from_block(&*vanilla_blocks::SPRUCE_LOG),
            birch_log: Item::from_block(&*vanilla_blocks::BIRCH_LOG),
            jungle_log: Item::from_block(&*vanilla_blocks::JUNGLE_LOG),
            acacia_log: Item::from_block(&*vanilla_blocks::ACACIA_LOG),
            cherry_log: Item::from_block(&*vanilla_blocks::CHERRY_LOG),
            pale_oak_log: Item::from_block(&*vanilla_blocks::PALE_OAK_LOG),
            dark_oak_log: Item::from_block(&*vanilla_blocks::DARK_OAK_LOG),
            mangrove_log: Item::from_block(&*vanilla_blocks::MANGROVE_LOG),
            mangrove_roots: Item::from_block(&*vanilla_blocks::MANGROVE_ROOTS),
            muddy_mangrove_roots: Item::from_block(&*vanilla_blocks::MUDDY_MANGROVE_ROOTS),
            crimson_stem: Item::from_block(&*vanilla_blocks::CRIMSON_STEM),
            warped_stem: Item::from_block(&*vanilla_blocks::WARPED_STEM),
            bamboo_block: Item::from_block(&*vanilla_blocks::BAMBOO_BLOCK),
            stripped_oak_log: Item::from_block(&*vanilla_blocks::STRIPPED_OAK_LOG),
            stripped_spruce_log: Item::from_block(&*vanilla_blocks::STRIPPED_SPRUCE_LOG),
            stripped_birch_log: Item::from_block(&*vanilla_blocks::STRIPPED_BIRCH_LOG),
            stripped_jungle_log: Item::from_block(&*vanilla_blocks::STRIPPED_JUNGLE_LOG),
            stripped_acacia_log: Item::from_block(&*vanilla_blocks::STRIPPED_ACACIA_LOG),
            stripped_cherry_log: Item::from_block(&*vanilla_blocks::STRIPPED_CHERRY_LOG),
            stripped_dark_oak_log: Item::from_block(&*vanilla_blocks::STRIPPED_DARK_OAK_LOG),
            stripped_pale_oak_log: Item::from_block(&*vanilla_blocks::STRIPPED_PALE_OAK_LOG),
            stripped_mangrove_log: Item::from_block(&*vanilla_blocks::STRIPPED_MANGROVE_LOG),
            stripped_crimson_stem: Item::from_block(&*vanilla_blocks::STRIPPED_CRIMSON_STEM),
            stripped_warped_stem: Item::from_block(&*vanilla_blocks::STRIPPED_WARPED_STEM),
            stripped_oak_wood: Item::from_block(&*vanilla_blocks::STRIPPED_OAK_WOOD),
            stripped_spruce_wood: Item::from_block(&*vanilla_blocks::STRIPPED_SPRUCE_WOOD),
            stripped_birch_wood: Item::from_block(&*vanilla_blocks::STRIPPED_BIRCH_WOOD),
            stripped_jungle_wood: Item::from_block(&*vanilla_blocks::STRIPPED_JUNGLE_WOOD),
            stripped_acacia_wood: Item::from_block(&*vanilla_blocks::STRIPPED_ACACIA_WOOD),
            stripped_cherry_wood: Item::from_block(&*vanilla_blocks::STRIPPED_CHERRY_WOOD),
            stripped_dark_oak_wood: Item::from_block(&*vanilla_blocks::STRIPPED_DARK_OAK_WOOD),
            stripped_pale_oak_wood: Item::from_block(&*vanilla_blocks::STRIPPED_PALE_OAK_WOOD),
            stripped_mangrove_wood: Item::from_block(&*vanilla_blocks::STRIPPED_MANGROVE_WOOD),
            stripped_crimson_hyphae: Item::from_block(&*vanilla_blocks::STRIPPED_CRIMSON_HYPHAE),
            stripped_warped_hyphae: Item::from_block(&*vanilla_blocks::STRIPPED_WARPED_HYPHAE),
            stripped_bamboo_block: Item::from_block(&*vanilla_blocks::STRIPPED_BAMBOO_BLOCK),
            oak_wood: Item::from_block(&*vanilla_blocks::OAK_WOOD),
            spruce_wood: Item::from_block(&*vanilla_blocks::SPRUCE_WOOD),
            birch_wood: Item::from_block(&*vanilla_blocks::BIRCH_WOOD),
            jungle_wood: Item::from_block(&*vanilla_blocks::JUNGLE_WOOD),
            acacia_wood: Item::from_block(&*vanilla_blocks::ACACIA_WOOD),
            cherry_wood: Item::from_block(&*vanilla_blocks::CHERRY_WOOD),
            pale_oak_wood: Item::from_block(&*vanilla_blocks::PALE_OAK_WOOD),
            dark_oak_wood: Item::from_block(&*vanilla_blocks::DARK_OAK_WOOD),
            mangrove_wood: Item::from_block(&*vanilla_blocks::MANGROVE_WOOD),
            crimson_hyphae: Item::from_block(&*vanilla_blocks::CRIMSON_HYPHAE),
            warped_hyphae: Item::from_block(&*vanilla_blocks::WARPED_HYPHAE),
            oak_leaves: Item::from_block(&*vanilla_blocks::OAK_LEAVES),
            spruce_leaves: Item::from_block(&*vanilla_blocks::SPRUCE_LEAVES),
            birch_leaves: Item::from_block(&*vanilla_blocks::BIRCH_LEAVES),
            jungle_leaves: Item::from_block(&*vanilla_blocks::JUNGLE_LEAVES),
            acacia_leaves: Item::from_block(&*vanilla_blocks::ACACIA_LEAVES),
            cherry_leaves: Item::from_block(&*vanilla_blocks::CHERRY_LEAVES),
            dark_oak_leaves: Item::from_block(&*vanilla_blocks::DARK_OAK_LEAVES),
            pale_oak_leaves: Item::from_block(&*vanilla_blocks::PALE_OAK_LEAVES),
            mangrove_leaves: Item::from_block(&*vanilla_blocks::MANGROVE_LEAVES),
            azalea_leaves: Item::from_block(&*vanilla_blocks::AZALEA_LEAVES),
            flowering_azalea_leaves: Item::from_block(&*vanilla_blocks::FLOWERING_AZALEA_LEAVES),
            sponge: Item::from_block(&*vanilla_blocks::SPONGE),
            wet_sponge: Item::from_block(&*vanilla_blocks::WET_SPONGE),
            glass: Item::from_block(&*vanilla_blocks::GLASS),
            tinted_glass: Item::from_block(&*vanilla_blocks::TINTED_GLASS),
            lapis_block: Item::from_block(&*vanilla_blocks::LAPIS_BLOCK),
            sandstone: Item::from_block(&*vanilla_blocks::SANDSTONE),
            chiseled_sandstone: Item::from_block(&*vanilla_blocks::CHISELED_SANDSTONE),
            cut_sandstone: Item::from_block(&*vanilla_blocks::CUT_SANDSTONE),
            cobweb: Item::from_block(&*vanilla_blocks::COBWEB),
            short_grass: Item::from_block(&*vanilla_blocks::SHORT_GRASS),
            fern: Item::from_block(&*vanilla_blocks::FERN),
            bush: Item::from_block(&*vanilla_blocks::BUSH),
            azalea: Item::from_block(&*vanilla_blocks::AZALEA),
            flowering_azalea: Item::from_block(&*vanilla_blocks::FLOWERING_AZALEA),
            dead_bush: Item::from_block(&*vanilla_blocks::DEAD_BUSH),
            firefly_bush: Item::from_block(&*vanilla_blocks::FIREFLY_BUSH),
            short_dry_grass: Item::from_block(&*vanilla_blocks::SHORT_DRY_GRASS),
            tall_dry_grass: Item::from_block(&*vanilla_blocks::TALL_DRY_GRASS),
            seagrass: Item::from_block(&*vanilla_blocks::SEAGRASS),
            sea_pickle: Item::from_block(&*vanilla_blocks::SEA_PICKLE),
            white_wool: Item::from_block(&*vanilla_blocks::WHITE_WOOL),
            orange_wool: Item::from_block(&*vanilla_blocks::ORANGE_WOOL),
            magenta_wool: Item::from_block(&*vanilla_blocks::MAGENTA_WOOL),
            light_blue_wool: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_WOOL),
            yellow_wool: Item::from_block(&*vanilla_blocks::YELLOW_WOOL),
            lime_wool: Item::from_block(&*vanilla_blocks::LIME_WOOL),
            pink_wool: Item::from_block(&*vanilla_blocks::PINK_WOOL),
            gray_wool: Item::from_block(&*vanilla_blocks::GRAY_WOOL),
            light_gray_wool: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_WOOL),
            cyan_wool: Item::from_block(&*vanilla_blocks::CYAN_WOOL),
            purple_wool: Item::from_block(&*vanilla_blocks::PURPLE_WOOL),
            blue_wool: Item::from_block(&*vanilla_blocks::BLUE_WOOL),
            brown_wool: Item::from_block(&*vanilla_blocks::BROWN_WOOL),
            green_wool: Item::from_block(&*vanilla_blocks::GREEN_WOOL),
            red_wool: Item::from_block(&*vanilla_blocks::RED_WOOL),
            black_wool: Item::from_block(&*vanilla_blocks::BLACK_WOOL),
            dandelion: Item::from_block(&*vanilla_blocks::DANDELION),
            open_eyeblossom: Item::from_block(&*vanilla_blocks::OPEN_EYEBLOSSOM),
            closed_eyeblossom: Item::from_block(&*vanilla_blocks::CLOSED_EYEBLOSSOM),
            poppy: Item::from_block(&*vanilla_blocks::POPPY),
            blue_orchid: Item::from_block(&*vanilla_blocks::BLUE_ORCHID),
            allium: Item::from_block(&*vanilla_blocks::ALLIUM),
            azure_bluet: Item::from_block(&*vanilla_blocks::AZURE_BLUET),
            red_tulip: Item::from_block(&*vanilla_blocks::RED_TULIP),
            orange_tulip: Item::from_block(&*vanilla_blocks::ORANGE_TULIP),
            white_tulip: Item::from_block(&*vanilla_blocks::WHITE_TULIP),
            pink_tulip: Item::from_block(&*vanilla_blocks::PINK_TULIP),
            oxeye_daisy: Item::from_block(&*vanilla_blocks::OXEYE_DAISY),
            cornflower: Item::from_block(&*vanilla_blocks::CORNFLOWER),
            lily_of_the_valley: Item::from_block(&*vanilla_blocks::LILY_OF_THE_VALLEY),
            wither_rose: Item::from_block(&*vanilla_blocks::WITHER_ROSE),
            torchflower: Item::from_block(&*vanilla_blocks::TORCHFLOWER),
            pitcher_plant: Item::from_block(&*vanilla_blocks::PITCHER_PLANT),
            spore_blossom: Item::from_block(&*vanilla_blocks::SPORE_BLOSSOM),
            brown_mushroom: Item::from_block(&*vanilla_blocks::BROWN_MUSHROOM),
            red_mushroom: Item::from_block(&*vanilla_blocks::RED_MUSHROOM),
            crimson_fungus: Item::from_block(&*vanilla_blocks::CRIMSON_FUNGUS),
            warped_fungus: Item::from_block(&*vanilla_blocks::WARPED_FUNGUS),
            crimson_roots: Item::from_block(&*vanilla_blocks::CRIMSON_ROOTS),
            warped_roots: Item::from_block(&*vanilla_blocks::WARPED_ROOTS),
            nether_sprouts: Item::from_block(&*vanilla_blocks::NETHER_SPROUTS),
            weeping_vines: Item::from_block(&*vanilla_blocks::WEEPING_VINES),
            twisting_vines: Item::from_block(&*vanilla_blocks::TWISTING_VINES),
            sugar_cane: Item::from_block(&*vanilla_blocks::SUGAR_CANE),
            kelp: Item::from_block(&*vanilla_blocks::KELP),
            pink_petals: Item::from_block(&*vanilla_blocks::PINK_PETALS),
            wildflowers: Item::from_block(&*vanilla_blocks::WILDFLOWERS),
            leaf_litter: Item::from_block(&*vanilla_blocks::LEAF_LITTER),
            moss_carpet: Item::from_block(&*vanilla_blocks::MOSS_CARPET),
            moss_block: Item::from_block(&*vanilla_blocks::MOSS_BLOCK),
            pale_moss_carpet: Item::from_block(&*vanilla_blocks::PALE_MOSS_CARPET),
            pale_hanging_moss: Item::from_block(&*vanilla_blocks::PALE_HANGING_MOSS),
            pale_moss_block: Item::from_block(&*vanilla_blocks::PALE_MOSS_BLOCK),
            hanging_roots: Item::from_block(&*vanilla_blocks::HANGING_ROOTS),
            big_dripleaf: Item::from_block(&*vanilla_blocks::BIG_DRIPLEAF),
            small_dripleaf: Item::from_block(&*vanilla_blocks::SMALL_DRIPLEAF),
            bamboo: Item::from_block(&*vanilla_blocks::BAMBOO),
            oak_slab: Item::from_block(&*vanilla_blocks::OAK_SLAB),
            spruce_slab: Item::from_block(&*vanilla_blocks::SPRUCE_SLAB),
            birch_slab: Item::from_block(&*vanilla_blocks::BIRCH_SLAB),
            jungle_slab: Item::from_block(&*vanilla_blocks::JUNGLE_SLAB),
            acacia_slab: Item::from_block(&*vanilla_blocks::ACACIA_SLAB),
            cherry_slab: Item::from_block(&*vanilla_blocks::CHERRY_SLAB),
            dark_oak_slab: Item::from_block(&*vanilla_blocks::DARK_OAK_SLAB),
            pale_oak_slab: Item::from_block(&*vanilla_blocks::PALE_OAK_SLAB),
            mangrove_slab: Item::from_block(&*vanilla_blocks::MANGROVE_SLAB),
            bamboo_slab: Item::from_block(&*vanilla_blocks::BAMBOO_SLAB),
            bamboo_mosaic_slab: Item::from_block(&*vanilla_blocks::BAMBOO_MOSAIC_SLAB),
            crimson_slab: Item::from_block(&*vanilla_blocks::CRIMSON_SLAB),
            warped_slab: Item::from_block(&*vanilla_blocks::WARPED_SLAB),
            stone_slab: Item::from_block(&*vanilla_blocks::STONE_SLAB),
            smooth_stone_slab: Item::from_block(&*vanilla_blocks::SMOOTH_STONE_SLAB),
            sandstone_slab: Item::from_block(&*vanilla_blocks::SANDSTONE_SLAB),
            cut_sandstone_slab: Item::from_block(&*vanilla_blocks::CUT_SANDSTONE_SLAB),
            petrified_oak_slab: Item::from_block(&*vanilla_blocks::PETRIFIED_OAK_SLAB),
            cobblestone_slab: Item::from_block(&*vanilla_blocks::COBBLESTONE_SLAB),
            brick_slab: Item::from_block(&*vanilla_blocks::BRICK_SLAB),
            stone_brick_slab: Item::from_block(&*vanilla_blocks::STONE_BRICK_SLAB),
            mud_brick_slab: Item::from_block(&*vanilla_blocks::MUD_BRICK_SLAB),
            nether_brick_slab: Item::from_block(&*vanilla_blocks::NETHER_BRICK_SLAB),
            quartz_slab: Item::from_block(&*vanilla_blocks::QUARTZ_SLAB),
            red_sandstone_slab: Item::from_block(&*vanilla_blocks::RED_SANDSTONE_SLAB),
            cut_red_sandstone_slab: Item::from_block(&*vanilla_blocks::CUT_RED_SANDSTONE_SLAB),
            purpur_slab: Item::from_block(&*vanilla_blocks::PURPUR_SLAB),
            prismarine_slab: Item::from_block(&*vanilla_blocks::PRISMARINE_SLAB),
            prismarine_brick_slab: Item::from_block(&*vanilla_blocks::PRISMARINE_BRICK_SLAB),
            dark_prismarine_slab: Item::from_block(&*vanilla_blocks::DARK_PRISMARINE_SLAB),
            smooth_quartz: Item::from_block(&*vanilla_blocks::SMOOTH_QUARTZ),
            smooth_red_sandstone: Item::from_block(&*vanilla_blocks::SMOOTH_RED_SANDSTONE),
            smooth_sandstone: Item::from_block(&*vanilla_blocks::SMOOTH_SANDSTONE),
            smooth_stone: Item::from_block(&*vanilla_blocks::SMOOTH_STONE),
            bricks: Item::from_block(&*vanilla_blocks::BRICKS),
            acacia_shelf: Item::from_block(&*vanilla_blocks::ACACIA_SHELF),
            bamboo_shelf: Item::from_block(&*vanilla_blocks::BAMBOO_SHELF),
            birch_shelf: Item::from_block(&*vanilla_blocks::BIRCH_SHELF),
            cherry_shelf: Item::from_block(&*vanilla_blocks::CHERRY_SHELF),
            crimson_shelf: Item::from_block(&*vanilla_blocks::CRIMSON_SHELF),
            dark_oak_shelf: Item::from_block(&*vanilla_blocks::DARK_OAK_SHELF),
            jungle_shelf: Item::from_block(&*vanilla_blocks::JUNGLE_SHELF),
            mangrove_shelf: Item::from_block(&*vanilla_blocks::MANGROVE_SHELF),
            oak_shelf: Item::from_block(&*vanilla_blocks::OAK_SHELF),
            pale_oak_shelf: Item::from_block(&*vanilla_blocks::PALE_OAK_SHELF),
            spruce_shelf: Item::from_block(&*vanilla_blocks::SPRUCE_SHELF),
            warped_shelf: Item::from_block(&*vanilla_blocks::WARPED_SHELF),
            bookshelf: Item::from_block(&*vanilla_blocks::BOOKSHELF),
            chiseled_bookshelf: Item::from_block(&*vanilla_blocks::CHISELED_BOOKSHELF),
            decorated_pot: Item::from_block(&*vanilla_blocks::DECORATED_POT),
            mossy_cobblestone: Item::from_block(&*vanilla_blocks::MOSSY_COBBLESTONE),
            obsidian: Item::from_block(&*vanilla_blocks::OBSIDIAN),
            torch: Item::from_block(&*vanilla_blocks::TORCH),
            end_rod: Item::from_block(&*vanilla_blocks::END_ROD),
            chorus_plant: Item::from_block(&*vanilla_blocks::CHORUS_PLANT),
            chorus_flower: Item::from_block(&*vanilla_blocks::CHORUS_FLOWER),
            purpur_block: Item::from_block(&*vanilla_blocks::PURPUR_BLOCK),
            purpur_pillar: Item::from_block(&*vanilla_blocks::PURPUR_PILLAR),
            purpur_stairs: Item::from_block(&*vanilla_blocks::PURPUR_STAIRS),
            spawner: Item::from_block(&*vanilla_blocks::SPAWNER),
            creaking_heart: Item::from_block(&*vanilla_blocks::CREAKING_HEART),
            chest: Item::from_block(&*vanilla_blocks::CHEST),
            crafting_table: Item::from_block(&*vanilla_blocks::CRAFTING_TABLE),
            farmland: Item::from_block(&*vanilla_blocks::FARMLAND),
            furnace: Item::from_block(&*vanilla_blocks::FURNACE),
            ladder: Item::from_block(&*vanilla_blocks::LADDER),
            cobblestone_stairs: Item::from_block(&*vanilla_blocks::COBBLESTONE_STAIRS),
            snow: Item::from_block(&*vanilla_blocks::SNOW),
            ice: Item::from_block(&*vanilla_blocks::ICE),
            snow_block: Item::from_block(&*vanilla_blocks::SNOW_BLOCK),
            cactus: Item::from_block(&*vanilla_blocks::CACTUS),
            cactus_flower: Item::from_block(&*vanilla_blocks::CACTUS_FLOWER),
            clay: Item::from_block(&*vanilla_blocks::CLAY),
            jukebox: Item::from_block(&*vanilla_blocks::JUKEBOX),
            oak_fence: Item::from_block(&*vanilla_blocks::OAK_FENCE),
            spruce_fence: Item::from_block(&*vanilla_blocks::SPRUCE_FENCE),
            birch_fence: Item::from_block(&*vanilla_blocks::BIRCH_FENCE),
            jungle_fence: Item::from_block(&*vanilla_blocks::JUNGLE_FENCE),
            acacia_fence: Item::from_block(&*vanilla_blocks::ACACIA_FENCE),
            cherry_fence: Item::from_block(&*vanilla_blocks::CHERRY_FENCE),
            dark_oak_fence: Item::from_block(&*vanilla_blocks::DARK_OAK_FENCE),
            pale_oak_fence: Item::from_block(&*vanilla_blocks::PALE_OAK_FENCE),
            mangrove_fence: Item::from_block(&*vanilla_blocks::MANGROVE_FENCE),
            bamboo_fence: Item::from_block(&*vanilla_blocks::BAMBOO_FENCE),
            crimson_fence: Item::from_block(&*vanilla_blocks::CRIMSON_FENCE),
            warped_fence: Item::from_block(&*vanilla_blocks::WARPED_FENCE),
            pumpkin: Item::from_block(&*vanilla_blocks::PUMPKIN),
            carved_pumpkin: Item::from_block(&*vanilla_blocks::CARVED_PUMPKIN),
            jack_o_lantern: Item::from_block(&*vanilla_blocks::JACK_O_LANTERN),
            netherrack: Item::from_block(&*vanilla_blocks::NETHERRACK),
            soul_sand: Item::from_block(&*vanilla_blocks::SOUL_SAND),
            soul_soil: Item::from_block(&*vanilla_blocks::SOUL_SOIL),
            basalt: Item::from_block(&*vanilla_blocks::BASALT),
            polished_basalt: Item::from_block(&*vanilla_blocks::POLISHED_BASALT),
            smooth_basalt: Item::from_block(&*vanilla_blocks::SMOOTH_BASALT),
            soul_torch: Item::from_block(&*vanilla_blocks::SOUL_TORCH),
            copper_torch: Item::from_block(&*vanilla_blocks::COPPER_TORCH),
            glowstone: Item::from_block(&*vanilla_blocks::GLOWSTONE),
            infested_stone: Item::from_block(&*vanilla_blocks::INFESTED_STONE),
            infested_cobblestone: Item::from_block(&*vanilla_blocks::INFESTED_COBBLESTONE),
            infested_stone_bricks: Item::from_block(&*vanilla_blocks::INFESTED_STONE_BRICKS),
            infested_mossy_stone_bricks: Item::from_block(
                &*vanilla_blocks::INFESTED_MOSSY_STONE_BRICKS,
            ),
            infested_cracked_stone_bricks: Item::from_block(
                &*vanilla_blocks::INFESTED_CRACKED_STONE_BRICKS,
            ),
            infested_chiseled_stone_bricks: Item::from_block(
                &*vanilla_blocks::INFESTED_CHISELED_STONE_BRICKS,
            ),
            infested_deepslate: Item::from_block(&*vanilla_blocks::INFESTED_DEEPSLATE),
            stone_bricks: Item::from_block(&*vanilla_blocks::STONE_BRICKS),
            mossy_stone_bricks: Item::from_block(&*vanilla_blocks::MOSSY_STONE_BRICKS),
            cracked_stone_bricks: Item::from_block(&*vanilla_blocks::CRACKED_STONE_BRICKS),
            chiseled_stone_bricks: Item::from_block(&*vanilla_blocks::CHISELED_STONE_BRICKS),
            packed_mud: Item::from_block(&*vanilla_blocks::PACKED_MUD),
            mud_bricks: Item::from_block(&*vanilla_blocks::MUD_BRICKS),
            deepslate_bricks: Item::from_block(&*vanilla_blocks::DEEPSLATE_BRICKS),
            cracked_deepslate_bricks: Item::from_block(&*vanilla_blocks::CRACKED_DEEPSLATE_BRICKS),
            deepslate_tiles: Item::from_block(&*vanilla_blocks::DEEPSLATE_TILES),
            cracked_deepslate_tiles: Item::from_block(&*vanilla_blocks::CRACKED_DEEPSLATE_TILES),
            chiseled_deepslate: Item::from_block(&*vanilla_blocks::CHISELED_DEEPSLATE),
            reinforced_deepslate: Item::from_block(&*vanilla_blocks::REINFORCED_DEEPSLATE),
            brown_mushroom_block: Item::from_block(&*vanilla_blocks::BROWN_MUSHROOM_BLOCK),
            red_mushroom_block: Item::from_block(&*vanilla_blocks::RED_MUSHROOM_BLOCK),
            mushroom_stem: Item::from_block(&*vanilla_blocks::MUSHROOM_STEM),
            iron_bars: Item::from_block(&*vanilla_blocks::IRON_BARS),
            copper_bars: Item::from_block(&*vanilla_blocks::COPPER_BARS),
            exposed_copper_bars: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER_BARS),
            weathered_copper_bars: Item::from_block(&*vanilla_blocks::WEATHERED_COPPER_BARS),
            oxidized_copper_bars: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER_BARS),
            waxed_copper_bars: Item::from_block(&*vanilla_blocks::WAXED_COPPER_BARS),
            waxed_exposed_copper_bars: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_BARS,
            ),
            waxed_weathered_copper_bars: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_BARS,
            ),
            waxed_oxidized_copper_bars: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_BARS,
            ),
            iron_chain: Item::from_block(&*vanilla_blocks::IRON_CHAIN),
            copper_chain: Item::from_block(&*vanilla_blocks::COPPER_CHAIN),
            exposed_copper_chain: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER_CHAIN),
            weathered_copper_chain: Item::from_block(&*vanilla_blocks::WEATHERED_COPPER_CHAIN),
            oxidized_copper_chain: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER_CHAIN),
            waxed_copper_chain: Item::from_block(&*vanilla_blocks::WAXED_COPPER_CHAIN),
            waxed_exposed_copper_chain: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_CHAIN,
            ),
            waxed_weathered_copper_chain: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_CHAIN,
            ),
            waxed_oxidized_copper_chain: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_CHAIN,
            ),
            glass_pane: Item::from_block(&*vanilla_blocks::GLASS_PANE),
            melon: Item::from_block(&*vanilla_blocks::MELON),
            vine: Item::from_block(&*vanilla_blocks::VINE),
            glow_lichen: Item::from_block(&*vanilla_blocks::GLOW_LICHEN),
            resin_clump: Item::from_block(&*vanilla_blocks::RESIN_CLUMP),
            resin_block: Item::from_block(&*vanilla_blocks::RESIN_BLOCK),
            resin_bricks: Item::from_block(&*vanilla_blocks::RESIN_BRICKS),
            resin_brick_stairs: Item::from_block(&*vanilla_blocks::RESIN_BRICK_STAIRS),
            resin_brick_slab: Item::from_block(&*vanilla_blocks::RESIN_BRICK_SLAB),
            resin_brick_wall: Item::from_block(&*vanilla_blocks::RESIN_BRICK_WALL),
            chiseled_resin_bricks: Item::from_block(&*vanilla_blocks::CHISELED_RESIN_BRICKS),
            brick_stairs: Item::from_block(&*vanilla_blocks::BRICK_STAIRS),
            stone_brick_stairs: Item::from_block(&*vanilla_blocks::STONE_BRICK_STAIRS),
            mud_brick_stairs: Item::from_block(&*vanilla_blocks::MUD_BRICK_STAIRS),
            mycelium: Item::from_block(&*vanilla_blocks::MYCELIUM),
            lily_pad: Item::from_block(&*vanilla_blocks::LILY_PAD),
            nether_bricks: Item::from_block(&*vanilla_blocks::NETHER_BRICKS),
            cracked_nether_bricks: Item::from_block(&*vanilla_blocks::CRACKED_NETHER_BRICKS),
            chiseled_nether_bricks: Item::from_block(&*vanilla_blocks::CHISELED_NETHER_BRICKS),
            nether_brick_fence: Item::from_block(&*vanilla_blocks::NETHER_BRICK_FENCE),
            nether_brick_stairs: Item::from_block(&*vanilla_blocks::NETHER_BRICK_STAIRS),
            sculk: Item::from_block(&*vanilla_blocks::SCULK),
            sculk_vein: Item::from_block(&*vanilla_blocks::SCULK_VEIN),
            sculk_catalyst: Item::from_block(&*vanilla_blocks::SCULK_CATALYST),
            sculk_shrieker: Item::from_block(&*vanilla_blocks::SCULK_SHRIEKER),
            enchanting_table: Item::from_block(&*vanilla_blocks::ENCHANTING_TABLE),
            end_portal_frame: Item::from_block(&*vanilla_blocks::END_PORTAL_FRAME),
            end_stone: Item::from_block(&*vanilla_blocks::END_STONE),
            end_stone_bricks: Item::from_block(&*vanilla_blocks::END_STONE_BRICKS),
            dragon_egg: Item::from_block(&*vanilla_blocks::DRAGON_EGG),
            sandstone_stairs: Item::from_block(&*vanilla_blocks::SANDSTONE_STAIRS),
            ender_chest: Item::from_block(&*vanilla_blocks::ENDER_CHEST),
            emerald_block: Item::from_block(&*vanilla_blocks::EMERALD_BLOCK),
            oak_stairs: Item::from_block(&*vanilla_blocks::OAK_STAIRS),
            spruce_stairs: Item::from_block(&*vanilla_blocks::SPRUCE_STAIRS),
            birch_stairs: Item::from_block(&*vanilla_blocks::BIRCH_STAIRS),
            jungle_stairs: Item::from_block(&*vanilla_blocks::JUNGLE_STAIRS),
            acacia_stairs: Item::from_block(&*vanilla_blocks::ACACIA_STAIRS),
            cherry_stairs: Item::from_block(&*vanilla_blocks::CHERRY_STAIRS),
            dark_oak_stairs: Item::from_block(&*vanilla_blocks::DARK_OAK_STAIRS),
            pale_oak_stairs: Item::from_block(&*vanilla_blocks::PALE_OAK_STAIRS),
            mangrove_stairs: Item::from_block(&*vanilla_blocks::MANGROVE_STAIRS),
            bamboo_stairs: Item::from_block(&*vanilla_blocks::BAMBOO_STAIRS),
            bamboo_mosaic_stairs: Item::from_block(&*vanilla_blocks::BAMBOO_MOSAIC_STAIRS),
            crimson_stairs: Item::from_block(&*vanilla_blocks::CRIMSON_STAIRS),
            warped_stairs: Item::from_block(&*vanilla_blocks::WARPED_STAIRS),
            command_block: Item::from_block(&*vanilla_blocks::COMMAND_BLOCK),
            beacon: Item::from_block(&*vanilla_blocks::BEACON),
            cobblestone_wall: Item::from_block(&*vanilla_blocks::COBBLESTONE_WALL),
            mossy_cobblestone_wall: Item::from_block(&*vanilla_blocks::MOSSY_COBBLESTONE_WALL),
            brick_wall: Item::from_block(&*vanilla_blocks::BRICK_WALL),
            prismarine_wall: Item::from_block(&*vanilla_blocks::PRISMARINE_WALL),
            red_sandstone_wall: Item::from_block(&*vanilla_blocks::RED_SANDSTONE_WALL),
            mossy_stone_brick_wall: Item::from_block(&*vanilla_blocks::MOSSY_STONE_BRICK_WALL),
            granite_wall: Item::from_block(&*vanilla_blocks::GRANITE_WALL),
            stone_brick_wall: Item::from_block(&*vanilla_blocks::STONE_BRICK_WALL),
            mud_brick_wall: Item::from_block(&*vanilla_blocks::MUD_BRICK_WALL),
            nether_brick_wall: Item::from_block(&*vanilla_blocks::NETHER_BRICK_WALL),
            andesite_wall: Item::from_block(&*vanilla_blocks::ANDESITE_WALL),
            red_nether_brick_wall: Item::from_block(&*vanilla_blocks::RED_NETHER_BRICK_WALL),
            sandstone_wall: Item::from_block(&*vanilla_blocks::SANDSTONE_WALL),
            end_stone_brick_wall: Item::from_block(&*vanilla_blocks::END_STONE_BRICK_WALL),
            diorite_wall: Item::from_block(&*vanilla_blocks::DIORITE_WALL),
            blackstone_wall: Item::from_block(&*vanilla_blocks::BLACKSTONE_WALL),
            polished_blackstone_wall: Item::from_block(&*vanilla_blocks::POLISHED_BLACKSTONE_WALL),
            polished_blackstone_brick_wall: Item::from_block(
                &*vanilla_blocks::POLISHED_BLACKSTONE_BRICK_WALL,
            ),
            cobbled_deepslate_wall: Item::from_block(&*vanilla_blocks::COBBLED_DEEPSLATE_WALL),
            polished_deepslate_wall: Item::from_block(&*vanilla_blocks::POLISHED_DEEPSLATE_WALL),
            deepslate_brick_wall: Item::from_block(&*vanilla_blocks::DEEPSLATE_BRICK_WALL),
            deepslate_tile_wall: Item::from_block(&*vanilla_blocks::DEEPSLATE_TILE_WALL),
            anvil: Item::from_block(&*vanilla_blocks::ANVIL),
            chipped_anvil: Item::from_block(&*vanilla_blocks::CHIPPED_ANVIL),
            damaged_anvil: Item::from_block(&*vanilla_blocks::DAMAGED_ANVIL),
            chiseled_quartz_block: Item::from_block(&*vanilla_blocks::CHISELED_QUARTZ_BLOCK),
            quartz_block: Item::from_block(&*vanilla_blocks::QUARTZ_BLOCK),
            quartz_bricks: Item::from_block(&*vanilla_blocks::QUARTZ_BRICKS),
            quartz_pillar: Item::from_block(&*vanilla_blocks::QUARTZ_PILLAR),
            quartz_stairs: Item::from_block(&*vanilla_blocks::QUARTZ_STAIRS),
            white_terracotta: Item::from_block(&*vanilla_blocks::WHITE_TERRACOTTA),
            orange_terracotta: Item::from_block(&*vanilla_blocks::ORANGE_TERRACOTTA),
            magenta_terracotta: Item::from_block(&*vanilla_blocks::MAGENTA_TERRACOTTA),
            light_blue_terracotta: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_TERRACOTTA),
            yellow_terracotta: Item::from_block(&*vanilla_blocks::YELLOW_TERRACOTTA),
            lime_terracotta: Item::from_block(&*vanilla_blocks::LIME_TERRACOTTA),
            pink_terracotta: Item::from_block(&*vanilla_blocks::PINK_TERRACOTTA),
            gray_terracotta: Item::from_block(&*vanilla_blocks::GRAY_TERRACOTTA),
            light_gray_terracotta: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_TERRACOTTA),
            cyan_terracotta: Item::from_block(&*vanilla_blocks::CYAN_TERRACOTTA),
            purple_terracotta: Item::from_block(&*vanilla_blocks::PURPLE_TERRACOTTA),
            blue_terracotta: Item::from_block(&*vanilla_blocks::BLUE_TERRACOTTA),
            brown_terracotta: Item::from_block(&*vanilla_blocks::BROWN_TERRACOTTA),
            green_terracotta: Item::from_block(&*vanilla_blocks::GREEN_TERRACOTTA),
            red_terracotta: Item::from_block(&*vanilla_blocks::RED_TERRACOTTA),
            black_terracotta: Item::from_block(&*vanilla_blocks::BLACK_TERRACOTTA),
            barrier: Item::from_block(&*vanilla_blocks::BARRIER),
            light: Item::from_block(&*vanilla_blocks::LIGHT),
            hay_block: Item::from_block(&*vanilla_blocks::HAY_BLOCK),
            white_carpet: Item::from_block(&*vanilla_blocks::WHITE_CARPET),
            orange_carpet: Item::from_block(&*vanilla_blocks::ORANGE_CARPET),
            magenta_carpet: Item::from_block(&*vanilla_blocks::MAGENTA_CARPET),
            light_blue_carpet: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_CARPET),
            yellow_carpet: Item::from_block(&*vanilla_blocks::YELLOW_CARPET),
            lime_carpet: Item::from_block(&*vanilla_blocks::LIME_CARPET),
            pink_carpet: Item::from_block(&*vanilla_blocks::PINK_CARPET),
            gray_carpet: Item::from_block(&*vanilla_blocks::GRAY_CARPET),
            light_gray_carpet: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_CARPET),
            cyan_carpet: Item::from_block(&*vanilla_blocks::CYAN_CARPET),
            purple_carpet: Item::from_block(&*vanilla_blocks::PURPLE_CARPET),
            blue_carpet: Item::from_block(&*vanilla_blocks::BLUE_CARPET),
            brown_carpet: Item::from_block(&*vanilla_blocks::BROWN_CARPET),
            green_carpet: Item::from_block(&*vanilla_blocks::GREEN_CARPET),
            red_carpet: Item::from_block(&*vanilla_blocks::RED_CARPET),
            black_carpet: Item::from_block(&*vanilla_blocks::BLACK_CARPET),
            terracotta: Item::from_block(&*vanilla_blocks::TERRACOTTA),
            packed_ice: Item::from_block(&*vanilla_blocks::PACKED_ICE),
            dirt_path: Item::from_block(&*vanilla_blocks::DIRT_PATH),
            sunflower: Item::from_block(&*vanilla_blocks::SUNFLOWER),
            lilac: Item::from_block(&*vanilla_blocks::LILAC),
            rose_bush: Item::from_block(&*vanilla_blocks::ROSE_BUSH),
            peony: Item::from_block(&*vanilla_blocks::PEONY),
            tall_grass: Item::from_block(&*vanilla_blocks::TALL_GRASS),
            large_fern: Item::from_block(&*vanilla_blocks::LARGE_FERN),
            white_stained_glass: Item::from_block(&*vanilla_blocks::WHITE_STAINED_GLASS),
            orange_stained_glass: Item::from_block(&*vanilla_blocks::ORANGE_STAINED_GLASS),
            magenta_stained_glass: Item::from_block(&*vanilla_blocks::MAGENTA_STAINED_GLASS),
            light_blue_stained_glass: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_STAINED_GLASS),
            yellow_stained_glass: Item::from_block(&*vanilla_blocks::YELLOW_STAINED_GLASS),
            lime_stained_glass: Item::from_block(&*vanilla_blocks::LIME_STAINED_GLASS),
            pink_stained_glass: Item::from_block(&*vanilla_blocks::PINK_STAINED_GLASS),
            gray_stained_glass: Item::from_block(&*vanilla_blocks::GRAY_STAINED_GLASS),
            light_gray_stained_glass: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_STAINED_GLASS),
            cyan_stained_glass: Item::from_block(&*vanilla_blocks::CYAN_STAINED_GLASS),
            purple_stained_glass: Item::from_block(&*vanilla_blocks::PURPLE_STAINED_GLASS),
            blue_stained_glass: Item::from_block(&*vanilla_blocks::BLUE_STAINED_GLASS),
            brown_stained_glass: Item::from_block(&*vanilla_blocks::BROWN_STAINED_GLASS),
            green_stained_glass: Item::from_block(&*vanilla_blocks::GREEN_STAINED_GLASS),
            red_stained_glass: Item::from_block(&*vanilla_blocks::RED_STAINED_GLASS),
            black_stained_glass: Item::from_block(&*vanilla_blocks::BLACK_STAINED_GLASS),
            white_stained_glass_pane: Item::from_block(&*vanilla_blocks::WHITE_STAINED_GLASS_PANE),
            orange_stained_glass_pane: Item::from_block(
                &*vanilla_blocks::ORANGE_STAINED_GLASS_PANE,
            ),
            magenta_stained_glass_pane: Item::from_block(
                &*vanilla_blocks::MAGENTA_STAINED_GLASS_PANE,
            ),
            light_blue_stained_glass_pane: Item::from_block(
                &*vanilla_blocks::LIGHT_BLUE_STAINED_GLASS_PANE,
            ),
            yellow_stained_glass_pane: Item::from_block(
                &*vanilla_blocks::YELLOW_STAINED_GLASS_PANE,
            ),
            lime_stained_glass_pane: Item::from_block(&*vanilla_blocks::LIME_STAINED_GLASS_PANE),
            pink_stained_glass_pane: Item::from_block(&*vanilla_blocks::PINK_STAINED_GLASS_PANE),
            gray_stained_glass_pane: Item::from_block(&*vanilla_blocks::GRAY_STAINED_GLASS_PANE),
            light_gray_stained_glass_pane: Item::from_block(
                &*vanilla_blocks::LIGHT_GRAY_STAINED_GLASS_PANE,
            ),
            cyan_stained_glass_pane: Item::from_block(&*vanilla_blocks::CYAN_STAINED_GLASS_PANE),
            purple_stained_glass_pane: Item::from_block(
                &*vanilla_blocks::PURPLE_STAINED_GLASS_PANE,
            ),
            blue_stained_glass_pane: Item::from_block(&*vanilla_blocks::BLUE_STAINED_GLASS_PANE),
            brown_stained_glass_pane: Item::from_block(&*vanilla_blocks::BROWN_STAINED_GLASS_PANE),
            green_stained_glass_pane: Item::from_block(&*vanilla_blocks::GREEN_STAINED_GLASS_PANE),
            red_stained_glass_pane: Item::from_block(&*vanilla_blocks::RED_STAINED_GLASS_PANE),
            black_stained_glass_pane: Item::from_block(&*vanilla_blocks::BLACK_STAINED_GLASS_PANE),
            prismarine: Item::from_block(&*vanilla_blocks::PRISMARINE),
            prismarine_bricks: Item::from_block(&*vanilla_blocks::PRISMARINE_BRICKS),
            dark_prismarine: Item::from_block(&*vanilla_blocks::DARK_PRISMARINE),
            prismarine_stairs: Item::from_block(&*vanilla_blocks::PRISMARINE_STAIRS),
            prismarine_brick_stairs: Item::from_block(&*vanilla_blocks::PRISMARINE_BRICK_STAIRS),
            dark_prismarine_stairs: Item::from_block(&*vanilla_blocks::DARK_PRISMARINE_STAIRS),
            sea_lantern: Item::from_block(&*vanilla_blocks::SEA_LANTERN),
            red_sandstone: Item::from_block(&*vanilla_blocks::RED_SANDSTONE),
            chiseled_red_sandstone: Item::from_block(&*vanilla_blocks::CHISELED_RED_SANDSTONE),
            cut_red_sandstone: Item::from_block(&*vanilla_blocks::CUT_RED_SANDSTONE),
            red_sandstone_stairs: Item::from_block(&*vanilla_blocks::RED_SANDSTONE_STAIRS),
            repeating_command_block: Item::from_block(&*vanilla_blocks::REPEATING_COMMAND_BLOCK),
            chain_command_block: Item::from_block(&*vanilla_blocks::CHAIN_COMMAND_BLOCK),
            magma_block: Item::from_block(&*vanilla_blocks::MAGMA_BLOCK),
            nether_wart_block: Item::from_block(&*vanilla_blocks::NETHER_WART_BLOCK),
            warped_wart_block: Item::from_block(&*vanilla_blocks::WARPED_WART_BLOCK),
            red_nether_bricks: Item::from_block(&*vanilla_blocks::RED_NETHER_BRICKS),
            bone_block: Item::from_block(&*vanilla_blocks::BONE_BLOCK),
            structure_void: Item::from_block(&*vanilla_blocks::STRUCTURE_VOID),
            shulker_box: Item::from_block(&*vanilla_blocks::SHULKER_BOX),
            white_shulker_box: Item::from_block(&*vanilla_blocks::WHITE_SHULKER_BOX),
            orange_shulker_box: Item::from_block(&*vanilla_blocks::ORANGE_SHULKER_BOX),
            magenta_shulker_box: Item::from_block(&*vanilla_blocks::MAGENTA_SHULKER_BOX),
            light_blue_shulker_box: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_SHULKER_BOX),
            yellow_shulker_box: Item::from_block(&*vanilla_blocks::YELLOW_SHULKER_BOX),
            lime_shulker_box: Item::from_block(&*vanilla_blocks::LIME_SHULKER_BOX),
            pink_shulker_box: Item::from_block(&*vanilla_blocks::PINK_SHULKER_BOX),
            gray_shulker_box: Item::from_block(&*vanilla_blocks::GRAY_SHULKER_BOX),
            light_gray_shulker_box: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_SHULKER_BOX),
            cyan_shulker_box: Item::from_block(&*vanilla_blocks::CYAN_SHULKER_BOX),
            purple_shulker_box: Item::from_block(&*vanilla_blocks::PURPLE_SHULKER_BOX),
            blue_shulker_box: Item::from_block(&*vanilla_blocks::BLUE_SHULKER_BOX),
            brown_shulker_box: Item::from_block(&*vanilla_blocks::BROWN_SHULKER_BOX),
            green_shulker_box: Item::from_block(&*vanilla_blocks::GREEN_SHULKER_BOX),
            red_shulker_box: Item::from_block(&*vanilla_blocks::RED_SHULKER_BOX),
            black_shulker_box: Item::from_block(&*vanilla_blocks::BLACK_SHULKER_BOX),
            white_glazed_terracotta: Item::from_block(&*vanilla_blocks::WHITE_GLAZED_TERRACOTTA),
            orange_glazed_terracotta: Item::from_block(&*vanilla_blocks::ORANGE_GLAZED_TERRACOTTA),
            magenta_glazed_terracotta: Item::from_block(
                &*vanilla_blocks::MAGENTA_GLAZED_TERRACOTTA,
            ),
            light_blue_glazed_terracotta: Item::from_block(
                &*vanilla_blocks::LIGHT_BLUE_GLAZED_TERRACOTTA,
            ),
            yellow_glazed_terracotta: Item::from_block(&*vanilla_blocks::YELLOW_GLAZED_TERRACOTTA),
            lime_glazed_terracotta: Item::from_block(&*vanilla_blocks::LIME_GLAZED_TERRACOTTA),
            pink_glazed_terracotta: Item::from_block(&*vanilla_blocks::PINK_GLAZED_TERRACOTTA),
            gray_glazed_terracotta: Item::from_block(&*vanilla_blocks::GRAY_GLAZED_TERRACOTTA),
            light_gray_glazed_terracotta: Item::from_block(
                &*vanilla_blocks::LIGHT_GRAY_GLAZED_TERRACOTTA,
            ),
            cyan_glazed_terracotta: Item::from_block(&*vanilla_blocks::CYAN_GLAZED_TERRACOTTA),
            purple_glazed_terracotta: Item::from_block(&*vanilla_blocks::PURPLE_GLAZED_TERRACOTTA),
            blue_glazed_terracotta: Item::from_block(&*vanilla_blocks::BLUE_GLAZED_TERRACOTTA),
            brown_glazed_terracotta: Item::from_block(&*vanilla_blocks::BROWN_GLAZED_TERRACOTTA),
            green_glazed_terracotta: Item::from_block(&*vanilla_blocks::GREEN_GLAZED_TERRACOTTA),
            red_glazed_terracotta: Item::from_block(&*vanilla_blocks::RED_GLAZED_TERRACOTTA),
            black_glazed_terracotta: Item::from_block(&*vanilla_blocks::BLACK_GLAZED_TERRACOTTA),
            white_concrete: Item::from_block(&*vanilla_blocks::WHITE_CONCRETE),
            orange_concrete: Item::from_block(&*vanilla_blocks::ORANGE_CONCRETE),
            magenta_concrete: Item::from_block(&*vanilla_blocks::MAGENTA_CONCRETE),
            light_blue_concrete: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_CONCRETE),
            yellow_concrete: Item::from_block(&*vanilla_blocks::YELLOW_CONCRETE),
            lime_concrete: Item::from_block(&*vanilla_blocks::LIME_CONCRETE),
            pink_concrete: Item::from_block(&*vanilla_blocks::PINK_CONCRETE),
            gray_concrete: Item::from_block(&*vanilla_blocks::GRAY_CONCRETE),
            light_gray_concrete: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_CONCRETE),
            cyan_concrete: Item::from_block(&*vanilla_blocks::CYAN_CONCRETE),
            purple_concrete: Item::from_block(&*vanilla_blocks::PURPLE_CONCRETE),
            blue_concrete: Item::from_block(&*vanilla_blocks::BLUE_CONCRETE),
            brown_concrete: Item::from_block(&*vanilla_blocks::BROWN_CONCRETE),
            green_concrete: Item::from_block(&*vanilla_blocks::GREEN_CONCRETE),
            red_concrete: Item::from_block(&*vanilla_blocks::RED_CONCRETE),
            black_concrete: Item::from_block(&*vanilla_blocks::BLACK_CONCRETE),
            white_concrete_powder: Item::from_block(&*vanilla_blocks::WHITE_CONCRETE_POWDER),
            orange_concrete_powder: Item::from_block(&*vanilla_blocks::ORANGE_CONCRETE_POWDER),
            magenta_concrete_powder: Item::from_block(&*vanilla_blocks::MAGENTA_CONCRETE_POWDER),
            light_blue_concrete_powder: Item::from_block(
                &*vanilla_blocks::LIGHT_BLUE_CONCRETE_POWDER,
            ),
            yellow_concrete_powder: Item::from_block(&*vanilla_blocks::YELLOW_CONCRETE_POWDER),
            lime_concrete_powder: Item::from_block(&*vanilla_blocks::LIME_CONCRETE_POWDER),
            pink_concrete_powder: Item::from_block(&*vanilla_blocks::PINK_CONCRETE_POWDER),
            gray_concrete_powder: Item::from_block(&*vanilla_blocks::GRAY_CONCRETE_POWDER),
            light_gray_concrete_powder: Item::from_block(
                &*vanilla_blocks::LIGHT_GRAY_CONCRETE_POWDER,
            ),
            cyan_concrete_powder: Item::from_block(&*vanilla_blocks::CYAN_CONCRETE_POWDER),
            purple_concrete_powder: Item::from_block(&*vanilla_blocks::PURPLE_CONCRETE_POWDER),
            blue_concrete_powder: Item::from_block(&*vanilla_blocks::BLUE_CONCRETE_POWDER),
            brown_concrete_powder: Item::from_block(&*vanilla_blocks::BROWN_CONCRETE_POWDER),
            green_concrete_powder: Item::from_block(&*vanilla_blocks::GREEN_CONCRETE_POWDER),
            red_concrete_powder: Item::from_block(&*vanilla_blocks::RED_CONCRETE_POWDER),
            black_concrete_powder: Item::from_block(&*vanilla_blocks::BLACK_CONCRETE_POWDER),
            turtle_egg: Item::from_block(&*vanilla_blocks::TURTLE_EGG),
            sniffer_egg: Item::from_block(&*vanilla_blocks::SNIFFER_EGG),
            dried_ghast: Item::from_block(&*vanilla_blocks::DRIED_GHAST),
            dead_tube_coral_block: Item::from_block(&*vanilla_blocks::DEAD_TUBE_CORAL_BLOCK),
            dead_brain_coral_block: Item::from_block(&*vanilla_blocks::DEAD_BRAIN_CORAL_BLOCK),
            dead_bubble_coral_block: Item::from_block(&*vanilla_blocks::DEAD_BUBBLE_CORAL_BLOCK),
            dead_fire_coral_block: Item::from_block(&*vanilla_blocks::DEAD_FIRE_CORAL_BLOCK),
            dead_horn_coral_block: Item::from_block(&*vanilla_blocks::DEAD_HORN_CORAL_BLOCK),
            tube_coral_block: Item::from_block(&*vanilla_blocks::TUBE_CORAL_BLOCK),
            brain_coral_block: Item::from_block(&*vanilla_blocks::BRAIN_CORAL_BLOCK),
            bubble_coral_block: Item::from_block(&*vanilla_blocks::BUBBLE_CORAL_BLOCK),
            fire_coral_block: Item::from_block(&*vanilla_blocks::FIRE_CORAL_BLOCK),
            horn_coral_block: Item::from_block(&*vanilla_blocks::HORN_CORAL_BLOCK),
            tube_coral: Item::from_block(&*vanilla_blocks::TUBE_CORAL),
            brain_coral: Item::from_block(&*vanilla_blocks::BRAIN_CORAL),
            bubble_coral: Item::from_block(&*vanilla_blocks::BUBBLE_CORAL),
            fire_coral: Item::from_block(&*vanilla_blocks::FIRE_CORAL),
            horn_coral: Item::from_block(&*vanilla_blocks::HORN_CORAL),
            dead_brain_coral: Item::from_block(&*vanilla_blocks::DEAD_BRAIN_CORAL),
            dead_bubble_coral: Item::from_block(&*vanilla_blocks::DEAD_BUBBLE_CORAL),
            dead_fire_coral: Item::from_block(&*vanilla_blocks::DEAD_FIRE_CORAL),
            dead_horn_coral: Item::from_block(&*vanilla_blocks::DEAD_HORN_CORAL),
            dead_tube_coral: Item::from_block(&*vanilla_blocks::DEAD_TUBE_CORAL),
            tube_coral_fan: Item::from_block(&*vanilla_blocks::TUBE_CORAL_FAN),
            brain_coral_fan: Item::from_block(&*vanilla_blocks::BRAIN_CORAL_FAN),
            bubble_coral_fan: Item::from_block(&*vanilla_blocks::BUBBLE_CORAL_FAN),
            fire_coral_fan: Item::from_block(&*vanilla_blocks::FIRE_CORAL_FAN),
            horn_coral_fan: Item::from_block(&*vanilla_blocks::HORN_CORAL_FAN),
            dead_tube_coral_fan: Item::from_block(&*vanilla_blocks::DEAD_TUBE_CORAL_FAN),
            dead_brain_coral_fan: Item::from_block(&*vanilla_blocks::DEAD_BRAIN_CORAL_FAN),
            dead_bubble_coral_fan: Item::from_block(&*vanilla_blocks::DEAD_BUBBLE_CORAL_FAN),
            dead_fire_coral_fan: Item::from_block(&*vanilla_blocks::DEAD_FIRE_CORAL_FAN),
            dead_horn_coral_fan: Item::from_block(&*vanilla_blocks::DEAD_HORN_CORAL_FAN),
            blue_ice: Item::from_block(&*vanilla_blocks::BLUE_ICE),
            conduit: Item::from_block(&*vanilla_blocks::CONDUIT),
            polished_granite_stairs: Item::from_block(&*vanilla_blocks::POLISHED_GRANITE_STAIRS),
            smooth_red_sandstone_stairs: Item::from_block(
                &*vanilla_blocks::SMOOTH_RED_SANDSTONE_STAIRS,
            ),
            mossy_stone_brick_stairs: Item::from_block(&*vanilla_blocks::MOSSY_STONE_BRICK_STAIRS),
            polished_diorite_stairs: Item::from_block(&*vanilla_blocks::POLISHED_DIORITE_STAIRS),
            mossy_cobblestone_stairs: Item::from_block(&*vanilla_blocks::MOSSY_COBBLESTONE_STAIRS),
            end_stone_brick_stairs: Item::from_block(&*vanilla_blocks::END_STONE_BRICK_STAIRS),
            stone_stairs: Item::from_block(&*vanilla_blocks::STONE_STAIRS),
            smooth_sandstone_stairs: Item::from_block(&*vanilla_blocks::SMOOTH_SANDSTONE_STAIRS),
            smooth_quartz_stairs: Item::from_block(&*vanilla_blocks::SMOOTH_QUARTZ_STAIRS),
            granite_stairs: Item::from_block(&*vanilla_blocks::GRANITE_STAIRS),
            andesite_stairs: Item::from_block(&*vanilla_blocks::ANDESITE_STAIRS),
            red_nether_brick_stairs: Item::from_block(&*vanilla_blocks::RED_NETHER_BRICK_STAIRS),
            polished_andesite_stairs: Item::from_block(&*vanilla_blocks::POLISHED_ANDESITE_STAIRS),
            diorite_stairs: Item::from_block(&*vanilla_blocks::DIORITE_STAIRS),
            cobbled_deepslate_stairs: Item::from_block(&*vanilla_blocks::COBBLED_DEEPSLATE_STAIRS),
            polished_deepslate_stairs: Item::from_block(
                &*vanilla_blocks::POLISHED_DEEPSLATE_STAIRS,
            ),
            deepslate_brick_stairs: Item::from_block(&*vanilla_blocks::DEEPSLATE_BRICK_STAIRS),
            deepslate_tile_stairs: Item::from_block(&*vanilla_blocks::DEEPSLATE_TILE_STAIRS),
            polished_granite_slab: Item::from_block(&*vanilla_blocks::POLISHED_GRANITE_SLAB),
            smooth_red_sandstone_slab: Item::from_block(
                &*vanilla_blocks::SMOOTH_RED_SANDSTONE_SLAB,
            ),
            mossy_stone_brick_slab: Item::from_block(&*vanilla_blocks::MOSSY_STONE_BRICK_SLAB),
            polished_diorite_slab: Item::from_block(&*vanilla_blocks::POLISHED_DIORITE_SLAB),
            mossy_cobblestone_slab: Item::from_block(&*vanilla_blocks::MOSSY_COBBLESTONE_SLAB),
            end_stone_brick_slab: Item::from_block(&*vanilla_blocks::END_STONE_BRICK_SLAB),
            smooth_sandstone_slab: Item::from_block(&*vanilla_blocks::SMOOTH_SANDSTONE_SLAB),
            smooth_quartz_slab: Item::from_block(&*vanilla_blocks::SMOOTH_QUARTZ_SLAB),
            granite_slab: Item::from_block(&*vanilla_blocks::GRANITE_SLAB),
            andesite_slab: Item::from_block(&*vanilla_blocks::ANDESITE_SLAB),
            red_nether_brick_slab: Item::from_block(&*vanilla_blocks::RED_NETHER_BRICK_SLAB),
            polished_andesite_slab: Item::from_block(&*vanilla_blocks::POLISHED_ANDESITE_SLAB),
            diorite_slab: Item::from_block(&*vanilla_blocks::DIORITE_SLAB),
            cobbled_deepslate_slab: Item::from_block(&*vanilla_blocks::COBBLED_DEEPSLATE_SLAB),
            polished_deepslate_slab: Item::from_block(&*vanilla_blocks::POLISHED_DEEPSLATE_SLAB),
            deepslate_brick_slab: Item::from_block(&*vanilla_blocks::DEEPSLATE_BRICK_SLAB),
            deepslate_tile_slab: Item::from_block(&*vanilla_blocks::DEEPSLATE_TILE_SLAB),
            scaffolding: Item::from_block(&*vanilla_blocks::SCAFFOLDING),
            redstone: Item::from_block_custom_name(&*vanilla_blocks::REDSTONE_WIRE, "redstone"),
            redstone_torch: Item::from_block(&*vanilla_blocks::REDSTONE_TORCH),
            redstone_block: Item::from_block(&*vanilla_blocks::REDSTONE_BLOCK),
            repeater: Item::from_block(&*vanilla_blocks::REPEATER),
            comparator: Item::from_block(&*vanilla_blocks::COMPARATOR),
            piston: Item::from_block(&*vanilla_blocks::PISTON),
            sticky_piston: Item::from_block(&*vanilla_blocks::STICKY_PISTON),
            slime_block: Item::from_block(&*vanilla_blocks::SLIME_BLOCK),
            honey_block: Item::from_block(&*vanilla_blocks::HONEY_BLOCK),
            observer: Item::from_block(&*vanilla_blocks::OBSERVER),
            hopper: Item::from_block(&*vanilla_blocks::HOPPER),
            dispenser: Item::from_block(&*vanilla_blocks::DISPENSER),
            dropper: Item::from_block(&*vanilla_blocks::DROPPER),
            lectern: Item::from_block(&*vanilla_blocks::LECTERN),
            target: Item::from_block(&*vanilla_blocks::TARGET),
            lever: Item::from_block(&*vanilla_blocks::LEVER),
            lightning_rod: Item::from_block(&*vanilla_blocks::LIGHTNING_ROD),
            exposed_lightning_rod: Item::from_block(&*vanilla_blocks::EXPOSED_LIGHTNING_ROD),
            weathered_lightning_rod: Item::from_block(&*vanilla_blocks::WEATHERED_LIGHTNING_ROD),
            oxidized_lightning_rod: Item::from_block(&*vanilla_blocks::OXIDIZED_LIGHTNING_ROD),
            waxed_lightning_rod: Item::from_block(&*vanilla_blocks::WAXED_LIGHTNING_ROD),
            waxed_exposed_lightning_rod: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_LIGHTNING_ROD,
            ),
            waxed_weathered_lightning_rod: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_LIGHTNING_ROD,
            ),
            waxed_oxidized_lightning_rod: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_LIGHTNING_ROD,
            ),
            daylight_detector: Item::from_block(&*vanilla_blocks::DAYLIGHT_DETECTOR),
            sculk_sensor: Item::from_block(&*vanilla_blocks::SCULK_SENSOR),
            calibrated_sculk_sensor: Item::from_block(&*vanilla_blocks::CALIBRATED_SCULK_SENSOR),
            tripwire_hook: Item::from_block(&*vanilla_blocks::TRIPWIRE_HOOK),
            trapped_chest: Item::from_block(&*vanilla_blocks::TRAPPED_CHEST),
            tnt: Item::from_block(&*vanilla_blocks::TNT),
            redstone_lamp: Item::from_block(&*vanilla_blocks::REDSTONE_LAMP),
            note_block: Item::from_block(&*vanilla_blocks::NOTE_BLOCK),
            stone_button: Item::from_block(&*vanilla_blocks::STONE_BUTTON),
            polished_blackstone_button: Item::from_block(
                &*vanilla_blocks::POLISHED_BLACKSTONE_BUTTON,
            ),
            oak_button: Item::from_block(&*vanilla_blocks::OAK_BUTTON),
            spruce_button: Item::from_block(&*vanilla_blocks::SPRUCE_BUTTON),
            birch_button: Item::from_block(&*vanilla_blocks::BIRCH_BUTTON),
            jungle_button: Item::from_block(&*vanilla_blocks::JUNGLE_BUTTON),
            acacia_button: Item::from_block(&*vanilla_blocks::ACACIA_BUTTON),
            cherry_button: Item::from_block(&*vanilla_blocks::CHERRY_BUTTON),
            dark_oak_button: Item::from_block(&*vanilla_blocks::DARK_OAK_BUTTON),
            pale_oak_button: Item::from_block(&*vanilla_blocks::PALE_OAK_BUTTON),
            mangrove_button: Item::from_block(&*vanilla_blocks::MANGROVE_BUTTON),
            bamboo_button: Item::from_block(&*vanilla_blocks::BAMBOO_BUTTON),
            crimson_button: Item::from_block(&*vanilla_blocks::CRIMSON_BUTTON),
            warped_button: Item::from_block(&*vanilla_blocks::WARPED_BUTTON),
            stone_pressure_plate: Item::from_block(&*vanilla_blocks::STONE_PRESSURE_PLATE),
            polished_blackstone_pressure_plate: Item::from_block(
                &*vanilla_blocks::POLISHED_BLACKSTONE_PRESSURE_PLATE,
            ),
            light_weighted_pressure_plate: Item::from_block(
                &*vanilla_blocks::LIGHT_WEIGHTED_PRESSURE_PLATE,
            ),
            heavy_weighted_pressure_plate: Item::from_block(
                &*vanilla_blocks::HEAVY_WEIGHTED_PRESSURE_PLATE,
            ),
            oak_pressure_plate: Item::from_block(&*vanilla_blocks::OAK_PRESSURE_PLATE),
            spruce_pressure_plate: Item::from_block(&*vanilla_blocks::SPRUCE_PRESSURE_PLATE),
            birch_pressure_plate: Item::from_block(&*vanilla_blocks::BIRCH_PRESSURE_PLATE),
            jungle_pressure_plate: Item::from_block(&*vanilla_blocks::JUNGLE_PRESSURE_PLATE),
            acacia_pressure_plate: Item::from_block(&*vanilla_blocks::ACACIA_PRESSURE_PLATE),
            cherry_pressure_plate: Item::from_block(&*vanilla_blocks::CHERRY_PRESSURE_PLATE),
            dark_oak_pressure_plate: Item::from_block(&*vanilla_blocks::DARK_OAK_PRESSURE_PLATE),
            pale_oak_pressure_plate: Item::from_block(&*vanilla_blocks::PALE_OAK_PRESSURE_PLATE),
            mangrove_pressure_plate: Item::from_block(&*vanilla_blocks::MANGROVE_PRESSURE_PLATE),
            bamboo_pressure_plate: Item::from_block(&*vanilla_blocks::BAMBOO_PRESSURE_PLATE),
            crimson_pressure_plate: Item::from_block(&*vanilla_blocks::CRIMSON_PRESSURE_PLATE),
            warped_pressure_plate: Item::from_block(&*vanilla_blocks::WARPED_PRESSURE_PLATE),
            iron_door: Item::from_block(&*vanilla_blocks::IRON_DOOR),
            oak_door: Item::from_block(&*vanilla_blocks::OAK_DOOR),
            spruce_door: Item::from_block(&*vanilla_blocks::SPRUCE_DOOR),
            birch_door: Item::from_block(&*vanilla_blocks::BIRCH_DOOR),
            jungle_door: Item::from_block(&*vanilla_blocks::JUNGLE_DOOR),
            acacia_door: Item::from_block(&*vanilla_blocks::ACACIA_DOOR),
            cherry_door: Item::from_block(&*vanilla_blocks::CHERRY_DOOR),
            dark_oak_door: Item::from_block(&*vanilla_blocks::DARK_OAK_DOOR),
            pale_oak_door: Item::from_block(&*vanilla_blocks::PALE_OAK_DOOR),
            mangrove_door: Item::from_block(&*vanilla_blocks::MANGROVE_DOOR),
            bamboo_door: Item::from_block(&*vanilla_blocks::BAMBOO_DOOR),
            crimson_door: Item::from_block(&*vanilla_blocks::CRIMSON_DOOR),
            warped_door: Item::from_block(&*vanilla_blocks::WARPED_DOOR),
            copper_door: Item::from_block(&*vanilla_blocks::COPPER_DOOR),
            exposed_copper_door: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER_DOOR),
            weathered_copper_door: Item::from_block(&*vanilla_blocks::WEATHERED_COPPER_DOOR),
            oxidized_copper_door: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER_DOOR),
            waxed_copper_door: Item::from_block(&*vanilla_blocks::WAXED_COPPER_DOOR),
            waxed_exposed_copper_door: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_DOOR,
            ),
            waxed_weathered_copper_door: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_DOOR,
            ),
            waxed_oxidized_copper_door: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_DOOR,
            ),
            iron_trapdoor: Item::from_block(&*vanilla_blocks::IRON_TRAPDOOR),
            oak_trapdoor: Item::from_block(&*vanilla_blocks::OAK_TRAPDOOR),
            spruce_trapdoor: Item::from_block(&*vanilla_blocks::SPRUCE_TRAPDOOR),
            birch_trapdoor: Item::from_block(&*vanilla_blocks::BIRCH_TRAPDOOR),
            jungle_trapdoor: Item::from_block(&*vanilla_blocks::JUNGLE_TRAPDOOR),
            acacia_trapdoor: Item::from_block(&*vanilla_blocks::ACACIA_TRAPDOOR),
            cherry_trapdoor: Item::from_block(&*vanilla_blocks::CHERRY_TRAPDOOR),
            dark_oak_trapdoor: Item::from_block(&*vanilla_blocks::DARK_OAK_TRAPDOOR),
            pale_oak_trapdoor: Item::from_block(&*vanilla_blocks::PALE_OAK_TRAPDOOR),
            mangrove_trapdoor: Item::from_block(&*vanilla_blocks::MANGROVE_TRAPDOOR),
            bamboo_trapdoor: Item::from_block(&*vanilla_blocks::BAMBOO_TRAPDOOR),
            crimson_trapdoor: Item::from_block(&*vanilla_blocks::CRIMSON_TRAPDOOR),
            warped_trapdoor: Item::from_block(&*vanilla_blocks::WARPED_TRAPDOOR),
            copper_trapdoor: Item::from_block(&*vanilla_blocks::COPPER_TRAPDOOR),
            exposed_copper_trapdoor: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER_TRAPDOOR),
            weathered_copper_trapdoor: Item::from_block(
                &*vanilla_blocks::WEATHERED_COPPER_TRAPDOOR,
            ),
            oxidized_copper_trapdoor: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER_TRAPDOOR),
            waxed_copper_trapdoor: Item::from_block(&*vanilla_blocks::WAXED_COPPER_TRAPDOOR),
            waxed_exposed_copper_trapdoor: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_TRAPDOOR,
            ),
            waxed_weathered_copper_trapdoor: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_TRAPDOOR,
            ),
            waxed_oxidized_copper_trapdoor: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_TRAPDOOR,
            ),
            oak_fence_gate: Item::from_block(&*vanilla_blocks::OAK_FENCE_GATE),
            spruce_fence_gate: Item::from_block(&*vanilla_blocks::SPRUCE_FENCE_GATE),
            birch_fence_gate: Item::from_block(&*vanilla_blocks::BIRCH_FENCE_GATE),
            jungle_fence_gate: Item::from_block(&*vanilla_blocks::JUNGLE_FENCE_GATE),
            acacia_fence_gate: Item::from_block(&*vanilla_blocks::ACACIA_FENCE_GATE),
            cherry_fence_gate: Item::from_block(&*vanilla_blocks::CHERRY_FENCE_GATE),
            dark_oak_fence_gate: Item::from_block(&*vanilla_blocks::DARK_OAK_FENCE_GATE),
            pale_oak_fence_gate: Item::from_block(&*vanilla_blocks::PALE_OAK_FENCE_GATE),
            mangrove_fence_gate: Item::from_block(&*vanilla_blocks::MANGROVE_FENCE_GATE),
            bamboo_fence_gate: Item::from_block(&*vanilla_blocks::BAMBOO_FENCE_GATE),
            crimson_fence_gate: Item::from_block(&*vanilla_blocks::CRIMSON_FENCE_GATE),
            warped_fence_gate: Item::from_block(&*vanilla_blocks::WARPED_FENCE_GATE),
            powered_rail: Item::from_block(&*vanilla_blocks::POWERED_RAIL),
            detector_rail: Item::from_block(&*vanilla_blocks::DETECTOR_RAIL),
            rail: Item::from_block(&*vanilla_blocks::RAIL),
            activator_rail: Item::from_block(&*vanilla_blocks::ACTIVATOR_RAIL),
            saddle: Item {
                key: Identifier::vanilla_static("saddle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            white_harness: Item {
                key: Identifier::vanilla_static("white_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            orange_harness: Item {
                key: Identifier::vanilla_static("orange_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            magenta_harness: Item {
                key: Identifier::vanilla_static("magenta_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            light_blue_harness: Item {
                key: Identifier::vanilla_static("light_blue_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            yellow_harness: Item {
                key: Identifier::vanilla_static("yellow_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            lime_harness: Item {
                key: Identifier::vanilla_static("lime_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            pink_harness: Item {
                key: Identifier::vanilla_static("pink_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            gray_harness: Item {
                key: Identifier::vanilla_static("gray_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            light_gray_harness: Item {
                key: Identifier::vanilla_static("light_gray_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            cyan_harness: Item {
                key: Identifier::vanilla_static("cyan_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            purple_harness: Item {
                key: Identifier::vanilla_static("purple_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            blue_harness: Item {
                key: Identifier::vanilla_static("blue_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            brown_harness: Item {
                key: Identifier::vanilla_static("brown_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            green_harness: Item {
                key: Identifier::vanilla_static("green_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            red_harness: Item {
                key: Identifier::vanilla_static("red_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            black_harness: Item {
                key: Identifier::vanilla_static("black_harness"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            minecart: Item {
                key: Identifier::vanilla_static("minecart"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            chest_minecart: Item {
                key: Identifier::vanilla_static("chest_minecart"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            furnace_minecart: Item {
                key: Identifier::vanilla_static("furnace_minecart"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            tnt_minecart: Item {
                key: Identifier::vanilla_static("tnt_minecart"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            hopper_minecart: Item {
                key: Identifier::vanilla_static("hopper_minecart"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            carrot_on_a_stick: Item {
                key: Identifier::vanilla_static("carrot_on_a_stick"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(25i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            warped_fungus_on_a_stick: Item {
                key: Identifier::vanilla_static("warped_fungus_on_a_stick"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(100i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            phantom_membrane: Item {
                key: Identifier::vanilla_static("phantom_membrane"),
                components: DataComponentMap::common_item_components(),
            },
            elytra: Item {
                key: Identifier::vanilla_static("elytra"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(432i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            oak_boat: Item {
                key: Identifier::vanilla_static("oak_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            oak_chest_boat: Item {
                key: Identifier::vanilla_static("oak_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            spruce_boat: Item {
                key: Identifier::vanilla_static("spruce_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            spruce_chest_boat: Item {
                key: Identifier::vanilla_static("spruce_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            birch_boat: Item {
                key: Identifier::vanilla_static("birch_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            birch_chest_boat: Item {
                key: Identifier::vanilla_static("birch_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            jungle_boat: Item {
                key: Identifier::vanilla_static("jungle_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            jungle_chest_boat: Item {
                key: Identifier::vanilla_static("jungle_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            acacia_boat: Item {
                key: Identifier::vanilla_static("acacia_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            acacia_chest_boat: Item {
                key: Identifier::vanilla_static("acacia_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            cherry_boat: Item {
                key: Identifier::vanilla_static("cherry_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            cherry_chest_boat: Item {
                key: Identifier::vanilla_static("cherry_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            dark_oak_boat: Item {
                key: Identifier::vanilla_static("dark_oak_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            dark_oak_chest_boat: Item {
                key: Identifier::vanilla_static("dark_oak_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            pale_oak_boat: Item {
                key: Identifier::vanilla_static("pale_oak_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            pale_oak_chest_boat: Item {
                key: Identifier::vanilla_static("pale_oak_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            mangrove_boat: Item {
                key: Identifier::vanilla_static("mangrove_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            mangrove_chest_boat: Item {
                key: Identifier::vanilla_static("mangrove_chest_boat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            bamboo_raft: Item {
                key: Identifier::vanilla_static("bamboo_raft"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            bamboo_chest_raft: Item {
                key: Identifier::vanilla_static("bamboo_chest_raft"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            structure_block: Item::from_block(&*vanilla_blocks::STRUCTURE_BLOCK),
            jigsaw: Item::from_block(&*vanilla_blocks::JIGSAW),
            test_block: Item::from_block(&*vanilla_blocks::TEST_BLOCK),
            test_instance_block: Item::from_block(&*vanilla_blocks::TEST_INSTANCE_BLOCK),
            turtle_helmet: Item {
                key: Identifier::vanilla_static("turtle_helmet"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(275i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            turtle_scute: Item {
                key: Identifier::vanilla_static("turtle_scute"),
                components: DataComponentMap::common_item_components(),
            },
            armadillo_scute: Item {
                key: Identifier::vanilla_static("armadillo_scute"),
                components: DataComponentMap::common_item_components(),
            },
            wolf_armor: Item {
                key: Identifier::vanilla_static("wolf_armor"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(64i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            flint_and_steel: Item {
                key: Identifier::vanilla_static("flint_and_steel"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(64i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            bowl: Item {
                key: Identifier::vanilla_static("bowl"),
                components: DataComponentMap::common_item_components(),
            },
            apple: Item {
                key: Identifier::vanilla_static("apple"),
                components: DataComponentMap::common_item_components(),
            },
            bow: Item {
                key: Identifier::vanilla_static("bow"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(384i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            arrow: Item {
                key: Identifier::vanilla_static("arrow"),
                components: DataComponentMap::common_item_components(),
            },
            coal: Item {
                key: Identifier::vanilla_static("coal"),
                components: DataComponentMap::common_item_components(),
            },
            charcoal: Item {
                key: Identifier::vanilla_static("charcoal"),
                components: DataComponentMap::common_item_components(),
            },
            diamond: Item {
                key: Identifier::vanilla_static("diamond"),
                components: DataComponentMap::common_item_components(),
            },
            emerald: Item {
                key: Identifier::vanilla_static("emerald"),
                components: DataComponentMap::common_item_components(),
            },
            lapis_lazuli: Item {
                key: Identifier::vanilla_static("lapis_lazuli"),
                components: DataComponentMap::common_item_components(),
            },
            quartz: Item {
                key: Identifier::vanilla_static("quartz"),
                components: DataComponentMap::common_item_components(),
            },
            amethyst_shard: Item {
                key: Identifier::vanilla_static("amethyst_shard"),
                components: DataComponentMap::common_item_components(),
            },
            raw_iron: Item {
                key: Identifier::vanilla_static("raw_iron"),
                components: DataComponentMap::common_item_components(),
            },
            iron_ingot: Item {
                key: Identifier::vanilla_static("iron_ingot"),
                components: DataComponentMap::common_item_components(),
            },
            raw_copper: Item {
                key: Identifier::vanilla_static("raw_copper"),
                components: DataComponentMap::common_item_components(),
            },
            copper_ingot: Item {
                key: Identifier::vanilla_static("copper_ingot"),
                components: DataComponentMap::common_item_components(),
            },
            raw_gold: Item {
                key: Identifier::vanilla_static("raw_gold"),
                components: DataComponentMap::common_item_components(),
            },
            gold_ingot: Item {
                key: Identifier::vanilla_static("gold_ingot"),
                components: DataComponentMap::common_item_components(),
            },
            netherite_ingot: Item {
                key: Identifier::vanilla_static("netherite_ingot"),
                components: DataComponentMap::common_item_components(),
            },
            netherite_scrap: Item {
                key: Identifier::vanilla_static("netherite_scrap"),
                components: DataComponentMap::common_item_components(),
            },
            wooden_sword: Item {
                key: Identifier::vanilla_static("wooden_sword"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(59i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            wooden_shovel: Item {
                key: Identifier::vanilla_static("wooden_shovel"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(59i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            wooden_pickaxe: Item {
                key: Identifier::vanilla_static("wooden_pickaxe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(59i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            wooden_axe: Item {
                key: Identifier::vanilla_static("wooden_axe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(59i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            wooden_hoe: Item {
                key: Identifier::vanilla_static("wooden_hoe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(59i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_sword: Item {
                key: Identifier::vanilla_static("copper_sword"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(190i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_shovel: Item {
                key: Identifier::vanilla_static("copper_shovel"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(190i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_pickaxe: Item {
                key: Identifier::vanilla_static("copper_pickaxe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(190i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_axe: Item {
                key: Identifier::vanilla_static("copper_axe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(190i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_hoe: Item {
                key: Identifier::vanilla_static("copper_hoe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(190i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            stone_sword: Item {
                key: Identifier::vanilla_static("stone_sword"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(131i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            stone_shovel: Item {
                key: Identifier::vanilla_static("stone_shovel"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(131i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            stone_pickaxe: Item {
                key: Identifier::vanilla_static("stone_pickaxe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(131i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            stone_axe: Item {
                key: Identifier::vanilla_static("stone_axe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(131i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            stone_hoe: Item {
                key: Identifier::vanilla_static("stone_hoe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(131i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_sword: Item {
                key: Identifier::vanilla_static("golden_sword"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(32i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_shovel: Item {
                key: Identifier::vanilla_static("golden_shovel"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(32i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_pickaxe: Item {
                key: Identifier::vanilla_static("golden_pickaxe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(32i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_axe: Item {
                key: Identifier::vanilla_static("golden_axe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(32i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_hoe: Item {
                key: Identifier::vanilla_static("golden_hoe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(32i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_sword: Item {
                key: Identifier::vanilla_static("iron_sword"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(250i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_shovel: Item {
                key: Identifier::vanilla_static("iron_shovel"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(250i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_pickaxe: Item {
                key: Identifier::vanilla_static("iron_pickaxe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(250i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_axe: Item {
                key: Identifier::vanilla_static("iron_axe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(250i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_hoe: Item {
                key: Identifier::vanilla_static("iron_hoe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(250i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_sword: Item {
                key: Identifier::vanilla_static("diamond_sword"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(1561i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_shovel: Item {
                key: Identifier::vanilla_static("diamond_shovel"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(1561i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_pickaxe: Item {
                key: Identifier::vanilla_static("diamond_pickaxe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(1561i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_axe: Item {
                key: Identifier::vanilla_static("diamond_axe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(1561i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_hoe: Item {
                key: Identifier::vanilla_static("diamond_hoe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(1561i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_sword: Item {
                key: Identifier::vanilla_static("netherite_sword"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(2031i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_shovel: Item {
                key: Identifier::vanilla_static("netherite_shovel"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(2031i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_pickaxe: Item {
                key: Identifier::vanilla_static("netherite_pickaxe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(2031i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_axe: Item {
                key: Identifier::vanilla_static("netherite_axe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(2031i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_hoe: Item {
                key: Identifier::vanilla_static("netherite_hoe"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(2031i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            stick: Item {
                key: Identifier::vanilla_static("stick"),
                components: DataComponentMap::common_item_components(),
            },
            mushroom_stew: Item {
                key: Identifier::vanilla_static("mushroom_stew"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            string: Item::from_block_custom_name(&*vanilla_blocks::TRIPWIRE, "string"),
            feather: Item {
                key: Identifier::vanilla_static("feather"),
                components: DataComponentMap::common_item_components(),
            },
            gunpowder: Item {
                key: Identifier::vanilla_static("gunpowder"),
                components: DataComponentMap::common_item_components(),
            },
            wheat_seeds: Item::from_block_custom_name(&*vanilla_blocks::WHEAT, "wheat_seeds"),
            wheat: Item {
                key: Identifier::vanilla_static("wheat"),
                components: DataComponentMap::common_item_components(),
            },
            bread: Item {
                key: Identifier::vanilla_static("bread"),
                components: DataComponentMap::common_item_components(),
            },
            leather_helmet: Item {
                key: Identifier::vanilla_static("leather_helmet"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(55i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            leather_chestplate: Item {
                key: Identifier::vanilla_static("leather_chestplate"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(80i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            leather_leggings: Item {
                key: Identifier::vanilla_static("leather_leggings"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(75i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            leather_boots: Item {
                key: Identifier::vanilla_static("leather_boots"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(65i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_helmet: Item {
                key: Identifier::vanilla_static("copper_helmet"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(121i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_chestplate: Item {
                key: Identifier::vanilla_static("copper_chestplate"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(176i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_leggings: Item {
                key: Identifier::vanilla_static("copper_leggings"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(165i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            copper_boots: Item {
                key: Identifier::vanilla_static("copper_boots"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(143i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            chainmail_helmet: Item {
                key: Identifier::vanilla_static("chainmail_helmet"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(165i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            chainmail_chestplate: Item {
                key: Identifier::vanilla_static("chainmail_chestplate"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(240i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            chainmail_leggings: Item {
                key: Identifier::vanilla_static("chainmail_leggings"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(225i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            chainmail_boots: Item {
                key: Identifier::vanilla_static("chainmail_boots"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(195i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_helmet: Item {
                key: Identifier::vanilla_static("iron_helmet"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(165i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_chestplate: Item {
                key: Identifier::vanilla_static("iron_chestplate"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(240i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_leggings: Item {
                key: Identifier::vanilla_static("iron_leggings"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(225i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_boots: Item {
                key: Identifier::vanilla_static("iron_boots"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(195i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_helmet: Item {
                key: Identifier::vanilla_static("diamond_helmet"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(363i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_chestplate: Item {
                key: Identifier::vanilla_static("diamond_chestplate"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(528i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_leggings: Item {
                key: Identifier::vanilla_static("diamond_leggings"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(495i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_boots: Item {
                key: Identifier::vanilla_static("diamond_boots"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(429i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_helmet: Item {
                key: Identifier::vanilla_static("golden_helmet"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(77i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_chestplate: Item {
                key: Identifier::vanilla_static("golden_chestplate"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(112i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_leggings: Item {
                key: Identifier::vanilla_static("golden_leggings"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(105i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_boots: Item {
                key: Identifier::vanilla_static("golden_boots"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(91i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_helmet: Item {
                key: Identifier::vanilla_static("netherite_helmet"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(407i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_chestplate: Item {
                key: Identifier::vanilla_static("netherite_chestplate"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(592i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_leggings: Item {
                key: Identifier::vanilla_static("netherite_leggings"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(555i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_boots: Item {
                key: Identifier::vanilla_static("netherite_boots"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(481i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            flint: Item {
                key: Identifier::vanilla_static("flint"),
                components: DataComponentMap::common_item_components(),
            },
            porkchop: Item {
                key: Identifier::vanilla_static("porkchop"),
                components: DataComponentMap::common_item_components(),
            },
            cooked_porkchop: Item {
                key: Identifier::vanilla_static("cooked_porkchop"),
                components: DataComponentMap::common_item_components(),
            },
            painting: Item {
                key: Identifier::vanilla_static("painting"),
                components: DataComponentMap::common_item_components(),
            },
            golden_apple: Item {
                key: Identifier::vanilla_static("golden_apple"),
                components: DataComponentMap::common_item_components(),
            },
            enchanted_golden_apple: Item {
                key: Identifier::vanilla_static("enchanted_golden_apple"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::ENCHANTMENT_GLINT_OVERRIDE, Some(true)),
            },
            oak_sign: Item::from_block(&*vanilla_blocks::OAK_SIGN),
            spruce_sign: Item::from_block(&*vanilla_blocks::SPRUCE_SIGN),
            birch_sign: Item::from_block(&*vanilla_blocks::BIRCH_SIGN),
            jungle_sign: Item::from_block(&*vanilla_blocks::JUNGLE_SIGN),
            acacia_sign: Item::from_block(&*vanilla_blocks::ACACIA_SIGN),
            cherry_sign: Item::from_block(&*vanilla_blocks::CHERRY_SIGN),
            dark_oak_sign: Item::from_block(&*vanilla_blocks::DARK_OAK_SIGN),
            pale_oak_sign: Item::from_block(&*vanilla_blocks::PALE_OAK_SIGN),
            mangrove_sign: Item::from_block(&*vanilla_blocks::MANGROVE_SIGN),
            bamboo_sign: Item::from_block(&*vanilla_blocks::BAMBOO_SIGN),
            crimson_sign: Item::from_block(&*vanilla_blocks::CRIMSON_SIGN),
            warped_sign: Item::from_block(&*vanilla_blocks::WARPED_SIGN),
            oak_hanging_sign: Item::from_block(&*vanilla_blocks::OAK_HANGING_SIGN),
            spruce_hanging_sign: Item::from_block(&*vanilla_blocks::SPRUCE_HANGING_SIGN),
            birch_hanging_sign: Item::from_block(&*vanilla_blocks::BIRCH_HANGING_SIGN),
            jungle_hanging_sign: Item::from_block(&*vanilla_blocks::JUNGLE_HANGING_SIGN),
            acacia_hanging_sign: Item::from_block(&*vanilla_blocks::ACACIA_HANGING_SIGN),
            cherry_hanging_sign: Item::from_block(&*vanilla_blocks::CHERRY_HANGING_SIGN),
            dark_oak_hanging_sign: Item::from_block(&*vanilla_blocks::DARK_OAK_HANGING_SIGN),
            pale_oak_hanging_sign: Item::from_block(&*vanilla_blocks::PALE_OAK_HANGING_SIGN),
            mangrove_hanging_sign: Item::from_block(&*vanilla_blocks::MANGROVE_HANGING_SIGN),
            bamboo_hanging_sign: Item::from_block(&*vanilla_blocks::BAMBOO_HANGING_SIGN),
            crimson_hanging_sign: Item::from_block(&*vanilla_blocks::CRIMSON_HANGING_SIGN),
            warped_hanging_sign: Item::from_block(&*vanilla_blocks::WARPED_HANGING_SIGN),
            bucket: Item {
                key: Identifier::vanilla_static("bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            water_bucket: Item {
                key: Identifier::vanilla_static("water_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            lava_bucket: Item {
                key: Identifier::vanilla_static("lava_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            powder_snow_bucket: Item::from_block_custom_name(
                &*vanilla_blocks::POWDER_SNOW,
                "powder_snow_bucket",
            ),
            snowball: Item {
                key: Identifier::vanilla_static("snowball"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            leather: Item {
                key: Identifier::vanilla_static("leather"),
                components: DataComponentMap::common_item_components(),
            },
            milk_bucket: Item {
                key: Identifier::vanilla_static("milk_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            pufferfish_bucket: Item {
                key: Identifier::vanilla_static("pufferfish_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            salmon_bucket: Item {
                key: Identifier::vanilla_static("salmon_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            cod_bucket: Item {
                key: Identifier::vanilla_static("cod_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            tropical_fish_bucket: Item {
                key: Identifier::vanilla_static("tropical_fish_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            axolotl_bucket: Item {
                key: Identifier::vanilla_static("axolotl_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            tadpole_bucket: Item {
                key: Identifier::vanilla_static("tadpole_bucket"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            brick: Item {
                key: Identifier::vanilla_static("brick"),
                components: DataComponentMap::common_item_components(),
            },
            clay_ball: Item {
                key: Identifier::vanilla_static("clay_ball"),
                components: DataComponentMap::common_item_components(),
            },
            dried_kelp_block: Item::from_block(&*vanilla_blocks::DRIED_KELP_BLOCK),
            paper: Item {
                key: Identifier::vanilla_static("paper"),
                components: DataComponentMap::common_item_components(),
            },
            book: Item {
                key: Identifier::vanilla_static("book"),
                components: DataComponentMap::common_item_components(),
            },
            slime_ball: Item {
                key: Identifier::vanilla_static("slime_ball"),
                components: DataComponentMap::common_item_components(),
            },
            egg: Item {
                key: Identifier::vanilla_static("egg"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            blue_egg: Item {
                key: Identifier::vanilla_static("blue_egg"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            brown_egg: Item {
                key: Identifier::vanilla_static("brown_egg"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            compass: Item {
                key: Identifier::vanilla_static("compass"),
                components: DataComponentMap::common_item_components(),
            },
            recovery_compass: Item {
                key: Identifier::vanilla_static("recovery_compass"),
                components: DataComponentMap::common_item_components(),
            },
            bundle: Item {
                key: Identifier::vanilla_static("bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            white_bundle: Item {
                key: Identifier::vanilla_static("white_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            orange_bundle: Item {
                key: Identifier::vanilla_static("orange_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            magenta_bundle: Item {
                key: Identifier::vanilla_static("magenta_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            light_blue_bundle: Item {
                key: Identifier::vanilla_static("light_blue_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            yellow_bundle: Item {
                key: Identifier::vanilla_static("yellow_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            lime_bundle: Item {
                key: Identifier::vanilla_static("lime_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            pink_bundle: Item {
                key: Identifier::vanilla_static("pink_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            gray_bundle: Item {
                key: Identifier::vanilla_static("gray_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            light_gray_bundle: Item {
                key: Identifier::vanilla_static("light_gray_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            cyan_bundle: Item {
                key: Identifier::vanilla_static("cyan_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            purple_bundle: Item {
                key: Identifier::vanilla_static("purple_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            blue_bundle: Item {
                key: Identifier::vanilla_static("blue_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            brown_bundle: Item {
                key: Identifier::vanilla_static("brown_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            green_bundle: Item {
                key: Identifier::vanilla_static("green_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            red_bundle: Item {
                key: Identifier::vanilla_static("red_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            black_bundle: Item {
                key: Identifier::vanilla_static("black_bundle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            fishing_rod: Item {
                key: Identifier::vanilla_static("fishing_rod"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(64i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            clock: Item {
                key: Identifier::vanilla_static("clock"),
                components: DataComponentMap::common_item_components(),
            },
            spyglass: Item {
                key: Identifier::vanilla_static("spyglass"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            glowstone_dust: Item {
                key: Identifier::vanilla_static("glowstone_dust"),
                components: DataComponentMap::common_item_components(),
            },
            cod: Item {
                key: Identifier::vanilla_static("cod"),
                components: DataComponentMap::common_item_components(),
            },
            salmon: Item {
                key: Identifier::vanilla_static("salmon"),
                components: DataComponentMap::common_item_components(),
            },
            tropical_fish: Item {
                key: Identifier::vanilla_static("tropical_fish"),
                components: DataComponentMap::common_item_components(),
            },
            pufferfish: Item {
                key: Identifier::vanilla_static("pufferfish"),
                components: DataComponentMap::common_item_components(),
            },
            cooked_cod: Item {
                key: Identifier::vanilla_static("cooked_cod"),
                components: DataComponentMap::common_item_components(),
            },
            cooked_salmon: Item {
                key: Identifier::vanilla_static("cooked_salmon"),
                components: DataComponentMap::common_item_components(),
            },
            ink_sac: Item {
                key: Identifier::vanilla_static("ink_sac"),
                components: DataComponentMap::common_item_components(),
            },
            glow_ink_sac: Item {
                key: Identifier::vanilla_static("glow_ink_sac"),
                components: DataComponentMap::common_item_components(),
            },
            cocoa_beans: Item::from_block_custom_name(&*vanilla_blocks::COCOA, "cocoa_beans"),
            white_dye: Item {
                key: Identifier::vanilla_static("white_dye"),
                components: DataComponentMap::common_item_components(),
            },
            orange_dye: Item {
                key: Identifier::vanilla_static("orange_dye"),
                components: DataComponentMap::common_item_components(),
            },
            magenta_dye: Item {
                key: Identifier::vanilla_static("magenta_dye"),
                components: DataComponentMap::common_item_components(),
            },
            light_blue_dye: Item {
                key: Identifier::vanilla_static("light_blue_dye"),
                components: DataComponentMap::common_item_components(),
            },
            yellow_dye: Item {
                key: Identifier::vanilla_static("yellow_dye"),
                components: DataComponentMap::common_item_components(),
            },
            lime_dye: Item {
                key: Identifier::vanilla_static("lime_dye"),
                components: DataComponentMap::common_item_components(),
            },
            pink_dye: Item {
                key: Identifier::vanilla_static("pink_dye"),
                components: DataComponentMap::common_item_components(),
            },
            gray_dye: Item {
                key: Identifier::vanilla_static("gray_dye"),
                components: DataComponentMap::common_item_components(),
            },
            light_gray_dye: Item {
                key: Identifier::vanilla_static("light_gray_dye"),
                components: DataComponentMap::common_item_components(),
            },
            cyan_dye: Item {
                key: Identifier::vanilla_static("cyan_dye"),
                components: DataComponentMap::common_item_components(),
            },
            purple_dye: Item {
                key: Identifier::vanilla_static("purple_dye"),
                components: DataComponentMap::common_item_components(),
            },
            blue_dye: Item {
                key: Identifier::vanilla_static("blue_dye"),
                components: DataComponentMap::common_item_components(),
            },
            brown_dye: Item {
                key: Identifier::vanilla_static("brown_dye"),
                components: DataComponentMap::common_item_components(),
            },
            green_dye: Item {
                key: Identifier::vanilla_static("green_dye"),
                components: DataComponentMap::common_item_components(),
            },
            red_dye: Item {
                key: Identifier::vanilla_static("red_dye"),
                components: DataComponentMap::common_item_components(),
            },
            black_dye: Item {
                key: Identifier::vanilla_static("black_dye"),
                components: DataComponentMap::common_item_components(),
            },
            bone_meal: Item {
                key: Identifier::vanilla_static("bone_meal"),
                components: DataComponentMap::common_item_components(),
            },
            bone: Item {
                key: Identifier::vanilla_static("bone"),
                components: DataComponentMap::common_item_components(),
            },
            sugar: Item {
                key: Identifier::vanilla_static("sugar"),
                components: DataComponentMap::common_item_components(),
            },
            cake: Item::from_block(&*vanilla_blocks::CAKE),
            white_bed: Item::from_block(&*vanilla_blocks::WHITE_BED),
            orange_bed: Item::from_block(&*vanilla_blocks::ORANGE_BED),
            magenta_bed: Item::from_block(&*vanilla_blocks::MAGENTA_BED),
            light_blue_bed: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_BED),
            yellow_bed: Item::from_block(&*vanilla_blocks::YELLOW_BED),
            lime_bed: Item::from_block(&*vanilla_blocks::LIME_BED),
            pink_bed: Item::from_block(&*vanilla_blocks::PINK_BED),
            gray_bed: Item::from_block(&*vanilla_blocks::GRAY_BED),
            light_gray_bed: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_BED),
            cyan_bed: Item::from_block(&*vanilla_blocks::CYAN_BED),
            purple_bed: Item::from_block(&*vanilla_blocks::PURPLE_BED),
            blue_bed: Item::from_block(&*vanilla_blocks::BLUE_BED),
            brown_bed: Item::from_block(&*vanilla_blocks::BROWN_BED),
            green_bed: Item::from_block(&*vanilla_blocks::GREEN_BED),
            red_bed: Item::from_block(&*vanilla_blocks::RED_BED),
            black_bed: Item::from_block(&*vanilla_blocks::BLACK_BED),
            cookie: Item {
                key: Identifier::vanilla_static("cookie"),
                components: DataComponentMap::common_item_components(),
            },
            crafter: Item::from_block(&*vanilla_blocks::CRAFTER),
            filled_map: Item {
                key: Identifier::vanilla_static("filled_map"),
                components: DataComponentMap::common_item_components(),
            },
            shears: Item {
                key: Identifier::vanilla_static("shears"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(238i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            melon_slice: Item {
                key: Identifier::vanilla_static("melon_slice"),
                components: DataComponentMap::common_item_components(),
            },
            dried_kelp: Item {
                key: Identifier::vanilla_static("dried_kelp"),
                components: DataComponentMap::common_item_components(),
            },
            pumpkin_seeds: Item::from_block_custom_name(
                &*vanilla_blocks::PUMPKIN_STEM,
                "pumpkin_seeds",
            ),
            melon_seeds: Item::from_block_custom_name(&*vanilla_blocks::MELON_STEM, "melon_seeds"),
            beef: Item {
                key: Identifier::vanilla_static("beef"),
                components: DataComponentMap::common_item_components(),
            },
            cooked_beef: Item {
                key: Identifier::vanilla_static("cooked_beef"),
                components: DataComponentMap::common_item_components(),
            },
            chicken: Item {
                key: Identifier::vanilla_static("chicken"),
                components: DataComponentMap::common_item_components(),
            },
            cooked_chicken: Item {
                key: Identifier::vanilla_static("cooked_chicken"),
                components: DataComponentMap::common_item_components(),
            },
            rotten_flesh: Item {
                key: Identifier::vanilla_static("rotten_flesh"),
                components: DataComponentMap::common_item_components(),
            },
            ender_pearl: Item {
                key: Identifier::vanilla_static("ender_pearl"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            blaze_rod: Item {
                key: Identifier::vanilla_static("blaze_rod"),
                components: DataComponentMap::common_item_components(),
            },
            ghast_tear: Item {
                key: Identifier::vanilla_static("ghast_tear"),
                components: DataComponentMap::common_item_components(),
            },
            gold_nugget: Item {
                key: Identifier::vanilla_static("gold_nugget"),
                components: DataComponentMap::common_item_components(),
            },
            nether_wart: Item::from_block(&*vanilla_blocks::NETHER_WART),
            glass_bottle: Item {
                key: Identifier::vanilla_static("glass_bottle"),
                components: DataComponentMap::common_item_components(),
            },
            potion: Item {
                key: Identifier::vanilla_static("potion"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            spider_eye: Item {
                key: Identifier::vanilla_static("spider_eye"),
                components: DataComponentMap::common_item_components(),
            },
            fermented_spider_eye: Item {
                key: Identifier::vanilla_static("fermented_spider_eye"),
                components: DataComponentMap::common_item_components(),
            },
            blaze_powder: Item {
                key: Identifier::vanilla_static("blaze_powder"),
                components: DataComponentMap::common_item_components(),
            },
            magma_cream: Item {
                key: Identifier::vanilla_static("magma_cream"),
                components: DataComponentMap::common_item_components(),
            },
            brewing_stand: Item::from_block(&*vanilla_blocks::BREWING_STAND),
            cauldron: Item::from_block(&*vanilla_blocks::CAULDRON),
            ender_eye: Item {
                key: Identifier::vanilla_static("ender_eye"),
                components: DataComponentMap::common_item_components(),
            },
            glistering_melon_slice: Item {
                key: Identifier::vanilla_static("glistering_melon_slice"),
                components: DataComponentMap::common_item_components(),
            },
            armadillo_spawn_egg: Item {
                key: Identifier::vanilla_static("armadillo_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            allay_spawn_egg: Item {
                key: Identifier::vanilla_static("allay_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            axolotl_spawn_egg: Item {
                key: Identifier::vanilla_static("axolotl_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            bat_spawn_egg: Item {
                key: Identifier::vanilla_static("bat_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            bee_spawn_egg: Item {
                key: Identifier::vanilla_static("bee_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            blaze_spawn_egg: Item {
                key: Identifier::vanilla_static("blaze_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            bogged_spawn_egg: Item {
                key: Identifier::vanilla_static("bogged_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            breeze_spawn_egg: Item {
                key: Identifier::vanilla_static("breeze_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            cat_spawn_egg: Item {
                key: Identifier::vanilla_static("cat_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            camel_spawn_egg: Item {
                key: Identifier::vanilla_static("camel_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            cave_spider_spawn_egg: Item {
                key: Identifier::vanilla_static("cave_spider_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            chicken_spawn_egg: Item {
                key: Identifier::vanilla_static("chicken_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            cod_spawn_egg: Item {
                key: Identifier::vanilla_static("cod_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            copper_golem_spawn_egg: Item {
                key: Identifier::vanilla_static("copper_golem_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            cow_spawn_egg: Item {
                key: Identifier::vanilla_static("cow_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            creeper_spawn_egg: Item {
                key: Identifier::vanilla_static("creeper_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            dolphin_spawn_egg: Item {
                key: Identifier::vanilla_static("dolphin_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            donkey_spawn_egg: Item {
                key: Identifier::vanilla_static("donkey_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            drowned_spawn_egg: Item {
                key: Identifier::vanilla_static("drowned_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            elder_guardian_spawn_egg: Item {
                key: Identifier::vanilla_static("elder_guardian_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            ender_dragon_spawn_egg: Item {
                key: Identifier::vanilla_static("ender_dragon_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            enderman_spawn_egg: Item {
                key: Identifier::vanilla_static("enderman_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            endermite_spawn_egg: Item {
                key: Identifier::vanilla_static("endermite_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            evoker_spawn_egg: Item {
                key: Identifier::vanilla_static("evoker_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            fox_spawn_egg: Item {
                key: Identifier::vanilla_static("fox_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            frog_spawn_egg: Item {
                key: Identifier::vanilla_static("frog_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            ghast_spawn_egg: Item {
                key: Identifier::vanilla_static("ghast_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            happy_ghast_spawn_egg: Item {
                key: Identifier::vanilla_static("happy_ghast_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            glow_squid_spawn_egg: Item {
                key: Identifier::vanilla_static("glow_squid_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            goat_spawn_egg: Item {
                key: Identifier::vanilla_static("goat_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            guardian_spawn_egg: Item {
                key: Identifier::vanilla_static("guardian_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            hoglin_spawn_egg: Item {
                key: Identifier::vanilla_static("hoglin_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            horse_spawn_egg: Item {
                key: Identifier::vanilla_static("horse_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            husk_spawn_egg: Item {
                key: Identifier::vanilla_static("husk_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            iron_golem_spawn_egg: Item {
                key: Identifier::vanilla_static("iron_golem_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            llama_spawn_egg: Item {
                key: Identifier::vanilla_static("llama_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            magma_cube_spawn_egg: Item {
                key: Identifier::vanilla_static("magma_cube_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            mooshroom_spawn_egg: Item {
                key: Identifier::vanilla_static("mooshroom_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            mule_spawn_egg: Item {
                key: Identifier::vanilla_static("mule_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            ocelot_spawn_egg: Item {
                key: Identifier::vanilla_static("ocelot_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            panda_spawn_egg: Item {
                key: Identifier::vanilla_static("panda_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            parrot_spawn_egg: Item {
                key: Identifier::vanilla_static("parrot_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            phantom_spawn_egg: Item {
                key: Identifier::vanilla_static("phantom_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            pig_spawn_egg: Item {
                key: Identifier::vanilla_static("pig_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            piglin_spawn_egg: Item {
                key: Identifier::vanilla_static("piglin_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            piglin_brute_spawn_egg: Item {
                key: Identifier::vanilla_static("piglin_brute_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            pillager_spawn_egg: Item {
                key: Identifier::vanilla_static("pillager_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            polar_bear_spawn_egg: Item {
                key: Identifier::vanilla_static("polar_bear_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            pufferfish_spawn_egg: Item {
                key: Identifier::vanilla_static("pufferfish_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            rabbit_spawn_egg: Item {
                key: Identifier::vanilla_static("rabbit_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            ravager_spawn_egg: Item {
                key: Identifier::vanilla_static("ravager_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            salmon_spawn_egg: Item {
                key: Identifier::vanilla_static("salmon_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            sheep_spawn_egg: Item {
                key: Identifier::vanilla_static("sheep_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            shulker_spawn_egg: Item {
                key: Identifier::vanilla_static("shulker_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            silverfish_spawn_egg: Item {
                key: Identifier::vanilla_static("silverfish_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            skeleton_spawn_egg: Item {
                key: Identifier::vanilla_static("skeleton_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            skeleton_horse_spawn_egg: Item {
                key: Identifier::vanilla_static("skeleton_horse_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            slime_spawn_egg: Item {
                key: Identifier::vanilla_static("slime_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            sniffer_spawn_egg: Item {
                key: Identifier::vanilla_static("sniffer_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            snow_golem_spawn_egg: Item {
                key: Identifier::vanilla_static("snow_golem_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            spider_spawn_egg: Item {
                key: Identifier::vanilla_static("spider_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            squid_spawn_egg: Item {
                key: Identifier::vanilla_static("squid_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            stray_spawn_egg: Item {
                key: Identifier::vanilla_static("stray_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            strider_spawn_egg: Item {
                key: Identifier::vanilla_static("strider_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            tadpole_spawn_egg: Item {
                key: Identifier::vanilla_static("tadpole_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            trader_llama_spawn_egg: Item {
                key: Identifier::vanilla_static("trader_llama_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            tropical_fish_spawn_egg: Item {
                key: Identifier::vanilla_static("tropical_fish_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            turtle_spawn_egg: Item {
                key: Identifier::vanilla_static("turtle_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            vex_spawn_egg: Item {
                key: Identifier::vanilla_static("vex_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            villager_spawn_egg: Item {
                key: Identifier::vanilla_static("villager_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            vindicator_spawn_egg: Item {
                key: Identifier::vanilla_static("vindicator_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            wandering_trader_spawn_egg: Item {
                key: Identifier::vanilla_static("wandering_trader_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            warden_spawn_egg: Item {
                key: Identifier::vanilla_static("warden_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            witch_spawn_egg: Item {
                key: Identifier::vanilla_static("witch_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            wither_spawn_egg: Item {
                key: Identifier::vanilla_static("wither_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            wither_skeleton_spawn_egg: Item {
                key: Identifier::vanilla_static("wither_skeleton_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            wolf_spawn_egg: Item {
                key: Identifier::vanilla_static("wolf_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            zoglin_spawn_egg: Item {
                key: Identifier::vanilla_static("zoglin_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            creaking_spawn_egg: Item {
                key: Identifier::vanilla_static("creaking_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            zombie_spawn_egg: Item {
                key: Identifier::vanilla_static("zombie_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            zombie_horse_spawn_egg: Item {
                key: Identifier::vanilla_static("zombie_horse_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            zombie_villager_spawn_egg: Item {
                key: Identifier::vanilla_static("zombie_villager_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            zombified_piglin_spawn_egg: Item {
                key: Identifier::vanilla_static("zombified_piglin_spawn_egg"),
                components: DataComponentMap::common_item_components(),
            },
            experience_bottle: Item {
                key: Identifier::vanilla_static("experience_bottle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::ENCHANTMENT_GLINT_OVERRIDE, Some(true)),
            },
            fire_charge: Item {
                key: Identifier::vanilla_static("fire_charge"),
                components: DataComponentMap::common_item_components(),
            },
            wind_charge: Item {
                key: Identifier::vanilla_static("wind_charge"),
                components: DataComponentMap::common_item_components(),
            },
            writable_book: Item {
                key: Identifier::vanilla_static("writable_book"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            written_book: Item {
                key: Identifier::vanilla_static("written_book"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::ENCHANTMENT_GLINT_OVERRIDE, Some(true))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            breeze_rod: Item {
                key: Identifier::vanilla_static("breeze_rod"),
                components: DataComponentMap::common_item_components(),
            },
            mace: Item {
                key: Identifier::vanilla_static("mace"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(500i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            item_frame: Item {
                key: Identifier::vanilla_static("item_frame"),
                components: DataComponentMap::common_item_components(),
            },
            glow_item_frame: Item {
                key: Identifier::vanilla_static("glow_item_frame"),
                components: DataComponentMap::common_item_components(),
            },
            flower_pot: Item::from_block(&*vanilla_blocks::FLOWER_POT),
            carrot: Item::from_block_custom_name(&*vanilla_blocks::CARROTS, "carrot"),
            potato: Item::from_block_custom_name(&*vanilla_blocks::POTATOES, "potato"),
            baked_potato: Item {
                key: Identifier::vanilla_static("baked_potato"),
                components: DataComponentMap::common_item_components(),
            },
            poisonous_potato: Item {
                key: Identifier::vanilla_static("poisonous_potato"),
                components: DataComponentMap::common_item_components(),
            },
            map: Item {
                key: Identifier::vanilla_static("map"),
                components: DataComponentMap::common_item_components(),
            },
            golden_carrot: Item {
                key: Identifier::vanilla_static("golden_carrot"),
                components: DataComponentMap::common_item_components(),
            },
            skeleton_skull: Item::from_block(&*vanilla_blocks::SKELETON_SKULL),
            wither_skeleton_skull: Item::from_block(&*vanilla_blocks::WITHER_SKELETON_SKULL),
            player_head: Item::from_block(&*vanilla_blocks::PLAYER_HEAD),
            zombie_head: Item::from_block(&*vanilla_blocks::ZOMBIE_HEAD),
            creeper_head: Item::from_block(&*vanilla_blocks::CREEPER_HEAD),
            dragon_head: Item::from_block(&*vanilla_blocks::DRAGON_HEAD),
            piglin_head: Item::from_block(&*vanilla_blocks::PIGLIN_HEAD),
            nether_star: Item {
                key: Identifier::vanilla_static("nether_star"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::ENCHANTMENT_GLINT_OVERRIDE, Some(true)),
            },
            pumpkin_pie: Item {
                key: Identifier::vanilla_static("pumpkin_pie"),
                components: DataComponentMap::common_item_components(),
            },
            firework_rocket: Item {
                key: Identifier::vanilla_static("firework_rocket"),
                components: DataComponentMap::common_item_components(),
            },
            firework_star: Item {
                key: Identifier::vanilla_static("firework_star"),
                components: DataComponentMap::common_item_components(),
            },
            enchanted_book: Item {
                key: Identifier::vanilla_static("enchanted_book"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::ENCHANTMENT_GLINT_OVERRIDE, Some(true))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            nether_brick: Item {
                key: Identifier::vanilla_static("nether_brick"),
                components: DataComponentMap::common_item_components(),
            },
            resin_brick: Item {
                key: Identifier::vanilla_static("resin_brick"),
                components: DataComponentMap::common_item_components(),
            },
            prismarine_shard: Item {
                key: Identifier::vanilla_static("prismarine_shard"),
                components: DataComponentMap::common_item_components(),
            },
            prismarine_crystals: Item {
                key: Identifier::vanilla_static("prismarine_crystals"),
                components: DataComponentMap::common_item_components(),
            },
            rabbit: Item {
                key: Identifier::vanilla_static("rabbit"),
                components: DataComponentMap::common_item_components(),
            },
            cooked_rabbit: Item {
                key: Identifier::vanilla_static("cooked_rabbit"),
                components: DataComponentMap::common_item_components(),
            },
            rabbit_stew: Item {
                key: Identifier::vanilla_static("rabbit_stew"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            rabbit_foot: Item {
                key: Identifier::vanilla_static("rabbit_foot"),
                components: DataComponentMap::common_item_components(),
            },
            rabbit_hide: Item {
                key: Identifier::vanilla_static("rabbit_hide"),
                components: DataComponentMap::common_item_components(),
            },
            armor_stand: Item {
                key: Identifier::vanilla_static("armor_stand"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            copper_horse_armor: Item {
                key: Identifier::vanilla_static("copper_horse_armor"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            iron_horse_armor: Item {
                key: Identifier::vanilla_static("iron_horse_armor"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            golden_horse_armor: Item {
                key: Identifier::vanilla_static("golden_horse_armor"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            diamond_horse_armor: Item {
                key: Identifier::vanilla_static("diamond_horse_armor"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            leather_horse_armor: Item {
                key: Identifier::vanilla_static("leather_horse_armor"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            lead: Item {
                key: Identifier::vanilla_static("lead"),
                components: DataComponentMap::common_item_components(),
            },
            name_tag: Item {
                key: Identifier::vanilla_static("name_tag"),
                components: DataComponentMap::common_item_components(),
            },
            command_block_minecart: Item {
                key: Identifier::vanilla_static("command_block_minecart"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            mutton: Item {
                key: Identifier::vanilla_static("mutton"),
                components: DataComponentMap::common_item_components(),
            },
            cooked_mutton: Item {
                key: Identifier::vanilla_static("cooked_mutton"),
                components: DataComponentMap::common_item_components(),
            },
            white_banner: Item::from_block(&*vanilla_blocks::WHITE_BANNER),
            orange_banner: Item::from_block(&*vanilla_blocks::ORANGE_BANNER),
            magenta_banner: Item::from_block(&*vanilla_blocks::MAGENTA_BANNER),
            light_blue_banner: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_BANNER),
            yellow_banner: Item::from_block(&*vanilla_blocks::YELLOW_BANNER),
            lime_banner: Item::from_block(&*vanilla_blocks::LIME_BANNER),
            pink_banner: Item::from_block(&*vanilla_blocks::PINK_BANNER),
            gray_banner: Item::from_block(&*vanilla_blocks::GRAY_BANNER),
            light_gray_banner: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_BANNER),
            cyan_banner: Item::from_block(&*vanilla_blocks::CYAN_BANNER),
            purple_banner: Item::from_block(&*vanilla_blocks::PURPLE_BANNER),
            blue_banner: Item::from_block(&*vanilla_blocks::BLUE_BANNER),
            brown_banner: Item::from_block(&*vanilla_blocks::BROWN_BANNER),
            green_banner: Item::from_block(&*vanilla_blocks::GREEN_BANNER),
            red_banner: Item::from_block(&*vanilla_blocks::RED_BANNER),
            black_banner: Item::from_block(&*vanilla_blocks::BLACK_BANNER),
            end_crystal: Item {
                key: Identifier::vanilla_static("end_crystal"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::ENCHANTMENT_GLINT_OVERRIDE, Some(true)),
            },
            chorus_fruit: Item {
                key: Identifier::vanilla_static("chorus_fruit"),
                components: DataComponentMap::common_item_components(),
            },
            popped_chorus_fruit: Item {
                key: Identifier::vanilla_static("popped_chorus_fruit"),
                components: DataComponentMap::common_item_components(),
            },
            torchflower_seeds: Item::from_block_custom_name(
                &*vanilla_blocks::TORCHFLOWER_CROP,
                "torchflower_seeds",
            ),
            pitcher_pod: Item::from_block_custom_name(
                &*vanilla_blocks::PITCHER_CROP,
                "pitcher_pod",
            ),
            beetroot: Item {
                key: Identifier::vanilla_static("beetroot"),
                components: DataComponentMap::common_item_components(),
            },
            beetroot_seeds: Item::from_block_custom_name(
                &*vanilla_blocks::BEETROOTS,
                "beetroot_seeds",
            ),
            beetroot_soup: Item {
                key: Identifier::vanilla_static("beetroot_soup"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            dragon_breath: Item {
                key: Identifier::vanilla_static("dragon_breath"),
                components: DataComponentMap::common_item_components(),
            },
            splash_potion: Item {
                key: Identifier::vanilla_static("splash_potion"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            spectral_arrow: Item {
                key: Identifier::vanilla_static("spectral_arrow"),
                components: DataComponentMap::common_item_components(),
            },
            tipped_arrow: Item {
                key: Identifier::vanilla_static("tipped_arrow"),
                components: DataComponentMap::common_item_components(),
            },
            lingering_potion: Item {
                key: Identifier::vanilla_static("lingering_potion"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            shield: Item {
                key: Identifier::vanilla_static("shield"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(336i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            totem_of_undying: Item {
                key: Identifier::vanilla_static("totem_of_undying"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            shulker_shell: Item {
                key: Identifier::vanilla_static("shulker_shell"),
                components: DataComponentMap::common_item_components(),
            },
            iron_nugget: Item {
                key: Identifier::vanilla_static("iron_nugget"),
                components: DataComponentMap::common_item_components(),
            },
            copper_nugget: Item {
                key: Identifier::vanilla_static("copper_nugget"),
                components: DataComponentMap::common_item_components(),
            },
            knowledge_book: Item {
                key: Identifier::vanilla_static("knowledge_book"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            debug_stick: Item {
                key: Identifier::vanilla_static("debug_stick"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::ENCHANTMENT_GLINT_OVERRIDE, Some(true))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_13: Item {
                key: Identifier::vanilla_static("music_disc_13"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_cat: Item {
                key: Identifier::vanilla_static("music_disc_cat"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_blocks: Item {
                key: Identifier::vanilla_static("music_disc_blocks"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_chirp: Item {
                key: Identifier::vanilla_static("music_disc_chirp"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_creator: Item {
                key: Identifier::vanilla_static("music_disc_creator"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_creator_music_box: Item {
                key: Identifier::vanilla_static("music_disc_creator_music_box"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_far: Item {
                key: Identifier::vanilla_static("music_disc_far"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_lava_chicken: Item {
                key: Identifier::vanilla_static("music_disc_lava_chicken"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_mall: Item {
                key: Identifier::vanilla_static("music_disc_mall"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_mellohi: Item {
                key: Identifier::vanilla_static("music_disc_mellohi"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_stal: Item {
                key: Identifier::vanilla_static("music_disc_stal"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_strad: Item {
                key: Identifier::vanilla_static("music_disc_strad"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_ward: Item {
                key: Identifier::vanilla_static("music_disc_ward"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_11: Item {
                key: Identifier::vanilla_static("music_disc_11"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_wait: Item {
                key: Identifier::vanilla_static("music_disc_wait"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_otherside: Item {
                key: Identifier::vanilla_static("music_disc_otherside"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_relic: Item {
                key: Identifier::vanilla_static("music_disc_relic"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_5: Item {
                key: Identifier::vanilla_static("music_disc_5"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_pigstep: Item {
                key: Identifier::vanilla_static("music_disc_pigstep"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_precipice: Item {
                key: Identifier::vanilla_static("music_disc_precipice"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            music_disc_tears: Item {
                key: Identifier::vanilla_static("music_disc_tears"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            disc_fragment_5: Item {
                key: Identifier::vanilla_static("disc_fragment_5"),
                components: DataComponentMap::common_item_components(),
            },
            trident: Item {
                key: Identifier::vanilla_static("trident"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(250i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            nautilus_shell: Item {
                key: Identifier::vanilla_static("nautilus_shell"),
                components: DataComponentMap::common_item_components(),
            },
            heart_of_the_sea: Item {
                key: Identifier::vanilla_static("heart_of_the_sea"),
                components: DataComponentMap::common_item_components(),
            },
            crossbow: Item {
                key: Identifier::vanilla_static("crossbow"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(465i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            suspicious_stew: Item {
                key: Identifier::vanilla_static("suspicious_stew"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            loom: Item::from_block(&*vanilla_blocks::LOOM),
            flower_banner_pattern: Item {
                key: Identifier::vanilla_static("flower_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            creeper_banner_pattern: Item {
                key: Identifier::vanilla_static("creeper_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            skull_banner_pattern: Item {
                key: Identifier::vanilla_static("skull_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            mojang_banner_pattern: Item {
                key: Identifier::vanilla_static("mojang_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            globe_banner_pattern: Item {
                key: Identifier::vanilla_static("globe_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            piglin_banner_pattern: Item {
                key: Identifier::vanilla_static("piglin_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            flow_banner_pattern: Item {
                key: Identifier::vanilla_static("flow_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            guster_banner_pattern: Item {
                key: Identifier::vanilla_static("guster_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            field_masoned_banner_pattern: Item {
                key: Identifier::vanilla_static("field_masoned_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            bordure_indented_banner_pattern: Item {
                key: Identifier::vanilla_static("bordure_indented_banner_pattern"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            goat_horn: Item {
                key: Identifier::vanilla_static("goat_horn"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            composter: Item::from_block(&*vanilla_blocks::COMPOSTER),
            barrel: Item::from_block(&*vanilla_blocks::BARREL),
            smoker: Item::from_block(&*vanilla_blocks::SMOKER),
            blast_furnace: Item::from_block(&*vanilla_blocks::BLAST_FURNACE),
            cartography_table: Item::from_block(&*vanilla_blocks::CARTOGRAPHY_TABLE),
            fletching_table: Item::from_block(&*vanilla_blocks::FLETCHING_TABLE),
            grindstone: Item::from_block(&*vanilla_blocks::GRINDSTONE),
            smithing_table: Item::from_block(&*vanilla_blocks::SMITHING_TABLE),
            stonecutter: Item::from_block(&*vanilla_blocks::STONECUTTER),
            bell: Item::from_block(&*vanilla_blocks::BELL),
            lantern: Item::from_block(&*vanilla_blocks::LANTERN),
            soul_lantern: Item::from_block(&*vanilla_blocks::SOUL_LANTERN),
            copper_lantern: Item::from_block(&*vanilla_blocks::COPPER_LANTERN),
            exposed_copper_lantern: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER_LANTERN),
            weathered_copper_lantern: Item::from_block(&*vanilla_blocks::WEATHERED_COPPER_LANTERN),
            oxidized_copper_lantern: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER_LANTERN),
            waxed_copper_lantern: Item::from_block(&*vanilla_blocks::WAXED_COPPER_LANTERN),
            waxed_exposed_copper_lantern: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_LANTERN,
            ),
            waxed_weathered_copper_lantern: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_LANTERN,
            ),
            waxed_oxidized_copper_lantern: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_LANTERN,
            ),
            sweet_berries: Item::from_block_custom_name(
                &*vanilla_blocks::SWEET_BERRY_BUSH,
                "sweet_berries",
            ),
            glow_berries: Item::from_block_custom_name(
                &*vanilla_blocks::CAVE_VINES,
                "glow_berries",
            ),
            campfire: Item::from_block(&*vanilla_blocks::CAMPFIRE),
            soul_campfire: Item::from_block(&*vanilla_blocks::SOUL_CAMPFIRE),
            shroomlight: Item::from_block(&*vanilla_blocks::SHROOMLIGHT),
            honeycomb: Item {
                key: Identifier::vanilla_static("honeycomb"),
                components: DataComponentMap::common_item_components(),
            },
            bee_nest: Item::from_block(&*vanilla_blocks::BEE_NEST),
            beehive: Item::from_block(&*vanilla_blocks::BEEHIVE),
            honey_bottle: Item {
                key: Identifier::vanilla_static("honey_bottle"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(16i32)),
            },
            honeycomb_block: Item::from_block(&*vanilla_blocks::HONEYCOMB_BLOCK),
            lodestone: Item::from_block(&*vanilla_blocks::LODESTONE),
            crying_obsidian: Item::from_block(&*vanilla_blocks::CRYING_OBSIDIAN),
            blackstone: Item::from_block(&*vanilla_blocks::BLACKSTONE),
            blackstone_slab: Item::from_block(&*vanilla_blocks::BLACKSTONE_SLAB),
            blackstone_stairs: Item::from_block(&*vanilla_blocks::BLACKSTONE_STAIRS),
            gilded_blackstone: Item::from_block(&*vanilla_blocks::GILDED_BLACKSTONE),
            polished_blackstone: Item::from_block(&*vanilla_blocks::POLISHED_BLACKSTONE),
            polished_blackstone_slab: Item::from_block(&*vanilla_blocks::POLISHED_BLACKSTONE_SLAB),
            polished_blackstone_stairs: Item::from_block(
                &*vanilla_blocks::POLISHED_BLACKSTONE_STAIRS,
            ),
            chiseled_polished_blackstone: Item::from_block(
                &*vanilla_blocks::CHISELED_POLISHED_BLACKSTONE,
            ),
            polished_blackstone_bricks: Item::from_block(
                &*vanilla_blocks::POLISHED_BLACKSTONE_BRICKS,
            ),
            polished_blackstone_brick_slab: Item::from_block(
                &*vanilla_blocks::POLISHED_BLACKSTONE_BRICK_SLAB,
            ),
            polished_blackstone_brick_stairs: Item::from_block(
                &*vanilla_blocks::POLISHED_BLACKSTONE_BRICK_STAIRS,
            ),
            cracked_polished_blackstone_bricks: Item::from_block(
                &*vanilla_blocks::CRACKED_POLISHED_BLACKSTONE_BRICKS,
            ),
            respawn_anchor: Item::from_block(&*vanilla_blocks::RESPAWN_ANCHOR),
            candle: Item::from_block(&*vanilla_blocks::CANDLE),
            white_candle: Item::from_block(&*vanilla_blocks::WHITE_CANDLE),
            orange_candle: Item::from_block(&*vanilla_blocks::ORANGE_CANDLE),
            magenta_candle: Item::from_block(&*vanilla_blocks::MAGENTA_CANDLE),
            light_blue_candle: Item::from_block(&*vanilla_blocks::LIGHT_BLUE_CANDLE),
            yellow_candle: Item::from_block(&*vanilla_blocks::YELLOW_CANDLE),
            lime_candle: Item::from_block(&*vanilla_blocks::LIME_CANDLE),
            pink_candle: Item::from_block(&*vanilla_blocks::PINK_CANDLE),
            gray_candle: Item::from_block(&*vanilla_blocks::GRAY_CANDLE),
            light_gray_candle: Item::from_block(&*vanilla_blocks::LIGHT_GRAY_CANDLE),
            cyan_candle: Item::from_block(&*vanilla_blocks::CYAN_CANDLE),
            purple_candle: Item::from_block(&*vanilla_blocks::PURPLE_CANDLE),
            blue_candle: Item::from_block(&*vanilla_blocks::BLUE_CANDLE),
            brown_candle: Item::from_block(&*vanilla_blocks::BROWN_CANDLE),
            green_candle: Item::from_block(&*vanilla_blocks::GREEN_CANDLE),
            red_candle: Item::from_block(&*vanilla_blocks::RED_CANDLE),
            black_candle: Item::from_block(&*vanilla_blocks::BLACK_CANDLE),
            small_amethyst_bud: Item::from_block(&*vanilla_blocks::SMALL_AMETHYST_BUD),
            medium_amethyst_bud: Item::from_block(&*vanilla_blocks::MEDIUM_AMETHYST_BUD),
            large_amethyst_bud: Item::from_block(&*vanilla_blocks::LARGE_AMETHYST_BUD),
            amethyst_cluster: Item::from_block(&*vanilla_blocks::AMETHYST_CLUSTER),
            pointed_dripstone: Item::from_block(&*vanilla_blocks::POINTED_DRIPSTONE),
            ochre_froglight: Item::from_block(&*vanilla_blocks::OCHRE_FROGLIGHT),
            verdant_froglight: Item::from_block(&*vanilla_blocks::VERDANT_FROGLIGHT),
            pearlescent_froglight: Item::from_block(&*vanilla_blocks::PEARLESCENT_FROGLIGHT),
            frogspawn: Item::from_block(&*vanilla_blocks::FROGSPAWN),
            echo_shard: Item {
                key: Identifier::vanilla_static("echo_shard"),
                components: DataComponentMap::common_item_components(),
            },
            brush: Item {
                key: Identifier::vanilla_static("brush"),
                components: DataComponentMap::common_item_components()
                    .builder_set(vanilla_components::MAX_DAMAGE, Some(64i32))
                    .builder_set(vanilla_components::MAX_STACK_SIZE, Some(1i32)),
            },
            netherite_upgrade_smithing_template: Item {
                key: Identifier::vanilla_static("netherite_upgrade_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            sentry_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("sentry_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            dune_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("dune_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            coast_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("coast_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            wild_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("wild_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            ward_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("ward_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            eye_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("eye_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            vex_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("vex_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            tide_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("tide_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            snout_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("snout_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            rib_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("rib_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            spire_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("spire_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            wayfinder_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("wayfinder_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            shaper_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("shaper_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            silence_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("silence_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            raiser_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("raiser_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            host_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("host_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            flow_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("flow_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            bolt_armor_trim_smithing_template: Item {
                key: Identifier::vanilla_static("bolt_armor_trim_smithing_template"),
                components: DataComponentMap::common_item_components(),
            },
            angler_pottery_sherd: Item {
                key: Identifier::vanilla_static("angler_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            archer_pottery_sherd: Item {
                key: Identifier::vanilla_static("archer_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            arms_up_pottery_sherd: Item {
                key: Identifier::vanilla_static("arms_up_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            blade_pottery_sherd: Item {
                key: Identifier::vanilla_static("blade_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            brewer_pottery_sherd: Item {
                key: Identifier::vanilla_static("brewer_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            burn_pottery_sherd: Item {
                key: Identifier::vanilla_static("burn_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            danger_pottery_sherd: Item {
                key: Identifier::vanilla_static("danger_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            explorer_pottery_sherd: Item {
                key: Identifier::vanilla_static("explorer_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            flow_pottery_sherd: Item {
                key: Identifier::vanilla_static("flow_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            friend_pottery_sherd: Item {
                key: Identifier::vanilla_static("friend_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            guster_pottery_sherd: Item {
                key: Identifier::vanilla_static("guster_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            heart_pottery_sherd: Item {
                key: Identifier::vanilla_static("heart_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            heartbreak_pottery_sherd: Item {
                key: Identifier::vanilla_static("heartbreak_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            howl_pottery_sherd: Item {
                key: Identifier::vanilla_static("howl_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            miner_pottery_sherd: Item {
                key: Identifier::vanilla_static("miner_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            mourner_pottery_sherd: Item {
                key: Identifier::vanilla_static("mourner_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            plenty_pottery_sherd: Item {
                key: Identifier::vanilla_static("plenty_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            prize_pottery_sherd: Item {
                key: Identifier::vanilla_static("prize_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            scrape_pottery_sherd: Item {
                key: Identifier::vanilla_static("scrape_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            sheaf_pottery_sherd: Item {
                key: Identifier::vanilla_static("sheaf_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            shelter_pottery_sherd: Item {
                key: Identifier::vanilla_static("shelter_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            skull_pottery_sherd: Item {
                key: Identifier::vanilla_static("skull_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            snort_pottery_sherd: Item {
                key: Identifier::vanilla_static("snort_pottery_sherd"),
                components: DataComponentMap::common_item_components(),
            },
            copper_grate: Item::from_block(&*vanilla_blocks::COPPER_GRATE),
            exposed_copper_grate: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER_GRATE),
            weathered_copper_grate: Item::from_block(&*vanilla_blocks::WEATHERED_COPPER_GRATE),
            oxidized_copper_grate: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER_GRATE),
            waxed_copper_grate: Item::from_block(&*vanilla_blocks::WAXED_COPPER_GRATE),
            waxed_exposed_copper_grate: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_GRATE,
            ),
            waxed_weathered_copper_grate: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_GRATE,
            ),
            waxed_oxidized_copper_grate: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_GRATE,
            ),
            copper_bulb: Item::from_block(&*vanilla_blocks::COPPER_BULB),
            exposed_copper_bulb: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER_BULB),
            weathered_copper_bulb: Item::from_block(&*vanilla_blocks::WEATHERED_COPPER_BULB),
            oxidized_copper_bulb: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER_BULB),
            waxed_copper_bulb: Item::from_block(&*vanilla_blocks::WAXED_COPPER_BULB),
            waxed_exposed_copper_bulb: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_BULB,
            ),
            waxed_weathered_copper_bulb: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_BULB,
            ),
            waxed_oxidized_copper_bulb: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_BULB,
            ),
            copper_chest: Item::from_block(&*vanilla_blocks::COPPER_CHEST),
            exposed_copper_chest: Item::from_block(&*vanilla_blocks::EXPOSED_COPPER_CHEST),
            weathered_copper_chest: Item::from_block(&*vanilla_blocks::WEATHERED_COPPER_CHEST),
            oxidized_copper_chest: Item::from_block(&*vanilla_blocks::OXIDIZED_COPPER_CHEST),
            waxed_copper_chest: Item::from_block(&*vanilla_blocks::WAXED_COPPER_CHEST),
            waxed_exposed_copper_chest: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_CHEST,
            ),
            waxed_weathered_copper_chest: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_CHEST,
            ),
            waxed_oxidized_copper_chest: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_CHEST,
            ),
            copper_golem_statue: Item::from_block(&*vanilla_blocks::COPPER_GOLEM_STATUE),
            exposed_copper_golem_statue: Item::from_block(
                &*vanilla_blocks::EXPOSED_COPPER_GOLEM_STATUE,
            ),
            weathered_copper_golem_statue: Item::from_block(
                &*vanilla_blocks::WEATHERED_COPPER_GOLEM_STATUE,
            ),
            oxidized_copper_golem_statue: Item::from_block(
                &*vanilla_blocks::OXIDIZED_COPPER_GOLEM_STATUE,
            ),
            waxed_copper_golem_statue: Item::from_block(
                &*vanilla_blocks::WAXED_COPPER_GOLEM_STATUE,
            ),
            waxed_exposed_copper_golem_statue: Item::from_block(
                &*vanilla_blocks::WAXED_EXPOSED_COPPER_GOLEM_STATUE,
            ),
            waxed_weathered_copper_golem_statue: Item::from_block(
                &*vanilla_blocks::WAXED_WEATHERED_COPPER_GOLEM_STATUE,
            ),
            waxed_oxidized_copper_golem_statue: Item::from_block(
                &*vanilla_blocks::WAXED_OXIDIZED_COPPER_GOLEM_STATUE,
            ),
            trial_spawner: Item::from_block(&*vanilla_blocks::TRIAL_SPAWNER),
            trial_key: Item {
                key: Identifier::vanilla_static("trial_key"),
                components: DataComponentMap::common_item_components(),
            },
            ominous_trial_key: Item {
                key: Identifier::vanilla_static("ominous_trial_key"),
                components: DataComponentMap::common_item_components(),
            },
            vault: Item::from_block(&*vanilla_blocks::VAULT),
            ominous_bottle: Item {
                key: Identifier::vanilla_static("ominous_bottle"),
                components: DataComponentMap::common_item_components(),
            },
        }
    }
}
pub fn register_items(registry: &mut ItemRegistry) {
    registry.register(&ITEMS.air);
    registry.register(&ITEMS.stone);
    registry.register(&ITEMS.granite);
    registry.register(&ITEMS.polished_granite);
    registry.register(&ITEMS.diorite);
    registry.register(&ITEMS.polished_diorite);
    registry.register(&ITEMS.andesite);
    registry.register(&ITEMS.polished_andesite);
    registry.register(&ITEMS.deepslate);
    registry.register(&ITEMS.cobbled_deepslate);
    registry.register(&ITEMS.polished_deepslate);
    registry.register(&ITEMS.calcite);
    registry.register(&ITEMS.tuff);
    registry.register(&ITEMS.tuff_slab);
    registry.register(&ITEMS.tuff_stairs);
    registry.register(&ITEMS.tuff_wall);
    registry.register(&ITEMS.chiseled_tuff);
    registry.register(&ITEMS.polished_tuff);
    registry.register(&ITEMS.polished_tuff_slab);
    registry.register(&ITEMS.polished_tuff_stairs);
    registry.register(&ITEMS.polished_tuff_wall);
    registry.register(&ITEMS.tuff_bricks);
    registry.register(&ITEMS.tuff_brick_slab);
    registry.register(&ITEMS.tuff_brick_stairs);
    registry.register(&ITEMS.tuff_brick_wall);
    registry.register(&ITEMS.chiseled_tuff_bricks);
    registry.register(&ITEMS.dripstone_block);
    registry.register(&ITEMS.grass_block);
    registry.register(&ITEMS.dirt);
    registry.register(&ITEMS.coarse_dirt);
    registry.register(&ITEMS.podzol);
    registry.register(&ITEMS.rooted_dirt);
    registry.register(&ITEMS.mud);
    registry.register(&ITEMS.crimson_nylium);
    registry.register(&ITEMS.warped_nylium);
    registry.register(&ITEMS.cobblestone);
    registry.register(&ITEMS.oak_planks);
    registry.register(&ITEMS.spruce_planks);
    registry.register(&ITEMS.birch_planks);
    registry.register(&ITEMS.jungle_planks);
    registry.register(&ITEMS.acacia_planks);
    registry.register(&ITEMS.cherry_planks);
    registry.register(&ITEMS.dark_oak_planks);
    registry.register(&ITEMS.pale_oak_planks);
    registry.register(&ITEMS.mangrove_planks);
    registry.register(&ITEMS.bamboo_planks);
    registry.register(&ITEMS.crimson_planks);
    registry.register(&ITEMS.warped_planks);
    registry.register(&ITEMS.bamboo_mosaic);
    registry.register(&ITEMS.oak_sapling);
    registry.register(&ITEMS.spruce_sapling);
    registry.register(&ITEMS.birch_sapling);
    registry.register(&ITEMS.jungle_sapling);
    registry.register(&ITEMS.acacia_sapling);
    registry.register(&ITEMS.cherry_sapling);
    registry.register(&ITEMS.dark_oak_sapling);
    registry.register(&ITEMS.pale_oak_sapling);
    registry.register(&ITEMS.mangrove_propagule);
    registry.register(&ITEMS.bedrock);
    registry.register(&ITEMS.sand);
    registry.register(&ITEMS.suspicious_sand);
    registry.register(&ITEMS.suspicious_gravel);
    registry.register(&ITEMS.red_sand);
    registry.register(&ITEMS.gravel);
    registry.register(&ITEMS.coal_ore);
    registry.register(&ITEMS.deepslate_coal_ore);
    registry.register(&ITEMS.iron_ore);
    registry.register(&ITEMS.deepslate_iron_ore);
    registry.register(&ITEMS.copper_ore);
    registry.register(&ITEMS.deepslate_copper_ore);
    registry.register(&ITEMS.gold_ore);
    registry.register(&ITEMS.deepslate_gold_ore);
    registry.register(&ITEMS.redstone_ore);
    registry.register(&ITEMS.deepslate_redstone_ore);
    registry.register(&ITEMS.emerald_ore);
    registry.register(&ITEMS.deepslate_emerald_ore);
    registry.register(&ITEMS.lapis_ore);
    registry.register(&ITEMS.deepslate_lapis_ore);
    registry.register(&ITEMS.diamond_ore);
    registry.register(&ITEMS.deepslate_diamond_ore);
    registry.register(&ITEMS.nether_gold_ore);
    registry.register(&ITEMS.nether_quartz_ore);
    registry.register(&ITEMS.ancient_debris);
    registry.register(&ITEMS.coal_block);
    registry.register(&ITEMS.raw_iron_block);
    registry.register(&ITEMS.raw_copper_block);
    registry.register(&ITEMS.raw_gold_block);
    registry.register(&ITEMS.heavy_core);
    registry.register(&ITEMS.amethyst_block);
    registry.register(&ITEMS.budding_amethyst);
    registry.register(&ITEMS.iron_block);
    registry.register(&ITEMS.copper_block);
    registry.register(&ITEMS.gold_block);
    registry.register(&ITEMS.diamond_block);
    registry.register(&ITEMS.netherite_block);
    registry.register(&ITEMS.exposed_copper);
    registry.register(&ITEMS.weathered_copper);
    registry.register(&ITEMS.oxidized_copper);
    registry.register(&ITEMS.chiseled_copper);
    registry.register(&ITEMS.exposed_chiseled_copper);
    registry.register(&ITEMS.weathered_chiseled_copper);
    registry.register(&ITEMS.oxidized_chiseled_copper);
    registry.register(&ITEMS.cut_copper);
    registry.register(&ITEMS.exposed_cut_copper);
    registry.register(&ITEMS.weathered_cut_copper);
    registry.register(&ITEMS.oxidized_cut_copper);
    registry.register(&ITEMS.cut_copper_stairs);
    registry.register(&ITEMS.exposed_cut_copper_stairs);
    registry.register(&ITEMS.weathered_cut_copper_stairs);
    registry.register(&ITEMS.oxidized_cut_copper_stairs);
    registry.register(&ITEMS.cut_copper_slab);
    registry.register(&ITEMS.exposed_cut_copper_slab);
    registry.register(&ITEMS.weathered_cut_copper_slab);
    registry.register(&ITEMS.oxidized_cut_copper_slab);
    registry.register(&ITEMS.waxed_copper_block);
    registry.register(&ITEMS.waxed_exposed_copper);
    registry.register(&ITEMS.waxed_weathered_copper);
    registry.register(&ITEMS.waxed_oxidized_copper);
    registry.register(&ITEMS.waxed_chiseled_copper);
    registry.register(&ITEMS.waxed_exposed_chiseled_copper);
    registry.register(&ITEMS.waxed_weathered_chiseled_copper);
    registry.register(&ITEMS.waxed_oxidized_chiseled_copper);
    registry.register(&ITEMS.waxed_cut_copper);
    registry.register(&ITEMS.waxed_exposed_cut_copper);
    registry.register(&ITEMS.waxed_weathered_cut_copper);
    registry.register(&ITEMS.waxed_oxidized_cut_copper);
    registry.register(&ITEMS.waxed_cut_copper_stairs);
    registry.register(&ITEMS.waxed_exposed_cut_copper_stairs);
    registry.register(&ITEMS.waxed_weathered_cut_copper_stairs);
    registry.register(&ITEMS.waxed_oxidized_cut_copper_stairs);
    registry.register(&ITEMS.waxed_cut_copper_slab);
    registry.register(&ITEMS.waxed_exposed_cut_copper_slab);
    registry.register(&ITEMS.waxed_weathered_cut_copper_slab);
    registry.register(&ITEMS.waxed_oxidized_cut_copper_slab);
    registry.register(&ITEMS.oak_log);
    registry.register(&ITEMS.spruce_log);
    registry.register(&ITEMS.birch_log);
    registry.register(&ITEMS.jungle_log);
    registry.register(&ITEMS.acacia_log);
    registry.register(&ITEMS.cherry_log);
    registry.register(&ITEMS.pale_oak_log);
    registry.register(&ITEMS.dark_oak_log);
    registry.register(&ITEMS.mangrove_log);
    registry.register(&ITEMS.mangrove_roots);
    registry.register(&ITEMS.muddy_mangrove_roots);
    registry.register(&ITEMS.crimson_stem);
    registry.register(&ITEMS.warped_stem);
    registry.register(&ITEMS.bamboo_block);
    registry.register(&ITEMS.stripped_oak_log);
    registry.register(&ITEMS.stripped_spruce_log);
    registry.register(&ITEMS.stripped_birch_log);
    registry.register(&ITEMS.stripped_jungle_log);
    registry.register(&ITEMS.stripped_acacia_log);
    registry.register(&ITEMS.stripped_cherry_log);
    registry.register(&ITEMS.stripped_dark_oak_log);
    registry.register(&ITEMS.stripped_pale_oak_log);
    registry.register(&ITEMS.stripped_mangrove_log);
    registry.register(&ITEMS.stripped_crimson_stem);
    registry.register(&ITEMS.stripped_warped_stem);
    registry.register(&ITEMS.stripped_oak_wood);
    registry.register(&ITEMS.stripped_spruce_wood);
    registry.register(&ITEMS.stripped_birch_wood);
    registry.register(&ITEMS.stripped_jungle_wood);
    registry.register(&ITEMS.stripped_acacia_wood);
    registry.register(&ITEMS.stripped_cherry_wood);
    registry.register(&ITEMS.stripped_dark_oak_wood);
    registry.register(&ITEMS.stripped_pale_oak_wood);
    registry.register(&ITEMS.stripped_mangrove_wood);
    registry.register(&ITEMS.stripped_crimson_hyphae);
    registry.register(&ITEMS.stripped_warped_hyphae);
    registry.register(&ITEMS.stripped_bamboo_block);
    registry.register(&ITEMS.oak_wood);
    registry.register(&ITEMS.spruce_wood);
    registry.register(&ITEMS.birch_wood);
    registry.register(&ITEMS.jungle_wood);
    registry.register(&ITEMS.acacia_wood);
    registry.register(&ITEMS.cherry_wood);
    registry.register(&ITEMS.pale_oak_wood);
    registry.register(&ITEMS.dark_oak_wood);
    registry.register(&ITEMS.mangrove_wood);
    registry.register(&ITEMS.crimson_hyphae);
    registry.register(&ITEMS.warped_hyphae);
    registry.register(&ITEMS.oak_leaves);
    registry.register(&ITEMS.spruce_leaves);
    registry.register(&ITEMS.birch_leaves);
    registry.register(&ITEMS.jungle_leaves);
    registry.register(&ITEMS.acacia_leaves);
    registry.register(&ITEMS.cherry_leaves);
    registry.register(&ITEMS.dark_oak_leaves);
    registry.register(&ITEMS.pale_oak_leaves);
    registry.register(&ITEMS.mangrove_leaves);
    registry.register(&ITEMS.azalea_leaves);
    registry.register(&ITEMS.flowering_azalea_leaves);
    registry.register(&ITEMS.sponge);
    registry.register(&ITEMS.wet_sponge);
    registry.register(&ITEMS.glass);
    registry.register(&ITEMS.tinted_glass);
    registry.register(&ITEMS.lapis_block);
    registry.register(&ITEMS.sandstone);
    registry.register(&ITEMS.chiseled_sandstone);
    registry.register(&ITEMS.cut_sandstone);
    registry.register(&ITEMS.cobweb);
    registry.register(&ITEMS.short_grass);
    registry.register(&ITEMS.fern);
    registry.register(&ITEMS.bush);
    registry.register(&ITEMS.azalea);
    registry.register(&ITEMS.flowering_azalea);
    registry.register(&ITEMS.dead_bush);
    registry.register(&ITEMS.firefly_bush);
    registry.register(&ITEMS.short_dry_grass);
    registry.register(&ITEMS.tall_dry_grass);
    registry.register(&ITEMS.seagrass);
    registry.register(&ITEMS.sea_pickle);
    registry.register(&ITEMS.white_wool);
    registry.register(&ITEMS.orange_wool);
    registry.register(&ITEMS.magenta_wool);
    registry.register(&ITEMS.light_blue_wool);
    registry.register(&ITEMS.yellow_wool);
    registry.register(&ITEMS.lime_wool);
    registry.register(&ITEMS.pink_wool);
    registry.register(&ITEMS.gray_wool);
    registry.register(&ITEMS.light_gray_wool);
    registry.register(&ITEMS.cyan_wool);
    registry.register(&ITEMS.purple_wool);
    registry.register(&ITEMS.blue_wool);
    registry.register(&ITEMS.brown_wool);
    registry.register(&ITEMS.green_wool);
    registry.register(&ITEMS.red_wool);
    registry.register(&ITEMS.black_wool);
    registry.register(&ITEMS.dandelion);
    registry.register(&ITEMS.open_eyeblossom);
    registry.register(&ITEMS.closed_eyeblossom);
    registry.register(&ITEMS.poppy);
    registry.register(&ITEMS.blue_orchid);
    registry.register(&ITEMS.allium);
    registry.register(&ITEMS.azure_bluet);
    registry.register(&ITEMS.red_tulip);
    registry.register(&ITEMS.orange_tulip);
    registry.register(&ITEMS.white_tulip);
    registry.register(&ITEMS.pink_tulip);
    registry.register(&ITEMS.oxeye_daisy);
    registry.register(&ITEMS.cornflower);
    registry.register(&ITEMS.lily_of_the_valley);
    registry.register(&ITEMS.wither_rose);
    registry.register(&ITEMS.torchflower);
    registry.register(&ITEMS.pitcher_plant);
    registry.register(&ITEMS.spore_blossom);
    registry.register(&ITEMS.brown_mushroom);
    registry.register(&ITEMS.red_mushroom);
    registry.register(&ITEMS.crimson_fungus);
    registry.register(&ITEMS.warped_fungus);
    registry.register(&ITEMS.crimson_roots);
    registry.register(&ITEMS.warped_roots);
    registry.register(&ITEMS.nether_sprouts);
    registry.register(&ITEMS.weeping_vines);
    registry.register(&ITEMS.twisting_vines);
    registry.register(&ITEMS.sugar_cane);
    registry.register(&ITEMS.kelp);
    registry.register(&ITEMS.pink_petals);
    registry.register(&ITEMS.wildflowers);
    registry.register(&ITEMS.leaf_litter);
    registry.register(&ITEMS.moss_carpet);
    registry.register(&ITEMS.moss_block);
    registry.register(&ITEMS.pale_moss_carpet);
    registry.register(&ITEMS.pale_hanging_moss);
    registry.register(&ITEMS.pale_moss_block);
    registry.register(&ITEMS.hanging_roots);
    registry.register(&ITEMS.big_dripleaf);
    registry.register(&ITEMS.small_dripleaf);
    registry.register(&ITEMS.bamboo);
    registry.register(&ITEMS.oak_slab);
    registry.register(&ITEMS.spruce_slab);
    registry.register(&ITEMS.birch_slab);
    registry.register(&ITEMS.jungle_slab);
    registry.register(&ITEMS.acacia_slab);
    registry.register(&ITEMS.cherry_slab);
    registry.register(&ITEMS.dark_oak_slab);
    registry.register(&ITEMS.pale_oak_slab);
    registry.register(&ITEMS.mangrove_slab);
    registry.register(&ITEMS.bamboo_slab);
    registry.register(&ITEMS.bamboo_mosaic_slab);
    registry.register(&ITEMS.crimson_slab);
    registry.register(&ITEMS.warped_slab);
    registry.register(&ITEMS.stone_slab);
    registry.register(&ITEMS.smooth_stone_slab);
    registry.register(&ITEMS.sandstone_slab);
    registry.register(&ITEMS.cut_sandstone_slab);
    registry.register(&ITEMS.petrified_oak_slab);
    registry.register(&ITEMS.cobblestone_slab);
    registry.register(&ITEMS.brick_slab);
    registry.register(&ITEMS.stone_brick_slab);
    registry.register(&ITEMS.mud_brick_slab);
    registry.register(&ITEMS.nether_brick_slab);
    registry.register(&ITEMS.quartz_slab);
    registry.register(&ITEMS.red_sandstone_slab);
    registry.register(&ITEMS.cut_red_sandstone_slab);
    registry.register(&ITEMS.purpur_slab);
    registry.register(&ITEMS.prismarine_slab);
    registry.register(&ITEMS.prismarine_brick_slab);
    registry.register(&ITEMS.dark_prismarine_slab);
    registry.register(&ITEMS.smooth_quartz);
    registry.register(&ITEMS.smooth_red_sandstone);
    registry.register(&ITEMS.smooth_sandstone);
    registry.register(&ITEMS.smooth_stone);
    registry.register(&ITEMS.bricks);
    registry.register(&ITEMS.acacia_shelf);
    registry.register(&ITEMS.bamboo_shelf);
    registry.register(&ITEMS.birch_shelf);
    registry.register(&ITEMS.cherry_shelf);
    registry.register(&ITEMS.crimson_shelf);
    registry.register(&ITEMS.dark_oak_shelf);
    registry.register(&ITEMS.jungle_shelf);
    registry.register(&ITEMS.mangrove_shelf);
    registry.register(&ITEMS.oak_shelf);
    registry.register(&ITEMS.pale_oak_shelf);
    registry.register(&ITEMS.spruce_shelf);
    registry.register(&ITEMS.warped_shelf);
    registry.register(&ITEMS.bookshelf);
    registry.register(&ITEMS.chiseled_bookshelf);
    registry.register(&ITEMS.decorated_pot);
    registry.register(&ITEMS.mossy_cobblestone);
    registry.register(&ITEMS.obsidian);
    registry.register(&ITEMS.torch);
    registry.register(&ITEMS.end_rod);
    registry.register(&ITEMS.chorus_plant);
    registry.register(&ITEMS.chorus_flower);
    registry.register(&ITEMS.purpur_block);
    registry.register(&ITEMS.purpur_pillar);
    registry.register(&ITEMS.purpur_stairs);
    registry.register(&ITEMS.spawner);
    registry.register(&ITEMS.creaking_heart);
    registry.register(&ITEMS.chest);
    registry.register(&ITEMS.crafting_table);
    registry.register(&ITEMS.farmland);
    registry.register(&ITEMS.furnace);
    registry.register(&ITEMS.ladder);
    registry.register(&ITEMS.cobblestone_stairs);
    registry.register(&ITEMS.snow);
    registry.register(&ITEMS.ice);
    registry.register(&ITEMS.snow_block);
    registry.register(&ITEMS.cactus);
    registry.register(&ITEMS.cactus_flower);
    registry.register(&ITEMS.clay);
    registry.register(&ITEMS.jukebox);
    registry.register(&ITEMS.oak_fence);
    registry.register(&ITEMS.spruce_fence);
    registry.register(&ITEMS.birch_fence);
    registry.register(&ITEMS.jungle_fence);
    registry.register(&ITEMS.acacia_fence);
    registry.register(&ITEMS.cherry_fence);
    registry.register(&ITEMS.dark_oak_fence);
    registry.register(&ITEMS.pale_oak_fence);
    registry.register(&ITEMS.mangrove_fence);
    registry.register(&ITEMS.bamboo_fence);
    registry.register(&ITEMS.crimson_fence);
    registry.register(&ITEMS.warped_fence);
    registry.register(&ITEMS.pumpkin);
    registry.register(&ITEMS.carved_pumpkin);
    registry.register(&ITEMS.jack_o_lantern);
    registry.register(&ITEMS.netherrack);
    registry.register(&ITEMS.soul_sand);
    registry.register(&ITEMS.soul_soil);
    registry.register(&ITEMS.basalt);
    registry.register(&ITEMS.polished_basalt);
    registry.register(&ITEMS.smooth_basalt);
    registry.register(&ITEMS.soul_torch);
    registry.register(&ITEMS.copper_torch);
    registry.register(&ITEMS.glowstone);
    registry.register(&ITEMS.infested_stone);
    registry.register(&ITEMS.infested_cobblestone);
    registry.register(&ITEMS.infested_stone_bricks);
    registry.register(&ITEMS.infested_mossy_stone_bricks);
    registry.register(&ITEMS.infested_cracked_stone_bricks);
    registry.register(&ITEMS.infested_chiseled_stone_bricks);
    registry.register(&ITEMS.infested_deepslate);
    registry.register(&ITEMS.stone_bricks);
    registry.register(&ITEMS.mossy_stone_bricks);
    registry.register(&ITEMS.cracked_stone_bricks);
    registry.register(&ITEMS.chiseled_stone_bricks);
    registry.register(&ITEMS.packed_mud);
    registry.register(&ITEMS.mud_bricks);
    registry.register(&ITEMS.deepslate_bricks);
    registry.register(&ITEMS.cracked_deepslate_bricks);
    registry.register(&ITEMS.deepslate_tiles);
    registry.register(&ITEMS.cracked_deepslate_tiles);
    registry.register(&ITEMS.chiseled_deepslate);
    registry.register(&ITEMS.reinforced_deepslate);
    registry.register(&ITEMS.brown_mushroom_block);
    registry.register(&ITEMS.red_mushroom_block);
    registry.register(&ITEMS.mushroom_stem);
    registry.register(&ITEMS.iron_bars);
    registry.register(&ITEMS.copper_bars);
    registry.register(&ITEMS.exposed_copper_bars);
    registry.register(&ITEMS.weathered_copper_bars);
    registry.register(&ITEMS.oxidized_copper_bars);
    registry.register(&ITEMS.waxed_copper_bars);
    registry.register(&ITEMS.waxed_exposed_copper_bars);
    registry.register(&ITEMS.waxed_weathered_copper_bars);
    registry.register(&ITEMS.waxed_oxidized_copper_bars);
    registry.register(&ITEMS.iron_chain);
    registry.register(&ITEMS.copper_chain);
    registry.register(&ITEMS.exposed_copper_chain);
    registry.register(&ITEMS.weathered_copper_chain);
    registry.register(&ITEMS.oxidized_copper_chain);
    registry.register(&ITEMS.waxed_copper_chain);
    registry.register(&ITEMS.waxed_exposed_copper_chain);
    registry.register(&ITEMS.waxed_weathered_copper_chain);
    registry.register(&ITEMS.waxed_oxidized_copper_chain);
    registry.register(&ITEMS.glass_pane);
    registry.register(&ITEMS.melon);
    registry.register(&ITEMS.vine);
    registry.register(&ITEMS.glow_lichen);
    registry.register(&ITEMS.resin_clump);
    registry.register(&ITEMS.resin_block);
    registry.register(&ITEMS.resin_bricks);
    registry.register(&ITEMS.resin_brick_stairs);
    registry.register(&ITEMS.resin_brick_slab);
    registry.register(&ITEMS.resin_brick_wall);
    registry.register(&ITEMS.chiseled_resin_bricks);
    registry.register(&ITEMS.brick_stairs);
    registry.register(&ITEMS.stone_brick_stairs);
    registry.register(&ITEMS.mud_brick_stairs);
    registry.register(&ITEMS.mycelium);
    registry.register(&ITEMS.lily_pad);
    registry.register(&ITEMS.nether_bricks);
    registry.register(&ITEMS.cracked_nether_bricks);
    registry.register(&ITEMS.chiseled_nether_bricks);
    registry.register(&ITEMS.nether_brick_fence);
    registry.register(&ITEMS.nether_brick_stairs);
    registry.register(&ITEMS.sculk);
    registry.register(&ITEMS.sculk_vein);
    registry.register(&ITEMS.sculk_catalyst);
    registry.register(&ITEMS.sculk_shrieker);
    registry.register(&ITEMS.enchanting_table);
    registry.register(&ITEMS.end_portal_frame);
    registry.register(&ITEMS.end_stone);
    registry.register(&ITEMS.end_stone_bricks);
    registry.register(&ITEMS.dragon_egg);
    registry.register(&ITEMS.sandstone_stairs);
    registry.register(&ITEMS.ender_chest);
    registry.register(&ITEMS.emerald_block);
    registry.register(&ITEMS.oak_stairs);
    registry.register(&ITEMS.spruce_stairs);
    registry.register(&ITEMS.birch_stairs);
    registry.register(&ITEMS.jungle_stairs);
    registry.register(&ITEMS.acacia_stairs);
    registry.register(&ITEMS.cherry_stairs);
    registry.register(&ITEMS.dark_oak_stairs);
    registry.register(&ITEMS.pale_oak_stairs);
    registry.register(&ITEMS.mangrove_stairs);
    registry.register(&ITEMS.bamboo_stairs);
    registry.register(&ITEMS.bamboo_mosaic_stairs);
    registry.register(&ITEMS.crimson_stairs);
    registry.register(&ITEMS.warped_stairs);
    registry.register(&ITEMS.command_block);
    registry.register(&ITEMS.beacon);
    registry.register(&ITEMS.cobblestone_wall);
    registry.register(&ITEMS.mossy_cobblestone_wall);
    registry.register(&ITEMS.brick_wall);
    registry.register(&ITEMS.prismarine_wall);
    registry.register(&ITEMS.red_sandstone_wall);
    registry.register(&ITEMS.mossy_stone_brick_wall);
    registry.register(&ITEMS.granite_wall);
    registry.register(&ITEMS.stone_brick_wall);
    registry.register(&ITEMS.mud_brick_wall);
    registry.register(&ITEMS.nether_brick_wall);
    registry.register(&ITEMS.andesite_wall);
    registry.register(&ITEMS.red_nether_brick_wall);
    registry.register(&ITEMS.sandstone_wall);
    registry.register(&ITEMS.end_stone_brick_wall);
    registry.register(&ITEMS.diorite_wall);
    registry.register(&ITEMS.blackstone_wall);
    registry.register(&ITEMS.polished_blackstone_wall);
    registry.register(&ITEMS.polished_blackstone_brick_wall);
    registry.register(&ITEMS.cobbled_deepslate_wall);
    registry.register(&ITEMS.polished_deepslate_wall);
    registry.register(&ITEMS.deepslate_brick_wall);
    registry.register(&ITEMS.deepslate_tile_wall);
    registry.register(&ITEMS.anvil);
    registry.register(&ITEMS.chipped_anvil);
    registry.register(&ITEMS.damaged_anvil);
    registry.register(&ITEMS.chiseled_quartz_block);
    registry.register(&ITEMS.quartz_block);
    registry.register(&ITEMS.quartz_bricks);
    registry.register(&ITEMS.quartz_pillar);
    registry.register(&ITEMS.quartz_stairs);
    registry.register(&ITEMS.white_terracotta);
    registry.register(&ITEMS.orange_terracotta);
    registry.register(&ITEMS.magenta_terracotta);
    registry.register(&ITEMS.light_blue_terracotta);
    registry.register(&ITEMS.yellow_terracotta);
    registry.register(&ITEMS.lime_terracotta);
    registry.register(&ITEMS.pink_terracotta);
    registry.register(&ITEMS.gray_terracotta);
    registry.register(&ITEMS.light_gray_terracotta);
    registry.register(&ITEMS.cyan_terracotta);
    registry.register(&ITEMS.purple_terracotta);
    registry.register(&ITEMS.blue_terracotta);
    registry.register(&ITEMS.brown_terracotta);
    registry.register(&ITEMS.green_terracotta);
    registry.register(&ITEMS.red_terracotta);
    registry.register(&ITEMS.black_terracotta);
    registry.register(&ITEMS.barrier);
    registry.register(&ITEMS.light);
    registry.register(&ITEMS.hay_block);
    registry.register(&ITEMS.white_carpet);
    registry.register(&ITEMS.orange_carpet);
    registry.register(&ITEMS.magenta_carpet);
    registry.register(&ITEMS.light_blue_carpet);
    registry.register(&ITEMS.yellow_carpet);
    registry.register(&ITEMS.lime_carpet);
    registry.register(&ITEMS.pink_carpet);
    registry.register(&ITEMS.gray_carpet);
    registry.register(&ITEMS.light_gray_carpet);
    registry.register(&ITEMS.cyan_carpet);
    registry.register(&ITEMS.purple_carpet);
    registry.register(&ITEMS.blue_carpet);
    registry.register(&ITEMS.brown_carpet);
    registry.register(&ITEMS.green_carpet);
    registry.register(&ITEMS.red_carpet);
    registry.register(&ITEMS.black_carpet);
    registry.register(&ITEMS.terracotta);
    registry.register(&ITEMS.packed_ice);
    registry.register(&ITEMS.dirt_path);
    registry.register(&ITEMS.sunflower);
    registry.register(&ITEMS.lilac);
    registry.register(&ITEMS.rose_bush);
    registry.register(&ITEMS.peony);
    registry.register(&ITEMS.tall_grass);
    registry.register(&ITEMS.large_fern);
    registry.register(&ITEMS.white_stained_glass);
    registry.register(&ITEMS.orange_stained_glass);
    registry.register(&ITEMS.magenta_stained_glass);
    registry.register(&ITEMS.light_blue_stained_glass);
    registry.register(&ITEMS.yellow_stained_glass);
    registry.register(&ITEMS.lime_stained_glass);
    registry.register(&ITEMS.pink_stained_glass);
    registry.register(&ITEMS.gray_stained_glass);
    registry.register(&ITEMS.light_gray_stained_glass);
    registry.register(&ITEMS.cyan_stained_glass);
    registry.register(&ITEMS.purple_stained_glass);
    registry.register(&ITEMS.blue_stained_glass);
    registry.register(&ITEMS.brown_stained_glass);
    registry.register(&ITEMS.green_stained_glass);
    registry.register(&ITEMS.red_stained_glass);
    registry.register(&ITEMS.black_stained_glass);
    registry.register(&ITEMS.white_stained_glass_pane);
    registry.register(&ITEMS.orange_stained_glass_pane);
    registry.register(&ITEMS.magenta_stained_glass_pane);
    registry.register(&ITEMS.light_blue_stained_glass_pane);
    registry.register(&ITEMS.yellow_stained_glass_pane);
    registry.register(&ITEMS.lime_stained_glass_pane);
    registry.register(&ITEMS.pink_stained_glass_pane);
    registry.register(&ITEMS.gray_stained_glass_pane);
    registry.register(&ITEMS.light_gray_stained_glass_pane);
    registry.register(&ITEMS.cyan_stained_glass_pane);
    registry.register(&ITEMS.purple_stained_glass_pane);
    registry.register(&ITEMS.blue_stained_glass_pane);
    registry.register(&ITEMS.brown_stained_glass_pane);
    registry.register(&ITEMS.green_stained_glass_pane);
    registry.register(&ITEMS.red_stained_glass_pane);
    registry.register(&ITEMS.black_stained_glass_pane);
    registry.register(&ITEMS.prismarine);
    registry.register(&ITEMS.prismarine_bricks);
    registry.register(&ITEMS.dark_prismarine);
    registry.register(&ITEMS.prismarine_stairs);
    registry.register(&ITEMS.prismarine_brick_stairs);
    registry.register(&ITEMS.dark_prismarine_stairs);
    registry.register(&ITEMS.sea_lantern);
    registry.register(&ITEMS.red_sandstone);
    registry.register(&ITEMS.chiseled_red_sandstone);
    registry.register(&ITEMS.cut_red_sandstone);
    registry.register(&ITEMS.red_sandstone_stairs);
    registry.register(&ITEMS.repeating_command_block);
    registry.register(&ITEMS.chain_command_block);
    registry.register(&ITEMS.magma_block);
    registry.register(&ITEMS.nether_wart_block);
    registry.register(&ITEMS.warped_wart_block);
    registry.register(&ITEMS.red_nether_bricks);
    registry.register(&ITEMS.bone_block);
    registry.register(&ITEMS.structure_void);
    registry.register(&ITEMS.shulker_box);
    registry.register(&ITEMS.white_shulker_box);
    registry.register(&ITEMS.orange_shulker_box);
    registry.register(&ITEMS.magenta_shulker_box);
    registry.register(&ITEMS.light_blue_shulker_box);
    registry.register(&ITEMS.yellow_shulker_box);
    registry.register(&ITEMS.lime_shulker_box);
    registry.register(&ITEMS.pink_shulker_box);
    registry.register(&ITEMS.gray_shulker_box);
    registry.register(&ITEMS.light_gray_shulker_box);
    registry.register(&ITEMS.cyan_shulker_box);
    registry.register(&ITEMS.purple_shulker_box);
    registry.register(&ITEMS.blue_shulker_box);
    registry.register(&ITEMS.brown_shulker_box);
    registry.register(&ITEMS.green_shulker_box);
    registry.register(&ITEMS.red_shulker_box);
    registry.register(&ITEMS.black_shulker_box);
    registry.register(&ITEMS.white_glazed_terracotta);
    registry.register(&ITEMS.orange_glazed_terracotta);
    registry.register(&ITEMS.magenta_glazed_terracotta);
    registry.register(&ITEMS.light_blue_glazed_terracotta);
    registry.register(&ITEMS.yellow_glazed_terracotta);
    registry.register(&ITEMS.lime_glazed_terracotta);
    registry.register(&ITEMS.pink_glazed_terracotta);
    registry.register(&ITEMS.gray_glazed_terracotta);
    registry.register(&ITEMS.light_gray_glazed_terracotta);
    registry.register(&ITEMS.cyan_glazed_terracotta);
    registry.register(&ITEMS.purple_glazed_terracotta);
    registry.register(&ITEMS.blue_glazed_terracotta);
    registry.register(&ITEMS.brown_glazed_terracotta);
    registry.register(&ITEMS.green_glazed_terracotta);
    registry.register(&ITEMS.red_glazed_terracotta);
    registry.register(&ITEMS.black_glazed_terracotta);
    registry.register(&ITEMS.white_concrete);
    registry.register(&ITEMS.orange_concrete);
    registry.register(&ITEMS.magenta_concrete);
    registry.register(&ITEMS.light_blue_concrete);
    registry.register(&ITEMS.yellow_concrete);
    registry.register(&ITEMS.lime_concrete);
    registry.register(&ITEMS.pink_concrete);
    registry.register(&ITEMS.gray_concrete);
    registry.register(&ITEMS.light_gray_concrete);
    registry.register(&ITEMS.cyan_concrete);
    registry.register(&ITEMS.purple_concrete);
    registry.register(&ITEMS.blue_concrete);
    registry.register(&ITEMS.brown_concrete);
    registry.register(&ITEMS.green_concrete);
    registry.register(&ITEMS.red_concrete);
    registry.register(&ITEMS.black_concrete);
    registry.register(&ITEMS.white_concrete_powder);
    registry.register(&ITEMS.orange_concrete_powder);
    registry.register(&ITEMS.magenta_concrete_powder);
    registry.register(&ITEMS.light_blue_concrete_powder);
    registry.register(&ITEMS.yellow_concrete_powder);
    registry.register(&ITEMS.lime_concrete_powder);
    registry.register(&ITEMS.pink_concrete_powder);
    registry.register(&ITEMS.gray_concrete_powder);
    registry.register(&ITEMS.light_gray_concrete_powder);
    registry.register(&ITEMS.cyan_concrete_powder);
    registry.register(&ITEMS.purple_concrete_powder);
    registry.register(&ITEMS.blue_concrete_powder);
    registry.register(&ITEMS.brown_concrete_powder);
    registry.register(&ITEMS.green_concrete_powder);
    registry.register(&ITEMS.red_concrete_powder);
    registry.register(&ITEMS.black_concrete_powder);
    registry.register(&ITEMS.turtle_egg);
    registry.register(&ITEMS.sniffer_egg);
    registry.register(&ITEMS.dried_ghast);
    registry.register(&ITEMS.dead_tube_coral_block);
    registry.register(&ITEMS.dead_brain_coral_block);
    registry.register(&ITEMS.dead_bubble_coral_block);
    registry.register(&ITEMS.dead_fire_coral_block);
    registry.register(&ITEMS.dead_horn_coral_block);
    registry.register(&ITEMS.tube_coral_block);
    registry.register(&ITEMS.brain_coral_block);
    registry.register(&ITEMS.bubble_coral_block);
    registry.register(&ITEMS.fire_coral_block);
    registry.register(&ITEMS.horn_coral_block);
    registry.register(&ITEMS.tube_coral);
    registry.register(&ITEMS.brain_coral);
    registry.register(&ITEMS.bubble_coral);
    registry.register(&ITEMS.fire_coral);
    registry.register(&ITEMS.horn_coral);
    registry.register(&ITEMS.dead_brain_coral);
    registry.register(&ITEMS.dead_bubble_coral);
    registry.register(&ITEMS.dead_fire_coral);
    registry.register(&ITEMS.dead_horn_coral);
    registry.register(&ITEMS.dead_tube_coral);
    registry.register(&ITEMS.tube_coral_fan);
    registry.register(&ITEMS.brain_coral_fan);
    registry.register(&ITEMS.bubble_coral_fan);
    registry.register(&ITEMS.fire_coral_fan);
    registry.register(&ITEMS.horn_coral_fan);
    registry.register(&ITEMS.dead_tube_coral_fan);
    registry.register(&ITEMS.dead_brain_coral_fan);
    registry.register(&ITEMS.dead_bubble_coral_fan);
    registry.register(&ITEMS.dead_fire_coral_fan);
    registry.register(&ITEMS.dead_horn_coral_fan);
    registry.register(&ITEMS.blue_ice);
    registry.register(&ITEMS.conduit);
    registry.register(&ITEMS.polished_granite_stairs);
    registry.register(&ITEMS.smooth_red_sandstone_stairs);
    registry.register(&ITEMS.mossy_stone_brick_stairs);
    registry.register(&ITEMS.polished_diorite_stairs);
    registry.register(&ITEMS.mossy_cobblestone_stairs);
    registry.register(&ITEMS.end_stone_brick_stairs);
    registry.register(&ITEMS.stone_stairs);
    registry.register(&ITEMS.smooth_sandstone_stairs);
    registry.register(&ITEMS.smooth_quartz_stairs);
    registry.register(&ITEMS.granite_stairs);
    registry.register(&ITEMS.andesite_stairs);
    registry.register(&ITEMS.red_nether_brick_stairs);
    registry.register(&ITEMS.polished_andesite_stairs);
    registry.register(&ITEMS.diorite_stairs);
    registry.register(&ITEMS.cobbled_deepslate_stairs);
    registry.register(&ITEMS.polished_deepslate_stairs);
    registry.register(&ITEMS.deepslate_brick_stairs);
    registry.register(&ITEMS.deepslate_tile_stairs);
    registry.register(&ITEMS.polished_granite_slab);
    registry.register(&ITEMS.smooth_red_sandstone_slab);
    registry.register(&ITEMS.mossy_stone_brick_slab);
    registry.register(&ITEMS.polished_diorite_slab);
    registry.register(&ITEMS.mossy_cobblestone_slab);
    registry.register(&ITEMS.end_stone_brick_slab);
    registry.register(&ITEMS.smooth_sandstone_slab);
    registry.register(&ITEMS.smooth_quartz_slab);
    registry.register(&ITEMS.granite_slab);
    registry.register(&ITEMS.andesite_slab);
    registry.register(&ITEMS.red_nether_brick_slab);
    registry.register(&ITEMS.polished_andesite_slab);
    registry.register(&ITEMS.diorite_slab);
    registry.register(&ITEMS.cobbled_deepslate_slab);
    registry.register(&ITEMS.polished_deepslate_slab);
    registry.register(&ITEMS.deepslate_brick_slab);
    registry.register(&ITEMS.deepslate_tile_slab);
    registry.register(&ITEMS.scaffolding);
    registry.register(&ITEMS.redstone);
    registry.register(&ITEMS.redstone_torch);
    registry.register(&ITEMS.redstone_block);
    registry.register(&ITEMS.repeater);
    registry.register(&ITEMS.comparator);
    registry.register(&ITEMS.piston);
    registry.register(&ITEMS.sticky_piston);
    registry.register(&ITEMS.slime_block);
    registry.register(&ITEMS.honey_block);
    registry.register(&ITEMS.observer);
    registry.register(&ITEMS.hopper);
    registry.register(&ITEMS.dispenser);
    registry.register(&ITEMS.dropper);
    registry.register(&ITEMS.lectern);
    registry.register(&ITEMS.target);
    registry.register(&ITEMS.lever);
    registry.register(&ITEMS.lightning_rod);
    registry.register(&ITEMS.exposed_lightning_rod);
    registry.register(&ITEMS.weathered_lightning_rod);
    registry.register(&ITEMS.oxidized_lightning_rod);
    registry.register(&ITEMS.waxed_lightning_rod);
    registry.register(&ITEMS.waxed_exposed_lightning_rod);
    registry.register(&ITEMS.waxed_weathered_lightning_rod);
    registry.register(&ITEMS.waxed_oxidized_lightning_rod);
    registry.register(&ITEMS.daylight_detector);
    registry.register(&ITEMS.sculk_sensor);
    registry.register(&ITEMS.calibrated_sculk_sensor);
    registry.register(&ITEMS.tripwire_hook);
    registry.register(&ITEMS.trapped_chest);
    registry.register(&ITEMS.tnt);
    registry.register(&ITEMS.redstone_lamp);
    registry.register(&ITEMS.note_block);
    registry.register(&ITEMS.stone_button);
    registry.register(&ITEMS.polished_blackstone_button);
    registry.register(&ITEMS.oak_button);
    registry.register(&ITEMS.spruce_button);
    registry.register(&ITEMS.birch_button);
    registry.register(&ITEMS.jungle_button);
    registry.register(&ITEMS.acacia_button);
    registry.register(&ITEMS.cherry_button);
    registry.register(&ITEMS.dark_oak_button);
    registry.register(&ITEMS.pale_oak_button);
    registry.register(&ITEMS.mangrove_button);
    registry.register(&ITEMS.bamboo_button);
    registry.register(&ITEMS.crimson_button);
    registry.register(&ITEMS.warped_button);
    registry.register(&ITEMS.stone_pressure_plate);
    registry.register(&ITEMS.polished_blackstone_pressure_plate);
    registry.register(&ITEMS.light_weighted_pressure_plate);
    registry.register(&ITEMS.heavy_weighted_pressure_plate);
    registry.register(&ITEMS.oak_pressure_plate);
    registry.register(&ITEMS.spruce_pressure_plate);
    registry.register(&ITEMS.birch_pressure_plate);
    registry.register(&ITEMS.jungle_pressure_plate);
    registry.register(&ITEMS.acacia_pressure_plate);
    registry.register(&ITEMS.cherry_pressure_plate);
    registry.register(&ITEMS.dark_oak_pressure_plate);
    registry.register(&ITEMS.pale_oak_pressure_plate);
    registry.register(&ITEMS.mangrove_pressure_plate);
    registry.register(&ITEMS.bamboo_pressure_plate);
    registry.register(&ITEMS.crimson_pressure_plate);
    registry.register(&ITEMS.warped_pressure_plate);
    registry.register(&ITEMS.iron_door);
    registry.register(&ITEMS.oak_door);
    registry.register(&ITEMS.spruce_door);
    registry.register(&ITEMS.birch_door);
    registry.register(&ITEMS.jungle_door);
    registry.register(&ITEMS.acacia_door);
    registry.register(&ITEMS.cherry_door);
    registry.register(&ITEMS.dark_oak_door);
    registry.register(&ITEMS.pale_oak_door);
    registry.register(&ITEMS.mangrove_door);
    registry.register(&ITEMS.bamboo_door);
    registry.register(&ITEMS.crimson_door);
    registry.register(&ITEMS.warped_door);
    registry.register(&ITEMS.copper_door);
    registry.register(&ITEMS.exposed_copper_door);
    registry.register(&ITEMS.weathered_copper_door);
    registry.register(&ITEMS.oxidized_copper_door);
    registry.register(&ITEMS.waxed_copper_door);
    registry.register(&ITEMS.waxed_exposed_copper_door);
    registry.register(&ITEMS.waxed_weathered_copper_door);
    registry.register(&ITEMS.waxed_oxidized_copper_door);
    registry.register(&ITEMS.iron_trapdoor);
    registry.register(&ITEMS.oak_trapdoor);
    registry.register(&ITEMS.spruce_trapdoor);
    registry.register(&ITEMS.birch_trapdoor);
    registry.register(&ITEMS.jungle_trapdoor);
    registry.register(&ITEMS.acacia_trapdoor);
    registry.register(&ITEMS.cherry_trapdoor);
    registry.register(&ITEMS.dark_oak_trapdoor);
    registry.register(&ITEMS.pale_oak_trapdoor);
    registry.register(&ITEMS.mangrove_trapdoor);
    registry.register(&ITEMS.bamboo_trapdoor);
    registry.register(&ITEMS.crimson_trapdoor);
    registry.register(&ITEMS.warped_trapdoor);
    registry.register(&ITEMS.copper_trapdoor);
    registry.register(&ITEMS.exposed_copper_trapdoor);
    registry.register(&ITEMS.weathered_copper_trapdoor);
    registry.register(&ITEMS.oxidized_copper_trapdoor);
    registry.register(&ITEMS.waxed_copper_trapdoor);
    registry.register(&ITEMS.waxed_exposed_copper_trapdoor);
    registry.register(&ITEMS.waxed_weathered_copper_trapdoor);
    registry.register(&ITEMS.waxed_oxidized_copper_trapdoor);
    registry.register(&ITEMS.oak_fence_gate);
    registry.register(&ITEMS.spruce_fence_gate);
    registry.register(&ITEMS.birch_fence_gate);
    registry.register(&ITEMS.jungle_fence_gate);
    registry.register(&ITEMS.acacia_fence_gate);
    registry.register(&ITEMS.cherry_fence_gate);
    registry.register(&ITEMS.dark_oak_fence_gate);
    registry.register(&ITEMS.pale_oak_fence_gate);
    registry.register(&ITEMS.mangrove_fence_gate);
    registry.register(&ITEMS.bamboo_fence_gate);
    registry.register(&ITEMS.crimson_fence_gate);
    registry.register(&ITEMS.warped_fence_gate);
    registry.register(&ITEMS.powered_rail);
    registry.register(&ITEMS.detector_rail);
    registry.register(&ITEMS.rail);
    registry.register(&ITEMS.activator_rail);
    registry.register(&ITEMS.saddle);
    registry.register(&ITEMS.white_harness);
    registry.register(&ITEMS.orange_harness);
    registry.register(&ITEMS.magenta_harness);
    registry.register(&ITEMS.light_blue_harness);
    registry.register(&ITEMS.yellow_harness);
    registry.register(&ITEMS.lime_harness);
    registry.register(&ITEMS.pink_harness);
    registry.register(&ITEMS.gray_harness);
    registry.register(&ITEMS.light_gray_harness);
    registry.register(&ITEMS.cyan_harness);
    registry.register(&ITEMS.purple_harness);
    registry.register(&ITEMS.blue_harness);
    registry.register(&ITEMS.brown_harness);
    registry.register(&ITEMS.green_harness);
    registry.register(&ITEMS.red_harness);
    registry.register(&ITEMS.black_harness);
    registry.register(&ITEMS.minecart);
    registry.register(&ITEMS.chest_minecart);
    registry.register(&ITEMS.furnace_minecart);
    registry.register(&ITEMS.tnt_minecart);
    registry.register(&ITEMS.hopper_minecart);
    registry.register(&ITEMS.carrot_on_a_stick);
    registry.register(&ITEMS.warped_fungus_on_a_stick);
    registry.register(&ITEMS.phantom_membrane);
    registry.register(&ITEMS.elytra);
    registry.register(&ITEMS.oak_boat);
    registry.register(&ITEMS.oak_chest_boat);
    registry.register(&ITEMS.spruce_boat);
    registry.register(&ITEMS.spruce_chest_boat);
    registry.register(&ITEMS.birch_boat);
    registry.register(&ITEMS.birch_chest_boat);
    registry.register(&ITEMS.jungle_boat);
    registry.register(&ITEMS.jungle_chest_boat);
    registry.register(&ITEMS.acacia_boat);
    registry.register(&ITEMS.acacia_chest_boat);
    registry.register(&ITEMS.cherry_boat);
    registry.register(&ITEMS.cherry_chest_boat);
    registry.register(&ITEMS.dark_oak_boat);
    registry.register(&ITEMS.dark_oak_chest_boat);
    registry.register(&ITEMS.pale_oak_boat);
    registry.register(&ITEMS.pale_oak_chest_boat);
    registry.register(&ITEMS.mangrove_boat);
    registry.register(&ITEMS.mangrove_chest_boat);
    registry.register(&ITEMS.bamboo_raft);
    registry.register(&ITEMS.bamboo_chest_raft);
    registry.register(&ITEMS.structure_block);
    registry.register(&ITEMS.jigsaw);
    registry.register(&ITEMS.test_block);
    registry.register(&ITEMS.test_instance_block);
    registry.register(&ITEMS.turtle_helmet);
    registry.register(&ITEMS.turtle_scute);
    registry.register(&ITEMS.armadillo_scute);
    registry.register(&ITEMS.wolf_armor);
    registry.register(&ITEMS.flint_and_steel);
    registry.register(&ITEMS.bowl);
    registry.register(&ITEMS.apple);
    registry.register(&ITEMS.bow);
    registry.register(&ITEMS.arrow);
    registry.register(&ITEMS.coal);
    registry.register(&ITEMS.charcoal);
    registry.register(&ITEMS.diamond);
    registry.register(&ITEMS.emerald);
    registry.register(&ITEMS.lapis_lazuli);
    registry.register(&ITEMS.quartz);
    registry.register(&ITEMS.amethyst_shard);
    registry.register(&ITEMS.raw_iron);
    registry.register(&ITEMS.iron_ingot);
    registry.register(&ITEMS.raw_copper);
    registry.register(&ITEMS.copper_ingot);
    registry.register(&ITEMS.raw_gold);
    registry.register(&ITEMS.gold_ingot);
    registry.register(&ITEMS.netherite_ingot);
    registry.register(&ITEMS.netherite_scrap);
    registry.register(&ITEMS.wooden_sword);
    registry.register(&ITEMS.wooden_shovel);
    registry.register(&ITEMS.wooden_pickaxe);
    registry.register(&ITEMS.wooden_axe);
    registry.register(&ITEMS.wooden_hoe);
    registry.register(&ITEMS.copper_sword);
    registry.register(&ITEMS.copper_shovel);
    registry.register(&ITEMS.copper_pickaxe);
    registry.register(&ITEMS.copper_axe);
    registry.register(&ITEMS.copper_hoe);
    registry.register(&ITEMS.stone_sword);
    registry.register(&ITEMS.stone_shovel);
    registry.register(&ITEMS.stone_pickaxe);
    registry.register(&ITEMS.stone_axe);
    registry.register(&ITEMS.stone_hoe);
    registry.register(&ITEMS.golden_sword);
    registry.register(&ITEMS.golden_shovel);
    registry.register(&ITEMS.golden_pickaxe);
    registry.register(&ITEMS.golden_axe);
    registry.register(&ITEMS.golden_hoe);
    registry.register(&ITEMS.iron_sword);
    registry.register(&ITEMS.iron_shovel);
    registry.register(&ITEMS.iron_pickaxe);
    registry.register(&ITEMS.iron_axe);
    registry.register(&ITEMS.iron_hoe);
    registry.register(&ITEMS.diamond_sword);
    registry.register(&ITEMS.diamond_shovel);
    registry.register(&ITEMS.diamond_pickaxe);
    registry.register(&ITEMS.diamond_axe);
    registry.register(&ITEMS.diamond_hoe);
    registry.register(&ITEMS.netherite_sword);
    registry.register(&ITEMS.netherite_shovel);
    registry.register(&ITEMS.netherite_pickaxe);
    registry.register(&ITEMS.netherite_axe);
    registry.register(&ITEMS.netherite_hoe);
    registry.register(&ITEMS.stick);
    registry.register(&ITEMS.mushroom_stew);
    registry.register(&ITEMS.string);
    registry.register(&ITEMS.feather);
    registry.register(&ITEMS.gunpowder);
    registry.register(&ITEMS.wheat_seeds);
    registry.register(&ITEMS.wheat);
    registry.register(&ITEMS.bread);
    registry.register(&ITEMS.leather_helmet);
    registry.register(&ITEMS.leather_chestplate);
    registry.register(&ITEMS.leather_leggings);
    registry.register(&ITEMS.leather_boots);
    registry.register(&ITEMS.copper_helmet);
    registry.register(&ITEMS.copper_chestplate);
    registry.register(&ITEMS.copper_leggings);
    registry.register(&ITEMS.copper_boots);
    registry.register(&ITEMS.chainmail_helmet);
    registry.register(&ITEMS.chainmail_chestplate);
    registry.register(&ITEMS.chainmail_leggings);
    registry.register(&ITEMS.chainmail_boots);
    registry.register(&ITEMS.iron_helmet);
    registry.register(&ITEMS.iron_chestplate);
    registry.register(&ITEMS.iron_leggings);
    registry.register(&ITEMS.iron_boots);
    registry.register(&ITEMS.diamond_helmet);
    registry.register(&ITEMS.diamond_chestplate);
    registry.register(&ITEMS.diamond_leggings);
    registry.register(&ITEMS.diamond_boots);
    registry.register(&ITEMS.golden_helmet);
    registry.register(&ITEMS.golden_chestplate);
    registry.register(&ITEMS.golden_leggings);
    registry.register(&ITEMS.golden_boots);
    registry.register(&ITEMS.netherite_helmet);
    registry.register(&ITEMS.netherite_chestplate);
    registry.register(&ITEMS.netherite_leggings);
    registry.register(&ITEMS.netherite_boots);
    registry.register(&ITEMS.flint);
    registry.register(&ITEMS.porkchop);
    registry.register(&ITEMS.cooked_porkchop);
    registry.register(&ITEMS.painting);
    registry.register(&ITEMS.golden_apple);
    registry.register(&ITEMS.enchanted_golden_apple);
    registry.register(&ITEMS.oak_sign);
    registry.register(&ITEMS.spruce_sign);
    registry.register(&ITEMS.birch_sign);
    registry.register(&ITEMS.jungle_sign);
    registry.register(&ITEMS.acacia_sign);
    registry.register(&ITEMS.cherry_sign);
    registry.register(&ITEMS.dark_oak_sign);
    registry.register(&ITEMS.pale_oak_sign);
    registry.register(&ITEMS.mangrove_sign);
    registry.register(&ITEMS.bamboo_sign);
    registry.register(&ITEMS.crimson_sign);
    registry.register(&ITEMS.warped_sign);
    registry.register(&ITEMS.oak_hanging_sign);
    registry.register(&ITEMS.spruce_hanging_sign);
    registry.register(&ITEMS.birch_hanging_sign);
    registry.register(&ITEMS.jungle_hanging_sign);
    registry.register(&ITEMS.acacia_hanging_sign);
    registry.register(&ITEMS.cherry_hanging_sign);
    registry.register(&ITEMS.dark_oak_hanging_sign);
    registry.register(&ITEMS.pale_oak_hanging_sign);
    registry.register(&ITEMS.mangrove_hanging_sign);
    registry.register(&ITEMS.bamboo_hanging_sign);
    registry.register(&ITEMS.crimson_hanging_sign);
    registry.register(&ITEMS.warped_hanging_sign);
    registry.register(&ITEMS.bucket);
    registry.register(&ITEMS.water_bucket);
    registry.register(&ITEMS.lava_bucket);
    registry.register(&ITEMS.powder_snow_bucket);
    registry.register(&ITEMS.snowball);
    registry.register(&ITEMS.leather);
    registry.register(&ITEMS.milk_bucket);
    registry.register(&ITEMS.pufferfish_bucket);
    registry.register(&ITEMS.salmon_bucket);
    registry.register(&ITEMS.cod_bucket);
    registry.register(&ITEMS.tropical_fish_bucket);
    registry.register(&ITEMS.axolotl_bucket);
    registry.register(&ITEMS.tadpole_bucket);
    registry.register(&ITEMS.brick);
    registry.register(&ITEMS.clay_ball);
    registry.register(&ITEMS.dried_kelp_block);
    registry.register(&ITEMS.paper);
    registry.register(&ITEMS.book);
    registry.register(&ITEMS.slime_ball);
    registry.register(&ITEMS.egg);
    registry.register(&ITEMS.blue_egg);
    registry.register(&ITEMS.brown_egg);
    registry.register(&ITEMS.compass);
    registry.register(&ITEMS.recovery_compass);
    registry.register(&ITEMS.bundle);
    registry.register(&ITEMS.white_bundle);
    registry.register(&ITEMS.orange_bundle);
    registry.register(&ITEMS.magenta_bundle);
    registry.register(&ITEMS.light_blue_bundle);
    registry.register(&ITEMS.yellow_bundle);
    registry.register(&ITEMS.lime_bundle);
    registry.register(&ITEMS.pink_bundle);
    registry.register(&ITEMS.gray_bundle);
    registry.register(&ITEMS.light_gray_bundle);
    registry.register(&ITEMS.cyan_bundle);
    registry.register(&ITEMS.purple_bundle);
    registry.register(&ITEMS.blue_bundle);
    registry.register(&ITEMS.brown_bundle);
    registry.register(&ITEMS.green_bundle);
    registry.register(&ITEMS.red_bundle);
    registry.register(&ITEMS.black_bundle);
    registry.register(&ITEMS.fishing_rod);
    registry.register(&ITEMS.clock);
    registry.register(&ITEMS.spyglass);
    registry.register(&ITEMS.glowstone_dust);
    registry.register(&ITEMS.cod);
    registry.register(&ITEMS.salmon);
    registry.register(&ITEMS.tropical_fish);
    registry.register(&ITEMS.pufferfish);
    registry.register(&ITEMS.cooked_cod);
    registry.register(&ITEMS.cooked_salmon);
    registry.register(&ITEMS.ink_sac);
    registry.register(&ITEMS.glow_ink_sac);
    registry.register(&ITEMS.cocoa_beans);
    registry.register(&ITEMS.white_dye);
    registry.register(&ITEMS.orange_dye);
    registry.register(&ITEMS.magenta_dye);
    registry.register(&ITEMS.light_blue_dye);
    registry.register(&ITEMS.yellow_dye);
    registry.register(&ITEMS.lime_dye);
    registry.register(&ITEMS.pink_dye);
    registry.register(&ITEMS.gray_dye);
    registry.register(&ITEMS.light_gray_dye);
    registry.register(&ITEMS.cyan_dye);
    registry.register(&ITEMS.purple_dye);
    registry.register(&ITEMS.blue_dye);
    registry.register(&ITEMS.brown_dye);
    registry.register(&ITEMS.green_dye);
    registry.register(&ITEMS.red_dye);
    registry.register(&ITEMS.black_dye);
    registry.register(&ITEMS.bone_meal);
    registry.register(&ITEMS.bone);
    registry.register(&ITEMS.sugar);
    registry.register(&ITEMS.cake);
    registry.register(&ITEMS.white_bed);
    registry.register(&ITEMS.orange_bed);
    registry.register(&ITEMS.magenta_bed);
    registry.register(&ITEMS.light_blue_bed);
    registry.register(&ITEMS.yellow_bed);
    registry.register(&ITEMS.lime_bed);
    registry.register(&ITEMS.pink_bed);
    registry.register(&ITEMS.gray_bed);
    registry.register(&ITEMS.light_gray_bed);
    registry.register(&ITEMS.cyan_bed);
    registry.register(&ITEMS.purple_bed);
    registry.register(&ITEMS.blue_bed);
    registry.register(&ITEMS.brown_bed);
    registry.register(&ITEMS.green_bed);
    registry.register(&ITEMS.red_bed);
    registry.register(&ITEMS.black_bed);
    registry.register(&ITEMS.cookie);
    registry.register(&ITEMS.crafter);
    registry.register(&ITEMS.filled_map);
    registry.register(&ITEMS.shears);
    registry.register(&ITEMS.melon_slice);
    registry.register(&ITEMS.dried_kelp);
    registry.register(&ITEMS.pumpkin_seeds);
    registry.register(&ITEMS.melon_seeds);
    registry.register(&ITEMS.beef);
    registry.register(&ITEMS.cooked_beef);
    registry.register(&ITEMS.chicken);
    registry.register(&ITEMS.cooked_chicken);
    registry.register(&ITEMS.rotten_flesh);
    registry.register(&ITEMS.ender_pearl);
    registry.register(&ITEMS.blaze_rod);
    registry.register(&ITEMS.ghast_tear);
    registry.register(&ITEMS.gold_nugget);
    registry.register(&ITEMS.nether_wart);
    registry.register(&ITEMS.glass_bottle);
    registry.register(&ITEMS.potion);
    registry.register(&ITEMS.spider_eye);
    registry.register(&ITEMS.fermented_spider_eye);
    registry.register(&ITEMS.blaze_powder);
    registry.register(&ITEMS.magma_cream);
    registry.register(&ITEMS.brewing_stand);
    registry.register(&ITEMS.cauldron);
    registry.register(&ITEMS.ender_eye);
    registry.register(&ITEMS.glistering_melon_slice);
    registry.register(&ITEMS.armadillo_spawn_egg);
    registry.register(&ITEMS.allay_spawn_egg);
    registry.register(&ITEMS.axolotl_spawn_egg);
    registry.register(&ITEMS.bat_spawn_egg);
    registry.register(&ITEMS.bee_spawn_egg);
    registry.register(&ITEMS.blaze_spawn_egg);
    registry.register(&ITEMS.bogged_spawn_egg);
    registry.register(&ITEMS.breeze_spawn_egg);
    registry.register(&ITEMS.cat_spawn_egg);
    registry.register(&ITEMS.camel_spawn_egg);
    registry.register(&ITEMS.cave_spider_spawn_egg);
    registry.register(&ITEMS.chicken_spawn_egg);
    registry.register(&ITEMS.cod_spawn_egg);
    registry.register(&ITEMS.copper_golem_spawn_egg);
    registry.register(&ITEMS.cow_spawn_egg);
    registry.register(&ITEMS.creeper_spawn_egg);
    registry.register(&ITEMS.dolphin_spawn_egg);
    registry.register(&ITEMS.donkey_spawn_egg);
    registry.register(&ITEMS.drowned_spawn_egg);
    registry.register(&ITEMS.elder_guardian_spawn_egg);
    registry.register(&ITEMS.ender_dragon_spawn_egg);
    registry.register(&ITEMS.enderman_spawn_egg);
    registry.register(&ITEMS.endermite_spawn_egg);
    registry.register(&ITEMS.evoker_spawn_egg);
    registry.register(&ITEMS.fox_spawn_egg);
    registry.register(&ITEMS.frog_spawn_egg);
    registry.register(&ITEMS.ghast_spawn_egg);
    registry.register(&ITEMS.happy_ghast_spawn_egg);
    registry.register(&ITEMS.glow_squid_spawn_egg);
    registry.register(&ITEMS.goat_spawn_egg);
    registry.register(&ITEMS.guardian_spawn_egg);
    registry.register(&ITEMS.hoglin_spawn_egg);
    registry.register(&ITEMS.horse_spawn_egg);
    registry.register(&ITEMS.husk_spawn_egg);
    registry.register(&ITEMS.iron_golem_spawn_egg);
    registry.register(&ITEMS.llama_spawn_egg);
    registry.register(&ITEMS.magma_cube_spawn_egg);
    registry.register(&ITEMS.mooshroom_spawn_egg);
    registry.register(&ITEMS.mule_spawn_egg);
    registry.register(&ITEMS.ocelot_spawn_egg);
    registry.register(&ITEMS.panda_spawn_egg);
    registry.register(&ITEMS.parrot_spawn_egg);
    registry.register(&ITEMS.phantom_spawn_egg);
    registry.register(&ITEMS.pig_spawn_egg);
    registry.register(&ITEMS.piglin_spawn_egg);
    registry.register(&ITEMS.piglin_brute_spawn_egg);
    registry.register(&ITEMS.pillager_spawn_egg);
    registry.register(&ITEMS.polar_bear_spawn_egg);
    registry.register(&ITEMS.pufferfish_spawn_egg);
    registry.register(&ITEMS.rabbit_spawn_egg);
    registry.register(&ITEMS.ravager_spawn_egg);
    registry.register(&ITEMS.salmon_spawn_egg);
    registry.register(&ITEMS.sheep_spawn_egg);
    registry.register(&ITEMS.shulker_spawn_egg);
    registry.register(&ITEMS.silverfish_spawn_egg);
    registry.register(&ITEMS.skeleton_spawn_egg);
    registry.register(&ITEMS.skeleton_horse_spawn_egg);
    registry.register(&ITEMS.slime_spawn_egg);
    registry.register(&ITEMS.sniffer_spawn_egg);
    registry.register(&ITEMS.snow_golem_spawn_egg);
    registry.register(&ITEMS.spider_spawn_egg);
    registry.register(&ITEMS.squid_spawn_egg);
    registry.register(&ITEMS.stray_spawn_egg);
    registry.register(&ITEMS.strider_spawn_egg);
    registry.register(&ITEMS.tadpole_spawn_egg);
    registry.register(&ITEMS.trader_llama_spawn_egg);
    registry.register(&ITEMS.tropical_fish_spawn_egg);
    registry.register(&ITEMS.turtle_spawn_egg);
    registry.register(&ITEMS.vex_spawn_egg);
    registry.register(&ITEMS.villager_spawn_egg);
    registry.register(&ITEMS.vindicator_spawn_egg);
    registry.register(&ITEMS.wandering_trader_spawn_egg);
    registry.register(&ITEMS.warden_spawn_egg);
    registry.register(&ITEMS.witch_spawn_egg);
    registry.register(&ITEMS.wither_spawn_egg);
    registry.register(&ITEMS.wither_skeleton_spawn_egg);
    registry.register(&ITEMS.wolf_spawn_egg);
    registry.register(&ITEMS.zoglin_spawn_egg);
    registry.register(&ITEMS.creaking_spawn_egg);
    registry.register(&ITEMS.zombie_spawn_egg);
    registry.register(&ITEMS.zombie_horse_spawn_egg);
    registry.register(&ITEMS.zombie_villager_spawn_egg);
    registry.register(&ITEMS.zombified_piglin_spawn_egg);
    registry.register(&ITEMS.experience_bottle);
    registry.register(&ITEMS.fire_charge);
    registry.register(&ITEMS.wind_charge);
    registry.register(&ITEMS.writable_book);
    registry.register(&ITEMS.written_book);
    registry.register(&ITEMS.breeze_rod);
    registry.register(&ITEMS.mace);
    registry.register(&ITEMS.item_frame);
    registry.register(&ITEMS.glow_item_frame);
    registry.register(&ITEMS.flower_pot);
    registry.register(&ITEMS.carrot);
    registry.register(&ITEMS.potato);
    registry.register(&ITEMS.baked_potato);
    registry.register(&ITEMS.poisonous_potato);
    registry.register(&ITEMS.map);
    registry.register(&ITEMS.golden_carrot);
    registry.register(&ITEMS.skeleton_skull);
    registry.register(&ITEMS.wither_skeleton_skull);
    registry.register(&ITEMS.player_head);
    registry.register(&ITEMS.zombie_head);
    registry.register(&ITEMS.creeper_head);
    registry.register(&ITEMS.dragon_head);
    registry.register(&ITEMS.piglin_head);
    registry.register(&ITEMS.nether_star);
    registry.register(&ITEMS.pumpkin_pie);
    registry.register(&ITEMS.firework_rocket);
    registry.register(&ITEMS.firework_star);
    registry.register(&ITEMS.enchanted_book);
    registry.register(&ITEMS.nether_brick);
    registry.register(&ITEMS.resin_brick);
    registry.register(&ITEMS.prismarine_shard);
    registry.register(&ITEMS.prismarine_crystals);
    registry.register(&ITEMS.rabbit);
    registry.register(&ITEMS.cooked_rabbit);
    registry.register(&ITEMS.rabbit_stew);
    registry.register(&ITEMS.rabbit_foot);
    registry.register(&ITEMS.rabbit_hide);
    registry.register(&ITEMS.armor_stand);
    registry.register(&ITEMS.copper_horse_armor);
    registry.register(&ITEMS.iron_horse_armor);
    registry.register(&ITEMS.golden_horse_armor);
    registry.register(&ITEMS.diamond_horse_armor);
    registry.register(&ITEMS.leather_horse_armor);
    registry.register(&ITEMS.lead);
    registry.register(&ITEMS.name_tag);
    registry.register(&ITEMS.command_block_minecart);
    registry.register(&ITEMS.mutton);
    registry.register(&ITEMS.cooked_mutton);
    registry.register(&ITEMS.white_banner);
    registry.register(&ITEMS.orange_banner);
    registry.register(&ITEMS.magenta_banner);
    registry.register(&ITEMS.light_blue_banner);
    registry.register(&ITEMS.yellow_banner);
    registry.register(&ITEMS.lime_banner);
    registry.register(&ITEMS.pink_banner);
    registry.register(&ITEMS.gray_banner);
    registry.register(&ITEMS.light_gray_banner);
    registry.register(&ITEMS.cyan_banner);
    registry.register(&ITEMS.purple_banner);
    registry.register(&ITEMS.blue_banner);
    registry.register(&ITEMS.brown_banner);
    registry.register(&ITEMS.green_banner);
    registry.register(&ITEMS.red_banner);
    registry.register(&ITEMS.black_banner);
    registry.register(&ITEMS.end_crystal);
    registry.register(&ITEMS.chorus_fruit);
    registry.register(&ITEMS.popped_chorus_fruit);
    registry.register(&ITEMS.torchflower_seeds);
    registry.register(&ITEMS.pitcher_pod);
    registry.register(&ITEMS.beetroot);
    registry.register(&ITEMS.beetroot_seeds);
    registry.register(&ITEMS.beetroot_soup);
    registry.register(&ITEMS.dragon_breath);
    registry.register(&ITEMS.splash_potion);
    registry.register(&ITEMS.spectral_arrow);
    registry.register(&ITEMS.tipped_arrow);
    registry.register(&ITEMS.lingering_potion);
    registry.register(&ITEMS.shield);
    registry.register(&ITEMS.totem_of_undying);
    registry.register(&ITEMS.shulker_shell);
    registry.register(&ITEMS.iron_nugget);
    registry.register(&ITEMS.copper_nugget);
    registry.register(&ITEMS.knowledge_book);
    registry.register(&ITEMS.debug_stick);
    registry.register(&ITEMS.music_disc_13);
    registry.register(&ITEMS.music_disc_cat);
    registry.register(&ITEMS.music_disc_blocks);
    registry.register(&ITEMS.music_disc_chirp);
    registry.register(&ITEMS.music_disc_creator);
    registry.register(&ITEMS.music_disc_creator_music_box);
    registry.register(&ITEMS.music_disc_far);
    registry.register(&ITEMS.music_disc_lava_chicken);
    registry.register(&ITEMS.music_disc_mall);
    registry.register(&ITEMS.music_disc_mellohi);
    registry.register(&ITEMS.music_disc_stal);
    registry.register(&ITEMS.music_disc_strad);
    registry.register(&ITEMS.music_disc_ward);
    registry.register(&ITEMS.music_disc_11);
    registry.register(&ITEMS.music_disc_wait);
    registry.register(&ITEMS.music_disc_otherside);
    registry.register(&ITEMS.music_disc_relic);
    registry.register(&ITEMS.music_disc_5);
    registry.register(&ITEMS.music_disc_pigstep);
    registry.register(&ITEMS.music_disc_precipice);
    registry.register(&ITEMS.music_disc_tears);
    registry.register(&ITEMS.disc_fragment_5);
    registry.register(&ITEMS.trident);
    registry.register(&ITEMS.nautilus_shell);
    registry.register(&ITEMS.heart_of_the_sea);
    registry.register(&ITEMS.crossbow);
    registry.register(&ITEMS.suspicious_stew);
    registry.register(&ITEMS.loom);
    registry.register(&ITEMS.flower_banner_pattern);
    registry.register(&ITEMS.creeper_banner_pattern);
    registry.register(&ITEMS.skull_banner_pattern);
    registry.register(&ITEMS.mojang_banner_pattern);
    registry.register(&ITEMS.globe_banner_pattern);
    registry.register(&ITEMS.piglin_banner_pattern);
    registry.register(&ITEMS.flow_banner_pattern);
    registry.register(&ITEMS.guster_banner_pattern);
    registry.register(&ITEMS.field_masoned_banner_pattern);
    registry.register(&ITEMS.bordure_indented_banner_pattern);
    registry.register(&ITEMS.goat_horn);
    registry.register(&ITEMS.composter);
    registry.register(&ITEMS.barrel);
    registry.register(&ITEMS.smoker);
    registry.register(&ITEMS.blast_furnace);
    registry.register(&ITEMS.cartography_table);
    registry.register(&ITEMS.fletching_table);
    registry.register(&ITEMS.grindstone);
    registry.register(&ITEMS.smithing_table);
    registry.register(&ITEMS.stonecutter);
    registry.register(&ITEMS.bell);
    registry.register(&ITEMS.lantern);
    registry.register(&ITEMS.soul_lantern);
    registry.register(&ITEMS.copper_lantern);
    registry.register(&ITEMS.exposed_copper_lantern);
    registry.register(&ITEMS.weathered_copper_lantern);
    registry.register(&ITEMS.oxidized_copper_lantern);
    registry.register(&ITEMS.waxed_copper_lantern);
    registry.register(&ITEMS.waxed_exposed_copper_lantern);
    registry.register(&ITEMS.waxed_weathered_copper_lantern);
    registry.register(&ITEMS.waxed_oxidized_copper_lantern);
    registry.register(&ITEMS.sweet_berries);
    registry.register(&ITEMS.glow_berries);
    registry.register(&ITEMS.campfire);
    registry.register(&ITEMS.soul_campfire);
    registry.register(&ITEMS.shroomlight);
    registry.register(&ITEMS.honeycomb);
    registry.register(&ITEMS.bee_nest);
    registry.register(&ITEMS.beehive);
    registry.register(&ITEMS.honey_bottle);
    registry.register(&ITEMS.honeycomb_block);
    registry.register(&ITEMS.lodestone);
    registry.register(&ITEMS.crying_obsidian);
    registry.register(&ITEMS.blackstone);
    registry.register(&ITEMS.blackstone_slab);
    registry.register(&ITEMS.blackstone_stairs);
    registry.register(&ITEMS.gilded_blackstone);
    registry.register(&ITEMS.polished_blackstone);
    registry.register(&ITEMS.polished_blackstone_slab);
    registry.register(&ITEMS.polished_blackstone_stairs);
    registry.register(&ITEMS.chiseled_polished_blackstone);
    registry.register(&ITEMS.polished_blackstone_bricks);
    registry.register(&ITEMS.polished_blackstone_brick_slab);
    registry.register(&ITEMS.polished_blackstone_brick_stairs);
    registry.register(&ITEMS.cracked_polished_blackstone_bricks);
    registry.register(&ITEMS.respawn_anchor);
    registry.register(&ITEMS.candle);
    registry.register(&ITEMS.white_candle);
    registry.register(&ITEMS.orange_candle);
    registry.register(&ITEMS.magenta_candle);
    registry.register(&ITEMS.light_blue_candle);
    registry.register(&ITEMS.yellow_candle);
    registry.register(&ITEMS.lime_candle);
    registry.register(&ITEMS.pink_candle);
    registry.register(&ITEMS.gray_candle);
    registry.register(&ITEMS.light_gray_candle);
    registry.register(&ITEMS.cyan_candle);
    registry.register(&ITEMS.purple_candle);
    registry.register(&ITEMS.blue_candle);
    registry.register(&ITEMS.brown_candle);
    registry.register(&ITEMS.green_candle);
    registry.register(&ITEMS.red_candle);
    registry.register(&ITEMS.black_candle);
    registry.register(&ITEMS.small_amethyst_bud);
    registry.register(&ITEMS.medium_amethyst_bud);
    registry.register(&ITEMS.large_amethyst_bud);
    registry.register(&ITEMS.amethyst_cluster);
    registry.register(&ITEMS.pointed_dripstone);
    registry.register(&ITEMS.ochre_froglight);
    registry.register(&ITEMS.verdant_froglight);
    registry.register(&ITEMS.pearlescent_froglight);
    registry.register(&ITEMS.frogspawn);
    registry.register(&ITEMS.echo_shard);
    registry.register(&ITEMS.brush);
    registry.register(&ITEMS.netherite_upgrade_smithing_template);
    registry.register(&ITEMS.sentry_armor_trim_smithing_template);
    registry.register(&ITEMS.dune_armor_trim_smithing_template);
    registry.register(&ITEMS.coast_armor_trim_smithing_template);
    registry.register(&ITEMS.wild_armor_trim_smithing_template);
    registry.register(&ITEMS.ward_armor_trim_smithing_template);
    registry.register(&ITEMS.eye_armor_trim_smithing_template);
    registry.register(&ITEMS.vex_armor_trim_smithing_template);
    registry.register(&ITEMS.tide_armor_trim_smithing_template);
    registry.register(&ITEMS.snout_armor_trim_smithing_template);
    registry.register(&ITEMS.rib_armor_trim_smithing_template);
    registry.register(&ITEMS.spire_armor_trim_smithing_template);
    registry.register(&ITEMS.wayfinder_armor_trim_smithing_template);
    registry.register(&ITEMS.shaper_armor_trim_smithing_template);
    registry.register(&ITEMS.silence_armor_trim_smithing_template);
    registry.register(&ITEMS.raiser_armor_trim_smithing_template);
    registry.register(&ITEMS.host_armor_trim_smithing_template);
    registry.register(&ITEMS.flow_armor_trim_smithing_template);
    registry.register(&ITEMS.bolt_armor_trim_smithing_template);
    registry.register(&ITEMS.angler_pottery_sherd);
    registry.register(&ITEMS.archer_pottery_sherd);
    registry.register(&ITEMS.arms_up_pottery_sherd);
    registry.register(&ITEMS.blade_pottery_sherd);
    registry.register(&ITEMS.brewer_pottery_sherd);
    registry.register(&ITEMS.burn_pottery_sherd);
    registry.register(&ITEMS.danger_pottery_sherd);
    registry.register(&ITEMS.explorer_pottery_sherd);
    registry.register(&ITEMS.flow_pottery_sherd);
    registry.register(&ITEMS.friend_pottery_sherd);
    registry.register(&ITEMS.guster_pottery_sherd);
    registry.register(&ITEMS.heart_pottery_sherd);
    registry.register(&ITEMS.heartbreak_pottery_sherd);
    registry.register(&ITEMS.howl_pottery_sherd);
    registry.register(&ITEMS.miner_pottery_sherd);
    registry.register(&ITEMS.mourner_pottery_sherd);
    registry.register(&ITEMS.plenty_pottery_sherd);
    registry.register(&ITEMS.prize_pottery_sherd);
    registry.register(&ITEMS.scrape_pottery_sherd);
    registry.register(&ITEMS.sheaf_pottery_sherd);
    registry.register(&ITEMS.shelter_pottery_sherd);
    registry.register(&ITEMS.skull_pottery_sherd);
    registry.register(&ITEMS.snort_pottery_sherd);
    registry.register(&ITEMS.copper_grate);
    registry.register(&ITEMS.exposed_copper_grate);
    registry.register(&ITEMS.weathered_copper_grate);
    registry.register(&ITEMS.oxidized_copper_grate);
    registry.register(&ITEMS.waxed_copper_grate);
    registry.register(&ITEMS.waxed_exposed_copper_grate);
    registry.register(&ITEMS.waxed_weathered_copper_grate);
    registry.register(&ITEMS.waxed_oxidized_copper_grate);
    registry.register(&ITEMS.copper_bulb);
    registry.register(&ITEMS.exposed_copper_bulb);
    registry.register(&ITEMS.weathered_copper_bulb);
    registry.register(&ITEMS.oxidized_copper_bulb);
    registry.register(&ITEMS.waxed_copper_bulb);
    registry.register(&ITEMS.waxed_exposed_copper_bulb);
    registry.register(&ITEMS.waxed_weathered_copper_bulb);
    registry.register(&ITEMS.waxed_oxidized_copper_bulb);
    registry.register(&ITEMS.copper_chest);
    registry.register(&ITEMS.exposed_copper_chest);
    registry.register(&ITEMS.weathered_copper_chest);
    registry.register(&ITEMS.oxidized_copper_chest);
    registry.register(&ITEMS.waxed_copper_chest);
    registry.register(&ITEMS.waxed_exposed_copper_chest);
    registry.register(&ITEMS.waxed_weathered_copper_chest);
    registry.register(&ITEMS.waxed_oxidized_copper_chest);
    registry.register(&ITEMS.copper_golem_statue);
    registry.register(&ITEMS.exposed_copper_golem_statue);
    registry.register(&ITEMS.weathered_copper_golem_statue);
    registry.register(&ITEMS.oxidized_copper_golem_statue);
    registry.register(&ITEMS.waxed_copper_golem_statue);
    registry.register(&ITEMS.waxed_exposed_copper_golem_statue);
    registry.register(&ITEMS.waxed_weathered_copper_golem_statue);
    registry.register(&ITEMS.waxed_oxidized_copper_golem_statue);
    registry.register(&ITEMS.trial_spawner);
    registry.register(&ITEMS.trial_key);
    registry.register(&ITEMS.ominous_trial_key);
    registry.register(&ITEMS.vault);
    registry.register(&ITEMS.ominous_bottle);
}
