use crate::{Identifier, Material};
use crate::vanilla_world_blocks::Block;
impl Material {
    pub const AIR: Self = Self::new(0, Identifier::vanilla_static("air"), Some(Block::AIR), 64);
    pub const STONE: Self = Self::new(1, Identifier::vanilla_static("stone"), Some(Block::STONE), 64);
    pub const GRANITE: Self = Self::new(2, Identifier::vanilla_static("granite"), Some(Block::GRANITE), 64);
    pub const POLISHED_GRANITE: Self = Self::new(3, Identifier::vanilla_static("polished_granite"), Some(Block::POLISHED_GRANITE), 64);
    pub const DIORITE: Self = Self::new(4, Identifier::vanilla_static("diorite"), Some(Block::DIORITE), 64);
    pub const POLISHED_DIORITE: Self = Self::new(5, Identifier::vanilla_static("polished_diorite"), Some(Block::POLISHED_DIORITE), 64);
    pub const ANDESITE: Self = Self::new(6, Identifier::vanilla_static("andesite"), Some(Block::ANDESITE), 64);
    pub const POLISHED_ANDESITE: Self = Self::new(7, Identifier::vanilla_static("polished_andesite"), Some(Block::POLISHED_ANDESITE), 64);
    pub const DEEPSLATE: Self = Self::new(8, Identifier::vanilla_static("deepslate"), Some(Block::DEEPSLATE), 64);
    pub const COBBLED_DEEPSLATE: Self = Self::new(9, Identifier::vanilla_static("cobbled_deepslate"), Some(Block::COBBLED_DEEPSLATE), 64);
    pub const POLISHED_DEEPSLATE: Self = Self::new(10, Identifier::vanilla_static("polished_deepslate"), Some(Block::POLISHED_DEEPSLATE), 64);
    pub const CALCITE: Self = Self::new(11, Identifier::vanilla_static("calcite"), Some(Block::CALCITE), 64);
    pub const TUFF: Self = Self::new(12, Identifier::vanilla_static("tuff"), Some(Block::TUFF), 64);
    pub const TUFF_SLAB: Self = Self::new(13, Identifier::vanilla_static("tuff_slab"), Some(Block::TUFF_SLAB), 64);
    pub const TUFF_STAIRS: Self = Self::new(14, Identifier::vanilla_static("tuff_stairs"), Some(Block::TUFF_STAIRS), 64);
    pub const TUFF_WALL: Self = Self::new(15, Identifier::vanilla_static("tuff_wall"), Some(Block::TUFF_WALL), 64);
    pub const CHISELED_TUFF: Self = Self::new(16, Identifier::vanilla_static("chiseled_tuff"), Some(Block::CHISELED_TUFF), 64);
    pub const POLISHED_TUFF: Self = Self::new(17, Identifier::vanilla_static("polished_tuff"), Some(Block::POLISHED_TUFF), 64);
    pub const POLISHED_TUFF_SLAB: Self = Self::new(18, Identifier::vanilla_static("polished_tuff_slab"), Some(Block::POLISHED_TUFF_SLAB), 64);
    pub const POLISHED_TUFF_STAIRS: Self = Self::new(19, Identifier::vanilla_static("polished_tuff_stairs"), Some(Block::POLISHED_TUFF_STAIRS), 64);
    pub const POLISHED_TUFF_WALL: Self = Self::new(20, Identifier::vanilla_static("polished_tuff_wall"), Some(Block::POLISHED_TUFF_WALL), 64);
    pub const TUFF_BRICKS: Self = Self::new(21, Identifier::vanilla_static("tuff_bricks"), Some(Block::TUFF_BRICKS), 64);
    pub const TUFF_BRICK_SLAB: Self = Self::new(22, Identifier::vanilla_static("tuff_brick_slab"), Some(Block::TUFF_BRICK_SLAB), 64);
    pub const TUFF_BRICK_STAIRS: Self = Self::new(23, Identifier::vanilla_static("tuff_brick_stairs"), Some(Block::TUFF_BRICK_STAIRS), 64);
    pub const TUFF_BRICK_WALL: Self = Self::new(24, Identifier::vanilla_static("tuff_brick_wall"), Some(Block::TUFF_BRICK_WALL), 64);
    pub const CHISELED_TUFF_BRICKS: Self = Self::new(25, Identifier::vanilla_static("chiseled_tuff_bricks"), Some(Block::CHISELED_TUFF_BRICKS), 64);
    pub const DRIPSTONE_BLOCK: Self = Self::new(26, Identifier::vanilla_static("dripstone_block"), Some(Block::DRIPSTONE_BLOCK), 64);
    pub const GRASS_BLOCK: Self = Self::new(27, Identifier::vanilla_static("grass_block"), Some(Block::GRASS_BLOCK), 64);
    pub const DIRT: Self = Self::new(28, Identifier::vanilla_static("dirt"), Some(Block::DIRT), 64);
    pub const COARSE_DIRT: Self = Self::new(29, Identifier::vanilla_static("coarse_dirt"), Some(Block::COARSE_DIRT), 64);
    pub const PODZOL: Self = Self::new(30, Identifier::vanilla_static("podzol"), Some(Block::PODZOL), 64);
    pub const ROOTED_DIRT: Self = Self::new(31, Identifier::vanilla_static("rooted_dirt"), Some(Block::ROOTED_DIRT), 64);
    pub const MUD: Self = Self::new(32, Identifier::vanilla_static("mud"), Some(Block::MUD), 64);
    pub const CRIMSON_NYLIUM: Self = Self::new(33, Identifier::vanilla_static("crimson_nylium"), Some(Block::CRIMSON_NYLIUM), 64);
    pub const WARPED_NYLIUM: Self = Self::new(34, Identifier::vanilla_static("warped_nylium"), Some(Block::WARPED_NYLIUM), 64);
    pub const COBBLESTONE: Self = Self::new(35, Identifier::vanilla_static("cobblestone"), Some(Block::COBBLESTONE), 64);
    pub const OAK_PLANKS: Self = Self::new(36, Identifier::vanilla_static("oak_planks"), Some(Block::OAK_PLANKS), 64);
    pub const SPRUCE_PLANKS: Self = Self::new(37, Identifier::vanilla_static("spruce_planks"), Some(Block::SPRUCE_PLANKS), 64);
    pub const BIRCH_PLANKS: Self = Self::new(38, Identifier::vanilla_static("birch_planks"), Some(Block::BIRCH_PLANKS), 64);
    pub const JUNGLE_PLANKS: Self = Self::new(39, Identifier::vanilla_static("jungle_planks"), Some(Block::JUNGLE_PLANKS), 64);
    pub const ACACIA_PLANKS: Self = Self::new(40, Identifier::vanilla_static("acacia_planks"), Some(Block::ACACIA_PLANKS), 64);
    pub const CHERRY_PLANKS: Self = Self::new(41, Identifier::vanilla_static("cherry_planks"), Some(Block::CHERRY_PLANKS), 64);
    pub const DARK_OAK_PLANKS: Self = Self::new(42, Identifier::vanilla_static("dark_oak_planks"), Some(Block::DARK_OAK_PLANKS), 64);
    pub const PALE_OAK_PLANKS: Self = Self::new(43, Identifier::vanilla_static("pale_oak_planks"), Some(Block::PALE_OAK_PLANKS), 64);
    pub const MANGROVE_PLANKS: Self = Self::new(44, Identifier::vanilla_static("mangrove_planks"), Some(Block::MANGROVE_PLANKS), 64);
    pub const BAMBOO_PLANKS: Self = Self::new(45, Identifier::vanilla_static("bamboo_planks"), Some(Block::BAMBOO_PLANKS), 64);
    pub const CRIMSON_PLANKS: Self = Self::new(46, Identifier::vanilla_static("crimson_planks"), Some(Block::CRIMSON_PLANKS), 64);
    pub const WARPED_PLANKS: Self = Self::new(47, Identifier::vanilla_static("warped_planks"), Some(Block::WARPED_PLANKS), 64);
    pub const BAMBOO_MOSAIC: Self = Self::new(48, Identifier::vanilla_static("bamboo_mosaic"), Some(Block::BAMBOO_MOSAIC), 64);
    pub const OAK_SAPLING: Self = Self::new(49, Identifier::vanilla_static("oak_sapling"), Some(Block::OAK_SAPLING), 64);
    pub const SPRUCE_SAPLING: Self = Self::new(50, Identifier::vanilla_static("spruce_sapling"), Some(Block::SPRUCE_SAPLING), 64);
    pub const BIRCH_SAPLING: Self = Self::new(51, Identifier::vanilla_static("birch_sapling"), Some(Block::BIRCH_SAPLING), 64);
    pub const JUNGLE_SAPLING: Self = Self::new(52, Identifier::vanilla_static("jungle_sapling"), Some(Block::JUNGLE_SAPLING), 64);
    pub const ACACIA_SAPLING: Self = Self::new(53, Identifier::vanilla_static("acacia_sapling"), Some(Block::ACACIA_SAPLING), 64);
    pub const CHERRY_SAPLING: Self = Self::new(54, Identifier::vanilla_static("cherry_sapling"), Some(Block::CHERRY_SAPLING), 64);
    pub const DARK_OAK_SAPLING: Self = Self::new(55, Identifier::vanilla_static("dark_oak_sapling"), Some(Block::DARK_OAK_SAPLING), 64);
    pub const PALE_OAK_SAPLING: Self = Self::new(56, Identifier::vanilla_static("pale_oak_sapling"), Some(Block::PALE_OAK_SAPLING), 64);
    pub const MANGROVE_PROPAGULE: Self = Self::new(57, Identifier::vanilla_static("mangrove_propagule"), Some(Block::MANGROVE_PROPAGULE), 64);
    pub const BEDROCK: Self = Self::new(58, Identifier::vanilla_static("bedrock"), Some(Block::BEDROCK), 64);
    pub const SAND: Self = Self::new(59, Identifier::vanilla_static("sand"), Some(Block::SAND), 64);
    pub const SUSPICIOUS_SAND: Self = Self::new(60, Identifier::vanilla_static("suspicious_sand"), Some(Block::SUSPICIOUS_SAND), 64);
    pub const SUSPICIOUS_GRAVEL: Self = Self::new(61, Identifier::vanilla_static("suspicious_gravel"), Some(Block::SUSPICIOUS_GRAVEL), 64);
    pub const RED_SAND: Self = Self::new(62, Identifier::vanilla_static("red_sand"), Some(Block::RED_SAND), 64);
    pub const GRAVEL: Self = Self::new(63, Identifier::vanilla_static("gravel"), Some(Block::GRAVEL), 64);
    pub const COAL_ORE: Self = Self::new(64, Identifier::vanilla_static("coal_ore"), Some(Block::COAL_ORE), 64);
    pub const DEEPSLATE_COAL_ORE: Self = Self::new(65, Identifier::vanilla_static("deepslate_coal_ore"), Some(Block::DEEPSLATE_COAL_ORE), 64);
    pub const IRON_ORE: Self = Self::new(66, Identifier::vanilla_static("iron_ore"), Some(Block::IRON_ORE), 64);
    pub const DEEPSLATE_IRON_ORE: Self = Self::new(67, Identifier::vanilla_static("deepslate_iron_ore"), Some(Block::DEEPSLATE_IRON_ORE), 64);
    pub const COPPER_ORE: Self = Self::new(68, Identifier::vanilla_static("copper_ore"), Some(Block::COPPER_ORE), 64);
    pub const DEEPSLATE_COPPER_ORE: Self = Self::new(69, Identifier::vanilla_static("deepslate_copper_ore"), Some(Block::DEEPSLATE_COPPER_ORE), 64);
    pub const GOLD_ORE: Self = Self::new(70, Identifier::vanilla_static("gold_ore"), Some(Block::GOLD_ORE), 64);
    pub const DEEPSLATE_GOLD_ORE: Self = Self::new(71, Identifier::vanilla_static("deepslate_gold_ore"), Some(Block::DEEPSLATE_GOLD_ORE), 64);
    pub const REDSTONE_ORE: Self = Self::new(72, Identifier::vanilla_static("redstone_ore"), Some(Block::REDSTONE_ORE), 64);
    pub const DEEPSLATE_REDSTONE_ORE: Self = Self::new(73, Identifier::vanilla_static("deepslate_redstone_ore"), Some(Block::DEEPSLATE_REDSTONE_ORE), 64);
    pub const EMERALD_ORE: Self = Self::new(74, Identifier::vanilla_static("emerald_ore"), Some(Block::EMERALD_ORE), 64);
    pub const DEEPSLATE_EMERALD_ORE: Self = Self::new(75, Identifier::vanilla_static("deepslate_emerald_ore"), Some(Block::DEEPSLATE_EMERALD_ORE), 64);
    pub const LAPIS_ORE: Self = Self::new(76, Identifier::vanilla_static("lapis_ore"), Some(Block::LAPIS_ORE), 64);
    pub const DEEPSLATE_LAPIS_ORE: Self = Self::new(77, Identifier::vanilla_static("deepslate_lapis_ore"), Some(Block::DEEPSLATE_LAPIS_ORE), 64);
    pub const DIAMOND_ORE: Self = Self::new(78, Identifier::vanilla_static("diamond_ore"), Some(Block::DIAMOND_ORE), 64);
    pub const DEEPSLATE_DIAMOND_ORE: Self = Self::new(79, Identifier::vanilla_static("deepslate_diamond_ore"), Some(Block::DEEPSLATE_DIAMOND_ORE), 64);
    pub const NETHER_GOLD_ORE: Self = Self::new(80, Identifier::vanilla_static("nether_gold_ore"), Some(Block::NETHER_GOLD_ORE), 64);
    pub const NETHER_QUARTZ_ORE: Self = Self::new(81, Identifier::vanilla_static("nether_quartz_ore"), Some(Block::NETHER_QUARTZ_ORE), 64);
    pub const ANCIENT_DEBRIS: Self = Self::new(82, Identifier::vanilla_static("ancient_debris"), Some(Block::ANCIENT_DEBRIS), 64);
    pub const COAL_BLOCK: Self = Self::new(83, Identifier::vanilla_static("coal_block"), Some(Block::COAL_BLOCK), 64);
    pub const RAW_IRON_BLOCK: Self = Self::new(84, Identifier::vanilla_static("raw_iron_block"), Some(Block::RAW_IRON_BLOCK), 64);
    pub const RAW_COPPER_BLOCK: Self = Self::new(85, Identifier::vanilla_static("raw_copper_block"), Some(Block::RAW_COPPER_BLOCK), 64);
    pub const RAW_GOLD_BLOCK: Self = Self::new(86, Identifier::vanilla_static("raw_gold_block"), Some(Block::RAW_GOLD_BLOCK), 64);
    pub const HEAVY_CORE: Self = Self::new(87, Identifier::vanilla_static("heavy_core"), Some(Block::HEAVY_CORE), 64);
    pub const AMETHYST_BLOCK: Self = Self::new(88, Identifier::vanilla_static("amethyst_block"), Some(Block::AMETHYST_BLOCK), 64);
    pub const BUDDING_AMETHYST: Self = Self::new(89, Identifier::vanilla_static("budding_amethyst"), Some(Block::BUDDING_AMETHYST), 64);
    pub const IRON_BLOCK: Self = Self::new(90, Identifier::vanilla_static("iron_block"), Some(Block::IRON_BLOCK), 64);
    pub const COPPER_BLOCK: Self = Self::new(91, Identifier::vanilla_static("copper_block"), Some(Block::COPPER_BLOCK), 64);
    pub const GOLD_BLOCK: Self = Self::new(92, Identifier::vanilla_static("gold_block"), Some(Block::GOLD_BLOCK), 64);
    pub const DIAMOND_BLOCK: Self = Self::new(93, Identifier::vanilla_static("diamond_block"), Some(Block::DIAMOND_BLOCK), 64);
    pub const NETHERITE_BLOCK: Self = Self::new(94, Identifier::vanilla_static("netherite_block"), Some(Block::NETHERITE_BLOCK), 64);
    pub const EXPOSED_COPPER: Self = Self::new(95, Identifier::vanilla_static("exposed_copper"), Some(Block::EXPOSED_COPPER), 64);
    pub const WEATHERED_COPPER: Self = Self::new(96, Identifier::vanilla_static("weathered_copper"), Some(Block::WEATHERED_COPPER), 64);
    pub const OXIDIZED_COPPER: Self = Self::new(97, Identifier::vanilla_static("oxidized_copper"), Some(Block::OXIDIZED_COPPER), 64);
    pub const CHISELED_COPPER: Self = Self::new(98, Identifier::vanilla_static("chiseled_copper"), Some(Block::CHISELED_COPPER), 64);
    pub const EXPOSED_CHISELED_COPPER: Self = Self::new(99, Identifier::vanilla_static("exposed_chiseled_copper"), Some(Block::EXPOSED_CHISELED_COPPER), 64);
    pub const WEATHERED_CHISELED_COPPER: Self = Self::new(100, Identifier::vanilla_static("weathered_chiseled_copper"), Some(Block::WEATHERED_CHISELED_COPPER), 64);
    pub const OXIDIZED_CHISELED_COPPER: Self = Self::new(101, Identifier::vanilla_static("oxidized_chiseled_copper"), Some(Block::OXIDIZED_CHISELED_COPPER), 64);
    pub const CUT_COPPER: Self = Self::new(102, Identifier::vanilla_static("cut_copper"), Some(Block::CUT_COPPER), 64);
    pub const EXPOSED_CUT_COPPER: Self = Self::new(103, Identifier::vanilla_static("exposed_cut_copper"), Some(Block::EXPOSED_CUT_COPPER), 64);
    pub const WEATHERED_CUT_COPPER: Self = Self::new(104, Identifier::vanilla_static("weathered_cut_copper"), Some(Block::WEATHERED_CUT_COPPER), 64);
    pub const OXIDIZED_CUT_COPPER: Self = Self::new(105, Identifier::vanilla_static("oxidized_cut_copper"), Some(Block::OXIDIZED_CUT_COPPER), 64);
    pub const CUT_COPPER_STAIRS: Self = Self::new(106, Identifier::vanilla_static("cut_copper_stairs"), Some(Block::CUT_COPPER_STAIRS), 64);
    pub const EXPOSED_CUT_COPPER_STAIRS: Self = Self::new(107, Identifier::vanilla_static("exposed_cut_copper_stairs"), Some(Block::EXPOSED_CUT_COPPER_STAIRS), 64);
    pub const WEATHERED_CUT_COPPER_STAIRS: Self = Self::new(108, Identifier::vanilla_static("weathered_cut_copper_stairs"), Some(Block::WEATHERED_CUT_COPPER_STAIRS), 64);
    pub const OXIDIZED_CUT_COPPER_STAIRS: Self = Self::new(109, Identifier::vanilla_static("oxidized_cut_copper_stairs"), Some(Block::OXIDIZED_CUT_COPPER_STAIRS), 64);
    pub const CUT_COPPER_SLAB: Self = Self::new(110, Identifier::vanilla_static("cut_copper_slab"), Some(Block::CUT_COPPER_SLAB), 64);
    pub const EXPOSED_CUT_COPPER_SLAB: Self = Self::new(111, Identifier::vanilla_static("exposed_cut_copper_slab"), Some(Block::EXPOSED_CUT_COPPER_SLAB), 64);
    pub const WEATHERED_CUT_COPPER_SLAB: Self = Self::new(112, Identifier::vanilla_static("weathered_cut_copper_slab"), Some(Block::WEATHERED_CUT_COPPER_SLAB), 64);
    pub const OXIDIZED_CUT_COPPER_SLAB: Self = Self::new(113, Identifier::vanilla_static("oxidized_cut_copper_slab"), Some(Block::OXIDIZED_CUT_COPPER_SLAB), 64);
    pub const WAXED_COPPER_BLOCK: Self = Self::new(114, Identifier::vanilla_static("waxed_copper_block"), Some(Block::WAXED_COPPER_BLOCK), 64);
    pub const WAXED_EXPOSED_COPPER: Self = Self::new(115, Identifier::vanilla_static("waxed_exposed_copper"), Some(Block::WAXED_EXPOSED_COPPER), 64);
    pub const WAXED_WEATHERED_COPPER: Self = Self::new(116, Identifier::vanilla_static("waxed_weathered_copper"), Some(Block::WAXED_WEATHERED_COPPER), 64);
    pub const WAXED_OXIDIZED_COPPER: Self = Self::new(117, Identifier::vanilla_static("waxed_oxidized_copper"), Some(Block::WAXED_OXIDIZED_COPPER), 64);
    pub const WAXED_CHISELED_COPPER: Self = Self::new(118, Identifier::vanilla_static("waxed_chiseled_copper"), Some(Block::WAXED_CHISELED_COPPER), 64);
    pub const WAXED_EXPOSED_CHISELED_COPPER: Self = Self::new(119, Identifier::vanilla_static("waxed_exposed_chiseled_copper"), Some(Block::WAXED_EXPOSED_CHISELED_COPPER), 64);
    pub const WAXED_WEATHERED_CHISELED_COPPER: Self = Self::new(120, Identifier::vanilla_static("waxed_weathered_chiseled_copper"), Some(Block::WAXED_WEATHERED_CHISELED_COPPER), 64);
    pub const WAXED_OXIDIZED_CHISELED_COPPER: Self = Self::new(121, Identifier::vanilla_static("waxed_oxidized_chiseled_copper"), Some(Block::WAXED_OXIDIZED_CHISELED_COPPER), 64);
    pub const WAXED_CUT_COPPER: Self = Self::new(122, Identifier::vanilla_static("waxed_cut_copper"), Some(Block::WAXED_CUT_COPPER), 64);
    pub const WAXED_EXPOSED_CUT_COPPER: Self = Self::new(123, Identifier::vanilla_static("waxed_exposed_cut_copper"), Some(Block::WAXED_EXPOSED_CUT_COPPER), 64);
    pub const WAXED_WEATHERED_CUT_COPPER: Self = Self::new(124, Identifier::vanilla_static("waxed_weathered_cut_copper"), Some(Block::WAXED_WEATHERED_CUT_COPPER), 64);
    pub const WAXED_OXIDIZED_CUT_COPPER: Self = Self::new(125, Identifier::vanilla_static("waxed_oxidized_cut_copper"), Some(Block::WAXED_OXIDIZED_CUT_COPPER), 64);
    pub const WAXED_CUT_COPPER_STAIRS: Self = Self::new(126, Identifier::vanilla_static("waxed_cut_copper_stairs"), Some(Block::WAXED_CUT_COPPER_STAIRS), 64);
    pub const WAXED_EXPOSED_CUT_COPPER_STAIRS: Self = Self::new(127, Identifier::vanilla_static("waxed_exposed_cut_copper_stairs"), Some(Block::WAXED_EXPOSED_CUT_COPPER_STAIRS), 64);
    pub const WAXED_WEATHERED_CUT_COPPER_STAIRS: Self = Self::new(128, Identifier::vanilla_static("waxed_weathered_cut_copper_stairs"), Some(Block::WAXED_WEATHERED_CUT_COPPER_STAIRS), 64);
    pub const WAXED_OXIDIZED_CUT_COPPER_STAIRS: Self = Self::new(129, Identifier::vanilla_static("waxed_oxidized_cut_copper_stairs"), Some(Block::WAXED_OXIDIZED_CUT_COPPER_STAIRS), 64);
    pub const WAXED_CUT_COPPER_SLAB: Self = Self::new(130, Identifier::vanilla_static("waxed_cut_copper_slab"), Some(Block::WAXED_CUT_COPPER_SLAB), 64);
    pub const WAXED_EXPOSED_CUT_COPPER_SLAB: Self = Self::new(131, Identifier::vanilla_static("waxed_exposed_cut_copper_slab"), Some(Block::WAXED_EXPOSED_CUT_COPPER_SLAB), 64);
    pub const WAXED_WEATHERED_CUT_COPPER_SLAB: Self = Self::new(132, Identifier::vanilla_static("waxed_weathered_cut_copper_slab"), Some(Block::WAXED_WEATHERED_CUT_COPPER_SLAB), 64);
    pub const WAXED_OXIDIZED_CUT_COPPER_SLAB: Self = Self::new(133, Identifier::vanilla_static("waxed_oxidized_cut_copper_slab"), Some(Block::WAXED_OXIDIZED_CUT_COPPER_SLAB), 64);
    pub const OAK_LOG: Self = Self::new(134, Identifier::vanilla_static("oak_log"), Some(Block::OAK_LOG), 64);
    pub const SPRUCE_LOG: Self = Self::new(135, Identifier::vanilla_static("spruce_log"), Some(Block::SPRUCE_LOG), 64);
    pub const BIRCH_LOG: Self = Self::new(136, Identifier::vanilla_static("birch_log"), Some(Block::BIRCH_LOG), 64);
    pub const JUNGLE_LOG: Self = Self::new(137, Identifier::vanilla_static("jungle_log"), Some(Block::JUNGLE_LOG), 64);
    pub const ACACIA_LOG: Self = Self::new(138, Identifier::vanilla_static("acacia_log"), Some(Block::ACACIA_LOG), 64);
    pub const CHERRY_LOG: Self = Self::new(139, Identifier::vanilla_static("cherry_log"), Some(Block::CHERRY_LOG), 64);
    pub const PALE_OAK_LOG: Self = Self::new(140, Identifier::vanilla_static("pale_oak_log"), Some(Block::PALE_OAK_LOG), 64);
    pub const DARK_OAK_LOG: Self = Self::new(141, Identifier::vanilla_static("dark_oak_log"), Some(Block::DARK_OAK_LOG), 64);
    pub const MANGROVE_LOG: Self = Self::new(142, Identifier::vanilla_static("mangrove_log"), Some(Block::MANGROVE_LOG), 64);
    pub const MANGROVE_ROOTS: Self = Self::new(143, Identifier::vanilla_static("mangrove_roots"), Some(Block::MANGROVE_ROOTS), 64);
    pub const MUDDY_MANGROVE_ROOTS: Self = Self::new(144, Identifier::vanilla_static("muddy_mangrove_roots"), Some(Block::MUDDY_MANGROVE_ROOTS), 64);
    pub const CRIMSON_STEM: Self = Self::new(145, Identifier::vanilla_static("crimson_stem"), Some(Block::CRIMSON_STEM), 64);
    pub const WARPED_STEM: Self = Self::new(146, Identifier::vanilla_static("warped_stem"), Some(Block::WARPED_STEM), 64);
    pub const BAMBOO_BLOCK: Self = Self::new(147, Identifier::vanilla_static("bamboo_block"), Some(Block::BAMBOO_BLOCK), 64);
    pub const STRIPPED_OAK_LOG: Self = Self::new(148, Identifier::vanilla_static("stripped_oak_log"), Some(Block::STRIPPED_OAK_LOG), 64);
    pub const STRIPPED_SPRUCE_LOG: Self = Self::new(149, Identifier::vanilla_static("stripped_spruce_log"), Some(Block::STRIPPED_SPRUCE_LOG), 64);
    pub const STRIPPED_BIRCH_LOG: Self = Self::new(150, Identifier::vanilla_static("stripped_birch_log"), Some(Block::STRIPPED_BIRCH_LOG), 64);
    pub const STRIPPED_JUNGLE_LOG: Self = Self::new(151, Identifier::vanilla_static("stripped_jungle_log"), Some(Block::STRIPPED_JUNGLE_LOG), 64);
    pub const STRIPPED_ACACIA_LOG: Self = Self::new(152, Identifier::vanilla_static("stripped_acacia_log"), Some(Block::STRIPPED_ACACIA_LOG), 64);
    pub const STRIPPED_CHERRY_LOG: Self = Self::new(153, Identifier::vanilla_static("stripped_cherry_log"), Some(Block::STRIPPED_CHERRY_LOG), 64);
    pub const STRIPPED_DARK_OAK_LOG: Self = Self::new(154, Identifier::vanilla_static("stripped_dark_oak_log"), Some(Block::STRIPPED_DARK_OAK_LOG), 64);
    pub const STRIPPED_PALE_OAK_LOG: Self = Self::new(155, Identifier::vanilla_static("stripped_pale_oak_log"), Some(Block::STRIPPED_PALE_OAK_LOG), 64);
    pub const STRIPPED_MANGROVE_LOG: Self = Self::new(156, Identifier::vanilla_static("stripped_mangrove_log"), Some(Block::STRIPPED_MANGROVE_LOG), 64);
    pub const STRIPPED_CRIMSON_STEM: Self = Self::new(157, Identifier::vanilla_static("stripped_crimson_stem"), Some(Block::STRIPPED_CRIMSON_STEM), 64);
    pub const STRIPPED_WARPED_STEM: Self = Self::new(158, Identifier::vanilla_static("stripped_warped_stem"), Some(Block::STRIPPED_WARPED_STEM), 64);
    pub const STRIPPED_OAK_WOOD: Self = Self::new(159, Identifier::vanilla_static("stripped_oak_wood"), Some(Block::STRIPPED_OAK_WOOD), 64);
    pub const STRIPPED_SPRUCE_WOOD: Self = Self::new(160, Identifier::vanilla_static("stripped_spruce_wood"), Some(Block::STRIPPED_SPRUCE_WOOD), 64);
    pub const STRIPPED_BIRCH_WOOD: Self = Self::new(161, Identifier::vanilla_static("stripped_birch_wood"), Some(Block::STRIPPED_BIRCH_WOOD), 64);
    pub const STRIPPED_JUNGLE_WOOD: Self = Self::new(162, Identifier::vanilla_static("stripped_jungle_wood"), Some(Block::STRIPPED_JUNGLE_WOOD), 64);
    pub const STRIPPED_ACACIA_WOOD: Self = Self::new(163, Identifier::vanilla_static("stripped_acacia_wood"), Some(Block::STRIPPED_ACACIA_WOOD), 64);
    pub const STRIPPED_CHERRY_WOOD: Self = Self::new(164, Identifier::vanilla_static("stripped_cherry_wood"), Some(Block::STRIPPED_CHERRY_WOOD), 64);
    pub const STRIPPED_DARK_OAK_WOOD: Self = Self::new(165, Identifier::vanilla_static("stripped_dark_oak_wood"), Some(Block::STRIPPED_DARK_OAK_WOOD), 64);
    pub const STRIPPED_PALE_OAK_WOOD: Self = Self::new(166, Identifier::vanilla_static("stripped_pale_oak_wood"), Some(Block::STRIPPED_PALE_OAK_WOOD), 64);
    pub const STRIPPED_MANGROVE_WOOD: Self = Self::new(167, Identifier::vanilla_static("stripped_mangrove_wood"), Some(Block::STRIPPED_MANGROVE_WOOD), 64);
    pub const STRIPPED_CRIMSON_HYPHAE: Self = Self::new(168, Identifier::vanilla_static("stripped_crimson_hyphae"), Some(Block::STRIPPED_CRIMSON_HYPHAE), 64);
    pub const STRIPPED_WARPED_HYPHAE: Self = Self::new(169, Identifier::vanilla_static("stripped_warped_hyphae"), Some(Block::STRIPPED_WARPED_HYPHAE), 64);
    pub const STRIPPED_BAMBOO_BLOCK: Self = Self::new(170, Identifier::vanilla_static("stripped_bamboo_block"), Some(Block::STRIPPED_BAMBOO_BLOCK), 64);
    pub const OAK_WOOD: Self = Self::new(171, Identifier::vanilla_static("oak_wood"), Some(Block::OAK_WOOD), 64);
    pub const SPRUCE_WOOD: Self = Self::new(172, Identifier::vanilla_static("spruce_wood"), Some(Block::SPRUCE_WOOD), 64);
    pub const BIRCH_WOOD: Self = Self::new(173, Identifier::vanilla_static("birch_wood"), Some(Block::BIRCH_WOOD), 64);
    pub const JUNGLE_WOOD: Self = Self::new(174, Identifier::vanilla_static("jungle_wood"), Some(Block::JUNGLE_WOOD), 64);
    pub const ACACIA_WOOD: Self = Self::new(175, Identifier::vanilla_static("acacia_wood"), Some(Block::ACACIA_WOOD), 64);
    pub const CHERRY_WOOD: Self = Self::new(176, Identifier::vanilla_static("cherry_wood"), Some(Block::CHERRY_WOOD), 64);
    pub const PALE_OAK_WOOD: Self = Self::new(177, Identifier::vanilla_static("pale_oak_wood"), Some(Block::PALE_OAK_WOOD), 64);
    pub const DARK_OAK_WOOD: Self = Self::new(178, Identifier::vanilla_static("dark_oak_wood"), Some(Block::DARK_OAK_WOOD), 64);
    pub const MANGROVE_WOOD: Self = Self::new(179, Identifier::vanilla_static("mangrove_wood"), Some(Block::MANGROVE_WOOD), 64);
    pub const CRIMSON_HYPHAE: Self = Self::new(180, Identifier::vanilla_static("crimson_hyphae"), Some(Block::CRIMSON_HYPHAE), 64);
    pub const WARPED_HYPHAE: Self = Self::new(181, Identifier::vanilla_static("warped_hyphae"), Some(Block::WARPED_HYPHAE), 64);
    pub const OAK_LEAVES: Self = Self::new(182, Identifier::vanilla_static("oak_leaves"), Some(Block::OAK_LEAVES), 64);
    pub const SPRUCE_LEAVES: Self = Self::new(183, Identifier::vanilla_static("spruce_leaves"), Some(Block::SPRUCE_LEAVES), 64);
    pub const BIRCH_LEAVES: Self = Self::new(184, Identifier::vanilla_static("birch_leaves"), Some(Block::BIRCH_LEAVES), 64);
    pub const JUNGLE_LEAVES: Self = Self::new(185, Identifier::vanilla_static("jungle_leaves"), Some(Block::JUNGLE_LEAVES), 64);
    pub const ACACIA_LEAVES: Self = Self::new(186, Identifier::vanilla_static("acacia_leaves"), Some(Block::ACACIA_LEAVES), 64);
    pub const CHERRY_LEAVES: Self = Self::new(187, Identifier::vanilla_static("cherry_leaves"), Some(Block::CHERRY_LEAVES), 64);
    pub const DARK_OAK_LEAVES: Self = Self::new(188, Identifier::vanilla_static("dark_oak_leaves"), Some(Block::DARK_OAK_LEAVES), 64);
    pub const PALE_OAK_LEAVES: Self = Self::new(189, Identifier::vanilla_static("pale_oak_leaves"), Some(Block::PALE_OAK_LEAVES), 64);
    pub const MANGROVE_LEAVES: Self = Self::new(190, Identifier::vanilla_static("mangrove_leaves"), Some(Block::MANGROVE_LEAVES), 64);
    pub const AZALEA_LEAVES: Self = Self::new(191, Identifier::vanilla_static("azalea_leaves"), Some(Block::AZALEA_LEAVES), 64);
    pub const FLOWERING_AZALEA_LEAVES: Self = Self::new(192, Identifier::vanilla_static("flowering_azalea_leaves"), Some(Block::FLOWERING_AZALEA_LEAVES), 64);
    pub const SPONGE: Self = Self::new(193, Identifier::vanilla_static("sponge"), Some(Block::SPONGE), 64);
    pub const WET_SPONGE: Self = Self::new(194, Identifier::vanilla_static("wet_sponge"), Some(Block::WET_SPONGE), 64);
    pub const GLASS: Self = Self::new(195, Identifier::vanilla_static("glass"), Some(Block::GLASS), 64);
    pub const TINTED_GLASS: Self = Self::new(196, Identifier::vanilla_static("tinted_glass"), Some(Block::TINTED_GLASS), 64);
    pub const LAPIS_BLOCK: Self = Self::new(197, Identifier::vanilla_static("lapis_block"), Some(Block::LAPIS_BLOCK), 64);
    pub const SANDSTONE: Self = Self::new(198, Identifier::vanilla_static("sandstone"), Some(Block::SANDSTONE), 64);
    pub const CHISELED_SANDSTONE: Self = Self::new(199, Identifier::vanilla_static("chiseled_sandstone"), Some(Block::CHISELED_SANDSTONE), 64);
    pub const CUT_SANDSTONE: Self = Self::new(200, Identifier::vanilla_static("cut_sandstone"), Some(Block::CUT_SANDSTONE), 64);
    pub const COBWEB: Self = Self::new(201, Identifier::vanilla_static("cobweb"), Some(Block::COBWEB), 64);
    pub const SHORT_GRASS: Self = Self::new(202, Identifier::vanilla_static("short_grass"), Some(Block::SHORT_GRASS), 64);
    pub const FERN: Self = Self::new(203, Identifier::vanilla_static("fern"), Some(Block::FERN), 64);
    pub const BUSH: Self = Self::new(204, Identifier::vanilla_static("bush"), Some(Block::BUSH), 64);
    pub const AZALEA: Self = Self::new(205, Identifier::vanilla_static("azalea"), Some(Block::AZALEA), 64);
    pub const FLOWERING_AZALEA: Self = Self::new(206, Identifier::vanilla_static("flowering_azalea"), Some(Block::FLOWERING_AZALEA), 64);
    pub const DEAD_BUSH: Self = Self::new(207, Identifier::vanilla_static("dead_bush"), Some(Block::DEAD_BUSH), 64);
    pub const FIREFLY_BUSH: Self = Self::new(208, Identifier::vanilla_static("firefly_bush"), Some(Block::FIREFLY_BUSH), 64);
    pub const SHORT_DRY_GRASS: Self = Self::new(209, Identifier::vanilla_static("short_dry_grass"), Some(Block::SHORT_DRY_GRASS), 64);
    pub const TALL_DRY_GRASS: Self = Self::new(210, Identifier::vanilla_static("tall_dry_grass"), Some(Block::TALL_DRY_GRASS), 64);
    pub const SEAGRASS: Self = Self::new(211, Identifier::vanilla_static("seagrass"), Some(Block::SEAGRASS), 64);
    pub const SEA_PICKLE: Self = Self::new(212, Identifier::vanilla_static("sea_pickle"), Some(Block::SEA_PICKLE), 64);
    pub const WHITE_WOOL: Self = Self::new(213, Identifier::vanilla_static("white_wool"), Some(Block::WHITE_WOOL), 64);
    pub const ORANGE_WOOL: Self = Self::new(214, Identifier::vanilla_static("orange_wool"), Some(Block::ORANGE_WOOL), 64);
    pub const MAGENTA_WOOL: Self = Self::new(215, Identifier::vanilla_static("magenta_wool"), Some(Block::MAGENTA_WOOL), 64);
    pub const LIGHT_BLUE_WOOL: Self = Self::new(216, Identifier::vanilla_static("light_blue_wool"), Some(Block::LIGHT_BLUE_WOOL), 64);
    pub const YELLOW_WOOL: Self = Self::new(217, Identifier::vanilla_static("yellow_wool"), Some(Block::YELLOW_WOOL), 64);
    pub const LIME_WOOL: Self = Self::new(218, Identifier::vanilla_static("lime_wool"), Some(Block::LIME_WOOL), 64);
    pub const PINK_WOOL: Self = Self::new(219, Identifier::vanilla_static("pink_wool"), Some(Block::PINK_WOOL), 64);
    pub const GRAY_WOOL: Self = Self::new(220, Identifier::vanilla_static("gray_wool"), Some(Block::GRAY_WOOL), 64);
    pub const LIGHT_GRAY_WOOL: Self = Self::new(221, Identifier::vanilla_static("light_gray_wool"), Some(Block::LIGHT_GRAY_WOOL), 64);
    pub const CYAN_WOOL: Self = Self::new(222, Identifier::vanilla_static("cyan_wool"), Some(Block::CYAN_WOOL), 64);
    pub const PURPLE_WOOL: Self = Self::new(223, Identifier::vanilla_static("purple_wool"), Some(Block::PURPLE_WOOL), 64);
    pub const BLUE_WOOL: Self = Self::new(224, Identifier::vanilla_static("blue_wool"), Some(Block::BLUE_WOOL), 64);
    pub const BROWN_WOOL: Self = Self::new(225, Identifier::vanilla_static("brown_wool"), Some(Block::BROWN_WOOL), 64);
    pub const GREEN_WOOL: Self = Self::new(226, Identifier::vanilla_static("green_wool"), Some(Block::GREEN_WOOL), 64);
    pub const RED_WOOL: Self = Self::new(227, Identifier::vanilla_static("red_wool"), Some(Block::RED_WOOL), 64);
    pub const BLACK_WOOL: Self = Self::new(228, Identifier::vanilla_static("black_wool"), Some(Block::BLACK_WOOL), 64);
    pub const DANDELION: Self = Self::new(229, Identifier::vanilla_static("dandelion"), Some(Block::DANDELION), 64);
    pub const OPEN_EYEBLOSSOM: Self = Self::new(230, Identifier::vanilla_static("open_eyeblossom"), Some(Block::OPEN_EYEBLOSSOM), 64);
    pub const CLOSED_EYEBLOSSOM: Self = Self::new(231, Identifier::vanilla_static("closed_eyeblossom"), Some(Block::CLOSED_EYEBLOSSOM), 64);
    pub const POPPY: Self = Self::new(232, Identifier::vanilla_static("poppy"), Some(Block::POPPY), 64);
    pub const BLUE_ORCHID: Self = Self::new(233, Identifier::vanilla_static("blue_orchid"), Some(Block::BLUE_ORCHID), 64);
    pub const ALLIUM: Self = Self::new(234, Identifier::vanilla_static("allium"), Some(Block::ALLIUM), 64);
    pub const AZURE_BLUET: Self = Self::new(235, Identifier::vanilla_static("azure_bluet"), Some(Block::AZURE_BLUET), 64);
    pub const RED_TULIP: Self = Self::new(236, Identifier::vanilla_static("red_tulip"), Some(Block::RED_TULIP), 64);
    pub const ORANGE_TULIP: Self = Self::new(237, Identifier::vanilla_static("orange_tulip"), Some(Block::ORANGE_TULIP), 64);
    pub const WHITE_TULIP: Self = Self::new(238, Identifier::vanilla_static("white_tulip"), Some(Block::WHITE_TULIP), 64);
    pub const PINK_TULIP: Self = Self::new(239, Identifier::vanilla_static("pink_tulip"), Some(Block::PINK_TULIP), 64);
    pub const OXEYE_DAISY: Self = Self::new(240, Identifier::vanilla_static("oxeye_daisy"), Some(Block::OXEYE_DAISY), 64);
    pub const CORNFLOWER: Self = Self::new(241, Identifier::vanilla_static("cornflower"), Some(Block::CORNFLOWER), 64);
    pub const LILY_OF_THE_VALLEY: Self = Self::new(242, Identifier::vanilla_static("lily_of_the_valley"), Some(Block::LILY_OF_THE_VALLEY), 64);
    pub const WITHER_ROSE: Self = Self::new(243, Identifier::vanilla_static("wither_rose"), Some(Block::WITHER_ROSE), 64);
    pub const TORCHFLOWER: Self = Self::new(244, Identifier::vanilla_static("torchflower"), Some(Block::TORCHFLOWER), 64);
    pub const PITCHER_PLANT: Self = Self::new(245, Identifier::vanilla_static("pitcher_plant"), Some(Block::PITCHER_PLANT), 64);
    pub const SPORE_BLOSSOM: Self = Self::new(246, Identifier::vanilla_static("spore_blossom"), Some(Block::SPORE_BLOSSOM), 64);
    pub const BROWN_MUSHROOM: Self = Self::new(247, Identifier::vanilla_static("brown_mushroom"), Some(Block::BROWN_MUSHROOM), 64);
    pub const RED_MUSHROOM: Self = Self::new(248, Identifier::vanilla_static("red_mushroom"), Some(Block::RED_MUSHROOM), 64);
    pub const CRIMSON_FUNGUS: Self = Self::new(249, Identifier::vanilla_static("crimson_fungus"), Some(Block::CRIMSON_FUNGUS), 64);
    pub const WARPED_FUNGUS: Self = Self::new(250, Identifier::vanilla_static("warped_fungus"), Some(Block::WARPED_FUNGUS), 64);
    pub const CRIMSON_ROOTS: Self = Self::new(251, Identifier::vanilla_static("crimson_roots"), Some(Block::CRIMSON_ROOTS), 64);
    pub const WARPED_ROOTS: Self = Self::new(252, Identifier::vanilla_static("warped_roots"), Some(Block::WARPED_ROOTS), 64);
    pub const NETHER_SPROUTS: Self = Self::new(253, Identifier::vanilla_static("nether_sprouts"), Some(Block::NETHER_SPROUTS), 64);
    pub const WEEPING_VINES: Self = Self::new(254, Identifier::vanilla_static("weeping_vines"), Some(Block::WEEPING_VINES), 64);
    pub const TWISTING_VINES: Self = Self::new(255, Identifier::vanilla_static("twisting_vines"), Some(Block::TWISTING_VINES), 64);
    pub const SUGAR_CANE: Self = Self::new(256, Identifier::vanilla_static("sugar_cane"), Some(Block::SUGAR_CANE), 64);
    pub const KELP: Self = Self::new(257, Identifier::vanilla_static("kelp"), Some(Block::KELP), 64);
    pub const PINK_PETALS: Self = Self::new(258, Identifier::vanilla_static("pink_petals"), Some(Block::PINK_PETALS), 64);
    pub const WILDFLOWERS: Self = Self::new(259, Identifier::vanilla_static("wildflowers"), Some(Block::WILDFLOWERS), 64);
    pub const LEAF_LITTER: Self = Self::new(260, Identifier::vanilla_static("leaf_litter"), Some(Block::LEAF_LITTER), 64);
    pub const MOSS_CARPET: Self = Self::new(261, Identifier::vanilla_static("moss_carpet"), Some(Block::MOSS_CARPET), 64);
    pub const MOSS_BLOCK: Self = Self::new(262, Identifier::vanilla_static("moss_block"), Some(Block::MOSS_BLOCK), 64);
    pub const PALE_MOSS_CARPET: Self = Self::new(263, Identifier::vanilla_static("pale_moss_carpet"), Some(Block::PALE_MOSS_CARPET), 64);
    pub const PALE_HANGING_MOSS: Self = Self::new(264, Identifier::vanilla_static("pale_hanging_moss"), Some(Block::PALE_HANGING_MOSS), 64);
    pub const PALE_MOSS_BLOCK: Self = Self::new(265, Identifier::vanilla_static("pale_moss_block"), Some(Block::PALE_MOSS_BLOCK), 64);
    pub const HANGING_ROOTS: Self = Self::new(266, Identifier::vanilla_static("hanging_roots"), Some(Block::HANGING_ROOTS), 64);
    pub const BIG_DRIPLEAF: Self = Self::new(267, Identifier::vanilla_static("big_dripleaf"), Some(Block::BIG_DRIPLEAF), 64);
    pub const SMALL_DRIPLEAF: Self = Self::new(268, Identifier::vanilla_static("small_dripleaf"), Some(Block::SMALL_DRIPLEAF), 64);
    pub const BAMBOO: Self = Self::new(269, Identifier::vanilla_static("bamboo"), Some(Block::BAMBOO), 64);
    pub const OAK_SLAB: Self = Self::new(270, Identifier::vanilla_static("oak_slab"), Some(Block::OAK_SLAB), 64);
    pub const SPRUCE_SLAB: Self = Self::new(271, Identifier::vanilla_static("spruce_slab"), Some(Block::SPRUCE_SLAB), 64);
    pub const BIRCH_SLAB: Self = Self::new(272, Identifier::vanilla_static("birch_slab"), Some(Block::BIRCH_SLAB), 64);
    pub const JUNGLE_SLAB: Self = Self::new(273, Identifier::vanilla_static("jungle_slab"), Some(Block::JUNGLE_SLAB), 64);
    pub const ACACIA_SLAB: Self = Self::new(274, Identifier::vanilla_static("acacia_slab"), Some(Block::ACACIA_SLAB), 64);
    pub const CHERRY_SLAB: Self = Self::new(275, Identifier::vanilla_static("cherry_slab"), Some(Block::CHERRY_SLAB), 64);
    pub const DARK_OAK_SLAB: Self = Self::new(276, Identifier::vanilla_static("dark_oak_slab"), Some(Block::DARK_OAK_SLAB), 64);
    pub const PALE_OAK_SLAB: Self = Self::new(277, Identifier::vanilla_static("pale_oak_slab"), Some(Block::PALE_OAK_SLAB), 64);
    pub const MANGROVE_SLAB: Self = Self::new(278, Identifier::vanilla_static("mangrove_slab"), Some(Block::MANGROVE_SLAB), 64);
    pub const BAMBOO_SLAB: Self = Self::new(279, Identifier::vanilla_static("bamboo_slab"), Some(Block::BAMBOO_SLAB), 64);
    pub const BAMBOO_MOSAIC_SLAB: Self = Self::new(280, Identifier::vanilla_static("bamboo_mosaic_slab"), Some(Block::BAMBOO_MOSAIC_SLAB), 64);
    pub const CRIMSON_SLAB: Self = Self::new(281, Identifier::vanilla_static("crimson_slab"), Some(Block::CRIMSON_SLAB), 64);
    pub const WARPED_SLAB: Self = Self::new(282, Identifier::vanilla_static("warped_slab"), Some(Block::WARPED_SLAB), 64);
    pub const STONE_SLAB: Self = Self::new(283, Identifier::vanilla_static("stone_slab"), Some(Block::STONE_SLAB), 64);
    pub const SMOOTH_STONE_SLAB: Self = Self::new(284, Identifier::vanilla_static("smooth_stone_slab"), Some(Block::SMOOTH_STONE_SLAB), 64);
    pub const SANDSTONE_SLAB: Self = Self::new(285, Identifier::vanilla_static("sandstone_slab"), Some(Block::SANDSTONE_SLAB), 64);
    pub const CUT_SANDSTONE_SLAB: Self = Self::new(286, Identifier::vanilla_static("cut_sandstone_slab"), Some(Block::CUT_SANDSTONE_SLAB), 64);
    pub const PETRIFIED_OAK_SLAB: Self = Self::new(287, Identifier::vanilla_static("petrified_oak_slab"), Some(Block::PETRIFIED_OAK_SLAB), 64);
    pub const COBBLESTONE_SLAB: Self = Self::new(288, Identifier::vanilla_static("cobblestone_slab"), Some(Block::COBBLESTONE_SLAB), 64);
    pub const BRICK_SLAB: Self = Self::new(289, Identifier::vanilla_static("brick_slab"), Some(Block::BRICK_SLAB), 64);
    pub const STONE_BRICK_SLAB: Self = Self::new(290, Identifier::vanilla_static("stone_brick_slab"), Some(Block::STONE_BRICK_SLAB), 64);
    pub const MUD_BRICK_SLAB: Self = Self::new(291, Identifier::vanilla_static("mud_brick_slab"), Some(Block::MUD_BRICK_SLAB), 64);
    pub const NETHER_BRICK_SLAB: Self = Self::new(292, Identifier::vanilla_static("nether_brick_slab"), Some(Block::NETHER_BRICK_SLAB), 64);
    pub const QUARTZ_SLAB: Self = Self::new(293, Identifier::vanilla_static("quartz_slab"), Some(Block::QUARTZ_SLAB), 64);
    pub const RED_SANDSTONE_SLAB: Self = Self::new(294, Identifier::vanilla_static("red_sandstone_slab"), Some(Block::RED_SANDSTONE_SLAB), 64);
    pub const CUT_RED_SANDSTONE_SLAB: Self = Self::new(295, Identifier::vanilla_static("cut_red_sandstone_slab"), Some(Block::CUT_RED_SANDSTONE_SLAB), 64);
    pub const PURPUR_SLAB: Self = Self::new(296, Identifier::vanilla_static("purpur_slab"), Some(Block::PURPUR_SLAB), 64);
    pub const PRISMARINE_SLAB: Self = Self::new(297, Identifier::vanilla_static("prismarine_slab"), Some(Block::PRISMARINE_SLAB), 64);
    pub const PRISMARINE_BRICK_SLAB: Self = Self::new(298, Identifier::vanilla_static("prismarine_brick_slab"), Some(Block::PRISMARINE_BRICK_SLAB), 64);
    pub const DARK_PRISMARINE_SLAB: Self = Self::new(299, Identifier::vanilla_static("dark_prismarine_slab"), Some(Block::DARK_PRISMARINE_SLAB), 64);
    pub const SMOOTH_QUARTZ: Self = Self::new(300, Identifier::vanilla_static("smooth_quartz"), Some(Block::SMOOTH_QUARTZ), 64);
    pub const SMOOTH_RED_SANDSTONE: Self = Self::new(301, Identifier::vanilla_static("smooth_red_sandstone"), Some(Block::SMOOTH_RED_SANDSTONE), 64);
    pub const SMOOTH_SANDSTONE: Self = Self::new(302, Identifier::vanilla_static("smooth_sandstone"), Some(Block::SMOOTH_SANDSTONE), 64);
    pub const SMOOTH_STONE: Self = Self::new(303, Identifier::vanilla_static("smooth_stone"), Some(Block::SMOOTH_STONE), 64);
    pub const BRICKS: Self = Self::new(304, Identifier::vanilla_static("bricks"), Some(Block::BRICKS), 64);
    pub const ACACIA_SHELF: Self = Self::new(305, Identifier::vanilla_static("acacia_shelf"), Some(Block::ACACIA_SHELF), 64);
    pub const BAMBOO_SHELF: Self = Self::new(306, Identifier::vanilla_static("bamboo_shelf"), Some(Block::BAMBOO_SHELF), 64);
    pub const BIRCH_SHELF: Self = Self::new(307, Identifier::vanilla_static("birch_shelf"), Some(Block::BIRCH_SHELF), 64);
    pub const CHERRY_SHELF: Self = Self::new(308, Identifier::vanilla_static("cherry_shelf"), Some(Block::CHERRY_SHELF), 64);
    pub const CRIMSON_SHELF: Self = Self::new(309, Identifier::vanilla_static("crimson_shelf"), Some(Block::CRIMSON_SHELF), 64);
    pub const DARK_OAK_SHELF: Self = Self::new(310, Identifier::vanilla_static("dark_oak_shelf"), Some(Block::DARK_OAK_SHELF), 64);
    pub const JUNGLE_SHELF: Self = Self::new(311, Identifier::vanilla_static("jungle_shelf"), Some(Block::JUNGLE_SHELF), 64);
    pub const MANGROVE_SHELF: Self = Self::new(312, Identifier::vanilla_static("mangrove_shelf"), Some(Block::MANGROVE_SHELF), 64);
    pub const OAK_SHELF: Self = Self::new(313, Identifier::vanilla_static("oak_shelf"), Some(Block::OAK_SHELF), 64);
    pub const PALE_OAK_SHELF: Self = Self::new(314, Identifier::vanilla_static("pale_oak_shelf"), Some(Block::PALE_OAK_SHELF), 64);
    pub const SPRUCE_SHELF: Self = Self::new(315, Identifier::vanilla_static("spruce_shelf"), Some(Block::SPRUCE_SHELF), 64);
    pub const WARPED_SHELF: Self = Self::new(316, Identifier::vanilla_static("warped_shelf"), Some(Block::WARPED_SHELF), 64);
    pub const BOOKSHELF: Self = Self::new(317, Identifier::vanilla_static("bookshelf"), Some(Block::BOOKSHELF), 64);
    pub const CHISELED_BOOKSHELF: Self = Self::new(318, Identifier::vanilla_static("chiseled_bookshelf"), Some(Block::CHISELED_BOOKSHELF), 64);
    pub const DECORATED_POT: Self = Self::new(319, Identifier::vanilla_static("decorated_pot"), Some(Block::DECORATED_POT), 64);
    pub const MOSSY_COBBLESTONE: Self = Self::new(320, Identifier::vanilla_static("mossy_cobblestone"), Some(Block::MOSSY_COBBLESTONE), 64);
    pub const OBSIDIAN: Self = Self::new(321, Identifier::vanilla_static("obsidian"), Some(Block::OBSIDIAN), 64);
    pub const TORCH: Self = Self::new(322, Identifier::vanilla_static("torch"), Some(Block::TORCH), 64);
    pub const END_ROD: Self = Self::new(323, Identifier::vanilla_static("end_rod"), Some(Block::END_ROD), 64);
    pub const CHORUS_PLANT: Self = Self::new(324, Identifier::vanilla_static("chorus_plant"), Some(Block::CHORUS_PLANT), 64);
    pub const CHORUS_FLOWER: Self = Self::new(325, Identifier::vanilla_static("chorus_flower"), Some(Block::CHORUS_FLOWER), 64);
    pub const PURPUR_BLOCK: Self = Self::new(326, Identifier::vanilla_static("purpur_block"), Some(Block::PURPUR_BLOCK), 64);
    pub const PURPUR_PILLAR: Self = Self::new(327, Identifier::vanilla_static("purpur_pillar"), Some(Block::PURPUR_PILLAR), 64);
    pub const PURPUR_STAIRS: Self = Self::new(328, Identifier::vanilla_static("purpur_stairs"), Some(Block::PURPUR_STAIRS), 64);
    pub const SPAWNER: Self = Self::new(329, Identifier::vanilla_static("spawner"), Some(Block::SPAWNER), 64);
    pub const CREAKING_HEART: Self = Self::new(330, Identifier::vanilla_static("creaking_heart"), Some(Block::CREAKING_HEART), 64);
    pub const CHEST: Self = Self::new(331, Identifier::vanilla_static("chest"), Some(Block::CHEST), 64);
    pub const CRAFTING_TABLE: Self = Self::new(332, Identifier::vanilla_static("crafting_table"), Some(Block::CRAFTING_TABLE), 64);
    pub const FARMLAND: Self = Self::new(333, Identifier::vanilla_static("farmland"), Some(Block::FARMLAND), 64);
    pub const FURNACE: Self = Self::new(334, Identifier::vanilla_static("furnace"), Some(Block::FURNACE), 64);
    pub const LADDER: Self = Self::new(335, Identifier::vanilla_static("ladder"), Some(Block::LADDER), 64);
    pub const COBBLESTONE_STAIRS: Self = Self::new(336, Identifier::vanilla_static("cobblestone_stairs"), Some(Block::COBBLESTONE_STAIRS), 64);
    pub const SNOW: Self = Self::new(337, Identifier::vanilla_static("snow"), Some(Block::SNOW), 64);
    pub const ICE: Self = Self::new(338, Identifier::vanilla_static("ice"), Some(Block::ICE), 64);
    pub const SNOW_BLOCK: Self = Self::new(339, Identifier::vanilla_static("snow_block"), Some(Block::SNOW_BLOCK), 64);
    pub const CACTUS: Self = Self::new(340, Identifier::vanilla_static("cactus"), Some(Block::CACTUS), 64);
    pub const CACTUS_FLOWER: Self = Self::new(341, Identifier::vanilla_static("cactus_flower"), Some(Block::CACTUS_FLOWER), 64);
    pub const CLAY: Self = Self::new(342, Identifier::vanilla_static("clay"), Some(Block::CLAY), 64);
    pub const JUKEBOX: Self = Self::new(343, Identifier::vanilla_static("jukebox"), Some(Block::JUKEBOX), 64);
    pub const OAK_FENCE: Self = Self::new(344, Identifier::vanilla_static("oak_fence"), Some(Block::OAK_FENCE), 64);
    pub const SPRUCE_FENCE: Self = Self::new(345, Identifier::vanilla_static("spruce_fence"), Some(Block::SPRUCE_FENCE), 64);
    pub const BIRCH_FENCE: Self = Self::new(346, Identifier::vanilla_static("birch_fence"), Some(Block::BIRCH_FENCE), 64);
    pub const JUNGLE_FENCE: Self = Self::new(347, Identifier::vanilla_static("jungle_fence"), Some(Block::JUNGLE_FENCE), 64);
    pub const ACACIA_FENCE: Self = Self::new(348, Identifier::vanilla_static("acacia_fence"), Some(Block::ACACIA_FENCE), 64);
    pub const CHERRY_FENCE: Self = Self::new(349, Identifier::vanilla_static("cherry_fence"), Some(Block::CHERRY_FENCE), 64);
    pub const DARK_OAK_FENCE: Self = Self::new(350, Identifier::vanilla_static("dark_oak_fence"), Some(Block::DARK_OAK_FENCE), 64);
    pub const PALE_OAK_FENCE: Self = Self::new(351, Identifier::vanilla_static("pale_oak_fence"), Some(Block::PALE_OAK_FENCE), 64);
    pub const MANGROVE_FENCE: Self = Self::new(352, Identifier::vanilla_static("mangrove_fence"), Some(Block::MANGROVE_FENCE), 64);
    pub const BAMBOO_FENCE: Self = Self::new(353, Identifier::vanilla_static("bamboo_fence"), Some(Block::BAMBOO_FENCE), 64);
    pub const CRIMSON_FENCE: Self = Self::new(354, Identifier::vanilla_static("crimson_fence"), Some(Block::CRIMSON_FENCE), 64);
    pub const WARPED_FENCE: Self = Self::new(355, Identifier::vanilla_static("warped_fence"), Some(Block::WARPED_FENCE), 64);
    pub const PUMPKIN: Self = Self::new(356, Identifier::vanilla_static("pumpkin"), Some(Block::PUMPKIN), 64);
    pub const CARVED_PUMPKIN: Self = Self::new(357, Identifier::vanilla_static("carved_pumpkin"), Some(Block::CARVED_PUMPKIN), 64);
    pub const JACK_O_LANTERN: Self = Self::new(358, Identifier::vanilla_static("jack_o_lantern"), Some(Block::JACK_O_LANTERN), 64);
    pub const NETHERRACK: Self = Self::new(359, Identifier::vanilla_static("netherrack"), Some(Block::NETHERRACK), 64);
    pub const SOUL_SAND: Self = Self::new(360, Identifier::vanilla_static("soul_sand"), Some(Block::SOUL_SAND), 64);
    pub const SOUL_SOIL: Self = Self::new(361, Identifier::vanilla_static("soul_soil"), Some(Block::SOUL_SOIL), 64);
    pub const BASALT: Self = Self::new(362, Identifier::vanilla_static("basalt"), Some(Block::BASALT), 64);
    pub const POLISHED_BASALT: Self = Self::new(363, Identifier::vanilla_static("polished_basalt"), Some(Block::POLISHED_BASALT), 64);
    pub const SMOOTH_BASALT: Self = Self::new(364, Identifier::vanilla_static("smooth_basalt"), Some(Block::SMOOTH_BASALT), 64);
    pub const SOUL_TORCH: Self = Self::new(365, Identifier::vanilla_static("soul_torch"), Some(Block::SOUL_TORCH), 64);
    pub const COPPER_TORCH: Self = Self::new(366, Identifier::vanilla_static("copper_torch"), Some(Block::COPPER_TORCH), 64);
    pub const GLOWSTONE: Self = Self::new(367, Identifier::vanilla_static("glowstone"), Some(Block::GLOWSTONE), 64);
    pub const INFESTED_STONE: Self = Self::new(368, Identifier::vanilla_static("infested_stone"), Some(Block::INFESTED_STONE), 64);
    pub const INFESTED_COBBLESTONE: Self = Self::new(369, Identifier::vanilla_static("infested_cobblestone"), Some(Block::INFESTED_COBBLESTONE), 64);
    pub const INFESTED_STONE_BRICKS: Self = Self::new(370, Identifier::vanilla_static("infested_stone_bricks"), Some(Block::INFESTED_STONE_BRICKS), 64);
    pub const INFESTED_MOSSY_STONE_BRICKS: Self = Self::new(371, Identifier::vanilla_static("infested_mossy_stone_bricks"), Some(Block::INFESTED_MOSSY_STONE_BRICKS), 64);
    pub const INFESTED_CRACKED_STONE_BRICKS: Self = Self::new(372, Identifier::vanilla_static("infested_cracked_stone_bricks"), Some(Block::INFESTED_CRACKED_STONE_BRICKS), 64);
    pub const INFESTED_CHISELED_STONE_BRICKS: Self = Self::new(373, Identifier::vanilla_static("infested_chiseled_stone_bricks"), Some(Block::INFESTED_CHISELED_STONE_BRICKS), 64);
    pub const INFESTED_DEEPSLATE: Self = Self::new(374, Identifier::vanilla_static("infested_deepslate"), Some(Block::INFESTED_DEEPSLATE), 64);
    pub const STONE_BRICKS: Self = Self::new(375, Identifier::vanilla_static("stone_bricks"), Some(Block::STONE_BRICKS), 64);
    pub const MOSSY_STONE_BRICKS: Self = Self::new(376, Identifier::vanilla_static("mossy_stone_bricks"), Some(Block::MOSSY_STONE_BRICKS), 64);
    pub const CRACKED_STONE_BRICKS: Self = Self::new(377, Identifier::vanilla_static("cracked_stone_bricks"), Some(Block::CRACKED_STONE_BRICKS), 64);
    pub const CHISELED_STONE_BRICKS: Self = Self::new(378, Identifier::vanilla_static("chiseled_stone_bricks"), Some(Block::CHISELED_STONE_BRICKS), 64);
    pub const PACKED_MUD: Self = Self::new(379, Identifier::vanilla_static("packed_mud"), Some(Block::PACKED_MUD), 64);
    pub const MUD_BRICKS: Self = Self::new(380, Identifier::vanilla_static("mud_bricks"), Some(Block::MUD_BRICKS), 64);
    pub const DEEPSLATE_BRICKS: Self = Self::new(381, Identifier::vanilla_static("deepslate_bricks"), Some(Block::DEEPSLATE_BRICKS), 64);
    pub const CRACKED_DEEPSLATE_BRICKS: Self = Self::new(382, Identifier::vanilla_static("cracked_deepslate_bricks"), Some(Block::CRACKED_DEEPSLATE_BRICKS), 64);
    pub const DEEPSLATE_TILES: Self = Self::new(383, Identifier::vanilla_static("deepslate_tiles"), Some(Block::DEEPSLATE_TILES), 64);
    pub const CRACKED_DEEPSLATE_TILES: Self = Self::new(384, Identifier::vanilla_static("cracked_deepslate_tiles"), Some(Block::CRACKED_DEEPSLATE_TILES), 64);
    pub const CHISELED_DEEPSLATE: Self = Self::new(385, Identifier::vanilla_static("chiseled_deepslate"), Some(Block::CHISELED_DEEPSLATE), 64);
    pub const REINFORCED_DEEPSLATE: Self = Self::new(386, Identifier::vanilla_static("reinforced_deepslate"), Some(Block::REINFORCED_DEEPSLATE), 64);
    pub const BROWN_MUSHROOM_BLOCK: Self = Self::new(387, Identifier::vanilla_static("brown_mushroom_block"), Some(Block::BROWN_MUSHROOM_BLOCK), 64);
    pub const RED_MUSHROOM_BLOCK: Self = Self::new(388, Identifier::vanilla_static("red_mushroom_block"), Some(Block::RED_MUSHROOM_BLOCK), 64);
    pub const MUSHROOM_STEM: Self = Self::new(389, Identifier::vanilla_static("mushroom_stem"), Some(Block::MUSHROOM_STEM), 64);
    pub const IRON_BARS: Self = Self::new(390, Identifier::vanilla_static("iron_bars"), Some(Block::IRON_BARS), 64);
    pub const COPPER_BARS: Self = Self::new(391, Identifier::vanilla_static("copper_bars"), Some(Block::COPPER_BARS), 64);
    pub const EXPOSED_COPPER_BARS: Self = Self::new(392, Identifier::vanilla_static("exposed_copper_bars"), Some(Block::EXPOSED_COPPER_BARS), 64);
    pub const WEATHERED_COPPER_BARS: Self = Self::new(393, Identifier::vanilla_static("weathered_copper_bars"), Some(Block::WEATHERED_COPPER_BARS), 64);
    pub const OXIDIZED_COPPER_BARS: Self = Self::new(394, Identifier::vanilla_static("oxidized_copper_bars"), Some(Block::OXIDIZED_COPPER_BARS), 64);
    pub const WAXED_COPPER_BARS: Self = Self::new(395, Identifier::vanilla_static("waxed_copper_bars"), Some(Block::WAXED_COPPER_BARS), 64);
    pub const WAXED_EXPOSED_COPPER_BARS: Self = Self::new(396, Identifier::vanilla_static("waxed_exposed_copper_bars"), Some(Block::WAXED_EXPOSED_COPPER_BARS), 64);
    pub const WAXED_WEATHERED_COPPER_BARS: Self = Self::new(397, Identifier::vanilla_static("waxed_weathered_copper_bars"), Some(Block::WAXED_WEATHERED_COPPER_BARS), 64);
    pub const WAXED_OXIDIZED_COPPER_BARS: Self = Self::new(398, Identifier::vanilla_static("waxed_oxidized_copper_bars"), Some(Block::WAXED_OXIDIZED_COPPER_BARS), 64);
    pub const IRON_CHAIN: Self = Self::new(399, Identifier::vanilla_static("iron_chain"), Some(Block::IRON_CHAIN), 64);
    pub const COPPER_CHAIN: Self = Self::new(400, Identifier::vanilla_static("copper_chain"), Some(Block::COPPER_CHAIN), 64);
    pub const EXPOSED_COPPER_CHAIN: Self = Self::new(401, Identifier::vanilla_static("exposed_copper_chain"), Some(Block::EXPOSED_COPPER_CHAIN), 64);
    pub const WEATHERED_COPPER_CHAIN: Self = Self::new(402, Identifier::vanilla_static("weathered_copper_chain"), Some(Block::WEATHERED_COPPER_CHAIN), 64);
    pub const OXIDIZED_COPPER_CHAIN: Self = Self::new(403, Identifier::vanilla_static("oxidized_copper_chain"), Some(Block::OXIDIZED_COPPER_CHAIN), 64);
    pub const WAXED_COPPER_CHAIN: Self = Self::new(404, Identifier::vanilla_static("waxed_copper_chain"), Some(Block::WAXED_COPPER_CHAIN), 64);
    pub const WAXED_EXPOSED_COPPER_CHAIN: Self = Self::new(405, Identifier::vanilla_static("waxed_exposed_copper_chain"), Some(Block::WAXED_EXPOSED_COPPER_CHAIN), 64);
    pub const WAXED_WEATHERED_COPPER_CHAIN: Self = Self::new(406, Identifier::vanilla_static("waxed_weathered_copper_chain"), Some(Block::WAXED_WEATHERED_COPPER_CHAIN), 64);
    pub const WAXED_OXIDIZED_COPPER_CHAIN: Self = Self::new(407, Identifier::vanilla_static("waxed_oxidized_copper_chain"), Some(Block::WAXED_OXIDIZED_COPPER_CHAIN), 64);
    pub const GLASS_PANE: Self = Self::new(408, Identifier::vanilla_static("glass_pane"), Some(Block::GLASS_PANE), 64);
    pub const MELON: Self = Self::new(409, Identifier::vanilla_static("melon"), Some(Block::MELON), 64);
    pub const VINE: Self = Self::new(410, Identifier::vanilla_static("vine"), Some(Block::VINE), 64);
    pub const GLOW_LICHEN: Self = Self::new(411, Identifier::vanilla_static("glow_lichen"), Some(Block::GLOW_LICHEN), 64);
    pub const RESIN_CLUMP: Self = Self::new(412, Identifier::vanilla_static("resin_clump"), Some(Block::RESIN_CLUMP), 64);
    pub const RESIN_BLOCK: Self = Self::new(413, Identifier::vanilla_static("resin_block"), Some(Block::RESIN_BLOCK), 64);
    pub const RESIN_BRICKS: Self = Self::new(414, Identifier::vanilla_static("resin_bricks"), Some(Block::RESIN_BRICKS), 64);
    pub const RESIN_BRICK_STAIRS: Self = Self::new(415, Identifier::vanilla_static("resin_brick_stairs"), Some(Block::RESIN_BRICK_STAIRS), 64);
    pub const RESIN_BRICK_SLAB: Self = Self::new(416, Identifier::vanilla_static("resin_brick_slab"), Some(Block::RESIN_BRICK_SLAB), 64);
    pub const RESIN_BRICK_WALL: Self = Self::new(417, Identifier::vanilla_static("resin_brick_wall"), Some(Block::RESIN_BRICK_WALL), 64);
    pub const CHISELED_RESIN_BRICKS: Self = Self::new(418, Identifier::vanilla_static("chiseled_resin_bricks"), Some(Block::CHISELED_RESIN_BRICKS), 64);
    pub const BRICK_STAIRS: Self = Self::new(419, Identifier::vanilla_static("brick_stairs"), Some(Block::BRICK_STAIRS), 64);
    pub const STONE_BRICK_STAIRS: Self = Self::new(420, Identifier::vanilla_static("stone_brick_stairs"), Some(Block::STONE_BRICK_STAIRS), 64);
    pub const MUD_BRICK_STAIRS: Self = Self::new(421, Identifier::vanilla_static("mud_brick_stairs"), Some(Block::MUD_BRICK_STAIRS), 64);
    pub const MYCELIUM: Self = Self::new(422, Identifier::vanilla_static("mycelium"), Some(Block::MYCELIUM), 64);
    pub const LILY_PAD: Self = Self::new(423, Identifier::vanilla_static("lily_pad"), Some(Block::LILY_PAD), 64);
    pub const NETHER_BRICKS: Self = Self::new(424, Identifier::vanilla_static("nether_bricks"), Some(Block::NETHER_BRICKS), 64);
    pub const CRACKED_NETHER_BRICKS: Self = Self::new(425, Identifier::vanilla_static("cracked_nether_bricks"), Some(Block::CRACKED_NETHER_BRICKS), 64);
    pub const CHISELED_NETHER_BRICKS: Self = Self::new(426, Identifier::vanilla_static("chiseled_nether_bricks"), Some(Block::CHISELED_NETHER_BRICKS), 64);
    pub const NETHER_BRICK_FENCE: Self = Self::new(427, Identifier::vanilla_static("nether_brick_fence"), Some(Block::NETHER_BRICK_FENCE), 64);
    pub const NETHER_BRICK_STAIRS: Self = Self::new(428, Identifier::vanilla_static("nether_brick_stairs"), Some(Block::NETHER_BRICK_STAIRS), 64);
    pub const SCULK: Self = Self::new(429, Identifier::vanilla_static("sculk"), Some(Block::SCULK), 64);
    pub const SCULK_VEIN: Self = Self::new(430, Identifier::vanilla_static("sculk_vein"), Some(Block::SCULK_VEIN), 64);
    pub const SCULK_CATALYST: Self = Self::new(431, Identifier::vanilla_static("sculk_catalyst"), Some(Block::SCULK_CATALYST), 64);
    pub const SCULK_SHRIEKER: Self = Self::new(432, Identifier::vanilla_static("sculk_shrieker"), Some(Block::SCULK_SHRIEKER), 64);
    pub const ENCHANTING_TABLE: Self = Self::new(433, Identifier::vanilla_static("enchanting_table"), Some(Block::ENCHANTING_TABLE), 64);
    pub const END_PORTAL_FRAME: Self = Self::new(434, Identifier::vanilla_static("end_portal_frame"), Some(Block::END_PORTAL_FRAME), 64);
    pub const END_STONE: Self = Self::new(435, Identifier::vanilla_static("end_stone"), Some(Block::END_STONE), 64);
    pub const END_STONE_BRICKS: Self = Self::new(436, Identifier::vanilla_static("end_stone_bricks"), Some(Block::END_STONE_BRICKS), 64);
    pub const DRAGON_EGG: Self = Self::new(437, Identifier::vanilla_static("dragon_egg"), Some(Block::DRAGON_EGG), 64);
    pub const SANDSTONE_STAIRS: Self = Self::new(438, Identifier::vanilla_static("sandstone_stairs"), Some(Block::SANDSTONE_STAIRS), 64);
    pub const ENDER_CHEST: Self = Self::new(439, Identifier::vanilla_static("ender_chest"), Some(Block::ENDER_CHEST), 64);
    pub const EMERALD_BLOCK: Self = Self::new(440, Identifier::vanilla_static("emerald_block"), Some(Block::EMERALD_BLOCK), 64);
    pub const OAK_STAIRS: Self = Self::new(441, Identifier::vanilla_static("oak_stairs"), Some(Block::OAK_STAIRS), 64);
    pub const SPRUCE_STAIRS: Self = Self::new(442, Identifier::vanilla_static("spruce_stairs"), Some(Block::SPRUCE_STAIRS), 64);
    pub const BIRCH_STAIRS: Self = Self::new(443, Identifier::vanilla_static("birch_stairs"), Some(Block::BIRCH_STAIRS), 64);
    pub const JUNGLE_STAIRS: Self = Self::new(444, Identifier::vanilla_static("jungle_stairs"), Some(Block::JUNGLE_STAIRS), 64);
    pub const ACACIA_STAIRS: Self = Self::new(445, Identifier::vanilla_static("acacia_stairs"), Some(Block::ACACIA_STAIRS), 64);
    pub const CHERRY_STAIRS: Self = Self::new(446, Identifier::vanilla_static("cherry_stairs"), Some(Block::CHERRY_STAIRS), 64);
    pub const DARK_OAK_STAIRS: Self = Self::new(447, Identifier::vanilla_static("dark_oak_stairs"), Some(Block::DARK_OAK_STAIRS), 64);
    pub const PALE_OAK_STAIRS: Self = Self::new(448, Identifier::vanilla_static("pale_oak_stairs"), Some(Block::PALE_OAK_STAIRS), 64);
    pub const MANGROVE_STAIRS: Self = Self::new(449, Identifier::vanilla_static("mangrove_stairs"), Some(Block::MANGROVE_STAIRS), 64);
    pub const BAMBOO_STAIRS: Self = Self::new(450, Identifier::vanilla_static("bamboo_stairs"), Some(Block::BAMBOO_STAIRS), 64);
    pub const BAMBOO_MOSAIC_STAIRS: Self = Self::new(451, Identifier::vanilla_static("bamboo_mosaic_stairs"), Some(Block::BAMBOO_MOSAIC_STAIRS), 64);
    pub const CRIMSON_STAIRS: Self = Self::new(452, Identifier::vanilla_static("crimson_stairs"), Some(Block::CRIMSON_STAIRS), 64);
    pub const WARPED_STAIRS: Self = Self::new(453, Identifier::vanilla_static("warped_stairs"), Some(Block::WARPED_STAIRS), 64);
    pub const COMMAND_BLOCK: Self = Self::new(454, Identifier::vanilla_static("command_block"), Some(Block::COMMAND_BLOCK), 64);
    pub const BEACON: Self = Self::new(455, Identifier::vanilla_static("beacon"), Some(Block::BEACON), 64);
    pub const COBBLESTONE_WALL: Self = Self::new(456, Identifier::vanilla_static("cobblestone_wall"), Some(Block::COBBLESTONE_WALL), 64);
    pub const MOSSY_COBBLESTONE_WALL: Self = Self::new(457, Identifier::vanilla_static("mossy_cobblestone_wall"), Some(Block::MOSSY_COBBLESTONE_WALL), 64);
    pub const BRICK_WALL: Self = Self::new(458, Identifier::vanilla_static("brick_wall"), Some(Block::BRICK_WALL), 64);
    pub const PRISMARINE_WALL: Self = Self::new(459, Identifier::vanilla_static("prismarine_wall"), Some(Block::PRISMARINE_WALL), 64);
    pub const RED_SANDSTONE_WALL: Self = Self::new(460, Identifier::vanilla_static("red_sandstone_wall"), Some(Block::RED_SANDSTONE_WALL), 64);
    pub const MOSSY_STONE_BRICK_WALL: Self = Self::new(461, Identifier::vanilla_static("mossy_stone_brick_wall"), Some(Block::MOSSY_STONE_BRICK_WALL), 64);
    pub const GRANITE_WALL: Self = Self::new(462, Identifier::vanilla_static("granite_wall"), Some(Block::GRANITE_WALL), 64);
    pub const STONE_BRICK_WALL: Self = Self::new(463, Identifier::vanilla_static("stone_brick_wall"), Some(Block::STONE_BRICK_WALL), 64);
    pub const MUD_BRICK_WALL: Self = Self::new(464, Identifier::vanilla_static("mud_brick_wall"), Some(Block::MUD_BRICK_WALL), 64);
    pub const NETHER_BRICK_WALL: Self = Self::new(465, Identifier::vanilla_static("nether_brick_wall"), Some(Block::NETHER_BRICK_WALL), 64);
    pub const ANDESITE_WALL: Self = Self::new(466, Identifier::vanilla_static("andesite_wall"), Some(Block::ANDESITE_WALL), 64);
    pub const RED_NETHER_BRICK_WALL: Self = Self::new(467, Identifier::vanilla_static("red_nether_brick_wall"), Some(Block::RED_NETHER_BRICK_WALL), 64);
    pub const SANDSTONE_WALL: Self = Self::new(468, Identifier::vanilla_static("sandstone_wall"), Some(Block::SANDSTONE_WALL), 64);
    pub const END_STONE_BRICK_WALL: Self = Self::new(469, Identifier::vanilla_static("end_stone_brick_wall"), Some(Block::END_STONE_BRICK_WALL), 64);
    pub const DIORITE_WALL: Self = Self::new(470, Identifier::vanilla_static("diorite_wall"), Some(Block::DIORITE_WALL), 64);
    pub const BLACKSTONE_WALL: Self = Self::new(471, Identifier::vanilla_static("blackstone_wall"), Some(Block::BLACKSTONE_WALL), 64);
    pub const POLISHED_BLACKSTONE_WALL: Self = Self::new(472, Identifier::vanilla_static("polished_blackstone_wall"), Some(Block::POLISHED_BLACKSTONE_WALL), 64);
    pub const POLISHED_BLACKSTONE_BRICK_WALL: Self = Self::new(473, Identifier::vanilla_static("polished_blackstone_brick_wall"), Some(Block::POLISHED_BLACKSTONE_BRICK_WALL), 64);
    pub const COBBLED_DEEPSLATE_WALL: Self = Self::new(474, Identifier::vanilla_static("cobbled_deepslate_wall"), Some(Block::COBBLED_DEEPSLATE_WALL), 64);
    pub const POLISHED_DEEPSLATE_WALL: Self = Self::new(475, Identifier::vanilla_static("polished_deepslate_wall"), Some(Block::POLISHED_DEEPSLATE_WALL), 64);
    pub const DEEPSLATE_BRICK_WALL: Self = Self::new(476, Identifier::vanilla_static("deepslate_brick_wall"), Some(Block::DEEPSLATE_BRICK_WALL), 64);
    pub const DEEPSLATE_TILE_WALL: Self = Self::new(477, Identifier::vanilla_static("deepslate_tile_wall"), Some(Block::DEEPSLATE_TILE_WALL), 64);
    pub const ANVIL: Self = Self::new(478, Identifier::vanilla_static("anvil"), Some(Block::ANVIL), 64);
    pub const CHIPPED_ANVIL: Self = Self::new(479, Identifier::vanilla_static("chipped_anvil"), Some(Block::CHIPPED_ANVIL), 64);
    pub const DAMAGED_ANVIL: Self = Self::new(480, Identifier::vanilla_static("damaged_anvil"), Some(Block::DAMAGED_ANVIL), 64);
    pub const CHISELED_QUARTZ_BLOCK: Self = Self::new(481, Identifier::vanilla_static("chiseled_quartz_block"), Some(Block::CHISELED_QUARTZ_BLOCK), 64);
    pub const QUARTZ_BLOCK: Self = Self::new(482, Identifier::vanilla_static("quartz_block"), Some(Block::QUARTZ_BLOCK), 64);
    pub const QUARTZ_BRICKS: Self = Self::new(483, Identifier::vanilla_static("quartz_bricks"), Some(Block::QUARTZ_BRICKS), 64);
    pub const QUARTZ_PILLAR: Self = Self::new(484, Identifier::vanilla_static("quartz_pillar"), Some(Block::QUARTZ_PILLAR), 64);
    pub const QUARTZ_STAIRS: Self = Self::new(485, Identifier::vanilla_static("quartz_stairs"), Some(Block::QUARTZ_STAIRS), 64);
    pub const WHITE_TERRACOTTA: Self = Self::new(486, Identifier::vanilla_static("white_terracotta"), Some(Block::WHITE_TERRACOTTA), 64);
    pub const ORANGE_TERRACOTTA: Self = Self::new(487, Identifier::vanilla_static("orange_terracotta"), Some(Block::ORANGE_TERRACOTTA), 64);
    pub const MAGENTA_TERRACOTTA: Self = Self::new(488, Identifier::vanilla_static("magenta_terracotta"), Some(Block::MAGENTA_TERRACOTTA), 64);
    pub const LIGHT_BLUE_TERRACOTTA: Self = Self::new(489, Identifier::vanilla_static("light_blue_terracotta"), Some(Block::LIGHT_BLUE_TERRACOTTA), 64);
    pub const YELLOW_TERRACOTTA: Self = Self::new(490, Identifier::vanilla_static("yellow_terracotta"), Some(Block::YELLOW_TERRACOTTA), 64);
    pub const LIME_TERRACOTTA: Self = Self::new(491, Identifier::vanilla_static("lime_terracotta"), Some(Block::LIME_TERRACOTTA), 64);
    pub const PINK_TERRACOTTA: Self = Self::new(492, Identifier::vanilla_static("pink_terracotta"), Some(Block::PINK_TERRACOTTA), 64);
    pub const GRAY_TERRACOTTA: Self = Self::new(493, Identifier::vanilla_static("gray_terracotta"), Some(Block::GRAY_TERRACOTTA), 64);
    pub const LIGHT_GRAY_TERRACOTTA: Self = Self::new(494, Identifier::vanilla_static("light_gray_terracotta"), Some(Block::LIGHT_GRAY_TERRACOTTA), 64);
    pub const CYAN_TERRACOTTA: Self = Self::new(495, Identifier::vanilla_static("cyan_terracotta"), Some(Block::CYAN_TERRACOTTA), 64);
    pub const PURPLE_TERRACOTTA: Self = Self::new(496, Identifier::vanilla_static("purple_terracotta"), Some(Block::PURPLE_TERRACOTTA), 64);
    pub const BLUE_TERRACOTTA: Self = Self::new(497, Identifier::vanilla_static("blue_terracotta"), Some(Block::BLUE_TERRACOTTA), 64);
    pub const BROWN_TERRACOTTA: Self = Self::new(498, Identifier::vanilla_static("brown_terracotta"), Some(Block::BROWN_TERRACOTTA), 64);
    pub const GREEN_TERRACOTTA: Self = Self::new(499, Identifier::vanilla_static("green_terracotta"), Some(Block::GREEN_TERRACOTTA), 64);
    pub const RED_TERRACOTTA: Self = Self::new(500, Identifier::vanilla_static("red_terracotta"), Some(Block::RED_TERRACOTTA), 64);
    pub const BLACK_TERRACOTTA: Self = Self::new(501, Identifier::vanilla_static("black_terracotta"), Some(Block::BLACK_TERRACOTTA), 64);
    pub const BARRIER: Self = Self::new(502, Identifier::vanilla_static("barrier"), Some(Block::BARRIER), 64);
    pub const LIGHT: Self = Self::new(503, Identifier::vanilla_static("light"), Some(Block::LIGHT), 64);
    pub const HAY_BLOCK: Self = Self::new(504, Identifier::vanilla_static("hay_block"), Some(Block::HAY_BLOCK), 64);
    pub const WHITE_CARPET: Self = Self::new(505, Identifier::vanilla_static("white_carpet"), Some(Block::WHITE_CARPET), 64);
    pub const ORANGE_CARPET: Self = Self::new(506, Identifier::vanilla_static("orange_carpet"), Some(Block::ORANGE_CARPET), 64);
    pub const MAGENTA_CARPET: Self = Self::new(507, Identifier::vanilla_static("magenta_carpet"), Some(Block::MAGENTA_CARPET), 64);
    pub const LIGHT_BLUE_CARPET: Self = Self::new(508, Identifier::vanilla_static("light_blue_carpet"), Some(Block::LIGHT_BLUE_CARPET), 64);
    pub const YELLOW_CARPET: Self = Self::new(509, Identifier::vanilla_static("yellow_carpet"), Some(Block::YELLOW_CARPET), 64);
    pub const LIME_CARPET: Self = Self::new(510, Identifier::vanilla_static("lime_carpet"), Some(Block::LIME_CARPET), 64);
    pub const PINK_CARPET: Self = Self::new(511, Identifier::vanilla_static("pink_carpet"), Some(Block::PINK_CARPET), 64);
    pub const GRAY_CARPET: Self = Self::new(512, Identifier::vanilla_static("gray_carpet"), Some(Block::GRAY_CARPET), 64);
    pub const LIGHT_GRAY_CARPET: Self = Self::new(513, Identifier::vanilla_static("light_gray_carpet"), Some(Block::LIGHT_GRAY_CARPET), 64);
    pub const CYAN_CARPET: Self = Self::new(514, Identifier::vanilla_static("cyan_carpet"), Some(Block::CYAN_CARPET), 64);
    pub const PURPLE_CARPET: Self = Self::new(515, Identifier::vanilla_static("purple_carpet"), Some(Block::PURPLE_CARPET), 64);
    pub const BLUE_CARPET: Self = Self::new(516, Identifier::vanilla_static("blue_carpet"), Some(Block::BLUE_CARPET), 64);
    pub const BROWN_CARPET: Self = Self::new(517, Identifier::vanilla_static("brown_carpet"), Some(Block::BROWN_CARPET), 64);
    pub const GREEN_CARPET: Self = Self::new(518, Identifier::vanilla_static("green_carpet"), Some(Block::GREEN_CARPET), 64);
    pub const RED_CARPET: Self = Self::new(519, Identifier::vanilla_static("red_carpet"), Some(Block::RED_CARPET), 64);
    pub const BLACK_CARPET: Self = Self::new(520, Identifier::vanilla_static("black_carpet"), Some(Block::BLACK_CARPET), 64);
    pub const TERRACOTTA: Self = Self::new(521, Identifier::vanilla_static("terracotta"), Some(Block::TERRACOTTA), 64);
    pub const PACKED_ICE: Self = Self::new(522, Identifier::vanilla_static("packed_ice"), Some(Block::PACKED_ICE), 64);
    pub const DIRT_PATH: Self = Self::new(523, Identifier::vanilla_static("dirt_path"), Some(Block::DIRT_PATH), 64);
    pub const SUNFLOWER: Self = Self::new(524, Identifier::vanilla_static("sunflower"), Some(Block::SUNFLOWER), 64);
    pub const LILAC: Self = Self::new(525, Identifier::vanilla_static("lilac"), Some(Block::LILAC), 64);
    pub const ROSE_BUSH: Self = Self::new(526, Identifier::vanilla_static("rose_bush"), Some(Block::ROSE_BUSH), 64);
    pub const PEONY: Self = Self::new(527, Identifier::vanilla_static("peony"), Some(Block::PEONY), 64);
    pub const TALL_GRASS: Self = Self::new(528, Identifier::vanilla_static("tall_grass"), Some(Block::TALL_GRASS), 64);
    pub const LARGE_FERN: Self = Self::new(529, Identifier::vanilla_static("large_fern"), Some(Block::LARGE_FERN), 64);
    pub const WHITE_STAINED_GLASS: Self = Self::new(530, Identifier::vanilla_static("white_stained_glass"), Some(Block::WHITE_STAINED_GLASS), 64);
    pub const ORANGE_STAINED_GLASS: Self = Self::new(531, Identifier::vanilla_static("orange_stained_glass"), Some(Block::ORANGE_STAINED_GLASS), 64);
    pub const MAGENTA_STAINED_GLASS: Self = Self::new(532, Identifier::vanilla_static("magenta_stained_glass"), Some(Block::MAGENTA_STAINED_GLASS), 64);
    pub const LIGHT_BLUE_STAINED_GLASS: Self = Self::new(533, Identifier::vanilla_static("light_blue_stained_glass"), Some(Block::LIGHT_BLUE_STAINED_GLASS), 64);
    pub const YELLOW_STAINED_GLASS: Self = Self::new(534, Identifier::vanilla_static("yellow_stained_glass"), Some(Block::YELLOW_STAINED_GLASS), 64);
    pub const LIME_STAINED_GLASS: Self = Self::new(535, Identifier::vanilla_static("lime_stained_glass"), Some(Block::LIME_STAINED_GLASS), 64);
    pub const PINK_STAINED_GLASS: Self = Self::new(536, Identifier::vanilla_static("pink_stained_glass"), Some(Block::PINK_STAINED_GLASS), 64);
    pub const GRAY_STAINED_GLASS: Self = Self::new(537, Identifier::vanilla_static("gray_stained_glass"), Some(Block::GRAY_STAINED_GLASS), 64);
    pub const LIGHT_GRAY_STAINED_GLASS: Self = Self::new(538, Identifier::vanilla_static("light_gray_stained_glass"), Some(Block::LIGHT_GRAY_STAINED_GLASS), 64);
    pub const CYAN_STAINED_GLASS: Self = Self::new(539, Identifier::vanilla_static("cyan_stained_glass"), Some(Block::CYAN_STAINED_GLASS), 64);
    pub const PURPLE_STAINED_GLASS: Self = Self::new(540, Identifier::vanilla_static("purple_stained_glass"), Some(Block::PURPLE_STAINED_GLASS), 64);
    pub const BLUE_STAINED_GLASS: Self = Self::new(541, Identifier::vanilla_static("blue_stained_glass"), Some(Block::BLUE_STAINED_GLASS), 64);
    pub const BROWN_STAINED_GLASS: Self = Self::new(542, Identifier::vanilla_static("brown_stained_glass"), Some(Block::BROWN_STAINED_GLASS), 64);
    pub const GREEN_STAINED_GLASS: Self = Self::new(543, Identifier::vanilla_static("green_stained_glass"), Some(Block::GREEN_STAINED_GLASS), 64);
    pub const RED_STAINED_GLASS: Self = Self::new(544, Identifier::vanilla_static("red_stained_glass"), Some(Block::RED_STAINED_GLASS), 64);
    pub const BLACK_STAINED_GLASS: Self = Self::new(545, Identifier::vanilla_static("black_stained_glass"), Some(Block::BLACK_STAINED_GLASS), 64);
    pub const WHITE_STAINED_GLASS_PANE: Self = Self::new(546, Identifier::vanilla_static("white_stained_glass_pane"), Some(Block::WHITE_STAINED_GLASS_PANE), 64);
    pub const ORANGE_STAINED_GLASS_PANE: Self = Self::new(547, Identifier::vanilla_static("orange_stained_glass_pane"), Some(Block::ORANGE_STAINED_GLASS_PANE), 64);
    pub const MAGENTA_STAINED_GLASS_PANE: Self = Self::new(548, Identifier::vanilla_static("magenta_stained_glass_pane"), Some(Block::MAGENTA_STAINED_GLASS_PANE), 64);
    pub const LIGHT_BLUE_STAINED_GLASS_PANE: Self = Self::new(549, Identifier::vanilla_static("light_blue_stained_glass_pane"), Some(Block::LIGHT_BLUE_STAINED_GLASS_PANE), 64);
    pub const YELLOW_STAINED_GLASS_PANE: Self = Self::new(550, Identifier::vanilla_static("yellow_stained_glass_pane"), Some(Block::YELLOW_STAINED_GLASS_PANE), 64);
    pub const LIME_STAINED_GLASS_PANE: Self = Self::new(551, Identifier::vanilla_static("lime_stained_glass_pane"), Some(Block::LIME_STAINED_GLASS_PANE), 64);
    pub const PINK_STAINED_GLASS_PANE: Self = Self::new(552, Identifier::vanilla_static("pink_stained_glass_pane"), Some(Block::PINK_STAINED_GLASS_PANE), 64);
    pub const GRAY_STAINED_GLASS_PANE: Self = Self::new(553, Identifier::vanilla_static("gray_stained_glass_pane"), Some(Block::GRAY_STAINED_GLASS_PANE), 64);
    pub const LIGHT_GRAY_STAINED_GLASS_PANE: Self = Self::new(554, Identifier::vanilla_static("light_gray_stained_glass_pane"), Some(Block::LIGHT_GRAY_STAINED_GLASS_PANE), 64);
    pub const CYAN_STAINED_GLASS_PANE: Self = Self::new(555, Identifier::vanilla_static("cyan_stained_glass_pane"), Some(Block::CYAN_STAINED_GLASS_PANE), 64);
    pub const PURPLE_STAINED_GLASS_PANE: Self = Self::new(556, Identifier::vanilla_static("purple_stained_glass_pane"), Some(Block::PURPLE_STAINED_GLASS_PANE), 64);
    pub const BLUE_STAINED_GLASS_PANE: Self = Self::new(557, Identifier::vanilla_static("blue_stained_glass_pane"), Some(Block::BLUE_STAINED_GLASS_PANE), 64);
    pub const BROWN_STAINED_GLASS_PANE: Self = Self::new(558, Identifier::vanilla_static("brown_stained_glass_pane"), Some(Block::BROWN_STAINED_GLASS_PANE), 64);
    pub const GREEN_STAINED_GLASS_PANE: Self = Self::new(559, Identifier::vanilla_static("green_stained_glass_pane"), Some(Block::GREEN_STAINED_GLASS_PANE), 64);
    pub const RED_STAINED_GLASS_PANE: Self = Self::new(560, Identifier::vanilla_static("red_stained_glass_pane"), Some(Block::RED_STAINED_GLASS_PANE), 64);
    pub const BLACK_STAINED_GLASS_PANE: Self = Self::new(561, Identifier::vanilla_static("black_stained_glass_pane"), Some(Block::BLACK_STAINED_GLASS_PANE), 64);
    pub const PRISMARINE: Self = Self::new(562, Identifier::vanilla_static("prismarine"), Some(Block::PRISMARINE), 64);
    pub const PRISMARINE_BRICKS: Self = Self::new(563, Identifier::vanilla_static("prismarine_bricks"), Some(Block::PRISMARINE_BRICKS), 64);
    pub const DARK_PRISMARINE: Self = Self::new(564, Identifier::vanilla_static("dark_prismarine"), Some(Block::DARK_PRISMARINE), 64);
    pub const PRISMARINE_STAIRS: Self = Self::new(565, Identifier::vanilla_static("prismarine_stairs"), Some(Block::PRISMARINE_STAIRS), 64);
    pub const PRISMARINE_BRICK_STAIRS: Self = Self::new(566, Identifier::vanilla_static("prismarine_brick_stairs"), Some(Block::PRISMARINE_BRICK_STAIRS), 64);
    pub const DARK_PRISMARINE_STAIRS: Self = Self::new(567, Identifier::vanilla_static("dark_prismarine_stairs"), Some(Block::DARK_PRISMARINE_STAIRS), 64);
    pub const SEA_LANTERN: Self = Self::new(568, Identifier::vanilla_static("sea_lantern"), Some(Block::SEA_LANTERN), 64);
    pub const RED_SANDSTONE: Self = Self::new(569, Identifier::vanilla_static("red_sandstone"), Some(Block::RED_SANDSTONE), 64);
    pub const CHISELED_RED_SANDSTONE: Self = Self::new(570, Identifier::vanilla_static("chiseled_red_sandstone"), Some(Block::CHISELED_RED_SANDSTONE), 64);
    pub const CUT_RED_SANDSTONE: Self = Self::new(571, Identifier::vanilla_static("cut_red_sandstone"), Some(Block::CUT_RED_SANDSTONE), 64);
    pub const RED_SANDSTONE_STAIRS: Self = Self::new(572, Identifier::vanilla_static("red_sandstone_stairs"), Some(Block::RED_SANDSTONE_STAIRS), 64);
    pub const REPEATING_COMMAND_BLOCK: Self = Self::new(573, Identifier::vanilla_static("repeating_command_block"), Some(Block::REPEATING_COMMAND_BLOCK), 64);
    pub const CHAIN_COMMAND_BLOCK: Self = Self::new(574, Identifier::vanilla_static("chain_command_block"), Some(Block::CHAIN_COMMAND_BLOCK), 64);
    pub const MAGMA_BLOCK: Self = Self::new(575, Identifier::vanilla_static("magma_block"), Some(Block::MAGMA_BLOCK), 64);
    pub const NETHER_WART_BLOCK: Self = Self::new(576, Identifier::vanilla_static("nether_wart_block"), Some(Block::NETHER_WART_BLOCK), 64);
    pub const WARPED_WART_BLOCK: Self = Self::new(577, Identifier::vanilla_static("warped_wart_block"), Some(Block::WARPED_WART_BLOCK), 64);
    pub const RED_NETHER_BRICKS: Self = Self::new(578, Identifier::vanilla_static("red_nether_bricks"), Some(Block::RED_NETHER_BRICKS), 64);
    pub const BONE_BLOCK: Self = Self::new(579, Identifier::vanilla_static("bone_block"), Some(Block::BONE_BLOCK), 64);
    pub const STRUCTURE_VOID: Self = Self::new(580, Identifier::vanilla_static("structure_void"), Some(Block::STRUCTURE_VOID), 64);
    pub const SHULKER_BOX: Self = Self::new(581, Identifier::vanilla_static("shulker_box"), Some(Block::SHULKER_BOX), 1);
    pub const WHITE_SHULKER_BOX: Self = Self::new(582, Identifier::vanilla_static("white_shulker_box"), Some(Block::WHITE_SHULKER_BOX), 1);
    pub const ORANGE_SHULKER_BOX: Self = Self::new(583, Identifier::vanilla_static("orange_shulker_box"), Some(Block::ORANGE_SHULKER_BOX), 1);
    pub const MAGENTA_SHULKER_BOX: Self = Self::new(584, Identifier::vanilla_static("magenta_shulker_box"), Some(Block::MAGENTA_SHULKER_BOX), 1);
    pub const LIGHT_BLUE_SHULKER_BOX: Self = Self::new(585, Identifier::vanilla_static("light_blue_shulker_box"), Some(Block::LIGHT_BLUE_SHULKER_BOX), 1);
    pub const YELLOW_SHULKER_BOX: Self = Self::new(586, Identifier::vanilla_static("yellow_shulker_box"), Some(Block::YELLOW_SHULKER_BOX), 1);
    pub const LIME_SHULKER_BOX: Self = Self::new(587, Identifier::vanilla_static("lime_shulker_box"), Some(Block::LIME_SHULKER_BOX), 1);
    pub const PINK_SHULKER_BOX: Self = Self::new(588, Identifier::vanilla_static("pink_shulker_box"), Some(Block::PINK_SHULKER_BOX), 1);
    pub const GRAY_SHULKER_BOX: Self = Self::new(589, Identifier::vanilla_static("gray_shulker_box"), Some(Block::GRAY_SHULKER_BOX), 1);
    pub const LIGHT_GRAY_SHULKER_BOX: Self = Self::new(590, Identifier::vanilla_static("light_gray_shulker_box"), Some(Block::LIGHT_GRAY_SHULKER_BOX), 1);
    pub const CYAN_SHULKER_BOX: Self = Self::new(591, Identifier::vanilla_static("cyan_shulker_box"), Some(Block::CYAN_SHULKER_BOX), 1);
    pub const PURPLE_SHULKER_BOX: Self = Self::new(592, Identifier::vanilla_static("purple_shulker_box"), Some(Block::PURPLE_SHULKER_BOX), 1);
    pub const BLUE_SHULKER_BOX: Self = Self::new(593, Identifier::vanilla_static("blue_shulker_box"), Some(Block::BLUE_SHULKER_BOX), 1);
    pub const BROWN_SHULKER_BOX: Self = Self::new(594, Identifier::vanilla_static("brown_shulker_box"), Some(Block::BROWN_SHULKER_BOX), 1);
    pub const GREEN_SHULKER_BOX: Self = Self::new(595, Identifier::vanilla_static("green_shulker_box"), Some(Block::GREEN_SHULKER_BOX), 1);
    pub const RED_SHULKER_BOX: Self = Self::new(596, Identifier::vanilla_static("red_shulker_box"), Some(Block::RED_SHULKER_BOX), 1);
    pub const BLACK_SHULKER_BOX: Self = Self::new(597, Identifier::vanilla_static("black_shulker_box"), Some(Block::BLACK_SHULKER_BOX), 1);
    pub const WHITE_GLAZED_TERRACOTTA: Self = Self::new(598, Identifier::vanilla_static("white_glazed_terracotta"), Some(Block::WHITE_GLAZED_TERRACOTTA), 64);
    pub const ORANGE_GLAZED_TERRACOTTA: Self = Self::new(599, Identifier::vanilla_static("orange_glazed_terracotta"), Some(Block::ORANGE_GLAZED_TERRACOTTA), 64);
    pub const MAGENTA_GLAZED_TERRACOTTA: Self = Self::new(600, Identifier::vanilla_static("magenta_glazed_terracotta"), Some(Block::MAGENTA_GLAZED_TERRACOTTA), 64);
    pub const LIGHT_BLUE_GLAZED_TERRACOTTA: Self = Self::new(601, Identifier::vanilla_static("light_blue_glazed_terracotta"), Some(Block::LIGHT_BLUE_GLAZED_TERRACOTTA), 64);
    pub const YELLOW_GLAZED_TERRACOTTA: Self = Self::new(602, Identifier::vanilla_static("yellow_glazed_terracotta"), Some(Block::YELLOW_GLAZED_TERRACOTTA), 64);
    pub const LIME_GLAZED_TERRACOTTA: Self = Self::new(603, Identifier::vanilla_static("lime_glazed_terracotta"), Some(Block::LIME_GLAZED_TERRACOTTA), 64);
    pub const PINK_GLAZED_TERRACOTTA: Self = Self::new(604, Identifier::vanilla_static("pink_glazed_terracotta"), Some(Block::PINK_GLAZED_TERRACOTTA), 64);
    pub const GRAY_GLAZED_TERRACOTTA: Self = Self::new(605, Identifier::vanilla_static("gray_glazed_terracotta"), Some(Block::GRAY_GLAZED_TERRACOTTA), 64);
    pub const LIGHT_GRAY_GLAZED_TERRACOTTA: Self = Self::new(606, Identifier::vanilla_static("light_gray_glazed_terracotta"), Some(Block::LIGHT_GRAY_GLAZED_TERRACOTTA), 64);
    pub const CYAN_GLAZED_TERRACOTTA: Self = Self::new(607, Identifier::vanilla_static("cyan_glazed_terracotta"), Some(Block::CYAN_GLAZED_TERRACOTTA), 64);
    pub const PURPLE_GLAZED_TERRACOTTA: Self = Self::new(608, Identifier::vanilla_static("purple_glazed_terracotta"), Some(Block::PURPLE_GLAZED_TERRACOTTA), 64);
    pub const BLUE_GLAZED_TERRACOTTA: Self = Self::new(609, Identifier::vanilla_static("blue_glazed_terracotta"), Some(Block::BLUE_GLAZED_TERRACOTTA), 64);
    pub const BROWN_GLAZED_TERRACOTTA: Self = Self::new(610, Identifier::vanilla_static("brown_glazed_terracotta"), Some(Block::BROWN_GLAZED_TERRACOTTA), 64);
    pub const GREEN_GLAZED_TERRACOTTA: Self = Self::new(611, Identifier::vanilla_static("green_glazed_terracotta"), Some(Block::GREEN_GLAZED_TERRACOTTA), 64);
    pub const RED_GLAZED_TERRACOTTA: Self = Self::new(612, Identifier::vanilla_static("red_glazed_terracotta"), Some(Block::RED_GLAZED_TERRACOTTA), 64);
    pub const BLACK_GLAZED_TERRACOTTA: Self = Self::new(613, Identifier::vanilla_static("black_glazed_terracotta"), Some(Block::BLACK_GLAZED_TERRACOTTA), 64);
    pub const WHITE_CONCRETE: Self = Self::new(614, Identifier::vanilla_static("white_concrete"), Some(Block::WHITE_CONCRETE), 64);
    pub const ORANGE_CONCRETE: Self = Self::new(615, Identifier::vanilla_static("orange_concrete"), Some(Block::ORANGE_CONCRETE), 64);
    pub const MAGENTA_CONCRETE: Self = Self::new(616, Identifier::vanilla_static("magenta_concrete"), Some(Block::MAGENTA_CONCRETE), 64);
    pub const LIGHT_BLUE_CONCRETE: Self = Self::new(617, Identifier::vanilla_static("light_blue_concrete"), Some(Block::LIGHT_BLUE_CONCRETE), 64);
    pub const YELLOW_CONCRETE: Self = Self::new(618, Identifier::vanilla_static("yellow_concrete"), Some(Block::YELLOW_CONCRETE), 64);
    pub const LIME_CONCRETE: Self = Self::new(619, Identifier::vanilla_static("lime_concrete"), Some(Block::LIME_CONCRETE), 64);
    pub const PINK_CONCRETE: Self = Self::new(620, Identifier::vanilla_static("pink_concrete"), Some(Block::PINK_CONCRETE), 64);
    pub const GRAY_CONCRETE: Self = Self::new(621, Identifier::vanilla_static("gray_concrete"), Some(Block::GRAY_CONCRETE), 64);
    pub const LIGHT_GRAY_CONCRETE: Self = Self::new(622, Identifier::vanilla_static("light_gray_concrete"), Some(Block::LIGHT_GRAY_CONCRETE), 64);
    pub const CYAN_CONCRETE: Self = Self::new(623, Identifier::vanilla_static("cyan_concrete"), Some(Block::CYAN_CONCRETE), 64);
    pub const PURPLE_CONCRETE: Self = Self::new(624, Identifier::vanilla_static("purple_concrete"), Some(Block::PURPLE_CONCRETE), 64);
    pub const BLUE_CONCRETE: Self = Self::new(625, Identifier::vanilla_static("blue_concrete"), Some(Block::BLUE_CONCRETE), 64);
    pub const BROWN_CONCRETE: Self = Self::new(626, Identifier::vanilla_static("brown_concrete"), Some(Block::BROWN_CONCRETE), 64);
    pub const GREEN_CONCRETE: Self = Self::new(627, Identifier::vanilla_static("green_concrete"), Some(Block::GREEN_CONCRETE), 64);
    pub const RED_CONCRETE: Self = Self::new(628, Identifier::vanilla_static("red_concrete"), Some(Block::RED_CONCRETE), 64);
    pub const BLACK_CONCRETE: Self = Self::new(629, Identifier::vanilla_static("black_concrete"), Some(Block::BLACK_CONCRETE), 64);
    pub const WHITE_CONCRETE_POWDER: Self = Self::new(630, Identifier::vanilla_static("white_concrete_powder"), Some(Block::WHITE_CONCRETE_POWDER), 64);
    pub const ORANGE_CONCRETE_POWDER: Self = Self::new(631, Identifier::vanilla_static("orange_concrete_powder"), Some(Block::ORANGE_CONCRETE_POWDER), 64);
    pub const MAGENTA_CONCRETE_POWDER: Self = Self::new(632, Identifier::vanilla_static("magenta_concrete_powder"), Some(Block::MAGENTA_CONCRETE_POWDER), 64);
    pub const LIGHT_BLUE_CONCRETE_POWDER: Self = Self::new(633, Identifier::vanilla_static("light_blue_concrete_powder"), Some(Block::LIGHT_BLUE_CONCRETE_POWDER), 64);
    pub const YELLOW_CONCRETE_POWDER: Self = Self::new(634, Identifier::vanilla_static("yellow_concrete_powder"), Some(Block::YELLOW_CONCRETE_POWDER), 64);
    pub const LIME_CONCRETE_POWDER: Self = Self::new(635, Identifier::vanilla_static("lime_concrete_powder"), Some(Block::LIME_CONCRETE_POWDER), 64);
    pub const PINK_CONCRETE_POWDER: Self = Self::new(636, Identifier::vanilla_static("pink_concrete_powder"), Some(Block::PINK_CONCRETE_POWDER), 64);
    pub const GRAY_CONCRETE_POWDER: Self = Self::new(637, Identifier::vanilla_static("gray_concrete_powder"), Some(Block::GRAY_CONCRETE_POWDER), 64);
    pub const LIGHT_GRAY_CONCRETE_POWDER: Self = Self::new(638, Identifier::vanilla_static("light_gray_concrete_powder"), Some(Block::LIGHT_GRAY_CONCRETE_POWDER), 64);
    pub const CYAN_CONCRETE_POWDER: Self = Self::new(639, Identifier::vanilla_static("cyan_concrete_powder"), Some(Block::CYAN_CONCRETE_POWDER), 64);
    pub const PURPLE_CONCRETE_POWDER: Self = Self::new(640, Identifier::vanilla_static("purple_concrete_powder"), Some(Block::PURPLE_CONCRETE_POWDER), 64);
    pub const BLUE_CONCRETE_POWDER: Self = Self::new(641, Identifier::vanilla_static("blue_concrete_powder"), Some(Block::BLUE_CONCRETE_POWDER), 64);
    pub const BROWN_CONCRETE_POWDER: Self = Self::new(642, Identifier::vanilla_static("brown_concrete_powder"), Some(Block::BROWN_CONCRETE_POWDER), 64);
    pub const GREEN_CONCRETE_POWDER: Self = Self::new(643, Identifier::vanilla_static("green_concrete_powder"), Some(Block::GREEN_CONCRETE_POWDER), 64);
    pub const RED_CONCRETE_POWDER: Self = Self::new(644, Identifier::vanilla_static("red_concrete_powder"), Some(Block::RED_CONCRETE_POWDER), 64);
    pub const BLACK_CONCRETE_POWDER: Self = Self::new(645, Identifier::vanilla_static("black_concrete_powder"), Some(Block::BLACK_CONCRETE_POWDER), 64);
    pub const TURTLE_EGG: Self = Self::new(646, Identifier::vanilla_static("turtle_egg"), Some(Block::TURTLE_EGG), 64);
    pub const SNIFFER_EGG: Self = Self::new(647, Identifier::vanilla_static("sniffer_egg"), Some(Block::SNIFFER_EGG), 64);
    pub const DRIED_GHAST: Self = Self::new(648, Identifier::vanilla_static("dried_ghast"), Some(Block::DRIED_GHAST), 64);
    pub const DEAD_TUBE_CORAL_BLOCK: Self = Self::new(649, Identifier::vanilla_static("dead_tube_coral_block"), Some(Block::DEAD_TUBE_CORAL_BLOCK), 64);
    pub const DEAD_BRAIN_CORAL_BLOCK: Self = Self::new(650, Identifier::vanilla_static("dead_brain_coral_block"), Some(Block::DEAD_BRAIN_CORAL_BLOCK), 64);
    pub const DEAD_BUBBLE_CORAL_BLOCK: Self = Self::new(651, Identifier::vanilla_static("dead_bubble_coral_block"), Some(Block::DEAD_BUBBLE_CORAL_BLOCK), 64);
    pub const DEAD_FIRE_CORAL_BLOCK: Self = Self::new(652, Identifier::vanilla_static("dead_fire_coral_block"), Some(Block::DEAD_FIRE_CORAL_BLOCK), 64);
    pub const DEAD_HORN_CORAL_BLOCK: Self = Self::new(653, Identifier::vanilla_static("dead_horn_coral_block"), Some(Block::DEAD_HORN_CORAL_BLOCK), 64);
    pub const TUBE_CORAL_BLOCK: Self = Self::new(654, Identifier::vanilla_static("tube_coral_block"), Some(Block::TUBE_CORAL_BLOCK), 64);
    pub const BRAIN_CORAL_BLOCK: Self = Self::new(655, Identifier::vanilla_static("brain_coral_block"), Some(Block::BRAIN_CORAL_BLOCK), 64);
    pub const BUBBLE_CORAL_BLOCK: Self = Self::new(656, Identifier::vanilla_static("bubble_coral_block"), Some(Block::BUBBLE_CORAL_BLOCK), 64);
    pub const FIRE_CORAL_BLOCK: Self = Self::new(657, Identifier::vanilla_static("fire_coral_block"), Some(Block::FIRE_CORAL_BLOCK), 64);
    pub const HORN_CORAL_BLOCK: Self = Self::new(658, Identifier::vanilla_static("horn_coral_block"), Some(Block::HORN_CORAL_BLOCK), 64);
    pub const TUBE_CORAL: Self = Self::new(659, Identifier::vanilla_static("tube_coral"), Some(Block::TUBE_CORAL), 64);
    pub const BRAIN_CORAL: Self = Self::new(660, Identifier::vanilla_static("brain_coral"), Some(Block::BRAIN_CORAL), 64);
    pub const BUBBLE_CORAL: Self = Self::new(661, Identifier::vanilla_static("bubble_coral"), Some(Block::BUBBLE_CORAL), 64);
    pub const FIRE_CORAL: Self = Self::new(662, Identifier::vanilla_static("fire_coral"), Some(Block::FIRE_CORAL), 64);
    pub const HORN_CORAL: Self = Self::new(663, Identifier::vanilla_static("horn_coral"), Some(Block::HORN_CORAL), 64);
    pub const DEAD_BRAIN_CORAL: Self = Self::new(664, Identifier::vanilla_static("dead_brain_coral"), Some(Block::DEAD_BRAIN_CORAL), 64);
    pub const DEAD_BUBBLE_CORAL: Self = Self::new(665, Identifier::vanilla_static("dead_bubble_coral"), Some(Block::DEAD_BUBBLE_CORAL), 64);
    pub const DEAD_FIRE_CORAL: Self = Self::new(666, Identifier::vanilla_static("dead_fire_coral"), Some(Block::DEAD_FIRE_CORAL), 64);
    pub const DEAD_HORN_CORAL: Self = Self::new(667, Identifier::vanilla_static("dead_horn_coral"), Some(Block::DEAD_HORN_CORAL), 64);
    pub const DEAD_TUBE_CORAL: Self = Self::new(668, Identifier::vanilla_static("dead_tube_coral"), Some(Block::DEAD_TUBE_CORAL), 64);
    pub const TUBE_CORAL_FAN: Self = Self::new(669, Identifier::vanilla_static("tube_coral_fan"), Some(Block::TUBE_CORAL_FAN), 64);
    pub const BRAIN_CORAL_FAN: Self = Self::new(670, Identifier::vanilla_static("brain_coral_fan"), Some(Block::BRAIN_CORAL_FAN), 64);
    pub const BUBBLE_CORAL_FAN: Self = Self::new(671, Identifier::vanilla_static("bubble_coral_fan"), Some(Block::BUBBLE_CORAL_FAN), 64);
    pub const FIRE_CORAL_FAN: Self = Self::new(672, Identifier::vanilla_static("fire_coral_fan"), Some(Block::FIRE_CORAL_FAN), 64);
    pub const HORN_CORAL_FAN: Self = Self::new(673, Identifier::vanilla_static("horn_coral_fan"), Some(Block::HORN_CORAL_FAN), 64);
    pub const DEAD_TUBE_CORAL_FAN: Self = Self::new(674, Identifier::vanilla_static("dead_tube_coral_fan"), Some(Block::DEAD_TUBE_CORAL_FAN), 64);
    pub const DEAD_BRAIN_CORAL_FAN: Self = Self::new(675, Identifier::vanilla_static("dead_brain_coral_fan"), Some(Block::DEAD_BRAIN_CORAL_FAN), 64);
    pub const DEAD_BUBBLE_CORAL_FAN: Self = Self::new(676, Identifier::vanilla_static("dead_bubble_coral_fan"), Some(Block::DEAD_BUBBLE_CORAL_FAN), 64);
    pub const DEAD_FIRE_CORAL_FAN: Self = Self::new(677, Identifier::vanilla_static("dead_fire_coral_fan"), Some(Block::DEAD_FIRE_CORAL_FAN), 64);
    pub const DEAD_HORN_CORAL_FAN: Self = Self::new(678, Identifier::vanilla_static("dead_horn_coral_fan"), Some(Block::DEAD_HORN_CORAL_FAN), 64);
    pub const BLUE_ICE: Self = Self::new(679, Identifier::vanilla_static("blue_ice"), Some(Block::BLUE_ICE), 64);
    pub const CONDUIT: Self = Self::new(680, Identifier::vanilla_static("conduit"), Some(Block::CONDUIT), 64);
    pub const POLISHED_GRANITE_STAIRS: Self = Self::new(681, Identifier::vanilla_static("polished_granite_stairs"), Some(Block::POLISHED_GRANITE_STAIRS), 64);
    pub const SMOOTH_RED_SANDSTONE_STAIRS: Self = Self::new(682, Identifier::vanilla_static("smooth_red_sandstone_stairs"), Some(Block::SMOOTH_RED_SANDSTONE_STAIRS), 64);
    pub const MOSSY_STONE_BRICK_STAIRS: Self = Self::new(683, Identifier::vanilla_static("mossy_stone_brick_stairs"), Some(Block::MOSSY_STONE_BRICK_STAIRS), 64);
    pub const POLISHED_DIORITE_STAIRS: Self = Self::new(684, Identifier::vanilla_static("polished_diorite_stairs"), Some(Block::POLISHED_DIORITE_STAIRS), 64);
    pub const MOSSY_COBBLESTONE_STAIRS: Self = Self::new(685, Identifier::vanilla_static("mossy_cobblestone_stairs"), Some(Block::MOSSY_COBBLESTONE_STAIRS), 64);
    pub const END_STONE_BRICK_STAIRS: Self = Self::new(686, Identifier::vanilla_static("end_stone_brick_stairs"), Some(Block::END_STONE_BRICK_STAIRS), 64);
    pub const STONE_STAIRS: Self = Self::new(687, Identifier::vanilla_static("stone_stairs"), Some(Block::STONE_STAIRS), 64);
    pub const SMOOTH_SANDSTONE_STAIRS: Self = Self::new(688, Identifier::vanilla_static("smooth_sandstone_stairs"), Some(Block::SMOOTH_SANDSTONE_STAIRS), 64);
    pub const SMOOTH_QUARTZ_STAIRS: Self = Self::new(689, Identifier::vanilla_static("smooth_quartz_stairs"), Some(Block::SMOOTH_QUARTZ_STAIRS), 64);
    pub const GRANITE_STAIRS: Self = Self::new(690, Identifier::vanilla_static("granite_stairs"), Some(Block::GRANITE_STAIRS), 64);
    pub const ANDESITE_STAIRS: Self = Self::new(691, Identifier::vanilla_static("andesite_stairs"), Some(Block::ANDESITE_STAIRS), 64);
    pub const RED_NETHER_BRICK_STAIRS: Self = Self::new(692, Identifier::vanilla_static("red_nether_brick_stairs"), Some(Block::RED_NETHER_BRICK_STAIRS), 64);
    pub const POLISHED_ANDESITE_STAIRS: Self = Self::new(693, Identifier::vanilla_static("polished_andesite_stairs"), Some(Block::POLISHED_ANDESITE_STAIRS), 64);
    pub const DIORITE_STAIRS: Self = Self::new(694, Identifier::vanilla_static("diorite_stairs"), Some(Block::DIORITE_STAIRS), 64);
    pub const COBBLED_DEEPSLATE_STAIRS: Self = Self::new(695, Identifier::vanilla_static("cobbled_deepslate_stairs"), Some(Block::COBBLED_DEEPSLATE_STAIRS), 64);
    pub const POLISHED_DEEPSLATE_STAIRS: Self = Self::new(696, Identifier::vanilla_static("polished_deepslate_stairs"), Some(Block::POLISHED_DEEPSLATE_STAIRS), 64);
    pub const DEEPSLATE_BRICK_STAIRS: Self = Self::new(697, Identifier::vanilla_static("deepslate_brick_stairs"), Some(Block::DEEPSLATE_BRICK_STAIRS), 64);
    pub const DEEPSLATE_TILE_STAIRS: Self = Self::new(698, Identifier::vanilla_static("deepslate_tile_stairs"), Some(Block::DEEPSLATE_TILE_STAIRS), 64);
    pub const POLISHED_GRANITE_SLAB: Self = Self::new(699, Identifier::vanilla_static("polished_granite_slab"), Some(Block::POLISHED_GRANITE_SLAB), 64);
    pub const SMOOTH_RED_SANDSTONE_SLAB: Self = Self::new(700, Identifier::vanilla_static("smooth_red_sandstone_slab"), Some(Block::SMOOTH_RED_SANDSTONE_SLAB), 64);
    pub const MOSSY_STONE_BRICK_SLAB: Self = Self::new(701, Identifier::vanilla_static("mossy_stone_brick_slab"), Some(Block::MOSSY_STONE_BRICK_SLAB), 64);
    pub const POLISHED_DIORITE_SLAB: Self = Self::new(702, Identifier::vanilla_static("polished_diorite_slab"), Some(Block::POLISHED_DIORITE_SLAB), 64);
    pub const MOSSY_COBBLESTONE_SLAB: Self = Self::new(703, Identifier::vanilla_static("mossy_cobblestone_slab"), Some(Block::MOSSY_COBBLESTONE_SLAB), 64);
    pub const END_STONE_BRICK_SLAB: Self = Self::new(704, Identifier::vanilla_static("end_stone_brick_slab"), Some(Block::END_STONE_BRICK_SLAB), 64);
    pub const SMOOTH_SANDSTONE_SLAB: Self = Self::new(705, Identifier::vanilla_static("smooth_sandstone_slab"), Some(Block::SMOOTH_SANDSTONE_SLAB), 64);
    pub const SMOOTH_QUARTZ_SLAB: Self = Self::new(706, Identifier::vanilla_static("smooth_quartz_slab"), Some(Block::SMOOTH_QUARTZ_SLAB), 64);
    pub const GRANITE_SLAB: Self = Self::new(707, Identifier::vanilla_static("granite_slab"), Some(Block::GRANITE_SLAB), 64);
    pub const ANDESITE_SLAB: Self = Self::new(708, Identifier::vanilla_static("andesite_slab"), Some(Block::ANDESITE_SLAB), 64);
    pub const RED_NETHER_BRICK_SLAB: Self = Self::new(709, Identifier::vanilla_static("red_nether_brick_slab"), Some(Block::RED_NETHER_BRICK_SLAB), 64);
    pub const POLISHED_ANDESITE_SLAB: Self = Self::new(710, Identifier::vanilla_static("polished_andesite_slab"), Some(Block::POLISHED_ANDESITE_SLAB), 64);
    pub const DIORITE_SLAB: Self = Self::new(711, Identifier::vanilla_static("diorite_slab"), Some(Block::DIORITE_SLAB), 64);
    pub const COBBLED_DEEPSLATE_SLAB: Self = Self::new(712, Identifier::vanilla_static("cobbled_deepslate_slab"), Some(Block::COBBLED_DEEPSLATE_SLAB), 64);
    pub const POLISHED_DEEPSLATE_SLAB: Self = Self::new(713, Identifier::vanilla_static("polished_deepslate_slab"), Some(Block::POLISHED_DEEPSLATE_SLAB), 64);
    pub const DEEPSLATE_BRICK_SLAB: Self = Self::new(714, Identifier::vanilla_static("deepslate_brick_slab"), Some(Block::DEEPSLATE_BRICK_SLAB), 64);
    pub const DEEPSLATE_TILE_SLAB: Self = Self::new(715, Identifier::vanilla_static("deepslate_tile_slab"), Some(Block::DEEPSLATE_TILE_SLAB), 64);
    pub const SCAFFOLDING: Self = Self::new(716, Identifier::vanilla_static("scaffolding"), Some(Block::SCAFFOLDING), 64);
    pub const REDSTONE: Self = Self::new(717, Identifier::vanilla_static("redstone"), Some(Block::REDSTONE_WIRE), 64);
    pub const REDSTONE_TORCH: Self = Self::new(718, Identifier::vanilla_static("redstone_torch"), Some(Block::REDSTONE_TORCH), 64);
    pub const REDSTONE_BLOCK: Self = Self::new(719, Identifier::vanilla_static("redstone_block"), Some(Block::REDSTONE_BLOCK), 64);
    pub const REPEATER: Self = Self::new(720, Identifier::vanilla_static("repeater"), Some(Block::REPEATER), 64);
    pub const COMPARATOR: Self = Self::new(721, Identifier::vanilla_static("comparator"), Some(Block::COMPARATOR), 64);
    pub const PISTON: Self = Self::new(722, Identifier::vanilla_static("piston"), Some(Block::PISTON), 64);
    pub const STICKY_PISTON: Self = Self::new(723, Identifier::vanilla_static("sticky_piston"), Some(Block::STICKY_PISTON), 64);
    pub const SLIME_BLOCK: Self = Self::new(724, Identifier::vanilla_static("slime_block"), Some(Block::SLIME_BLOCK), 64);
    pub const HONEY_BLOCK: Self = Self::new(725, Identifier::vanilla_static("honey_block"), Some(Block::HONEY_BLOCK), 64);
    pub const OBSERVER: Self = Self::new(726, Identifier::vanilla_static("observer"), Some(Block::OBSERVER), 64);
    pub const HOPPER: Self = Self::new(727, Identifier::vanilla_static("hopper"), Some(Block::HOPPER), 64);
    pub const DISPENSER: Self = Self::new(728, Identifier::vanilla_static("dispenser"), Some(Block::DISPENSER), 64);
    pub const DROPPER: Self = Self::new(729, Identifier::vanilla_static("dropper"), Some(Block::DROPPER), 64);
    pub const LECTERN: Self = Self::new(730, Identifier::vanilla_static("lectern"), Some(Block::LECTERN), 64);
    pub const TARGET: Self = Self::new(731, Identifier::vanilla_static("target"), Some(Block::TARGET), 64);
    pub const LEVER: Self = Self::new(732, Identifier::vanilla_static("lever"), Some(Block::LEVER), 64);
    pub const LIGHTNING_ROD: Self = Self::new(733, Identifier::vanilla_static("lightning_rod"), Some(Block::LIGHTNING_ROD), 64);
    pub const EXPOSED_LIGHTNING_ROD: Self = Self::new(734, Identifier::vanilla_static("exposed_lightning_rod"), Some(Block::EXPOSED_LIGHTNING_ROD), 64);
    pub const WEATHERED_LIGHTNING_ROD: Self = Self::new(735, Identifier::vanilla_static("weathered_lightning_rod"), Some(Block::WEATHERED_LIGHTNING_ROD), 64);
    pub const OXIDIZED_LIGHTNING_ROD: Self = Self::new(736, Identifier::vanilla_static("oxidized_lightning_rod"), Some(Block::OXIDIZED_LIGHTNING_ROD), 64);
    pub const WAXED_LIGHTNING_ROD: Self = Self::new(737, Identifier::vanilla_static("waxed_lightning_rod"), Some(Block::WAXED_LIGHTNING_ROD), 64);
    pub const WAXED_EXPOSED_LIGHTNING_ROD: Self = Self::new(738, Identifier::vanilla_static("waxed_exposed_lightning_rod"), Some(Block::WAXED_EXPOSED_LIGHTNING_ROD), 64);
    pub const WAXED_WEATHERED_LIGHTNING_ROD: Self = Self::new(739, Identifier::vanilla_static("waxed_weathered_lightning_rod"), Some(Block::WAXED_WEATHERED_LIGHTNING_ROD), 64);
    pub const WAXED_OXIDIZED_LIGHTNING_ROD: Self = Self::new(740, Identifier::vanilla_static("waxed_oxidized_lightning_rod"), Some(Block::WAXED_OXIDIZED_LIGHTNING_ROD), 64);
    pub const DAYLIGHT_DETECTOR: Self = Self::new(741, Identifier::vanilla_static("daylight_detector"), Some(Block::DAYLIGHT_DETECTOR), 64);
    pub const SCULK_SENSOR: Self = Self::new(742, Identifier::vanilla_static("sculk_sensor"), Some(Block::SCULK_SENSOR), 64);
    pub const CALIBRATED_SCULK_SENSOR: Self = Self::new(743, Identifier::vanilla_static("calibrated_sculk_sensor"), Some(Block::CALIBRATED_SCULK_SENSOR), 64);
    pub const TRIPWIRE_HOOK: Self = Self::new(744, Identifier::vanilla_static("tripwire_hook"), Some(Block::TRIPWIRE_HOOK), 64);
    pub const TRAPPED_CHEST: Self = Self::new(745, Identifier::vanilla_static("trapped_chest"), Some(Block::TRAPPED_CHEST), 64);
    pub const TNT: Self = Self::new(746, Identifier::vanilla_static("tnt"), Some(Block::TNT), 64);
    pub const REDSTONE_LAMP: Self = Self::new(747, Identifier::vanilla_static("redstone_lamp"), Some(Block::REDSTONE_LAMP), 64);
    pub const NOTE_BLOCK: Self = Self::new(748, Identifier::vanilla_static("note_block"), Some(Block::NOTE_BLOCK), 64);
    pub const STONE_BUTTON: Self = Self::new(749, Identifier::vanilla_static("stone_button"), Some(Block::STONE_BUTTON), 64);
    pub const POLISHED_BLACKSTONE_BUTTON: Self = Self::new(750, Identifier::vanilla_static("polished_blackstone_button"), Some(Block::POLISHED_BLACKSTONE_BUTTON), 64);
    pub const OAK_BUTTON: Self = Self::new(751, Identifier::vanilla_static("oak_button"), Some(Block::OAK_BUTTON), 64);
    pub const SPRUCE_BUTTON: Self = Self::new(752, Identifier::vanilla_static("spruce_button"), Some(Block::SPRUCE_BUTTON), 64);
    pub const BIRCH_BUTTON: Self = Self::new(753, Identifier::vanilla_static("birch_button"), Some(Block::BIRCH_BUTTON), 64);
    pub const JUNGLE_BUTTON: Self = Self::new(754, Identifier::vanilla_static("jungle_button"), Some(Block::JUNGLE_BUTTON), 64);
    pub const ACACIA_BUTTON: Self = Self::new(755, Identifier::vanilla_static("acacia_button"), Some(Block::ACACIA_BUTTON), 64);
    pub const CHERRY_BUTTON: Self = Self::new(756, Identifier::vanilla_static("cherry_button"), Some(Block::CHERRY_BUTTON), 64);
    pub const DARK_OAK_BUTTON: Self = Self::new(757, Identifier::vanilla_static("dark_oak_button"), Some(Block::DARK_OAK_BUTTON), 64);
    pub const PALE_OAK_BUTTON: Self = Self::new(758, Identifier::vanilla_static("pale_oak_button"), Some(Block::PALE_OAK_BUTTON), 64);
    pub const MANGROVE_BUTTON: Self = Self::new(759, Identifier::vanilla_static("mangrove_button"), Some(Block::MANGROVE_BUTTON), 64);
    pub const BAMBOO_BUTTON: Self = Self::new(760, Identifier::vanilla_static("bamboo_button"), Some(Block::BAMBOO_BUTTON), 64);
    pub const CRIMSON_BUTTON: Self = Self::new(761, Identifier::vanilla_static("crimson_button"), Some(Block::CRIMSON_BUTTON), 64);
    pub const WARPED_BUTTON: Self = Self::new(762, Identifier::vanilla_static("warped_button"), Some(Block::WARPED_BUTTON), 64);
    pub const STONE_PRESSURE_PLATE: Self = Self::new(763, Identifier::vanilla_static("stone_pressure_plate"), Some(Block::STONE_PRESSURE_PLATE), 64);
    pub const POLISHED_BLACKSTONE_PRESSURE_PLATE: Self = Self::new(764, Identifier::vanilla_static("polished_blackstone_pressure_plate"), Some(Block::POLISHED_BLACKSTONE_PRESSURE_PLATE), 64);
    pub const LIGHT_WEIGHTED_PRESSURE_PLATE: Self = Self::new(765, Identifier::vanilla_static("light_weighted_pressure_plate"), Some(Block::LIGHT_WEIGHTED_PRESSURE_PLATE), 64);
    pub const HEAVY_WEIGHTED_PRESSURE_PLATE: Self = Self::new(766, Identifier::vanilla_static("heavy_weighted_pressure_plate"), Some(Block::HEAVY_WEIGHTED_PRESSURE_PLATE), 64);
    pub const OAK_PRESSURE_PLATE: Self = Self::new(767, Identifier::vanilla_static("oak_pressure_plate"), Some(Block::OAK_PRESSURE_PLATE), 64);
    pub const SPRUCE_PRESSURE_PLATE: Self = Self::new(768, Identifier::vanilla_static("spruce_pressure_plate"), Some(Block::SPRUCE_PRESSURE_PLATE), 64);
    pub const BIRCH_PRESSURE_PLATE: Self = Self::new(769, Identifier::vanilla_static("birch_pressure_plate"), Some(Block::BIRCH_PRESSURE_PLATE), 64);
    pub const JUNGLE_PRESSURE_PLATE: Self = Self::new(770, Identifier::vanilla_static("jungle_pressure_plate"), Some(Block::JUNGLE_PRESSURE_PLATE), 64);
    pub const ACACIA_PRESSURE_PLATE: Self = Self::new(771, Identifier::vanilla_static("acacia_pressure_plate"), Some(Block::ACACIA_PRESSURE_PLATE), 64);
    pub const CHERRY_PRESSURE_PLATE: Self = Self::new(772, Identifier::vanilla_static("cherry_pressure_plate"), Some(Block::CHERRY_PRESSURE_PLATE), 64);
    pub const DARK_OAK_PRESSURE_PLATE: Self = Self::new(773, Identifier::vanilla_static("dark_oak_pressure_plate"), Some(Block::DARK_OAK_PRESSURE_PLATE), 64);
    pub const PALE_OAK_PRESSURE_PLATE: Self = Self::new(774, Identifier::vanilla_static("pale_oak_pressure_plate"), Some(Block::PALE_OAK_PRESSURE_PLATE), 64);
    pub const MANGROVE_PRESSURE_PLATE: Self = Self::new(775, Identifier::vanilla_static("mangrove_pressure_plate"), Some(Block::MANGROVE_PRESSURE_PLATE), 64);
    pub const BAMBOO_PRESSURE_PLATE: Self = Self::new(776, Identifier::vanilla_static("bamboo_pressure_plate"), Some(Block::BAMBOO_PRESSURE_PLATE), 64);
    pub const CRIMSON_PRESSURE_PLATE: Self = Self::new(777, Identifier::vanilla_static("crimson_pressure_plate"), Some(Block::CRIMSON_PRESSURE_PLATE), 64);
    pub const WARPED_PRESSURE_PLATE: Self = Self::new(778, Identifier::vanilla_static("warped_pressure_plate"), Some(Block::WARPED_PRESSURE_PLATE), 64);
    pub const IRON_DOOR: Self = Self::new(779, Identifier::vanilla_static("iron_door"), Some(Block::IRON_DOOR), 64);
    pub const OAK_DOOR: Self = Self::new(780, Identifier::vanilla_static("oak_door"), Some(Block::OAK_DOOR), 64);
    pub const SPRUCE_DOOR: Self = Self::new(781, Identifier::vanilla_static("spruce_door"), Some(Block::SPRUCE_DOOR), 64);
    pub const BIRCH_DOOR: Self = Self::new(782, Identifier::vanilla_static("birch_door"), Some(Block::BIRCH_DOOR), 64);
    pub const JUNGLE_DOOR: Self = Self::new(783, Identifier::vanilla_static("jungle_door"), Some(Block::JUNGLE_DOOR), 64);
    pub const ACACIA_DOOR: Self = Self::new(784, Identifier::vanilla_static("acacia_door"), Some(Block::ACACIA_DOOR), 64);
    pub const CHERRY_DOOR: Self = Self::new(785, Identifier::vanilla_static("cherry_door"), Some(Block::CHERRY_DOOR), 64);
    pub const DARK_OAK_DOOR: Self = Self::new(786, Identifier::vanilla_static("dark_oak_door"), Some(Block::DARK_OAK_DOOR), 64);
    pub const PALE_OAK_DOOR: Self = Self::new(787, Identifier::vanilla_static("pale_oak_door"), Some(Block::PALE_OAK_DOOR), 64);
    pub const MANGROVE_DOOR: Self = Self::new(788, Identifier::vanilla_static("mangrove_door"), Some(Block::MANGROVE_DOOR), 64);
    pub const BAMBOO_DOOR: Self = Self::new(789, Identifier::vanilla_static("bamboo_door"), Some(Block::BAMBOO_DOOR), 64);
    pub const CRIMSON_DOOR: Self = Self::new(790, Identifier::vanilla_static("crimson_door"), Some(Block::CRIMSON_DOOR), 64);
    pub const WARPED_DOOR: Self = Self::new(791, Identifier::vanilla_static("warped_door"), Some(Block::WARPED_DOOR), 64);
    pub const COPPER_DOOR: Self = Self::new(792, Identifier::vanilla_static("copper_door"), Some(Block::COPPER_DOOR), 64);
    pub const EXPOSED_COPPER_DOOR: Self = Self::new(793, Identifier::vanilla_static("exposed_copper_door"), Some(Block::EXPOSED_COPPER_DOOR), 64);
    pub const WEATHERED_COPPER_DOOR: Self = Self::new(794, Identifier::vanilla_static("weathered_copper_door"), Some(Block::WEATHERED_COPPER_DOOR), 64);
    pub const OXIDIZED_COPPER_DOOR: Self = Self::new(795, Identifier::vanilla_static("oxidized_copper_door"), Some(Block::OXIDIZED_COPPER_DOOR), 64);
    pub const WAXED_COPPER_DOOR: Self = Self::new(796, Identifier::vanilla_static("waxed_copper_door"), Some(Block::WAXED_COPPER_DOOR), 64);
    pub const WAXED_EXPOSED_COPPER_DOOR: Self = Self::new(797, Identifier::vanilla_static("waxed_exposed_copper_door"), Some(Block::WAXED_EXPOSED_COPPER_DOOR), 64);
    pub const WAXED_WEATHERED_COPPER_DOOR: Self = Self::new(798, Identifier::vanilla_static("waxed_weathered_copper_door"), Some(Block::WAXED_WEATHERED_COPPER_DOOR), 64);
    pub const WAXED_OXIDIZED_COPPER_DOOR: Self = Self::new(799, Identifier::vanilla_static("waxed_oxidized_copper_door"), Some(Block::WAXED_OXIDIZED_COPPER_DOOR), 64);
    pub const IRON_TRAPDOOR: Self = Self::new(800, Identifier::vanilla_static("iron_trapdoor"), Some(Block::IRON_TRAPDOOR), 64);
    pub const OAK_TRAPDOOR: Self = Self::new(801, Identifier::vanilla_static("oak_trapdoor"), Some(Block::OAK_TRAPDOOR), 64);
    pub const SPRUCE_TRAPDOOR: Self = Self::new(802, Identifier::vanilla_static("spruce_trapdoor"), Some(Block::SPRUCE_TRAPDOOR), 64);
    pub const BIRCH_TRAPDOOR: Self = Self::new(803, Identifier::vanilla_static("birch_trapdoor"), Some(Block::BIRCH_TRAPDOOR), 64);
    pub const JUNGLE_TRAPDOOR: Self = Self::new(804, Identifier::vanilla_static("jungle_trapdoor"), Some(Block::JUNGLE_TRAPDOOR), 64);
    pub const ACACIA_TRAPDOOR: Self = Self::new(805, Identifier::vanilla_static("acacia_trapdoor"), Some(Block::ACACIA_TRAPDOOR), 64);
    pub const CHERRY_TRAPDOOR: Self = Self::new(806, Identifier::vanilla_static("cherry_trapdoor"), Some(Block::CHERRY_TRAPDOOR), 64);
    pub const DARK_OAK_TRAPDOOR: Self = Self::new(807, Identifier::vanilla_static("dark_oak_trapdoor"), Some(Block::DARK_OAK_TRAPDOOR), 64);
    pub const PALE_OAK_TRAPDOOR: Self = Self::new(808, Identifier::vanilla_static("pale_oak_trapdoor"), Some(Block::PALE_OAK_TRAPDOOR), 64);
    pub const MANGROVE_TRAPDOOR: Self = Self::new(809, Identifier::vanilla_static("mangrove_trapdoor"), Some(Block::MANGROVE_TRAPDOOR), 64);
    pub const BAMBOO_TRAPDOOR: Self = Self::new(810, Identifier::vanilla_static("bamboo_trapdoor"), Some(Block::BAMBOO_TRAPDOOR), 64);
    pub const CRIMSON_TRAPDOOR: Self = Self::new(811, Identifier::vanilla_static("crimson_trapdoor"), Some(Block::CRIMSON_TRAPDOOR), 64);
    pub const WARPED_TRAPDOOR: Self = Self::new(812, Identifier::vanilla_static("warped_trapdoor"), Some(Block::WARPED_TRAPDOOR), 64);
    pub const COPPER_TRAPDOOR: Self = Self::new(813, Identifier::vanilla_static("copper_trapdoor"), Some(Block::COPPER_TRAPDOOR), 64);
    pub const EXPOSED_COPPER_TRAPDOOR: Self = Self::new(814, Identifier::vanilla_static("exposed_copper_trapdoor"), Some(Block::EXPOSED_COPPER_TRAPDOOR), 64);
    pub const WEATHERED_COPPER_TRAPDOOR: Self = Self::new(815, Identifier::vanilla_static("weathered_copper_trapdoor"), Some(Block::WEATHERED_COPPER_TRAPDOOR), 64);
    pub const OXIDIZED_COPPER_TRAPDOOR: Self = Self::new(816, Identifier::vanilla_static("oxidized_copper_trapdoor"), Some(Block::OXIDIZED_COPPER_TRAPDOOR), 64);
    pub const WAXED_COPPER_TRAPDOOR: Self = Self::new(817, Identifier::vanilla_static("waxed_copper_trapdoor"), Some(Block::WAXED_COPPER_TRAPDOOR), 64);
    pub const WAXED_EXPOSED_COPPER_TRAPDOOR: Self = Self::new(818, Identifier::vanilla_static("waxed_exposed_copper_trapdoor"), Some(Block::WAXED_EXPOSED_COPPER_TRAPDOOR), 64);
    pub const WAXED_WEATHERED_COPPER_TRAPDOOR: Self = Self::new(819, Identifier::vanilla_static("waxed_weathered_copper_trapdoor"), Some(Block::WAXED_WEATHERED_COPPER_TRAPDOOR), 64);
    pub const WAXED_OXIDIZED_COPPER_TRAPDOOR: Self = Self::new(820, Identifier::vanilla_static("waxed_oxidized_copper_trapdoor"), Some(Block::WAXED_OXIDIZED_COPPER_TRAPDOOR), 64);
    pub const OAK_FENCE_GATE: Self = Self::new(821, Identifier::vanilla_static("oak_fence_gate"), Some(Block::OAK_FENCE_GATE), 64);
    pub const SPRUCE_FENCE_GATE: Self = Self::new(822, Identifier::vanilla_static("spruce_fence_gate"), Some(Block::SPRUCE_FENCE_GATE), 64);
    pub const BIRCH_FENCE_GATE: Self = Self::new(823, Identifier::vanilla_static("birch_fence_gate"), Some(Block::BIRCH_FENCE_GATE), 64);
    pub const JUNGLE_FENCE_GATE: Self = Self::new(824, Identifier::vanilla_static("jungle_fence_gate"), Some(Block::JUNGLE_FENCE_GATE), 64);
    pub const ACACIA_FENCE_GATE: Self = Self::new(825, Identifier::vanilla_static("acacia_fence_gate"), Some(Block::ACACIA_FENCE_GATE), 64);
    pub const CHERRY_FENCE_GATE: Self = Self::new(826, Identifier::vanilla_static("cherry_fence_gate"), Some(Block::CHERRY_FENCE_GATE), 64);
    pub const DARK_OAK_FENCE_GATE: Self = Self::new(827, Identifier::vanilla_static("dark_oak_fence_gate"), Some(Block::DARK_OAK_FENCE_GATE), 64);
    pub const PALE_OAK_FENCE_GATE: Self = Self::new(828, Identifier::vanilla_static("pale_oak_fence_gate"), Some(Block::PALE_OAK_FENCE_GATE), 64);
    pub const MANGROVE_FENCE_GATE: Self = Self::new(829, Identifier::vanilla_static("mangrove_fence_gate"), Some(Block::MANGROVE_FENCE_GATE), 64);
    pub const BAMBOO_FENCE_GATE: Self = Self::new(830, Identifier::vanilla_static("bamboo_fence_gate"), Some(Block::BAMBOO_FENCE_GATE), 64);
    pub const CRIMSON_FENCE_GATE: Self = Self::new(831, Identifier::vanilla_static("crimson_fence_gate"), Some(Block::CRIMSON_FENCE_GATE), 64);
    pub const WARPED_FENCE_GATE: Self = Self::new(832, Identifier::vanilla_static("warped_fence_gate"), Some(Block::WARPED_FENCE_GATE), 64);
    pub const POWERED_RAIL: Self = Self::new(833, Identifier::vanilla_static("powered_rail"), Some(Block::POWERED_RAIL), 64);
    pub const DETECTOR_RAIL: Self = Self::new(834, Identifier::vanilla_static("detector_rail"), Some(Block::DETECTOR_RAIL), 64);
    pub const RAIL: Self = Self::new(835, Identifier::vanilla_static("rail"), Some(Block::RAIL), 64);
    pub const ACTIVATOR_RAIL: Self = Self::new(836, Identifier::vanilla_static("activator_rail"), Some(Block::ACTIVATOR_RAIL), 64);
    pub const SADDLE: Self = Self::new(837, Identifier::vanilla_static("saddle"), None, 1);
    pub const WHITE_HARNESS: Self = Self::new(838, Identifier::vanilla_static("white_harness"), None, 1);
    pub const ORANGE_HARNESS: Self = Self::new(839, Identifier::vanilla_static("orange_harness"), None, 1);
    pub const MAGENTA_HARNESS: Self = Self::new(840, Identifier::vanilla_static("magenta_harness"), None, 1);
    pub const LIGHT_BLUE_HARNESS: Self = Self::new(841, Identifier::vanilla_static("light_blue_harness"), None, 1);
    pub const YELLOW_HARNESS: Self = Self::new(842, Identifier::vanilla_static("yellow_harness"), None, 1);
    pub const LIME_HARNESS: Self = Self::new(843, Identifier::vanilla_static("lime_harness"), None, 1);
    pub const PINK_HARNESS: Self = Self::new(844, Identifier::vanilla_static("pink_harness"), None, 1);
    pub const GRAY_HARNESS: Self = Self::new(845, Identifier::vanilla_static("gray_harness"), None, 1);
    pub const LIGHT_GRAY_HARNESS: Self = Self::new(846, Identifier::vanilla_static("light_gray_harness"), None, 1);
    pub const CYAN_HARNESS: Self = Self::new(847, Identifier::vanilla_static("cyan_harness"), None, 1);
    pub const PURPLE_HARNESS: Self = Self::new(848, Identifier::vanilla_static("purple_harness"), None, 1);
    pub const BLUE_HARNESS: Self = Self::new(849, Identifier::vanilla_static("blue_harness"), None, 1);
    pub const BROWN_HARNESS: Self = Self::new(850, Identifier::vanilla_static("brown_harness"), None, 1);
    pub const GREEN_HARNESS: Self = Self::new(851, Identifier::vanilla_static("green_harness"), None, 1);
    pub const RED_HARNESS: Self = Self::new(852, Identifier::vanilla_static("red_harness"), None, 1);
    pub const BLACK_HARNESS: Self = Self::new(853, Identifier::vanilla_static("black_harness"), None, 1);
    pub const MINECART: Self = Self::new(854, Identifier::vanilla_static("minecart"), None, 1);
    pub const CHEST_MINECART: Self = Self::new(855, Identifier::vanilla_static("chest_minecart"), None, 1);
    pub const FURNACE_MINECART: Self = Self::new(856, Identifier::vanilla_static("furnace_minecart"), None, 1);
    pub const TNT_MINECART: Self = Self::new(857, Identifier::vanilla_static("tnt_minecart"), None, 1);
    pub const HOPPER_MINECART: Self = Self::new(858, Identifier::vanilla_static("hopper_minecart"), None, 1);
    pub const CARROT_ON_A_STICK: Self = Self::new(859, Identifier::vanilla_static("carrot_on_a_stick"), None, 1);
    pub const WARPED_FUNGUS_ON_A_STICK: Self = Self::new(860, Identifier::vanilla_static("warped_fungus_on_a_stick"), None, 1);
    pub const PHANTOM_MEMBRANE: Self = Self::new(861, Identifier::vanilla_static("phantom_membrane"), None, 64);
    pub const ELYTRA: Self = Self::new(862, Identifier::vanilla_static("elytra"), None, 1);
    pub const OAK_BOAT: Self = Self::new(863, Identifier::vanilla_static("oak_boat"), None, 1);
    pub const OAK_CHEST_BOAT: Self = Self::new(864, Identifier::vanilla_static("oak_chest_boat"), None, 1);
    pub const SPRUCE_BOAT: Self = Self::new(865, Identifier::vanilla_static("spruce_boat"), None, 1);
    pub const SPRUCE_CHEST_BOAT: Self = Self::new(866, Identifier::vanilla_static("spruce_chest_boat"), None, 1);
    pub const BIRCH_BOAT: Self = Self::new(867, Identifier::vanilla_static("birch_boat"), None, 1);
    pub const BIRCH_CHEST_BOAT: Self = Self::new(868, Identifier::vanilla_static("birch_chest_boat"), None, 1);
    pub const JUNGLE_BOAT: Self = Self::new(869, Identifier::vanilla_static("jungle_boat"), None, 1);
    pub const JUNGLE_CHEST_BOAT: Self = Self::new(870, Identifier::vanilla_static("jungle_chest_boat"), None, 1);
    pub const ACACIA_BOAT: Self = Self::new(871, Identifier::vanilla_static("acacia_boat"), None, 1);
    pub const ACACIA_CHEST_BOAT: Self = Self::new(872, Identifier::vanilla_static("acacia_chest_boat"), None, 1);
    pub const CHERRY_BOAT: Self = Self::new(873, Identifier::vanilla_static("cherry_boat"), None, 1);
    pub const CHERRY_CHEST_BOAT: Self = Self::new(874, Identifier::vanilla_static("cherry_chest_boat"), None, 1);
    pub const DARK_OAK_BOAT: Self = Self::new(875, Identifier::vanilla_static("dark_oak_boat"), None, 1);
    pub const DARK_OAK_CHEST_BOAT: Self = Self::new(876, Identifier::vanilla_static("dark_oak_chest_boat"), None, 1);
    pub const PALE_OAK_BOAT: Self = Self::new(877, Identifier::vanilla_static("pale_oak_boat"), None, 1);
    pub const PALE_OAK_CHEST_BOAT: Self = Self::new(878, Identifier::vanilla_static("pale_oak_chest_boat"), None, 1);
    pub const MANGROVE_BOAT: Self = Self::new(879, Identifier::vanilla_static("mangrove_boat"), None, 1);
    pub const MANGROVE_CHEST_BOAT: Self = Self::new(880, Identifier::vanilla_static("mangrove_chest_boat"), None, 1);
    pub const BAMBOO_RAFT: Self = Self::new(881, Identifier::vanilla_static("bamboo_raft"), None, 1);
    pub const BAMBOO_CHEST_RAFT: Self = Self::new(882, Identifier::vanilla_static("bamboo_chest_raft"), None, 1);
    pub const STRUCTURE_BLOCK: Self = Self::new(883, Identifier::vanilla_static("structure_block"), Some(Block::STRUCTURE_BLOCK), 64);
    pub const JIGSAW: Self = Self::new(884, Identifier::vanilla_static("jigsaw"), Some(Block::JIGSAW), 64);
    pub const TEST_BLOCK: Self = Self::new(885, Identifier::vanilla_static("test_block"), Some(Block::TEST_BLOCK), 64);
    pub const TEST_INSTANCE_BLOCK: Self = Self::new(886, Identifier::vanilla_static("test_instance_block"), Some(Block::TEST_INSTANCE_BLOCK), 64);
    pub const TURTLE_HELMET: Self = Self::new(887, Identifier::vanilla_static("turtle_helmet"), None, 1);
    pub const TURTLE_SCUTE: Self = Self::new(888, Identifier::vanilla_static("turtle_scute"), None, 64);
    pub const ARMADILLO_SCUTE: Self = Self::new(889, Identifier::vanilla_static("armadillo_scute"), None, 64);
    pub const WOLF_ARMOR: Self = Self::new(890, Identifier::vanilla_static("wolf_armor"), None, 1);
    pub const FLINT_AND_STEEL: Self = Self::new(891, Identifier::vanilla_static("flint_and_steel"), None, 1);
    pub const BOWL: Self = Self::new(892, Identifier::vanilla_static("bowl"), None, 64);
    pub const APPLE: Self = Self::new(893, Identifier::vanilla_static("apple"), None, 64);
    pub const BOW: Self = Self::new(894, Identifier::vanilla_static("bow"), None, 1);
    pub const ARROW: Self = Self::new(895, Identifier::vanilla_static("arrow"), None, 64);
    pub const COAL: Self = Self::new(896, Identifier::vanilla_static("coal"), None, 64);
    pub const CHARCOAL: Self = Self::new(897, Identifier::vanilla_static("charcoal"), None, 64);
    pub const DIAMOND: Self = Self::new(898, Identifier::vanilla_static("diamond"), None, 64);
    pub const EMERALD: Self = Self::new(899, Identifier::vanilla_static("emerald"), None, 64);
    pub const LAPIS_LAZULI: Self = Self::new(900, Identifier::vanilla_static("lapis_lazuli"), None, 64);
    pub const QUARTZ: Self = Self::new(901, Identifier::vanilla_static("quartz"), None, 64);
    pub const AMETHYST_SHARD: Self = Self::new(902, Identifier::vanilla_static("amethyst_shard"), None, 64);
    pub const RAW_IRON: Self = Self::new(903, Identifier::vanilla_static("raw_iron"), None, 64);
    pub const IRON_INGOT: Self = Self::new(904, Identifier::vanilla_static("iron_ingot"), None, 64);
    pub const RAW_COPPER: Self = Self::new(905, Identifier::vanilla_static("raw_copper"), None, 64);
    pub const COPPER_INGOT: Self = Self::new(906, Identifier::vanilla_static("copper_ingot"), None, 64);
    pub const RAW_GOLD: Self = Self::new(907, Identifier::vanilla_static("raw_gold"), None, 64);
    pub const GOLD_INGOT: Self = Self::new(908, Identifier::vanilla_static("gold_ingot"), None, 64);
    pub const NETHERITE_INGOT: Self = Self::new(909, Identifier::vanilla_static("netherite_ingot"), None, 64);
    pub const NETHERITE_SCRAP: Self = Self::new(910, Identifier::vanilla_static("netherite_scrap"), None, 64);
    pub const WOODEN_SWORD: Self = Self::new(911, Identifier::vanilla_static("wooden_sword"), None, 1);
    pub const WOODEN_SHOVEL: Self = Self::new(912, Identifier::vanilla_static("wooden_shovel"), None, 1);
    pub const WOODEN_PICKAXE: Self = Self::new(913, Identifier::vanilla_static("wooden_pickaxe"), None, 1);
    pub const WOODEN_AXE: Self = Self::new(914, Identifier::vanilla_static("wooden_axe"), None, 1);
    pub const WOODEN_HOE: Self = Self::new(915, Identifier::vanilla_static("wooden_hoe"), None, 1);
    pub const COPPER_SWORD: Self = Self::new(916, Identifier::vanilla_static("copper_sword"), None, 1);
    pub const COPPER_SHOVEL: Self = Self::new(917, Identifier::vanilla_static("copper_shovel"), None, 1);
    pub const COPPER_PICKAXE: Self = Self::new(918, Identifier::vanilla_static("copper_pickaxe"), None, 1);
    pub const COPPER_AXE: Self = Self::new(919, Identifier::vanilla_static("copper_axe"), None, 1);
    pub const COPPER_HOE: Self = Self::new(920, Identifier::vanilla_static("copper_hoe"), None, 1);
    pub const STONE_SWORD: Self = Self::new(921, Identifier::vanilla_static("stone_sword"), None, 1);
    pub const STONE_SHOVEL: Self = Self::new(922, Identifier::vanilla_static("stone_shovel"), None, 1);
    pub const STONE_PICKAXE: Self = Self::new(923, Identifier::vanilla_static("stone_pickaxe"), None, 1);
    pub const STONE_AXE: Self = Self::new(924, Identifier::vanilla_static("stone_axe"), None, 1);
    pub const STONE_HOE: Self = Self::new(925, Identifier::vanilla_static("stone_hoe"), None, 1);
    pub const GOLDEN_SWORD: Self = Self::new(926, Identifier::vanilla_static("golden_sword"), None, 1);
    pub const GOLDEN_SHOVEL: Self = Self::new(927, Identifier::vanilla_static("golden_shovel"), None, 1);
    pub const GOLDEN_PICKAXE: Self = Self::new(928, Identifier::vanilla_static("golden_pickaxe"), None, 1);
    pub const GOLDEN_AXE: Self = Self::new(929, Identifier::vanilla_static("golden_axe"), None, 1);
    pub const GOLDEN_HOE: Self = Self::new(930, Identifier::vanilla_static("golden_hoe"), None, 1);
    pub const IRON_SWORD: Self = Self::new(931, Identifier::vanilla_static("iron_sword"), None, 1);
    pub const IRON_SHOVEL: Self = Self::new(932, Identifier::vanilla_static("iron_shovel"), None, 1);
    pub const IRON_PICKAXE: Self = Self::new(933, Identifier::vanilla_static("iron_pickaxe"), None, 1);
    pub const IRON_AXE: Self = Self::new(934, Identifier::vanilla_static("iron_axe"), None, 1);
    pub const IRON_HOE: Self = Self::new(935, Identifier::vanilla_static("iron_hoe"), None, 1);
    pub const DIAMOND_SWORD: Self = Self::new(936, Identifier::vanilla_static("diamond_sword"), None, 1);
    pub const DIAMOND_SHOVEL: Self = Self::new(937, Identifier::vanilla_static("diamond_shovel"), None, 1);
    pub const DIAMOND_PICKAXE: Self = Self::new(938, Identifier::vanilla_static("diamond_pickaxe"), None, 1);
    pub const DIAMOND_AXE: Self = Self::new(939, Identifier::vanilla_static("diamond_axe"), None, 1);
    pub const DIAMOND_HOE: Self = Self::new(940, Identifier::vanilla_static("diamond_hoe"), None, 1);
    pub const NETHERITE_SWORD: Self = Self::new(941, Identifier::vanilla_static("netherite_sword"), None, 1);
    pub const NETHERITE_SHOVEL: Self = Self::new(942, Identifier::vanilla_static("netherite_shovel"), None, 1);
    pub const NETHERITE_PICKAXE: Self = Self::new(943, Identifier::vanilla_static("netherite_pickaxe"), None, 1);
    pub const NETHERITE_AXE: Self = Self::new(944, Identifier::vanilla_static("netherite_axe"), None, 1);
    pub const NETHERITE_HOE: Self = Self::new(945, Identifier::vanilla_static("netherite_hoe"), None, 1);
    pub const STICK: Self = Self::new(946, Identifier::vanilla_static("stick"), None, 64);
    pub const MUSHROOM_STEW: Self = Self::new(947, Identifier::vanilla_static("mushroom_stew"), None, 1);
    pub const STRING: Self = Self::new(948, Identifier::vanilla_static("string"), Some(Block::TRIPWIRE), 64);
    pub const FEATHER: Self = Self::new(949, Identifier::vanilla_static("feather"), None, 64);
    pub const GUNPOWDER: Self = Self::new(950, Identifier::vanilla_static("gunpowder"), None, 64);
    pub const WHEAT_SEEDS: Self = Self::new(951, Identifier::vanilla_static("wheat_seeds"), Some(Block::WHEAT), 64);
    pub const WHEAT: Self = Self::new(952, Identifier::vanilla_static("wheat"), Some(Block::WHEAT), 64);
    pub const BREAD: Self = Self::new(953, Identifier::vanilla_static("bread"), None, 64);
    pub const LEATHER_HELMET: Self = Self::new(954, Identifier::vanilla_static("leather_helmet"), None, 1);
    pub const LEATHER_CHESTPLATE: Self = Self::new(955, Identifier::vanilla_static("leather_chestplate"), None, 1);
    pub const LEATHER_LEGGINGS: Self = Self::new(956, Identifier::vanilla_static("leather_leggings"), None, 1);
    pub const LEATHER_BOOTS: Self = Self::new(957, Identifier::vanilla_static("leather_boots"), None, 1);
    pub const COPPER_HELMET: Self = Self::new(958, Identifier::vanilla_static("copper_helmet"), None, 1);
    pub const COPPER_CHESTPLATE: Self = Self::new(959, Identifier::vanilla_static("copper_chestplate"), None, 1);
    pub const COPPER_LEGGINGS: Self = Self::new(960, Identifier::vanilla_static("copper_leggings"), None, 1);
    pub const COPPER_BOOTS: Self = Self::new(961, Identifier::vanilla_static("copper_boots"), None, 1);
    pub const CHAINMAIL_HELMET: Self = Self::new(962, Identifier::vanilla_static("chainmail_helmet"), None, 1);
    pub const CHAINMAIL_CHESTPLATE: Self = Self::new(963, Identifier::vanilla_static("chainmail_chestplate"), None, 1);
    pub const CHAINMAIL_LEGGINGS: Self = Self::new(964, Identifier::vanilla_static("chainmail_leggings"), None, 1);
    pub const CHAINMAIL_BOOTS: Self = Self::new(965, Identifier::vanilla_static("chainmail_boots"), None, 1);
    pub const IRON_HELMET: Self = Self::new(966, Identifier::vanilla_static("iron_helmet"), None, 1);
    pub const IRON_CHESTPLATE: Self = Self::new(967, Identifier::vanilla_static("iron_chestplate"), None, 1);
    pub const IRON_LEGGINGS: Self = Self::new(968, Identifier::vanilla_static("iron_leggings"), None, 1);
    pub const IRON_BOOTS: Self = Self::new(969, Identifier::vanilla_static("iron_boots"), None, 1);
    pub const DIAMOND_HELMET: Self = Self::new(970, Identifier::vanilla_static("diamond_helmet"), None, 1);
    pub const DIAMOND_CHESTPLATE: Self = Self::new(971, Identifier::vanilla_static("diamond_chestplate"), None, 1);
    pub const DIAMOND_LEGGINGS: Self = Self::new(972, Identifier::vanilla_static("diamond_leggings"), None, 1);
    pub const DIAMOND_BOOTS: Self = Self::new(973, Identifier::vanilla_static("diamond_boots"), None, 1);
    pub const GOLDEN_HELMET: Self = Self::new(974, Identifier::vanilla_static("golden_helmet"), None, 1);
    pub const GOLDEN_CHESTPLATE: Self = Self::new(975, Identifier::vanilla_static("golden_chestplate"), None, 1);
    pub const GOLDEN_LEGGINGS: Self = Self::new(976, Identifier::vanilla_static("golden_leggings"), None, 1);
    pub const GOLDEN_BOOTS: Self = Self::new(977, Identifier::vanilla_static("golden_boots"), None, 1);
    pub const NETHERITE_HELMET: Self = Self::new(978, Identifier::vanilla_static("netherite_helmet"), None, 1);
    pub const NETHERITE_CHESTPLATE: Self = Self::new(979, Identifier::vanilla_static("netherite_chestplate"), None, 1);
    pub const NETHERITE_LEGGINGS: Self = Self::new(980, Identifier::vanilla_static("netherite_leggings"), None, 1);
    pub const NETHERITE_BOOTS: Self = Self::new(981, Identifier::vanilla_static("netherite_boots"), None, 1);
    pub const FLINT: Self = Self::new(982, Identifier::vanilla_static("flint"), None, 64);
    pub const PORKCHOP: Self = Self::new(983, Identifier::vanilla_static("porkchop"), None, 64);
    pub const COOKED_PORKCHOP: Self = Self::new(984, Identifier::vanilla_static("cooked_porkchop"), None, 64);
    pub const PAINTING: Self = Self::new(985, Identifier::vanilla_static("painting"), None, 64);
    pub const GOLDEN_APPLE: Self = Self::new(986, Identifier::vanilla_static("golden_apple"), None, 64);
    pub const ENCHANTED_GOLDEN_APPLE: Self = Self::new(987, Identifier::vanilla_static("enchanted_golden_apple"), None, 64);
    pub const OAK_SIGN: Self = Self::new(988, Identifier::vanilla_static("oak_sign"), Some(Block::OAK_SIGN), 16);
    pub const SPRUCE_SIGN: Self = Self::new(989, Identifier::vanilla_static("spruce_sign"), Some(Block::SPRUCE_SIGN), 16);
    pub const BIRCH_SIGN: Self = Self::new(990, Identifier::vanilla_static("birch_sign"), Some(Block::BIRCH_SIGN), 16);
    pub const JUNGLE_SIGN: Self = Self::new(991, Identifier::vanilla_static("jungle_sign"), Some(Block::JUNGLE_SIGN), 16);
    pub const ACACIA_SIGN: Self = Self::new(992, Identifier::vanilla_static("acacia_sign"), Some(Block::ACACIA_SIGN), 16);
    pub const CHERRY_SIGN: Self = Self::new(993, Identifier::vanilla_static("cherry_sign"), Some(Block::CHERRY_SIGN), 16);
    pub const DARK_OAK_SIGN: Self = Self::new(994, Identifier::vanilla_static("dark_oak_sign"), Some(Block::DARK_OAK_SIGN), 16);
    pub const PALE_OAK_SIGN: Self = Self::new(995, Identifier::vanilla_static("pale_oak_sign"), Some(Block::PALE_OAK_SIGN), 16);
    pub const MANGROVE_SIGN: Self = Self::new(996, Identifier::vanilla_static("mangrove_sign"), Some(Block::MANGROVE_SIGN), 16);
    pub const BAMBOO_SIGN: Self = Self::new(997, Identifier::vanilla_static("bamboo_sign"), Some(Block::BAMBOO_SIGN), 16);
    pub const CRIMSON_SIGN: Self = Self::new(998, Identifier::vanilla_static("crimson_sign"), Some(Block::CRIMSON_SIGN), 16);
    pub const WARPED_SIGN: Self = Self::new(999, Identifier::vanilla_static("warped_sign"), Some(Block::WARPED_SIGN), 16);
    pub const OAK_HANGING_SIGN: Self = Self::new(1000, Identifier::vanilla_static("oak_hanging_sign"), Some(Block::OAK_HANGING_SIGN), 16);
    pub const SPRUCE_HANGING_SIGN: Self = Self::new(1001, Identifier::vanilla_static("spruce_hanging_sign"), Some(Block::SPRUCE_HANGING_SIGN), 16);
    pub const BIRCH_HANGING_SIGN: Self = Self::new(1002, Identifier::vanilla_static("birch_hanging_sign"), Some(Block::BIRCH_HANGING_SIGN), 16);
    pub const JUNGLE_HANGING_SIGN: Self = Self::new(1003, Identifier::vanilla_static("jungle_hanging_sign"), Some(Block::JUNGLE_HANGING_SIGN), 16);
    pub const ACACIA_HANGING_SIGN: Self = Self::new(1004, Identifier::vanilla_static("acacia_hanging_sign"), Some(Block::ACACIA_HANGING_SIGN), 16);
    pub const CHERRY_HANGING_SIGN: Self = Self::new(1005, Identifier::vanilla_static("cherry_hanging_sign"), Some(Block::CHERRY_HANGING_SIGN), 16);
    pub const DARK_OAK_HANGING_SIGN: Self = Self::new(1006, Identifier::vanilla_static("dark_oak_hanging_sign"), Some(Block::DARK_OAK_HANGING_SIGN), 16);
    pub const PALE_OAK_HANGING_SIGN: Self = Self::new(1007, Identifier::vanilla_static("pale_oak_hanging_sign"), Some(Block::PALE_OAK_HANGING_SIGN), 16);
    pub const MANGROVE_HANGING_SIGN: Self = Self::new(1008, Identifier::vanilla_static("mangrove_hanging_sign"), Some(Block::MANGROVE_HANGING_SIGN), 16);
    pub const BAMBOO_HANGING_SIGN: Self = Self::new(1009, Identifier::vanilla_static("bamboo_hanging_sign"), Some(Block::BAMBOO_HANGING_SIGN), 16);
    pub const CRIMSON_HANGING_SIGN: Self = Self::new(1010, Identifier::vanilla_static("crimson_hanging_sign"), Some(Block::CRIMSON_HANGING_SIGN), 16);
    pub const WARPED_HANGING_SIGN: Self = Self::new(1011, Identifier::vanilla_static("warped_hanging_sign"), Some(Block::WARPED_HANGING_SIGN), 16);
    pub const BUCKET: Self = Self::new(1012, Identifier::vanilla_static("bucket"), None, 16);
    pub const WATER_BUCKET: Self = Self::new(1013, Identifier::vanilla_static("water_bucket"), None, 1);
    pub const LAVA_BUCKET: Self = Self::new(1014, Identifier::vanilla_static("lava_bucket"), None, 1);
    pub const POWDER_SNOW_BUCKET: Self = Self::new(1015, Identifier::vanilla_static("powder_snow_bucket"), Some(Block::POWDER_SNOW), 1);
    pub const SNOWBALL: Self = Self::new(1016, Identifier::vanilla_static("snowball"), None, 16);
    pub const LEATHER: Self = Self::new(1017, Identifier::vanilla_static("leather"), None, 64);
    pub const MILK_BUCKET: Self = Self::new(1018, Identifier::vanilla_static("milk_bucket"), None, 1);
    pub const PUFFERFISH_BUCKET: Self = Self::new(1019, Identifier::vanilla_static("pufferfish_bucket"), None, 1);
    pub const SALMON_BUCKET: Self = Self::new(1020, Identifier::vanilla_static("salmon_bucket"), None, 1);
    pub const COD_BUCKET: Self = Self::new(1021, Identifier::vanilla_static("cod_bucket"), None, 1);
    pub const TROPICAL_FISH_BUCKET: Self = Self::new(1022, Identifier::vanilla_static("tropical_fish_bucket"), None, 1);
    pub const AXOLOTL_BUCKET: Self = Self::new(1023, Identifier::vanilla_static("axolotl_bucket"), None, 1);
    pub const TADPOLE_BUCKET: Self = Self::new(1024, Identifier::vanilla_static("tadpole_bucket"), None, 1);
    pub const BRICK: Self = Self::new(1025, Identifier::vanilla_static("brick"), None, 64);
    pub const CLAY_BALL: Self = Self::new(1026, Identifier::vanilla_static("clay_ball"), None, 64);
    pub const DRIED_KELP_BLOCK: Self = Self::new(1027, Identifier::vanilla_static("dried_kelp_block"), Some(Block::DRIED_KELP_BLOCK), 64);
    pub const PAPER: Self = Self::new(1028, Identifier::vanilla_static("paper"), None, 64);
    pub const BOOK: Self = Self::new(1029, Identifier::vanilla_static("book"), None, 64);
    pub const SLIME_BALL: Self = Self::new(1030, Identifier::vanilla_static("slime_ball"), None, 64);
    pub const EGG: Self = Self::new(1031, Identifier::vanilla_static("egg"), None, 16);
    pub const BLUE_EGG: Self = Self::new(1032, Identifier::vanilla_static("blue_egg"), None, 16);
    pub const BROWN_EGG: Self = Self::new(1033, Identifier::vanilla_static("brown_egg"), None, 16);
    pub const COMPASS: Self = Self::new(1034, Identifier::vanilla_static("compass"), None, 64);
    pub const RECOVERY_COMPASS: Self = Self::new(1035, Identifier::vanilla_static("recovery_compass"), None, 64);
    pub const BUNDLE: Self = Self::new(1036, Identifier::vanilla_static("bundle"), None, 1);
    pub const WHITE_BUNDLE: Self = Self::new(1037, Identifier::vanilla_static("white_bundle"), None, 1);
    pub const ORANGE_BUNDLE: Self = Self::new(1038, Identifier::vanilla_static("orange_bundle"), None, 1);
    pub const MAGENTA_BUNDLE: Self = Self::new(1039, Identifier::vanilla_static("magenta_bundle"), None, 1);
    pub const LIGHT_BLUE_BUNDLE: Self = Self::new(1040, Identifier::vanilla_static("light_blue_bundle"), None, 1);
    pub const YELLOW_BUNDLE: Self = Self::new(1041, Identifier::vanilla_static("yellow_bundle"), None, 1);
    pub const LIME_BUNDLE: Self = Self::new(1042, Identifier::vanilla_static("lime_bundle"), None, 1);
    pub const PINK_BUNDLE: Self = Self::new(1043, Identifier::vanilla_static("pink_bundle"), None, 1);
    pub const GRAY_BUNDLE: Self = Self::new(1044, Identifier::vanilla_static("gray_bundle"), None, 1);
    pub const LIGHT_GRAY_BUNDLE: Self = Self::new(1045, Identifier::vanilla_static("light_gray_bundle"), None, 1);
    pub const CYAN_BUNDLE: Self = Self::new(1046, Identifier::vanilla_static("cyan_bundle"), None, 1);
    pub const PURPLE_BUNDLE: Self = Self::new(1047, Identifier::vanilla_static("purple_bundle"), None, 1);
    pub const BLUE_BUNDLE: Self = Self::new(1048, Identifier::vanilla_static("blue_bundle"), None, 1);
    pub const BROWN_BUNDLE: Self = Self::new(1049, Identifier::vanilla_static("brown_bundle"), None, 1);
    pub const GREEN_BUNDLE: Self = Self::new(1050, Identifier::vanilla_static("green_bundle"), None, 1);
    pub const RED_BUNDLE: Self = Self::new(1051, Identifier::vanilla_static("red_bundle"), None, 1);
    pub const BLACK_BUNDLE: Self = Self::new(1052, Identifier::vanilla_static("black_bundle"), None, 1);
    pub const FISHING_ROD: Self = Self::new(1053, Identifier::vanilla_static("fishing_rod"), None, 1);
    pub const CLOCK: Self = Self::new(1054, Identifier::vanilla_static("clock"), None, 64);
    pub const SPYGLASS: Self = Self::new(1055, Identifier::vanilla_static("spyglass"), None, 1);
    pub const GLOWSTONE_DUST: Self = Self::new(1056, Identifier::vanilla_static("glowstone_dust"), None, 64);
    pub const COD: Self = Self::new(1057, Identifier::vanilla_static("cod"), None, 64);
    pub const SALMON: Self = Self::new(1058, Identifier::vanilla_static("salmon"), None, 64);
    pub const TROPICAL_FISH: Self = Self::new(1059, Identifier::vanilla_static("tropical_fish"), None, 64);
    pub const PUFFERFISH: Self = Self::new(1060, Identifier::vanilla_static("pufferfish"), None, 64);
    pub const COOKED_COD: Self = Self::new(1061, Identifier::vanilla_static("cooked_cod"), None, 64);
    pub const COOKED_SALMON: Self = Self::new(1062, Identifier::vanilla_static("cooked_salmon"), None, 64);
    pub const INK_SAC: Self = Self::new(1063, Identifier::vanilla_static("ink_sac"), None, 64);
    pub const GLOW_INK_SAC: Self = Self::new(1064, Identifier::vanilla_static("glow_ink_sac"), None, 64);
    pub const COCOA_BEANS: Self = Self::new(1065, Identifier::vanilla_static("cocoa_beans"), Some(Block::COCOA), 64);
    pub const WHITE_DYE: Self = Self::new(1066, Identifier::vanilla_static("white_dye"), None, 64);
    pub const ORANGE_DYE: Self = Self::new(1067, Identifier::vanilla_static("orange_dye"), None, 64);
    pub const MAGENTA_DYE: Self = Self::new(1068, Identifier::vanilla_static("magenta_dye"), None, 64);
    pub const LIGHT_BLUE_DYE: Self = Self::new(1069, Identifier::vanilla_static("light_blue_dye"), None, 64);
    pub const YELLOW_DYE: Self = Self::new(1070, Identifier::vanilla_static("yellow_dye"), None, 64);
    pub const LIME_DYE: Self = Self::new(1071, Identifier::vanilla_static("lime_dye"), None, 64);
    pub const PINK_DYE: Self = Self::new(1072, Identifier::vanilla_static("pink_dye"), None, 64);
    pub const GRAY_DYE: Self = Self::new(1073, Identifier::vanilla_static("gray_dye"), None, 64);
    pub const LIGHT_GRAY_DYE: Self = Self::new(1074, Identifier::vanilla_static("light_gray_dye"), None, 64);
    pub const CYAN_DYE: Self = Self::new(1075, Identifier::vanilla_static("cyan_dye"), None, 64);
    pub const PURPLE_DYE: Self = Self::new(1076, Identifier::vanilla_static("purple_dye"), None, 64);
    pub const BLUE_DYE: Self = Self::new(1077, Identifier::vanilla_static("blue_dye"), None, 64);
    pub const BROWN_DYE: Self = Self::new(1078, Identifier::vanilla_static("brown_dye"), None, 64);
    pub const GREEN_DYE: Self = Self::new(1079, Identifier::vanilla_static("green_dye"), None, 64);
    pub const RED_DYE: Self = Self::new(1080, Identifier::vanilla_static("red_dye"), None, 64);
    pub const BLACK_DYE: Self = Self::new(1081, Identifier::vanilla_static("black_dye"), None, 64);
    pub const BONE_MEAL: Self = Self::new(1082, Identifier::vanilla_static("bone_meal"), None, 64);
    pub const BONE: Self = Self::new(1083, Identifier::vanilla_static("bone"), None, 64);
    pub const SUGAR: Self = Self::new(1084, Identifier::vanilla_static("sugar"), None, 64);
    pub const CAKE: Self = Self::new(1085, Identifier::vanilla_static("cake"), Some(Block::CAKE), 1);
    pub const WHITE_BED: Self = Self::new(1086, Identifier::vanilla_static("white_bed"), Some(Block::WHITE_BED), 1);
    pub const ORANGE_BED: Self = Self::new(1087, Identifier::vanilla_static("orange_bed"), Some(Block::ORANGE_BED), 1);
    pub const MAGENTA_BED: Self = Self::new(1088, Identifier::vanilla_static("magenta_bed"), Some(Block::MAGENTA_BED), 1);
    pub const LIGHT_BLUE_BED: Self = Self::new(1089, Identifier::vanilla_static("light_blue_bed"), Some(Block::LIGHT_BLUE_BED), 1);
    pub const YELLOW_BED: Self = Self::new(1090, Identifier::vanilla_static("yellow_bed"), Some(Block::YELLOW_BED), 1);
    pub const LIME_BED: Self = Self::new(1091, Identifier::vanilla_static("lime_bed"), Some(Block::LIME_BED), 1);
    pub const PINK_BED: Self = Self::new(1092, Identifier::vanilla_static("pink_bed"), Some(Block::PINK_BED), 1);
    pub const GRAY_BED: Self = Self::new(1093, Identifier::vanilla_static("gray_bed"), Some(Block::GRAY_BED), 1);
    pub const LIGHT_GRAY_BED: Self = Self::new(1094, Identifier::vanilla_static("light_gray_bed"), Some(Block::LIGHT_GRAY_BED), 1);
    pub const CYAN_BED: Self = Self::new(1095, Identifier::vanilla_static("cyan_bed"), Some(Block::CYAN_BED), 1);
    pub const PURPLE_BED: Self = Self::new(1096, Identifier::vanilla_static("purple_bed"), Some(Block::PURPLE_BED), 1);
    pub const BLUE_BED: Self = Self::new(1097, Identifier::vanilla_static("blue_bed"), Some(Block::BLUE_BED), 1);
    pub const BROWN_BED: Self = Self::new(1098, Identifier::vanilla_static("brown_bed"), Some(Block::BROWN_BED), 1);
    pub const GREEN_BED: Self = Self::new(1099, Identifier::vanilla_static("green_bed"), Some(Block::GREEN_BED), 1);
    pub const RED_BED: Self = Self::new(1100, Identifier::vanilla_static("red_bed"), Some(Block::RED_BED), 1);
    pub const BLACK_BED: Self = Self::new(1101, Identifier::vanilla_static("black_bed"), Some(Block::BLACK_BED), 1);
    pub const COOKIE: Self = Self::new(1102, Identifier::vanilla_static("cookie"), None, 64);
    pub const CRAFTER: Self = Self::new(1103, Identifier::vanilla_static("crafter"), Some(Block::CRAFTER), 64);
    pub const FILLED_MAP: Self = Self::new(1104, Identifier::vanilla_static("filled_map"), None, 64);
    pub const SHEARS: Self = Self::new(1105, Identifier::vanilla_static("shears"), None, 1);
    pub const MELON_SLICE: Self = Self::new(1106, Identifier::vanilla_static("melon_slice"), None, 64);
    pub const DRIED_KELP: Self = Self::new(1107, Identifier::vanilla_static("dried_kelp"), None, 64);
    pub const PUMPKIN_SEEDS: Self = Self::new(1108, Identifier::vanilla_static("pumpkin_seeds"), Some(Block::PUMPKIN_STEM), 64);
    pub const MELON_SEEDS: Self = Self::new(1109, Identifier::vanilla_static("melon_seeds"), Some(Block::MELON_STEM), 64);
    pub const BEEF: Self = Self::new(1110, Identifier::vanilla_static("beef"), None, 64);
    pub const COOKED_BEEF: Self = Self::new(1111, Identifier::vanilla_static("cooked_beef"), None, 64);
    pub const CHICKEN: Self = Self::new(1112, Identifier::vanilla_static("chicken"), None, 64);
    pub const COOKED_CHICKEN: Self = Self::new(1113, Identifier::vanilla_static("cooked_chicken"), None, 64);
    pub const ROTTEN_FLESH: Self = Self::new(1114, Identifier::vanilla_static("rotten_flesh"), None, 64);
    pub const ENDER_PEARL: Self = Self::new(1115, Identifier::vanilla_static("ender_pearl"), None, 16);
    pub const BLAZE_ROD: Self = Self::new(1116, Identifier::vanilla_static("blaze_rod"), None, 64);
    pub const GHAST_TEAR: Self = Self::new(1117, Identifier::vanilla_static("ghast_tear"), None, 64);
    pub const GOLD_NUGGET: Self = Self::new(1118, Identifier::vanilla_static("gold_nugget"), None, 64);
    pub const NETHER_WART: Self = Self::new(1119, Identifier::vanilla_static("nether_wart"), Some(Block::NETHER_WART), 64);
    pub const GLASS_BOTTLE: Self = Self::new(1120, Identifier::vanilla_static("glass_bottle"), None, 64);
    pub const POTION: Self = Self::new(1121, Identifier::vanilla_static("potion"), None, 1);
    pub const SPIDER_EYE: Self = Self::new(1122, Identifier::vanilla_static("spider_eye"), None, 64);
    pub const FERMENTED_SPIDER_EYE: Self = Self::new(1123, Identifier::vanilla_static("fermented_spider_eye"), None, 64);
    pub const BLAZE_POWDER: Self = Self::new(1124, Identifier::vanilla_static("blaze_powder"), None, 64);
    pub const MAGMA_CREAM: Self = Self::new(1125, Identifier::vanilla_static("magma_cream"), None, 64);
    pub const BREWING_STAND: Self = Self::new(1126, Identifier::vanilla_static("brewing_stand"), Some(Block::BREWING_STAND), 64);
    pub const CAULDRON: Self = Self::new(1127, Identifier::vanilla_static("cauldron"), Some(Block::CAULDRON), 64);
    pub const ENDER_EYE: Self = Self::new(1128, Identifier::vanilla_static("ender_eye"), None, 64);
    pub const GLISTERING_MELON_SLICE: Self = Self::new(1129, Identifier::vanilla_static("glistering_melon_slice"), None, 64);
    pub const CHICKEN_SPAWN_EGG: Self = Self::new(1130, Identifier::vanilla_static("chicken_spawn_egg"), None, 64);
    pub const COW_SPAWN_EGG: Self = Self::new(1131, Identifier::vanilla_static("cow_spawn_egg"), None, 64);
    pub const PIG_SPAWN_EGG: Self = Self::new(1132, Identifier::vanilla_static("pig_spawn_egg"), None, 64);
    pub const SHEEP_SPAWN_EGG: Self = Self::new(1133, Identifier::vanilla_static("sheep_spawn_egg"), None, 64);
    pub const CAMEL_SPAWN_EGG: Self = Self::new(1134, Identifier::vanilla_static("camel_spawn_egg"), None, 64);
    pub const DONKEY_SPAWN_EGG: Self = Self::new(1135, Identifier::vanilla_static("donkey_spawn_egg"), None, 64);
    pub const HORSE_SPAWN_EGG: Self = Self::new(1136, Identifier::vanilla_static("horse_spawn_egg"), None, 64);
    pub const MULE_SPAWN_EGG: Self = Self::new(1137, Identifier::vanilla_static("mule_spawn_egg"), None, 64);
    pub const CAT_SPAWN_EGG: Self = Self::new(1138, Identifier::vanilla_static("cat_spawn_egg"), None, 64);
    pub const PARROT_SPAWN_EGG: Self = Self::new(1139, Identifier::vanilla_static("parrot_spawn_egg"), None, 64);
    pub const WOLF_SPAWN_EGG: Self = Self::new(1140, Identifier::vanilla_static("wolf_spawn_egg"), None, 64);
    pub const ARMADILLO_SPAWN_EGG: Self = Self::new(1141, Identifier::vanilla_static("armadillo_spawn_egg"), None, 64);
    pub const BAT_SPAWN_EGG: Self = Self::new(1142, Identifier::vanilla_static("bat_spawn_egg"), None, 64);
    pub const BEE_SPAWN_EGG: Self = Self::new(1143, Identifier::vanilla_static("bee_spawn_egg"), None, 64);
    pub const FOX_SPAWN_EGG: Self = Self::new(1144, Identifier::vanilla_static("fox_spawn_egg"), None, 64);
    pub const GOAT_SPAWN_EGG: Self = Self::new(1145, Identifier::vanilla_static("goat_spawn_egg"), None, 64);
    pub const LLAMA_SPAWN_EGG: Self = Self::new(1146, Identifier::vanilla_static("llama_spawn_egg"), None, 64);
    pub const OCELOT_SPAWN_EGG: Self = Self::new(1147, Identifier::vanilla_static("ocelot_spawn_egg"), None, 64);
    pub const PANDA_SPAWN_EGG: Self = Self::new(1148, Identifier::vanilla_static("panda_spawn_egg"), None, 64);
    pub const POLAR_BEAR_SPAWN_EGG: Self = Self::new(1149, Identifier::vanilla_static("polar_bear_spawn_egg"), None, 64);
    pub const RABBIT_SPAWN_EGG: Self = Self::new(1150, Identifier::vanilla_static("rabbit_spawn_egg"), None, 64);
    pub const AXOLOTL_SPAWN_EGG: Self = Self::new(1151, Identifier::vanilla_static("axolotl_spawn_egg"), None, 64);
    pub const COD_SPAWN_EGG: Self = Self::new(1152, Identifier::vanilla_static("cod_spawn_egg"), None, 64);
    pub const DOLPHIN_SPAWN_EGG: Self = Self::new(1153, Identifier::vanilla_static("dolphin_spawn_egg"), None, 64);
    pub const FROG_SPAWN_EGG: Self = Self::new(1154, Identifier::vanilla_static("frog_spawn_egg"), None, 64);
    pub const GLOW_SQUID_SPAWN_EGG: Self = Self::new(1155, Identifier::vanilla_static("glow_squid_spawn_egg"), None, 64);
    pub const NAUTILUS_SPAWN_EGG: Self = Self::new(1156, Identifier::vanilla_static("nautilus_spawn_egg"), None, 64);
    pub const PUFFERFISH_SPAWN_EGG: Self = Self::new(1157, Identifier::vanilla_static("pufferfish_spawn_egg"), None, 64);
    pub const SALMON_SPAWN_EGG: Self = Self::new(1158, Identifier::vanilla_static("salmon_spawn_egg"), None, 64);
    pub const SQUID_SPAWN_EGG: Self = Self::new(1159, Identifier::vanilla_static("squid_spawn_egg"), None, 64);
    pub const TADPOLE_SPAWN_EGG: Self = Self::new(1160, Identifier::vanilla_static("tadpole_spawn_egg"), None, 64);
    pub const TROPICAL_FISH_SPAWN_EGG: Self = Self::new(1161, Identifier::vanilla_static("tropical_fish_spawn_egg"), None, 64);
    pub const TURTLE_SPAWN_EGG: Self = Self::new(1162, Identifier::vanilla_static("turtle_spawn_egg"), None, 64);
    pub const ALLAY_SPAWN_EGG: Self = Self::new(1163, Identifier::vanilla_static("allay_spawn_egg"), None, 64);
    pub const MOOSHROOM_SPAWN_EGG: Self = Self::new(1164, Identifier::vanilla_static("mooshroom_spawn_egg"), None, 64);
    pub const SNIFFER_SPAWN_EGG: Self = Self::new(1165, Identifier::vanilla_static("sniffer_spawn_egg"), None, 64);
    pub const COPPER_GOLEM_SPAWN_EGG: Self = Self::new(1166, Identifier::vanilla_static("copper_golem_spawn_egg"), None, 64);
    pub const IRON_GOLEM_SPAWN_EGG: Self = Self::new(1167, Identifier::vanilla_static("iron_golem_spawn_egg"), None, 64);
    pub const SNOW_GOLEM_SPAWN_EGG: Self = Self::new(1168, Identifier::vanilla_static("snow_golem_spawn_egg"), None, 64);
    pub const TRADER_LLAMA_SPAWN_EGG: Self = Self::new(1169, Identifier::vanilla_static("trader_llama_spawn_egg"), None, 64);
    pub const VILLAGER_SPAWN_EGG: Self = Self::new(1170, Identifier::vanilla_static("villager_spawn_egg"), None, 64);
    pub const WANDERING_TRADER_SPAWN_EGG: Self = Self::new(1171, Identifier::vanilla_static("wandering_trader_spawn_egg"), None, 64);
    pub const BOGGED_SPAWN_EGG: Self = Self::new(1172, Identifier::vanilla_static("bogged_spawn_egg"), None, 64);
    pub const CAMEL_HUSK_SPAWN_EGG: Self = Self::new(1173, Identifier::vanilla_static("camel_husk_spawn_egg"), None, 64);
    pub const DROWNED_SPAWN_EGG: Self = Self::new(1174, Identifier::vanilla_static("drowned_spawn_egg"), None, 64);
    pub const HUSK_SPAWN_EGG: Self = Self::new(1175, Identifier::vanilla_static("husk_spawn_egg"), None, 64);
    pub const PARCHED_SPAWN_EGG: Self = Self::new(1176, Identifier::vanilla_static("parched_spawn_egg"), None, 64);
    pub const SKELETON_SPAWN_EGG: Self = Self::new(1177, Identifier::vanilla_static("skeleton_spawn_egg"), None, 64);
    pub const SKELETON_HORSE_SPAWN_EGG: Self = Self::new(1178, Identifier::vanilla_static("skeleton_horse_spawn_egg"), None, 64);
    pub const STRAY_SPAWN_EGG: Self = Self::new(1179, Identifier::vanilla_static("stray_spawn_egg"), None, 64);
    pub const WITHER_SPAWN_EGG: Self = Self::new(1180, Identifier::vanilla_static("wither_spawn_egg"), None, 64);
    pub const WITHER_SKELETON_SPAWN_EGG: Self = Self::new(1181, Identifier::vanilla_static("wither_skeleton_spawn_egg"), None, 64);
    pub const ZOMBIE_SPAWN_EGG: Self = Self::new(1182, Identifier::vanilla_static("zombie_spawn_egg"), None, 64);
    pub const ZOMBIE_HORSE_SPAWN_EGG: Self = Self::new(1183, Identifier::vanilla_static("zombie_horse_spawn_egg"), None, 64);
    pub const ZOMBIE_NAUTILUS_SPAWN_EGG: Self = Self::new(1184, Identifier::vanilla_static("zombie_nautilus_spawn_egg"), None, 64);
    pub const ZOMBIE_VILLAGER_SPAWN_EGG: Self = Self::new(1185, Identifier::vanilla_static("zombie_villager_spawn_egg"), None, 64);
    pub const CAVE_SPIDER_SPAWN_EGG: Self = Self::new(1186, Identifier::vanilla_static("cave_spider_spawn_egg"), None, 64);
    pub const SPIDER_SPAWN_EGG: Self = Self::new(1187, Identifier::vanilla_static("spider_spawn_egg"), None, 64);
    pub const BREEZE_SPAWN_EGG: Self = Self::new(1188, Identifier::vanilla_static("breeze_spawn_egg"), None, 64);
    pub const CREAKING_SPAWN_EGG: Self = Self::new(1189, Identifier::vanilla_static("creaking_spawn_egg"), None, 64);
    pub const CREEPER_SPAWN_EGG: Self = Self::new(1190, Identifier::vanilla_static("creeper_spawn_egg"), None, 64);
    pub const ELDER_GUARDIAN_SPAWN_EGG: Self = Self::new(1191, Identifier::vanilla_static("elder_guardian_spawn_egg"), None, 64);
    pub const GUARDIAN_SPAWN_EGG: Self = Self::new(1192, Identifier::vanilla_static("guardian_spawn_egg"), None, 64);
    pub const PHANTOM_SPAWN_EGG: Self = Self::new(1193, Identifier::vanilla_static("phantom_spawn_egg"), None, 64);
    pub const SILVERFISH_SPAWN_EGG: Self = Self::new(1194, Identifier::vanilla_static("silverfish_spawn_egg"), None, 64);
    pub const SLIME_SPAWN_EGG: Self = Self::new(1195, Identifier::vanilla_static("slime_spawn_egg"), None, 64);
    pub const WARDEN_SPAWN_EGG: Self = Self::new(1196, Identifier::vanilla_static("warden_spawn_egg"), None, 64);
    pub const WITCH_SPAWN_EGG: Self = Self::new(1197, Identifier::vanilla_static("witch_spawn_egg"), None, 64);
    pub const EVOKER_SPAWN_EGG: Self = Self::new(1198, Identifier::vanilla_static("evoker_spawn_egg"), None, 64);
    pub const PILLAGER_SPAWN_EGG: Self = Self::new(1199, Identifier::vanilla_static("pillager_spawn_egg"), None, 64);
    pub const RAVAGER_SPAWN_EGG: Self = Self::new(1200, Identifier::vanilla_static("ravager_spawn_egg"), None, 64);
    pub const VINDICATOR_SPAWN_EGG: Self = Self::new(1201, Identifier::vanilla_static("vindicator_spawn_egg"), None, 64);
    pub const VEX_SPAWN_EGG: Self = Self::new(1202, Identifier::vanilla_static("vex_spawn_egg"), None, 64);
    pub const BLAZE_SPAWN_EGG: Self = Self::new(1203, Identifier::vanilla_static("blaze_spawn_egg"), None, 64);
    pub const GHAST_SPAWN_EGG: Self = Self::new(1204, Identifier::vanilla_static("ghast_spawn_egg"), None, 64);
    pub const HAPPY_GHAST_SPAWN_EGG: Self = Self::new(1205, Identifier::vanilla_static("happy_ghast_spawn_egg"), None, 64);
    pub const HOGLIN_SPAWN_EGG: Self = Self::new(1206, Identifier::vanilla_static("hoglin_spawn_egg"), None, 64);
    pub const MAGMA_CUBE_SPAWN_EGG: Self = Self::new(1207, Identifier::vanilla_static("magma_cube_spawn_egg"), None, 64);
    pub const PIGLIN_SPAWN_EGG: Self = Self::new(1208, Identifier::vanilla_static("piglin_spawn_egg"), None, 64);
    pub const PIGLIN_BRUTE_SPAWN_EGG: Self = Self::new(1209, Identifier::vanilla_static("piglin_brute_spawn_egg"), None, 64);
    pub const STRIDER_SPAWN_EGG: Self = Self::new(1210, Identifier::vanilla_static("strider_spawn_egg"), None, 64);
    pub const ZOGLIN_SPAWN_EGG: Self = Self::new(1211, Identifier::vanilla_static("zoglin_spawn_egg"), None, 64);
    pub const ZOMBIFIED_PIGLIN_SPAWN_EGG: Self = Self::new(1212, Identifier::vanilla_static("zombified_piglin_spawn_egg"), None, 64);
    pub const ENDER_DRAGON_SPAWN_EGG: Self = Self::new(1213, Identifier::vanilla_static("ender_dragon_spawn_egg"), None, 64);
    pub const ENDERMAN_SPAWN_EGG: Self = Self::new(1214, Identifier::vanilla_static("enderman_spawn_egg"), None, 64);
    pub const ENDERMITE_SPAWN_EGG: Self = Self::new(1215, Identifier::vanilla_static("endermite_spawn_egg"), None, 64);
    pub const SHULKER_SPAWN_EGG: Self = Self::new(1216, Identifier::vanilla_static("shulker_spawn_egg"), None, 64);
    pub const EXPERIENCE_BOTTLE: Self = Self::new(1217, Identifier::vanilla_static("experience_bottle"), None, 64);
    pub const FIRE_CHARGE: Self = Self::new(1218, Identifier::vanilla_static("fire_charge"), None, 64);
    pub const WIND_CHARGE: Self = Self::new(1219, Identifier::vanilla_static("wind_charge"), None, 64);
    pub const WRITABLE_BOOK: Self = Self::new(1220, Identifier::vanilla_static("writable_book"), None, 1);
    pub const WRITTEN_BOOK: Self = Self::new(1221, Identifier::vanilla_static("written_book"), None, 16);
    pub const BREEZE_ROD: Self = Self::new(1222, Identifier::vanilla_static("breeze_rod"), None, 64);
    pub const MACE: Self = Self::new(1223, Identifier::vanilla_static("mace"), None, 1);
    pub const ITEM_FRAME: Self = Self::new(1224, Identifier::vanilla_static("item_frame"), None, 64);
    pub const GLOW_ITEM_FRAME: Self = Self::new(1225, Identifier::vanilla_static("glow_item_frame"), None, 64);
    pub const FLOWER_POT: Self = Self::new(1226, Identifier::vanilla_static("flower_pot"), Some(Block::FLOWER_POT), 64);
    pub const CARROT: Self = Self::new(1227, Identifier::vanilla_static("carrot"), Some(Block::CARROTS), 64);
    pub const POTATO: Self = Self::new(1228, Identifier::vanilla_static("potato"), Some(Block::POTATOES), 64);
    pub const BAKED_POTATO: Self = Self::new(1229, Identifier::vanilla_static("baked_potato"), None, 64);
    pub const POISONOUS_POTATO: Self = Self::new(1230, Identifier::vanilla_static("poisonous_potato"), None, 64);
    pub const MAP: Self = Self::new(1231, Identifier::vanilla_static("map"), None, 64);
    pub const GOLDEN_CARROT: Self = Self::new(1232, Identifier::vanilla_static("golden_carrot"), None, 64);
    pub const SKELETON_SKULL: Self = Self::new(1233, Identifier::vanilla_static("skeleton_skull"), Some(Block::SKELETON_SKULL), 64);
    pub const WITHER_SKELETON_SKULL: Self = Self::new(1234, Identifier::vanilla_static("wither_skeleton_skull"), Some(Block::WITHER_SKELETON_SKULL), 64);
    pub const PLAYER_HEAD: Self = Self::new(1235, Identifier::vanilla_static("player_head"), Some(Block::PLAYER_HEAD), 64);
    pub const ZOMBIE_HEAD: Self = Self::new(1236, Identifier::vanilla_static("zombie_head"), Some(Block::ZOMBIE_HEAD), 64);
    pub const CREEPER_HEAD: Self = Self::new(1237, Identifier::vanilla_static("creeper_head"), Some(Block::CREEPER_HEAD), 64);
    pub const DRAGON_HEAD: Self = Self::new(1238, Identifier::vanilla_static("dragon_head"), Some(Block::DRAGON_HEAD), 64);
    pub const PIGLIN_HEAD: Self = Self::new(1239, Identifier::vanilla_static("piglin_head"), Some(Block::PIGLIN_HEAD), 64);
    pub const NETHER_STAR: Self = Self::new(1240, Identifier::vanilla_static("nether_star"), None, 64);
    pub const PUMPKIN_PIE: Self = Self::new(1241, Identifier::vanilla_static("pumpkin_pie"), None, 64);
    pub const FIREWORK_ROCKET: Self = Self::new(1242, Identifier::vanilla_static("firework_rocket"), None, 64);
    pub const FIREWORK_STAR: Self = Self::new(1243, Identifier::vanilla_static("firework_star"), None, 64);
    pub const ENCHANTED_BOOK: Self = Self::new(1244, Identifier::vanilla_static("enchanted_book"), None, 1);
    pub const NETHER_BRICK: Self = Self::new(1245, Identifier::vanilla_static("nether_brick"), None, 64);
    pub const RESIN_BRICK: Self = Self::new(1246, Identifier::vanilla_static("resin_brick"), None, 64);
    pub const PRISMARINE_SHARD: Self = Self::new(1247, Identifier::vanilla_static("prismarine_shard"), None, 64);
    pub const PRISMARINE_CRYSTALS: Self = Self::new(1248, Identifier::vanilla_static("prismarine_crystals"), None, 64);
    pub const RABBIT: Self = Self::new(1249, Identifier::vanilla_static("rabbit"), None, 64);
    pub const COOKED_RABBIT: Self = Self::new(1250, Identifier::vanilla_static("cooked_rabbit"), None, 64);
    pub const RABBIT_STEW: Self = Self::new(1251, Identifier::vanilla_static("rabbit_stew"), None, 1);
    pub const RABBIT_FOOT: Self = Self::new(1252, Identifier::vanilla_static("rabbit_foot"), None, 64);
    pub const RABBIT_HIDE: Self = Self::new(1253, Identifier::vanilla_static("rabbit_hide"), None, 64);
    pub const ARMOR_STAND: Self = Self::new(1254, Identifier::vanilla_static("armor_stand"), None, 16);
    pub const COPPER_HORSE_ARMOR: Self = Self::new(1255, Identifier::vanilla_static("copper_horse_armor"), None, 1);
    pub const IRON_HORSE_ARMOR: Self = Self::new(1256, Identifier::vanilla_static("iron_horse_armor"), None, 1);
    pub const GOLDEN_HORSE_ARMOR: Self = Self::new(1257, Identifier::vanilla_static("golden_horse_armor"), None, 1);
    pub const DIAMOND_HORSE_ARMOR: Self = Self::new(1258, Identifier::vanilla_static("diamond_horse_armor"), None, 1);
    pub const NETHERITE_HORSE_ARMOR: Self = Self::new(1259, Identifier::vanilla_static("netherite_horse_armor"), None, 1);
    pub const LEATHER_HORSE_ARMOR: Self = Self::new(1260, Identifier::vanilla_static("leather_horse_armor"), None, 1);
    pub const LEAD: Self = Self::new(1261, Identifier::vanilla_static("lead"), None, 64);
    pub const NAME_TAG: Self = Self::new(1262, Identifier::vanilla_static("name_tag"), None, 64);
    pub const COMMAND_BLOCK_MINECART: Self = Self::new(1263, Identifier::vanilla_static("command_block_minecart"), None, 1);
    pub const MUTTON: Self = Self::new(1264, Identifier::vanilla_static("mutton"), None, 64);
    pub const COOKED_MUTTON: Self = Self::new(1265, Identifier::vanilla_static("cooked_mutton"), None, 64);
    pub const WHITE_BANNER: Self = Self::new(1266, Identifier::vanilla_static("white_banner"), Some(Block::WHITE_BANNER), 16);
    pub const ORANGE_BANNER: Self = Self::new(1267, Identifier::vanilla_static("orange_banner"), Some(Block::ORANGE_BANNER), 16);
    pub const MAGENTA_BANNER: Self = Self::new(1268, Identifier::vanilla_static("magenta_banner"), Some(Block::MAGENTA_BANNER), 16);
    pub const LIGHT_BLUE_BANNER: Self = Self::new(1269, Identifier::vanilla_static("light_blue_banner"), Some(Block::LIGHT_BLUE_BANNER), 16);
    pub const YELLOW_BANNER: Self = Self::new(1270, Identifier::vanilla_static("yellow_banner"), Some(Block::YELLOW_BANNER), 16);
    pub const LIME_BANNER: Self = Self::new(1271, Identifier::vanilla_static("lime_banner"), Some(Block::LIME_BANNER), 16);
    pub const PINK_BANNER: Self = Self::new(1272, Identifier::vanilla_static("pink_banner"), Some(Block::PINK_BANNER), 16);
    pub const GRAY_BANNER: Self = Self::new(1273, Identifier::vanilla_static("gray_banner"), Some(Block::GRAY_BANNER), 16);
    pub const LIGHT_GRAY_BANNER: Self = Self::new(1274, Identifier::vanilla_static("light_gray_banner"), Some(Block::LIGHT_GRAY_BANNER), 16);
    pub const CYAN_BANNER: Self = Self::new(1275, Identifier::vanilla_static("cyan_banner"), Some(Block::CYAN_BANNER), 16);
    pub const PURPLE_BANNER: Self = Self::new(1276, Identifier::vanilla_static("purple_banner"), Some(Block::PURPLE_BANNER), 16);
    pub const BLUE_BANNER: Self = Self::new(1277, Identifier::vanilla_static("blue_banner"), Some(Block::BLUE_BANNER), 16);
    pub const BROWN_BANNER: Self = Self::new(1278, Identifier::vanilla_static("brown_banner"), Some(Block::BROWN_BANNER), 16);
    pub const GREEN_BANNER: Self = Self::new(1279, Identifier::vanilla_static("green_banner"), Some(Block::GREEN_BANNER), 16);
    pub const RED_BANNER: Self = Self::new(1280, Identifier::vanilla_static("red_banner"), Some(Block::RED_BANNER), 16);
    pub const BLACK_BANNER: Self = Self::new(1281, Identifier::vanilla_static("black_banner"), Some(Block::BLACK_BANNER), 16);
    pub const END_CRYSTAL: Self = Self::new(1282, Identifier::vanilla_static("end_crystal"), None, 64);
    pub const CHORUS_FRUIT: Self = Self::new(1283, Identifier::vanilla_static("chorus_fruit"), None, 64);
    pub const POPPED_CHORUS_FRUIT: Self = Self::new(1284, Identifier::vanilla_static("popped_chorus_fruit"), None, 64);
    pub const TORCHFLOWER_SEEDS: Self = Self::new(1285, Identifier::vanilla_static("torchflower_seeds"), Some(Block::TORCHFLOWER_CROP), 64);
    pub const PITCHER_POD: Self = Self::new(1286, Identifier::vanilla_static("pitcher_pod"), Some(Block::PITCHER_CROP), 64);
    pub const BEETROOT: Self = Self::new(1287, Identifier::vanilla_static("beetroot"), None, 64);
    pub const BEETROOT_SEEDS: Self = Self::new(1288, Identifier::vanilla_static("beetroot_seeds"), Some(Block::BEETROOTS), 64);
    pub const BEETROOT_SOUP: Self = Self::new(1289, Identifier::vanilla_static("beetroot_soup"), None, 1);
    pub const DRAGON_BREATH: Self = Self::new(1290, Identifier::vanilla_static("dragon_breath"), None, 64);
    pub const SPLASH_POTION: Self = Self::new(1291, Identifier::vanilla_static("splash_potion"), None, 1);
    pub const SPECTRAL_ARROW: Self = Self::new(1292, Identifier::vanilla_static("spectral_arrow"), None, 64);
    pub const TIPPED_ARROW: Self = Self::new(1293, Identifier::vanilla_static("tipped_arrow"), None, 64);
    pub const LINGERING_POTION: Self = Self::new(1294, Identifier::vanilla_static("lingering_potion"), None, 1);
    pub const SHIELD: Self = Self::new(1295, Identifier::vanilla_static("shield"), None, 1);
    pub const WOODEN_SPEAR: Self = Self::new(1296, Identifier::vanilla_static("wooden_spear"), None, 1);
    pub const STONE_SPEAR: Self = Self::new(1297, Identifier::vanilla_static("stone_spear"), None, 1);
    pub const COPPER_SPEAR: Self = Self::new(1298, Identifier::vanilla_static("copper_spear"), None, 1);
    pub const IRON_SPEAR: Self = Self::new(1299, Identifier::vanilla_static("iron_spear"), None, 1);
    pub const GOLDEN_SPEAR: Self = Self::new(1300, Identifier::vanilla_static("golden_spear"), None, 1);
    pub const DIAMOND_SPEAR: Self = Self::new(1301, Identifier::vanilla_static("diamond_spear"), None, 1);
    pub const NETHERITE_SPEAR: Self = Self::new(1302, Identifier::vanilla_static("netherite_spear"), None, 1);
    pub const TOTEM_OF_UNDYING: Self = Self::new(1303, Identifier::vanilla_static("totem_of_undying"), None, 1);
    pub const SHULKER_SHELL: Self = Self::new(1304, Identifier::vanilla_static("shulker_shell"), None, 64);
    pub const IRON_NUGGET: Self = Self::new(1305, Identifier::vanilla_static("iron_nugget"), None, 64);
    pub const COPPER_NUGGET: Self = Self::new(1306, Identifier::vanilla_static("copper_nugget"), None, 64);
    pub const KNOWLEDGE_BOOK: Self = Self::new(1307, Identifier::vanilla_static("knowledge_book"), None, 1);
    pub const DEBUG_STICK: Self = Self::new(1308, Identifier::vanilla_static("debug_stick"), None, 1);
    pub const MUSIC_DISC_13: Self = Self::new(1309, Identifier::vanilla_static("music_disc_13"), None, 1);
    pub const MUSIC_DISC_CAT: Self = Self::new(1310, Identifier::vanilla_static("music_disc_cat"), None, 1);
    pub const MUSIC_DISC_BLOCKS: Self = Self::new(1311, Identifier::vanilla_static("music_disc_blocks"), None, 1);
    pub const MUSIC_DISC_CHIRP: Self = Self::new(1312, Identifier::vanilla_static("music_disc_chirp"), None, 1);
    pub const MUSIC_DISC_CREATOR: Self = Self::new(1313, Identifier::vanilla_static("music_disc_creator"), None, 1);
    pub const MUSIC_DISC_CREATOR_MUSIC_BOX: Self = Self::new(1314, Identifier::vanilla_static("music_disc_creator_music_box"), None, 1);
    pub const MUSIC_DISC_FAR: Self = Self::new(1315, Identifier::vanilla_static("music_disc_far"), None, 1);
    pub const MUSIC_DISC_LAVA_CHICKEN: Self = Self::new(1316, Identifier::vanilla_static("music_disc_lava_chicken"), None, 1);
    pub const MUSIC_DISC_MALL: Self = Self::new(1317, Identifier::vanilla_static("music_disc_mall"), None, 1);
    pub const MUSIC_DISC_MELLOHI: Self = Self::new(1318, Identifier::vanilla_static("music_disc_mellohi"), None, 1);
    pub const MUSIC_DISC_STAL: Self = Self::new(1319, Identifier::vanilla_static("music_disc_stal"), None, 1);
    pub const MUSIC_DISC_STRAD: Self = Self::new(1320, Identifier::vanilla_static("music_disc_strad"), None, 1);
    pub const MUSIC_DISC_WARD: Self = Self::new(1321, Identifier::vanilla_static("music_disc_ward"), None, 1);
    pub const MUSIC_DISC_11: Self = Self::new(1322, Identifier::vanilla_static("music_disc_11"), None, 1);
    pub const MUSIC_DISC_WAIT: Self = Self::new(1323, Identifier::vanilla_static("music_disc_wait"), None, 1);
    pub const MUSIC_DISC_OTHERSIDE: Self = Self::new(1324, Identifier::vanilla_static("music_disc_otherside"), None, 1);
    pub const MUSIC_DISC_RELIC: Self = Self::new(1325, Identifier::vanilla_static("music_disc_relic"), None, 1);
    pub const MUSIC_DISC_5: Self = Self::new(1326, Identifier::vanilla_static("music_disc_5"), None, 1);
    pub const MUSIC_DISC_PIGSTEP: Self = Self::new(1327, Identifier::vanilla_static("music_disc_pigstep"), None, 1);
    pub const MUSIC_DISC_PRECIPICE: Self = Self::new(1328, Identifier::vanilla_static("music_disc_precipice"), None, 1);
    pub const MUSIC_DISC_TEARS: Self = Self::new(1329, Identifier::vanilla_static("music_disc_tears"), None, 1);
    pub const DISC_FRAGMENT_5: Self = Self::new(1330, Identifier::vanilla_static("disc_fragment_5"), None, 64);
    pub const TRIDENT: Self = Self::new(1331, Identifier::vanilla_static("trident"), None, 1);
    pub const NAUTILUS_SHELL: Self = Self::new(1332, Identifier::vanilla_static("nautilus_shell"), None, 64);
    pub const IRON_NAUTILUS_ARMOR: Self = Self::new(1333, Identifier::vanilla_static("iron_nautilus_armor"), None, 1);
    pub const GOLDEN_NAUTILUS_ARMOR: Self = Self::new(1334, Identifier::vanilla_static("golden_nautilus_armor"), None, 1);
    pub const DIAMOND_NAUTILUS_ARMOR: Self = Self::new(1335, Identifier::vanilla_static("diamond_nautilus_armor"), None, 1);
    pub const NETHERITE_NAUTILUS_ARMOR: Self = Self::new(1336, Identifier::vanilla_static("netherite_nautilus_armor"), None, 1);
    pub const COPPER_NAUTILUS_ARMOR: Self = Self::new(1337, Identifier::vanilla_static("copper_nautilus_armor"), None, 1);
    pub const HEART_OF_THE_SEA: Self = Self::new(1338, Identifier::vanilla_static("heart_of_the_sea"), None, 64);
    pub const CROSSBOW: Self = Self::new(1339, Identifier::vanilla_static("crossbow"), None, 1);
    pub const SUSPICIOUS_STEW: Self = Self::new(1340, Identifier::vanilla_static("suspicious_stew"), None, 1);
    pub const LOOM: Self = Self::new(1341, Identifier::vanilla_static("loom"), Some(Block::LOOM), 64);
    pub const FLOWER_BANNER_PATTERN: Self = Self::new(1342, Identifier::vanilla_static("flower_banner_pattern"), None, 1);
    pub const CREEPER_BANNER_PATTERN: Self = Self::new(1343, Identifier::vanilla_static("creeper_banner_pattern"), None, 1);
    pub const SKULL_BANNER_PATTERN: Self = Self::new(1344, Identifier::vanilla_static("skull_banner_pattern"), None, 1);
    pub const MOJANG_BANNER_PATTERN: Self = Self::new(1345, Identifier::vanilla_static("mojang_banner_pattern"), None, 1);
    pub const GLOBE_BANNER_PATTERN: Self = Self::new(1346, Identifier::vanilla_static("globe_banner_pattern"), None, 1);
    pub const PIGLIN_BANNER_PATTERN: Self = Self::new(1347, Identifier::vanilla_static("piglin_banner_pattern"), None, 1);
    pub const FLOW_BANNER_PATTERN: Self = Self::new(1348, Identifier::vanilla_static("flow_banner_pattern"), None, 1);
    pub const GUSTER_BANNER_PATTERN: Self = Self::new(1349, Identifier::vanilla_static("guster_banner_pattern"), None, 1);
    pub const FIELD_MASONED_BANNER_PATTERN: Self = Self::new(1350, Identifier::vanilla_static("field_masoned_banner_pattern"), None, 1);
    pub const BORDURE_INDENTED_BANNER_PATTERN: Self = Self::new(1351, Identifier::vanilla_static("bordure_indented_banner_pattern"), None, 1);
    pub const GOAT_HORN: Self = Self::new(1352, Identifier::vanilla_static("goat_horn"), None, 1);
    pub const COMPOSTER: Self = Self::new(1353, Identifier::vanilla_static("composter"), Some(Block::COMPOSTER), 64);
    pub const BARREL: Self = Self::new(1354, Identifier::vanilla_static("barrel"), Some(Block::BARREL), 64);
    pub const SMOKER: Self = Self::new(1355, Identifier::vanilla_static("smoker"), Some(Block::SMOKER), 64);
    pub const BLAST_FURNACE: Self = Self::new(1356, Identifier::vanilla_static("blast_furnace"), Some(Block::BLAST_FURNACE), 64);
    pub const CARTOGRAPHY_TABLE: Self = Self::new(1357, Identifier::vanilla_static("cartography_table"), Some(Block::CARTOGRAPHY_TABLE), 64);
    pub const FLETCHING_TABLE: Self = Self::new(1358, Identifier::vanilla_static("fletching_table"), Some(Block::FLETCHING_TABLE), 64);
    pub const GRINDSTONE: Self = Self::new(1359, Identifier::vanilla_static("grindstone"), Some(Block::GRINDSTONE), 64);
    pub const SMITHING_TABLE: Self = Self::new(1360, Identifier::vanilla_static("smithing_table"), Some(Block::SMITHING_TABLE), 64);
    pub const STONECUTTER: Self = Self::new(1361, Identifier::vanilla_static("stonecutter"), Some(Block::STONECUTTER), 64);
    pub const BELL: Self = Self::new(1362, Identifier::vanilla_static("bell"), Some(Block::BELL), 64);
    pub const LANTERN: Self = Self::new(1363, Identifier::vanilla_static("lantern"), Some(Block::LANTERN), 64);
    pub const SOUL_LANTERN: Self = Self::new(1364, Identifier::vanilla_static("soul_lantern"), Some(Block::SOUL_LANTERN), 64);
    pub const COPPER_LANTERN: Self = Self::new(1365, Identifier::vanilla_static("copper_lantern"), Some(Block::COPPER_LANTERN), 64);
    pub const EXPOSED_COPPER_LANTERN: Self = Self::new(1366, Identifier::vanilla_static("exposed_copper_lantern"), Some(Block::EXPOSED_COPPER_LANTERN), 64);
    pub const WEATHERED_COPPER_LANTERN: Self = Self::new(1367, Identifier::vanilla_static("weathered_copper_lantern"), Some(Block::WEATHERED_COPPER_LANTERN), 64);
    pub const OXIDIZED_COPPER_LANTERN: Self = Self::new(1368, Identifier::vanilla_static("oxidized_copper_lantern"), Some(Block::OXIDIZED_COPPER_LANTERN), 64);
    pub const WAXED_COPPER_LANTERN: Self = Self::new(1369, Identifier::vanilla_static("waxed_copper_lantern"), Some(Block::WAXED_COPPER_LANTERN), 64);
    pub const WAXED_EXPOSED_COPPER_LANTERN: Self = Self::new(1370, Identifier::vanilla_static("waxed_exposed_copper_lantern"), Some(Block::WAXED_EXPOSED_COPPER_LANTERN), 64);
    pub const WAXED_WEATHERED_COPPER_LANTERN: Self = Self::new(1371, Identifier::vanilla_static("waxed_weathered_copper_lantern"), Some(Block::WAXED_WEATHERED_COPPER_LANTERN), 64);
    pub const WAXED_OXIDIZED_COPPER_LANTERN: Self = Self::new(1372, Identifier::vanilla_static("waxed_oxidized_copper_lantern"), Some(Block::WAXED_OXIDIZED_COPPER_LANTERN), 64);
    pub const SWEET_BERRIES: Self = Self::new(1373, Identifier::vanilla_static("sweet_berries"), Some(Block::SWEET_BERRY_BUSH), 64);
    pub const GLOW_BERRIES: Self = Self::new(1374, Identifier::vanilla_static("glow_berries"), Some(Block::CAVE_VINES), 64);
    pub const CAMPFIRE: Self = Self::new(1375, Identifier::vanilla_static("campfire"), Some(Block::CAMPFIRE), 64);
    pub const SOUL_CAMPFIRE: Self = Self::new(1376, Identifier::vanilla_static("soul_campfire"), Some(Block::SOUL_CAMPFIRE), 64);
    pub const SHROOMLIGHT: Self = Self::new(1377, Identifier::vanilla_static("shroomlight"), Some(Block::SHROOMLIGHT), 64);
    pub const HONEYCOMB: Self = Self::new(1378, Identifier::vanilla_static("honeycomb"), None, 64);
    pub const BEE_NEST: Self = Self::new(1379, Identifier::vanilla_static("bee_nest"), Some(Block::BEE_NEST), 64);
    pub const BEEHIVE: Self = Self::new(1380, Identifier::vanilla_static("beehive"), Some(Block::BEEHIVE), 64);
    pub const HONEY_BOTTLE: Self = Self::new(1381, Identifier::vanilla_static("honey_bottle"), None, 16);
    pub const HONEYCOMB_BLOCK: Self = Self::new(1382, Identifier::vanilla_static("honeycomb_block"), Some(Block::HONEYCOMB_BLOCK), 64);
    pub const LODESTONE: Self = Self::new(1383, Identifier::vanilla_static("lodestone"), Some(Block::LODESTONE), 64);
    pub const CRYING_OBSIDIAN: Self = Self::new(1384, Identifier::vanilla_static("crying_obsidian"), Some(Block::CRYING_OBSIDIAN), 64);
    pub const BLACKSTONE: Self = Self::new(1385, Identifier::vanilla_static("blackstone"), Some(Block::BLACKSTONE), 64);
    pub const BLACKSTONE_SLAB: Self = Self::new(1386, Identifier::vanilla_static("blackstone_slab"), Some(Block::BLACKSTONE_SLAB), 64);
    pub const BLACKSTONE_STAIRS: Self = Self::new(1387, Identifier::vanilla_static("blackstone_stairs"), Some(Block::BLACKSTONE_STAIRS), 64);
    pub const GILDED_BLACKSTONE: Self = Self::new(1388, Identifier::vanilla_static("gilded_blackstone"), Some(Block::GILDED_BLACKSTONE), 64);
    pub const POLISHED_BLACKSTONE: Self = Self::new(1389, Identifier::vanilla_static("polished_blackstone"), Some(Block::POLISHED_BLACKSTONE), 64);
    pub const POLISHED_BLACKSTONE_SLAB: Self = Self::new(1390, Identifier::vanilla_static("polished_blackstone_slab"), Some(Block::POLISHED_BLACKSTONE_SLAB), 64);
    pub const POLISHED_BLACKSTONE_STAIRS: Self = Self::new(1391, Identifier::vanilla_static("polished_blackstone_stairs"), Some(Block::POLISHED_BLACKSTONE_STAIRS), 64);
    pub const CHISELED_POLISHED_BLACKSTONE: Self = Self::new(1392, Identifier::vanilla_static("chiseled_polished_blackstone"), Some(Block::CHISELED_POLISHED_BLACKSTONE), 64);
    pub const POLISHED_BLACKSTONE_BRICKS: Self = Self::new(1393, Identifier::vanilla_static("polished_blackstone_bricks"), Some(Block::POLISHED_BLACKSTONE_BRICKS), 64);
    pub const POLISHED_BLACKSTONE_BRICK_SLAB: Self = Self::new(1394, Identifier::vanilla_static("polished_blackstone_brick_slab"), Some(Block::POLISHED_BLACKSTONE_BRICK_SLAB), 64);
    pub const POLISHED_BLACKSTONE_BRICK_STAIRS: Self = Self::new(1395, Identifier::vanilla_static("polished_blackstone_brick_stairs"), Some(Block::POLISHED_BLACKSTONE_BRICK_STAIRS), 64);
    pub const CRACKED_POLISHED_BLACKSTONE_BRICKS: Self = Self::new(1396, Identifier::vanilla_static("cracked_polished_blackstone_bricks"), Some(Block::CRACKED_POLISHED_BLACKSTONE_BRICKS), 64);
    pub const RESPAWN_ANCHOR: Self = Self::new(1397, Identifier::vanilla_static("respawn_anchor"), Some(Block::RESPAWN_ANCHOR), 64);
    pub const CANDLE: Self = Self::new(1398, Identifier::vanilla_static("candle"), Some(Block::CANDLE), 64);
    pub const WHITE_CANDLE: Self = Self::new(1399, Identifier::vanilla_static("white_candle"), Some(Block::WHITE_CANDLE), 64);
    pub const ORANGE_CANDLE: Self = Self::new(1400, Identifier::vanilla_static("orange_candle"), Some(Block::ORANGE_CANDLE), 64);
    pub const MAGENTA_CANDLE: Self = Self::new(1401, Identifier::vanilla_static("magenta_candle"), Some(Block::MAGENTA_CANDLE), 64);
    pub const LIGHT_BLUE_CANDLE: Self = Self::new(1402, Identifier::vanilla_static("light_blue_candle"), Some(Block::LIGHT_BLUE_CANDLE), 64);
    pub const YELLOW_CANDLE: Self = Self::new(1403, Identifier::vanilla_static("yellow_candle"), Some(Block::YELLOW_CANDLE), 64);
    pub const LIME_CANDLE: Self = Self::new(1404, Identifier::vanilla_static("lime_candle"), Some(Block::LIME_CANDLE), 64);
    pub const PINK_CANDLE: Self = Self::new(1405, Identifier::vanilla_static("pink_candle"), Some(Block::PINK_CANDLE), 64);
    pub const GRAY_CANDLE: Self = Self::new(1406, Identifier::vanilla_static("gray_candle"), Some(Block::GRAY_CANDLE), 64);
    pub const LIGHT_GRAY_CANDLE: Self = Self::new(1407, Identifier::vanilla_static("light_gray_candle"), Some(Block::LIGHT_GRAY_CANDLE), 64);
    pub const CYAN_CANDLE: Self = Self::new(1408, Identifier::vanilla_static("cyan_candle"), Some(Block::CYAN_CANDLE), 64);
    pub const PURPLE_CANDLE: Self = Self::new(1409, Identifier::vanilla_static("purple_candle"), Some(Block::PURPLE_CANDLE), 64);
    pub const BLUE_CANDLE: Self = Self::new(1410, Identifier::vanilla_static("blue_candle"), Some(Block::BLUE_CANDLE), 64);
    pub const BROWN_CANDLE: Self = Self::new(1411, Identifier::vanilla_static("brown_candle"), Some(Block::BROWN_CANDLE), 64);
    pub const GREEN_CANDLE: Self = Self::new(1412, Identifier::vanilla_static("green_candle"), Some(Block::GREEN_CANDLE), 64);
    pub const RED_CANDLE: Self = Self::new(1413, Identifier::vanilla_static("red_candle"), Some(Block::RED_CANDLE), 64);
    pub const BLACK_CANDLE: Self = Self::new(1414, Identifier::vanilla_static("black_candle"), Some(Block::BLACK_CANDLE), 64);
    pub const SMALL_AMETHYST_BUD: Self = Self::new(1415, Identifier::vanilla_static("small_amethyst_bud"), Some(Block::SMALL_AMETHYST_BUD), 64);
    pub const MEDIUM_AMETHYST_BUD: Self = Self::new(1416, Identifier::vanilla_static("medium_amethyst_bud"), Some(Block::MEDIUM_AMETHYST_BUD), 64);
    pub const LARGE_AMETHYST_BUD: Self = Self::new(1417, Identifier::vanilla_static("large_amethyst_bud"), Some(Block::LARGE_AMETHYST_BUD), 64);
    pub const AMETHYST_CLUSTER: Self = Self::new(1418, Identifier::vanilla_static("amethyst_cluster"), Some(Block::AMETHYST_CLUSTER), 64);
    pub const POINTED_DRIPSTONE: Self = Self::new(1419, Identifier::vanilla_static("pointed_dripstone"), Some(Block::POINTED_DRIPSTONE), 64);
    pub const OCHRE_FROGLIGHT: Self = Self::new(1420, Identifier::vanilla_static("ochre_froglight"), Some(Block::OCHRE_FROGLIGHT), 64);
    pub const VERDANT_FROGLIGHT: Self = Self::new(1421, Identifier::vanilla_static("verdant_froglight"), Some(Block::VERDANT_FROGLIGHT), 64);
    pub const PEARLESCENT_FROGLIGHT: Self = Self::new(1422, Identifier::vanilla_static("pearlescent_froglight"), Some(Block::PEARLESCENT_FROGLIGHT), 64);
    pub const FROGSPAWN: Self = Self::new(1423, Identifier::vanilla_static("frogspawn"), Some(Block::FROGSPAWN), 64);
    pub const ECHO_SHARD: Self = Self::new(1424, Identifier::vanilla_static("echo_shard"), None, 64);
    pub const BRUSH: Self = Self::new(1425, Identifier::vanilla_static("brush"), None, 1);
    pub const NETHERITE_UPGRADE_SMITHING_TEMPLATE: Self = Self::new(1426, Identifier::vanilla_static("netherite_upgrade_smithing_template"), None, 64);
    pub const SENTRY_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1427, Identifier::vanilla_static("sentry_armor_trim_smithing_template"), None, 64);
    pub const DUNE_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1428, Identifier::vanilla_static("dune_armor_trim_smithing_template"), None, 64);
    pub const COAST_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1429, Identifier::vanilla_static("coast_armor_trim_smithing_template"), None, 64);
    pub const WILD_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1430, Identifier::vanilla_static("wild_armor_trim_smithing_template"), None, 64);
    pub const WARD_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1431, Identifier::vanilla_static("ward_armor_trim_smithing_template"), None, 64);
    pub const EYE_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1432, Identifier::vanilla_static("eye_armor_trim_smithing_template"), None, 64);
    pub const VEX_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1433, Identifier::vanilla_static("vex_armor_trim_smithing_template"), None, 64);
    pub const TIDE_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1434, Identifier::vanilla_static("tide_armor_trim_smithing_template"), None, 64);
    pub const SNOUT_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1435, Identifier::vanilla_static("snout_armor_trim_smithing_template"), None, 64);
    pub const RIB_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1436, Identifier::vanilla_static("rib_armor_trim_smithing_template"), None, 64);
    pub const SPIRE_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1437, Identifier::vanilla_static("spire_armor_trim_smithing_template"), None, 64);
    pub const WAYFINDER_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1438, Identifier::vanilla_static("wayfinder_armor_trim_smithing_template"), None, 64);
    pub const SHAPER_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1439, Identifier::vanilla_static("shaper_armor_trim_smithing_template"), None, 64);
    pub const SILENCE_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1440, Identifier::vanilla_static("silence_armor_trim_smithing_template"), None, 64);
    pub const RAISER_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1441, Identifier::vanilla_static("raiser_armor_trim_smithing_template"), None, 64);
    pub const HOST_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1442, Identifier::vanilla_static("host_armor_trim_smithing_template"), None, 64);
    pub const FLOW_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1443, Identifier::vanilla_static("flow_armor_trim_smithing_template"), None, 64);
    pub const BOLT_ARMOR_TRIM_SMITHING_TEMPLATE: Self = Self::new(1444, Identifier::vanilla_static("bolt_armor_trim_smithing_template"), None, 64);
    pub const ANGLER_POTTERY_SHERD: Self = Self::new(1445, Identifier::vanilla_static("angler_pottery_sherd"), None, 64);
    pub const ARCHER_POTTERY_SHERD: Self = Self::new(1446, Identifier::vanilla_static("archer_pottery_sherd"), None, 64);
    pub const ARMS_UP_POTTERY_SHERD: Self = Self::new(1447, Identifier::vanilla_static("arms_up_pottery_sherd"), None, 64);
    pub const BLADE_POTTERY_SHERD: Self = Self::new(1448, Identifier::vanilla_static("blade_pottery_sherd"), None, 64);
    pub const BREWER_POTTERY_SHERD: Self = Self::new(1449, Identifier::vanilla_static("brewer_pottery_sherd"), None, 64);
    pub const BURN_POTTERY_SHERD: Self = Self::new(1450, Identifier::vanilla_static("burn_pottery_sherd"), None, 64);
    pub const DANGER_POTTERY_SHERD: Self = Self::new(1451, Identifier::vanilla_static("danger_pottery_sherd"), None, 64);
    pub const EXPLORER_POTTERY_SHERD: Self = Self::new(1452, Identifier::vanilla_static("explorer_pottery_sherd"), None, 64);
    pub const FLOW_POTTERY_SHERD: Self = Self::new(1453, Identifier::vanilla_static("flow_pottery_sherd"), None, 64);
    pub const FRIEND_POTTERY_SHERD: Self = Self::new(1454, Identifier::vanilla_static("friend_pottery_sherd"), None, 64);
    pub const GUSTER_POTTERY_SHERD: Self = Self::new(1455, Identifier::vanilla_static("guster_pottery_sherd"), None, 64);
    pub const HEART_POTTERY_SHERD: Self = Self::new(1456, Identifier::vanilla_static("heart_pottery_sherd"), None, 64);
    pub const HEARTBREAK_POTTERY_SHERD: Self = Self::new(1457, Identifier::vanilla_static("heartbreak_pottery_sherd"), None, 64);
    pub const HOWL_POTTERY_SHERD: Self = Self::new(1458, Identifier::vanilla_static("howl_pottery_sherd"), None, 64);
    pub const MINER_POTTERY_SHERD: Self = Self::new(1459, Identifier::vanilla_static("miner_pottery_sherd"), None, 64);
    pub const MOURNER_POTTERY_SHERD: Self = Self::new(1460, Identifier::vanilla_static("mourner_pottery_sherd"), None, 64);
    pub const PLENTY_POTTERY_SHERD: Self = Self::new(1461, Identifier::vanilla_static("plenty_pottery_sherd"), None, 64);
    pub const PRIZE_POTTERY_SHERD: Self = Self::new(1462, Identifier::vanilla_static("prize_pottery_sherd"), None, 64);
    pub const SCRAPE_POTTERY_SHERD: Self = Self::new(1463, Identifier::vanilla_static("scrape_pottery_sherd"), None, 64);
    pub const SHEAF_POTTERY_SHERD: Self = Self::new(1464, Identifier::vanilla_static("sheaf_pottery_sherd"), None, 64);
    pub const SHELTER_POTTERY_SHERD: Self = Self::new(1465, Identifier::vanilla_static("shelter_pottery_sherd"), None, 64);
    pub const SKULL_POTTERY_SHERD: Self = Self::new(1466, Identifier::vanilla_static("skull_pottery_sherd"), None, 64);
    pub const SNORT_POTTERY_SHERD: Self = Self::new(1467, Identifier::vanilla_static("snort_pottery_sherd"), None, 64);
    pub const COPPER_GRATE: Self = Self::new(1468, Identifier::vanilla_static("copper_grate"), Some(Block::COPPER_GRATE), 64);
    pub const EXPOSED_COPPER_GRATE: Self = Self::new(1469, Identifier::vanilla_static("exposed_copper_grate"), Some(Block::EXPOSED_COPPER_GRATE), 64);
    pub const WEATHERED_COPPER_GRATE: Self = Self::new(1470, Identifier::vanilla_static("weathered_copper_grate"), Some(Block::WEATHERED_COPPER_GRATE), 64);
    pub const OXIDIZED_COPPER_GRATE: Self = Self::new(1471, Identifier::vanilla_static("oxidized_copper_grate"), Some(Block::OXIDIZED_COPPER_GRATE), 64);
    pub const WAXED_COPPER_GRATE: Self = Self::new(1472, Identifier::vanilla_static("waxed_copper_grate"), Some(Block::WAXED_COPPER_GRATE), 64);
    pub const WAXED_EXPOSED_COPPER_GRATE: Self = Self::new(1473, Identifier::vanilla_static("waxed_exposed_copper_grate"), Some(Block::WAXED_EXPOSED_COPPER_GRATE), 64);
    pub const WAXED_WEATHERED_COPPER_GRATE: Self = Self::new(1474, Identifier::vanilla_static("waxed_weathered_copper_grate"), Some(Block::WAXED_WEATHERED_COPPER_GRATE), 64);
    pub const WAXED_OXIDIZED_COPPER_GRATE: Self = Self::new(1475, Identifier::vanilla_static("waxed_oxidized_copper_grate"), Some(Block::WAXED_OXIDIZED_COPPER_GRATE), 64);
    pub const COPPER_BULB: Self = Self::new(1476, Identifier::vanilla_static("copper_bulb"), Some(Block::COPPER_BULB), 64);
    pub const EXPOSED_COPPER_BULB: Self = Self::new(1477, Identifier::vanilla_static("exposed_copper_bulb"), Some(Block::EXPOSED_COPPER_BULB), 64);
    pub const WEATHERED_COPPER_BULB: Self = Self::new(1478, Identifier::vanilla_static("weathered_copper_bulb"), Some(Block::WEATHERED_COPPER_BULB), 64);
    pub const OXIDIZED_COPPER_BULB: Self = Self::new(1479, Identifier::vanilla_static("oxidized_copper_bulb"), Some(Block::OXIDIZED_COPPER_BULB), 64);
    pub const WAXED_COPPER_BULB: Self = Self::new(1480, Identifier::vanilla_static("waxed_copper_bulb"), Some(Block::WAXED_COPPER_BULB), 64);
    pub const WAXED_EXPOSED_COPPER_BULB: Self = Self::new(1481, Identifier::vanilla_static("waxed_exposed_copper_bulb"), Some(Block::WAXED_EXPOSED_COPPER_BULB), 64);
    pub const WAXED_WEATHERED_COPPER_BULB: Self = Self::new(1482, Identifier::vanilla_static("waxed_weathered_copper_bulb"), Some(Block::WAXED_WEATHERED_COPPER_BULB), 64);
    pub const WAXED_OXIDIZED_COPPER_BULB: Self = Self::new(1483, Identifier::vanilla_static("waxed_oxidized_copper_bulb"), Some(Block::WAXED_OXIDIZED_COPPER_BULB), 64);
    pub const COPPER_CHEST: Self = Self::new(1484, Identifier::vanilla_static("copper_chest"), Some(Block::COPPER_CHEST), 64);
    pub const EXPOSED_COPPER_CHEST: Self = Self::new(1485, Identifier::vanilla_static("exposed_copper_chest"), Some(Block::EXPOSED_COPPER_CHEST), 64);
    pub const WEATHERED_COPPER_CHEST: Self = Self::new(1486, Identifier::vanilla_static("weathered_copper_chest"), Some(Block::WEATHERED_COPPER_CHEST), 64);
    pub const OXIDIZED_COPPER_CHEST: Self = Self::new(1487, Identifier::vanilla_static("oxidized_copper_chest"), Some(Block::OXIDIZED_COPPER_CHEST), 64);
    pub const WAXED_COPPER_CHEST: Self = Self::new(1488, Identifier::vanilla_static("waxed_copper_chest"), Some(Block::WAXED_COPPER_CHEST), 64);
    pub const WAXED_EXPOSED_COPPER_CHEST: Self = Self::new(1489, Identifier::vanilla_static("waxed_exposed_copper_chest"), Some(Block::WAXED_EXPOSED_COPPER_CHEST), 64);
    pub const WAXED_WEATHERED_COPPER_CHEST: Self = Self::new(1490, Identifier::vanilla_static("waxed_weathered_copper_chest"), Some(Block::WAXED_WEATHERED_COPPER_CHEST), 64);
    pub const WAXED_OXIDIZED_COPPER_CHEST: Self = Self::new(1491, Identifier::vanilla_static("waxed_oxidized_copper_chest"), Some(Block::WAXED_OXIDIZED_COPPER_CHEST), 64);
    pub const COPPER_GOLEM_STATUE: Self = Self::new(1492, Identifier::vanilla_static("copper_golem_statue"), Some(Block::COPPER_GOLEM_STATUE), 64);
    pub const EXPOSED_COPPER_GOLEM_STATUE: Self = Self::new(1493, Identifier::vanilla_static("exposed_copper_golem_statue"), Some(Block::EXPOSED_COPPER_GOLEM_STATUE), 64);
    pub const WEATHERED_COPPER_GOLEM_STATUE: Self = Self::new(1494, Identifier::vanilla_static("weathered_copper_golem_statue"), Some(Block::WEATHERED_COPPER_GOLEM_STATUE), 64);
    pub const OXIDIZED_COPPER_GOLEM_STATUE: Self = Self::new(1495, Identifier::vanilla_static("oxidized_copper_golem_statue"), Some(Block::OXIDIZED_COPPER_GOLEM_STATUE), 64);
    pub const WAXED_COPPER_GOLEM_STATUE: Self = Self::new(1496, Identifier::vanilla_static("waxed_copper_golem_statue"), Some(Block::WAXED_COPPER_GOLEM_STATUE), 64);
    pub const WAXED_EXPOSED_COPPER_GOLEM_STATUE: Self = Self::new(1497, Identifier::vanilla_static("waxed_exposed_copper_golem_statue"), Some(Block::WAXED_EXPOSED_COPPER_GOLEM_STATUE), 64);
    pub const WAXED_WEATHERED_COPPER_GOLEM_STATUE: Self = Self::new(1498, Identifier::vanilla_static("waxed_weathered_copper_golem_statue"), Some(Block::WAXED_WEATHERED_COPPER_GOLEM_STATUE), 64);
    pub const WAXED_OXIDIZED_COPPER_GOLEM_STATUE: Self = Self::new(1499, Identifier::vanilla_static("waxed_oxidized_copper_golem_statue"), Some(Block::WAXED_OXIDIZED_COPPER_GOLEM_STATUE), 64);
    pub const TRIAL_SPAWNER: Self = Self::new(1500, Identifier::vanilla_static("trial_spawner"), Some(Block::TRIAL_SPAWNER), 64);
    pub const TRIAL_KEY: Self = Self::new(1501, Identifier::vanilla_static("trial_key"), None, 64);
    pub const OMINOUS_TRIAL_KEY: Self = Self::new(1502, Identifier::vanilla_static("ominous_trial_key"), None, 64);
    pub const VAULT: Self = Self::new(1503, Identifier::vanilla_static("vault"), Some(Block::VAULT), 64);
    pub const OMINOUS_BOTTLE: Self = Self::new(1504, Identifier::vanilla_static("ominous_bottle"), None, 64);
    pub const ALL: &'static [Self] = &[
        Self::AIR,
        Self::STONE,
        Self::GRANITE,
        Self::POLISHED_GRANITE,
        Self::DIORITE,
        Self::POLISHED_DIORITE,
        Self::ANDESITE,
        Self::POLISHED_ANDESITE,
        Self::DEEPSLATE,
        Self::COBBLED_DEEPSLATE,
        Self::POLISHED_DEEPSLATE,
        Self::CALCITE,
        Self::TUFF,
        Self::TUFF_SLAB,
        Self::TUFF_STAIRS,
        Self::TUFF_WALL,
        Self::CHISELED_TUFF,
        Self::POLISHED_TUFF,
        Self::POLISHED_TUFF_SLAB,
        Self::POLISHED_TUFF_STAIRS,
        Self::POLISHED_TUFF_WALL,
        Self::TUFF_BRICKS,
        Self::TUFF_BRICK_SLAB,
        Self::TUFF_BRICK_STAIRS,
        Self::TUFF_BRICK_WALL,
        Self::CHISELED_TUFF_BRICKS,
        Self::DRIPSTONE_BLOCK,
        Self::GRASS_BLOCK,
        Self::DIRT,
        Self::COARSE_DIRT,
        Self::PODZOL,
        Self::ROOTED_DIRT,
        Self::MUD,
        Self::CRIMSON_NYLIUM,
        Self::WARPED_NYLIUM,
        Self::COBBLESTONE,
        Self::OAK_PLANKS,
        Self::SPRUCE_PLANKS,
        Self::BIRCH_PLANKS,
        Self::JUNGLE_PLANKS,
        Self::ACACIA_PLANKS,
        Self::CHERRY_PLANKS,
        Self::DARK_OAK_PLANKS,
        Self::PALE_OAK_PLANKS,
        Self::MANGROVE_PLANKS,
        Self::BAMBOO_PLANKS,
        Self::CRIMSON_PLANKS,
        Self::WARPED_PLANKS,
        Self::BAMBOO_MOSAIC,
        Self::OAK_SAPLING,
        Self::SPRUCE_SAPLING,
        Self::BIRCH_SAPLING,
        Self::JUNGLE_SAPLING,
        Self::ACACIA_SAPLING,
        Self::CHERRY_SAPLING,
        Self::DARK_OAK_SAPLING,
        Self::PALE_OAK_SAPLING,
        Self::MANGROVE_PROPAGULE,
        Self::BEDROCK,
        Self::SAND,
        Self::SUSPICIOUS_SAND,
        Self::SUSPICIOUS_GRAVEL,
        Self::RED_SAND,
        Self::GRAVEL,
        Self::COAL_ORE,
        Self::DEEPSLATE_COAL_ORE,
        Self::IRON_ORE,
        Self::DEEPSLATE_IRON_ORE,
        Self::COPPER_ORE,
        Self::DEEPSLATE_COPPER_ORE,
        Self::GOLD_ORE,
        Self::DEEPSLATE_GOLD_ORE,
        Self::REDSTONE_ORE,
        Self::DEEPSLATE_REDSTONE_ORE,
        Self::EMERALD_ORE,
        Self::DEEPSLATE_EMERALD_ORE,
        Self::LAPIS_ORE,
        Self::DEEPSLATE_LAPIS_ORE,
        Self::DIAMOND_ORE,
        Self::DEEPSLATE_DIAMOND_ORE,
        Self::NETHER_GOLD_ORE,
        Self::NETHER_QUARTZ_ORE,
        Self::ANCIENT_DEBRIS,
        Self::COAL_BLOCK,
        Self::RAW_IRON_BLOCK,
        Self::RAW_COPPER_BLOCK,
        Self::RAW_GOLD_BLOCK,
        Self::HEAVY_CORE,
        Self::AMETHYST_BLOCK,
        Self::BUDDING_AMETHYST,
        Self::IRON_BLOCK,
        Self::COPPER_BLOCK,
        Self::GOLD_BLOCK,
        Self::DIAMOND_BLOCK,
        Self::NETHERITE_BLOCK,
        Self::EXPOSED_COPPER,
        Self::WEATHERED_COPPER,
        Self::OXIDIZED_COPPER,
        Self::CHISELED_COPPER,
        Self::EXPOSED_CHISELED_COPPER,
        Self::WEATHERED_CHISELED_COPPER,
        Self::OXIDIZED_CHISELED_COPPER,
        Self::CUT_COPPER,
        Self::EXPOSED_CUT_COPPER,
        Self::WEATHERED_CUT_COPPER,
        Self::OXIDIZED_CUT_COPPER,
        Self::CUT_COPPER_STAIRS,
        Self::EXPOSED_CUT_COPPER_STAIRS,
        Self::WEATHERED_CUT_COPPER_STAIRS,
        Self::OXIDIZED_CUT_COPPER_STAIRS,
        Self::CUT_COPPER_SLAB,
        Self::EXPOSED_CUT_COPPER_SLAB,
        Self::WEATHERED_CUT_COPPER_SLAB,
        Self::OXIDIZED_CUT_COPPER_SLAB,
        Self::WAXED_COPPER_BLOCK,
        Self::WAXED_EXPOSED_COPPER,
        Self::WAXED_WEATHERED_COPPER,
        Self::WAXED_OXIDIZED_COPPER,
        Self::WAXED_CHISELED_COPPER,
        Self::WAXED_EXPOSED_CHISELED_COPPER,
        Self::WAXED_WEATHERED_CHISELED_COPPER,
        Self::WAXED_OXIDIZED_CHISELED_COPPER,
        Self::WAXED_CUT_COPPER,
        Self::WAXED_EXPOSED_CUT_COPPER,
        Self::WAXED_WEATHERED_CUT_COPPER,
        Self::WAXED_OXIDIZED_CUT_COPPER,
        Self::WAXED_CUT_COPPER_STAIRS,
        Self::WAXED_EXPOSED_CUT_COPPER_STAIRS,
        Self::WAXED_WEATHERED_CUT_COPPER_STAIRS,
        Self::WAXED_OXIDIZED_CUT_COPPER_STAIRS,
        Self::WAXED_CUT_COPPER_SLAB,
        Self::WAXED_EXPOSED_CUT_COPPER_SLAB,
        Self::WAXED_WEATHERED_CUT_COPPER_SLAB,
        Self::WAXED_OXIDIZED_CUT_COPPER_SLAB,
        Self::OAK_LOG,
        Self::SPRUCE_LOG,
        Self::BIRCH_LOG,
        Self::JUNGLE_LOG,
        Self::ACACIA_LOG,
        Self::CHERRY_LOG,
        Self::PALE_OAK_LOG,
        Self::DARK_OAK_LOG,
        Self::MANGROVE_LOG,
        Self::MANGROVE_ROOTS,
        Self::MUDDY_MANGROVE_ROOTS,
        Self::CRIMSON_STEM,
        Self::WARPED_STEM,
        Self::BAMBOO_BLOCK,
        Self::STRIPPED_OAK_LOG,
        Self::STRIPPED_SPRUCE_LOG,
        Self::STRIPPED_BIRCH_LOG,
        Self::STRIPPED_JUNGLE_LOG,
        Self::STRIPPED_ACACIA_LOG,
        Self::STRIPPED_CHERRY_LOG,
        Self::STRIPPED_DARK_OAK_LOG,
        Self::STRIPPED_PALE_OAK_LOG,
        Self::STRIPPED_MANGROVE_LOG,
        Self::STRIPPED_CRIMSON_STEM,
        Self::STRIPPED_WARPED_STEM,
        Self::STRIPPED_OAK_WOOD,
        Self::STRIPPED_SPRUCE_WOOD,
        Self::STRIPPED_BIRCH_WOOD,
        Self::STRIPPED_JUNGLE_WOOD,
        Self::STRIPPED_ACACIA_WOOD,
        Self::STRIPPED_CHERRY_WOOD,
        Self::STRIPPED_DARK_OAK_WOOD,
        Self::STRIPPED_PALE_OAK_WOOD,
        Self::STRIPPED_MANGROVE_WOOD,
        Self::STRIPPED_CRIMSON_HYPHAE,
        Self::STRIPPED_WARPED_HYPHAE,
        Self::STRIPPED_BAMBOO_BLOCK,
        Self::OAK_WOOD,
        Self::SPRUCE_WOOD,
        Self::BIRCH_WOOD,
        Self::JUNGLE_WOOD,
        Self::ACACIA_WOOD,
        Self::CHERRY_WOOD,
        Self::PALE_OAK_WOOD,
        Self::DARK_OAK_WOOD,
        Self::MANGROVE_WOOD,
        Self::CRIMSON_HYPHAE,
        Self::WARPED_HYPHAE,
        Self::OAK_LEAVES,
        Self::SPRUCE_LEAVES,
        Self::BIRCH_LEAVES,
        Self::JUNGLE_LEAVES,
        Self::ACACIA_LEAVES,
        Self::CHERRY_LEAVES,
        Self::DARK_OAK_LEAVES,
        Self::PALE_OAK_LEAVES,
        Self::MANGROVE_LEAVES,
        Self::AZALEA_LEAVES,
        Self::FLOWERING_AZALEA_LEAVES,
        Self::SPONGE,
        Self::WET_SPONGE,
        Self::GLASS,
        Self::TINTED_GLASS,
        Self::LAPIS_BLOCK,
        Self::SANDSTONE,
        Self::CHISELED_SANDSTONE,
        Self::CUT_SANDSTONE,
        Self::COBWEB,
        Self::SHORT_GRASS,
        Self::FERN,
        Self::BUSH,
        Self::AZALEA,
        Self::FLOWERING_AZALEA,
        Self::DEAD_BUSH,
        Self::FIREFLY_BUSH,
        Self::SHORT_DRY_GRASS,
        Self::TALL_DRY_GRASS,
        Self::SEAGRASS,
        Self::SEA_PICKLE,
        Self::WHITE_WOOL,
        Self::ORANGE_WOOL,
        Self::MAGENTA_WOOL,
        Self::LIGHT_BLUE_WOOL,
        Self::YELLOW_WOOL,
        Self::LIME_WOOL,
        Self::PINK_WOOL,
        Self::GRAY_WOOL,
        Self::LIGHT_GRAY_WOOL,
        Self::CYAN_WOOL,
        Self::PURPLE_WOOL,
        Self::BLUE_WOOL,
        Self::BROWN_WOOL,
        Self::GREEN_WOOL,
        Self::RED_WOOL,
        Self::BLACK_WOOL,
        Self::DANDELION,
        Self::OPEN_EYEBLOSSOM,
        Self::CLOSED_EYEBLOSSOM,
        Self::POPPY,
        Self::BLUE_ORCHID,
        Self::ALLIUM,
        Self::AZURE_BLUET,
        Self::RED_TULIP,
        Self::ORANGE_TULIP,
        Self::WHITE_TULIP,
        Self::PINK_TULIP,
        Self::OXEYE_DAISY,
        Self::CORNFLOWER,
        Self::LILY_OF_THE_VALLEY,
        Self::WITHER_ROSE,
        Self::TORCHFLOWER,
        Self::PITCHER_PLANT,
        Self::SPORE_BLOSSOM,
        Self::BROWN_MUSHROOM,
        Self::RED_MUSHROOM,
        Self::CRIMSON_FUNGUS,
        Self::WARPED_FUNGUS,
        Self::CRIMSON_ROOTS,
        Self::WARPED_ROOTS,
        Self::NETHER_SPROUTS,
        Self::WEEPING_VINES,
        Self::TWISTING_VINES,
        Self::SUGAR_CANE,
        Self::KELP,
        Self::PINK_PETALS,
        Self::WILDFLOWERS,
        Self::LEAF_LITTER,
        Self::MOSS_CARPET,
        Self::MOSS_BLOCK,
        Self::PALE_MOSS_CARPET,
        Self::PALE_HANGING_MOSS,
        Self::PALE_MOSS_BLOCK,
        Self::HANGING_ROOTS,
        Self::BIG_DRIPLEAF,
        Self::SMALL_DRIPLEAF,
        Self::BAMBOO,
        Self::OAK_SLAB,
        Self::SPRUCE_SLAB,
        Self::BIRCH_SLAB,
        Self::JUNGLE_SLAB,
        Self::ACACIA_SLAB,
        Self::CHERRY_SLAB,
        Self::DARK_OAK_SLAB,
        Self::PALE_OAK_SLAB,
        Self::MANGROVE_SLAB,
        Self::BAMBOO_SLAB,
        Self::BAMBOO_MOSAIC_SLAB,
        Self::CRIMSON_SLAB,
        Self::WARPED_SLAB,
        Self::STONE_SLAB,
        Self::SMOOTH_STONE_SLAB,
        Self::SANDSTONE_SLAB,
        Self::CUT_SANDSTONE_SLAB,
        Self::PETRIFIED_OAK_SLAB,
        Self::COBBLESTONE_SLAB,
        Self::BRICK_SLAB,
        Self::STONE_BRICK_SLAB,
        Self::MUD_BRICK_SLAB,
        Self::NETHER_BRICK_SLAB,
        Self::QUARTZ_SLAB,
        Self::RED_SANDSTONE_SLAB,
        Self::CUT_RED_SANDSTONE_SLAB,
        Self::PURPUR_SLAB,
        Self::PRISMARINE_SLAB,
        Self::PRISMARINE_BRICK_SLAB,
        Self::DARK_PRISMARINE_SLAB,
        Self::SMOOTH_QUARTZ,
        Self::SMOOTH_RED_SANDSTONE,
        Self::SMOOTH_SANDSTONE,
        Self::SMOOTH_STONE,
        Self::BRICKS,
        Self::ACACIA_SHELF,
        Self::BAMBOO_SHELF,
        Self::BIRCH_SHELF,
        Self::CHERRY_SHELF,
        Self::CRIMSON_SHELF,
        Self::DARK_OAK_SHELF,
        Self::JUNGLE_SHELF,
        Self::MANGROVE_SHELF,
        Self::OAK_SHELF,
        Self::PALE_OAK_SHELF,
        Self::SPRUCE_SHELF,
        Self::WARPED_SHELF,
        Self::BOOKSHELF,
        Self::CHISELED_BOOKSHELF,
        Self::DECORATED_POT,
        Self::MOSSY_COBBLESTONE,
        Self::OBSIDIAN,
        Self::TORCH,
        Self::END_ROD,
        Self::CHORUS_PLANT,
        Self::CHORUS_FLOWER,
        Self::PURPUR_BLOCK,
        Self::PURPUR_PILLAR,
        Self::PURPUR_STAIRS,
        Self::SPAWNER,
        Self::CREAKING_HEART,
        Self::CHEST,
        Self::CRAFTING_TABLE,
        Self::FARMLAND,
        Self::FURNACE,
        Self::LADDER,
        Self::COBBLESTONE_STAIRS,
        Self::SNOW,
        Self::ICE,
        Self::SNOW_BLOCK,
        Self::CACTUS,
        Self::CACTUS_FLOWER,
        Self::CLAY,
        Self::JUKEBOX,
        Self::OAK_FENCE,
        Self::SPRUCE_FENCE,
        Self::BIRCH_FENCE,
        Self::JUNGLE_FENCE,
        Self::ACACIA_FENCE,
        Self::CHERRY_FENCE,
        Self::DARK_OAK_FENCE,
        Self::PALE_OAK_FENCE,
        Self::MANGROVE_FENCE,
        Self::BAMBOO_FENCE,
        Self::CRIMSON_FENCE,
        Self::WARPED_FENCE,
        Self::PUMPKIN,
        Self::CARVED_PUMPKIN,
        Self::JACK_O_LANTERN,
        Self::NETHERRACK,
        Self::SOUL_SAND,
        Self::SOUL_SOIL,
        Self::BASALT,
        Self::POLISHED_BASALT,
        Self::SMOOTH_BASALT,
        Self::SOUL_TORCH,
        Self::COPPER_TORCH,
        Self::GLOWSTONE,
        Self::INFESTED_STONE,
        Self::INFESTED_COBBLESTONE,
        Self::INFESTED_STONE_BRICKS,
        Self::INFESTED_MOSSY_STONE_BRICKS,
        Self::INFESTED_CRACKED_STONE_BRICKS,
        Self::INFESTED_CHISELED_STONE_BRICKS,
        Self::INFESTED_DEEPSLATE,
        Self::STONE_BRICKS,
        Self::MOSSY_STONE_BRICKS,
        Self::CRACKED_STONE_BRICKS,
        Self::CHISELED_STONE_BRICKS,
        Self::PACKED_MUD,
        Self::MUD_BRICKS,
        Self::DEEPSLATE_BRICKS,
        Self::CRACKED_DEEPSLATE_BRICKS,
        Self::DEEPSLATE_TILES,
        Self::CRACKED_DEEPSLATE_TILES,
        Self::CHISELED_DEEPSLATE,
        Self::REINFORCED_DEEPSLATE,
        Self::BROWN_MUSHROOM_BLOCK,
        Self::RED_MUSHROOM_BLOCK,
        Self::MUSHROOM_STEM,
        Self::IRON_BARS,
        Self::COPPER_BARS,
        Self::EXPOSED_COPPER_BARS,
        Self::WEATHERED_COPPER_BARS,
        Self::OXIDIZED_COPPER_BARS,
        Self::WAXED_COPPER_BARS,
        Self::WAXED_EXPOSED_COPPER_BARS,
        Self::WAXED_WEATHERED_COPPER_BARS,
        Self::WAXED_OXIDIZED_COPPER_BARS,
        Self::IRON_CHAIN,
        Self::COPPER_CHAIN,
        Self::EXPOSED_COPPER_CHAIN,
        Self::WEATHERED_COPPER_CHAIN,
        Self::OXIDIZED_COPPER_CHAIN,
        Self::WAXED_COPPER_CHAIN,
        Self::WAXED_EXPOSED_COPPER_CHAIN,
        Self::WAXED_WEATHERED_COPPER_CHAIN,
        Self::WAXED_OXIDIZED_COPPER_CHAIN,
        Self::GLASS_PANE,
        Self::MELON,
        Self::VINE,
        Self::GLOW_LICHEN,
        Self::RESIN_CLUMP,
        Self::RESIN_BLOCK,
        Self::RESIN_BRICKS,
        Self::RESIN_BRICK_STAIRS,
        Self::RESIN_BRICK_SLAB,
        Self::RESIN_BRICK_WALL,
        Self::CHISELED_RESIN_BRICKS,
        Self::BRICK_STAIRS,
        Self::STONE_BRICK_STAIRS,
        Self::MUD_BRICK_STAIRS,
        Self::MYCELIUM,
        Self::LILY_PAD,
        Self::NETHER_BRICKS,
        Self::CRACKED_NETHER_BRICKS,
        Self::CHISELED_NETHER_BRICKS,
        Self::NETHER_BRICK_FENCE,
        Self::NETHER_BRICK_STAIRS,
        Self::SCULK,
        Self::SCULK_VEIN,
        Self::SCULK_CATALYST,
        Self::SCULK_SHRIEKER,
        Self::ENCHANTING_TABLE,
        Self::END_PORTAL_FRAME,
        Self::END_STONE,
        Self::END_STONE_BRICKS,
        Self::DRAGON_EGG,
        Self::SANDSTONE_STAIRS,
        Self::ENDER_CHEST,
        Self::EMERALD_BLOCK,
        Self::OAK_STAIRS,
        Self::SPRUCE_STAIRS,
        Self::BIRCH_STAIRS,
        Self::JUNGLE_STAIRS,
        Self::ACACIA_STAIRS,
        Self::CHERRY_STAIRS,
        Self::DARK_OAK_STAIRS,
        Self::PALE_OAK_STAIRS,
        Self::MANGROVE_STAIRS,
        Self::BAMBOO_STAIRS,
        Self::BAMBOO_MOSAIC_STAIRS,
        Self::CRIMSON_STAIRS,
        Self::WARPED_STAIRS,
        Self::COMMAND_BLOCK,
        Self::BEACON,
        Self::COBBLESTONE_WALL,
        Self::MOSSY_COBBLESTONE_WALL,
        Self::BRICK_WALL,
        Self::PRISMARINE_WALL,
        Self::RED_SANDSTONE_WALL,
        Self::MOSSY_STONE_BRICK_WALL,
        Self::GRANITE_WALL,
        Self::STONE_BRICK_WALL,
        Self::MUD_BRICK_WALL,
        Self::NETHER_BRICK_WALL,
        Self::ANDESITE_WALL,
        Self::RED_NETHER_BRICK_WALL,
        Self::SANDSTONE_WALL,
        Self::END_STONE_BRICK_WALL,
        Self::DIORITE_WALL,
        Self::BLACKSTONE_WALL,
        Self::POLISHED_BLACKSTONE_WALL,
        Self::POLISHED_BLACKSTONE_BRICK_WALL,
        Self::COBBLED_DEEPSLATE_WALL,
        Self::POLISHED_DEEPSLATE_WALL,
        Self::DEEPSLATE_BRICK_WALL,
        Self::DEEPSLATE_TILE_WALL,
        Self::ANVIL,
        Self::CHIPPED_ANVIL,
        Self::DAMAGED_ANVIL,
        Self::CHISELED_QUARTZ_BLOCK,
        Self::QUARTZ_BLOCK,
        Self::QUARTZ_BRICKS,
        Self::QUARTZ_PILLAR,
        Self::QUARTZ_STAIRS,
        Self::WHITE_TERRACOTTA,
        Self::ORANGE_TERRACOTTA,
        Self::MAGENTA_TERRACOTTA,
        Self::LIGHT_BLUE_TERRACOTTA,
        Self::YELLOW_TERRACOTTA,
        Self::LIME_TERRACOTTA,
        Self::PINK_TERRACOTTA,
        Self::GRAY_TERRACOTTA,
        Self::LIGHT_GRAY_TERRACOTTA,
        Self::CYAN_TERRACOTTA,
        Self::PURPLE_TERRACOTTA,
        Self::BLUE_TERRACOTTA,
        Self::BROWN_TERRACOTTA,
        Self::GREEN_TERRACOTTA,
        Self::RED_TERRACOTTA,
        Self::BLACK_TERRACOTTA,
        Self::BARRIER,
        Self::LIGHT,
        Self::HAY_BLOCK,
        Self::WHITE_CARPET,
        Self::ORANGE_CARPET,
        Self::MAGENTA_CARPET,
        Self::LIGHT_BLUE_CARPET,
        Self::YELLOW_CARPET,
        Self::LIME_CARPET,
        Self::PINK_CARPET,
        Self::GRAY_CARPET,
        Self::LIGHT_GRAY_CARPET,
        Self::CYAN_CARPET,
        Self::PURPLE_CARPET,
        Self::BLUE_CARPET,
        Self::BROWN_CARPET,
        Self::GREEN_CARPET,
        Self::RED_CARPET,
        Self::BLACK_CARPET,
        Self::TERRACOTTA,
        Self::PACKED_ICE,
        Self::DIRT_PATH,
        Self::SUNFLOWER,
        Self::LILAC,
        Self::ROSE_BUSH,
        Self::PEONY,
        Self::TALL_GRASS,
        Self::LARGE_FERN,
        Self::WHITE_STAINED_GLASS,
        Self::ORANGE_STAINED_GLASS,
        Self::MAGENTA_STAINED_GLASS,
        Self::LIGHT_BLUE_STAINED_GLASS,
        Self::YELLOW_STAINED_GLASS,
        Self::LIME_STAINED_GLASS,
        Self::PINK_STAINED_GLASS,
        Self::GRAY_STAINED_GLASS,
        Self::LIGHT_GRAY_STAINED_GLASS,
        Self::CYAN_STAINED_GLASS,
        Self::PURPLE_STAINED_GLASS,
        Self::BLUE_STAINED_GLASS,
        Self::BROWN_STAINED_GLASS,
        Self::GREEN_STAINED_GLASS,
        Self::RED_STAINED_GLASS,
        Self::BLACK_STAINED_GLASS,
        Self::WHITE_STAINED_GLASS_PANE,
        Self::ORANGE_STAINED_GLASS_PANE,
        Self::MAGENTA_STAINED_GLASS_PANE,
        Self::LIGHT_BLUE_STAINED_GLASS_PANE,
        Self::YELLOW_STAINED_GLASS_PANE,
        Self::LIME_STAINED_GLASS_PANE,
        Self::PINK_STAINED_GLASS_PANE,
        Self::GRAY_STAINED_GLASS_PANE,
        Self::LIGHT_GRAY_STAINED_GLASS_PANE,
        Self::CYAN_STAINED_GLASS_PANE,
        Self::PURPLE_STAINED_GLASS_PANE,
        Self::BLUE_STAINED_GLASS_PANE,
        Self::BROWN_STAINED_GLASS_PANE,
        Self::GREEN_STAINED_GLASS_PANE,
        Self::RED_STAINED_GLASS_PANE,
        Self::BLACK_STAINED_GLASS_PANE,
        Self::PRISMARINE,
        Self::PRISMARINE_BRICKS,
        Self::DARK_PRISMARINE,
        Self::PRISMARINE_STAIRS,
        Self::PRISMARINE_BRICK_STAIRS,
        Self::DARK_PRISMARINE_STAIRS,
        Self::SEA_LANTERN,
        Self::RED_SANDSTONE,
        Self::CHISELED_RED_SANDSTONE,
        Self::CUT_RED_SANDSTONE,
        Self::RED_SANDSTONE_STAIRS,
        Self::REPEATING_COMMAND_BLOCK,
        Self::CHAIN_COMMAND_BLOCK,
        Self::MAGMA_BLOCK,
        Self::NETHER_WART_BLOCK,
        Self::WARPED_WART_BLOCK,
        Self::RED_NETHER_BRICKS,
        Self::BONE_BLOCK,
        Self::STRUCTURE_VOID,
        Self::SHULKER_BOX,
        Self::WHITE_SHULKER_BOX,
        Self::ORANGE_SHULKER_BOX,
        Self::MAGENTA_SHULKER_BOX,
        Self::LIGHT_BLUE_SHULKER_BOX,
        Self::YELLOW_SHULKER_BOX,
        Self::LIME_SHULKER_BOX,
        Self::PINK_SHULKER_BOX,
        Self::GRAY_SHULKER_BOX,
        Self::LIGHT_GRAY_SHULKER_BOX,
        Self::CYAN_SHULKER_BOX,
        Self::PURPLE_SHULKER_BOX,
        Self::BLUE_SHULKER_BOX,
        Self::BROWN_SHULKER_BOX,
        Self::GREEN_SHULKER_BOX,
        Self::RED_SHULKER_BOX,
        Self::BLACK_SHULKER_BOX,
        Self::WHITE_GLAZED_TERRACOTTA,
        Self::ORANGE_GLAZED_TERRACOTTA,
        Self::MAGENTA_GLAZED_TERRACOTTA,
        Self::LIGHT_BLUE_GLAZED_TERRACOTTA,
        Self::YELLOW_GLAZED_TERRACOTTA,
        Self::LIME_GLAZED_TERRACOTTA,
        Self::PINK_GLAZED_TERRACOTTA,
        Self::GRAY_GLAZED_TERRACOTTA,
        Self::LIGHT_GRAY_GLAZED_TERRACOTTA,
        Self::CYAN_GLAZED_TERRACOTTA,
        Self::PURPLE_GLAZED_TERRACOTTA,
        Self::BLUE_GLAZED_TERRACOTTA,
        Self::BROWN_GLAZED_TERRACOTTA,
        Self::GREEN_GLAZED_TERRACOTTA,
        Self::RED_GLAZED_TERRACOTTA,
        Self::BLACK_GLAZED_TERRACOTTA,
        Self::WHITE_CONCRETE,
        Self::ORANGE_CONCRETE,
        Self::MAGENTA_CONCRETE,
        Self::LIGHT_BLUE_CONCRETE,
        Self::YELLOW_CONCRETE,
        Self::LIME_CONCRETE,
        Self::PINK_CONCRETE,
        Self::GRAY_CONCRETE,
        Self::LIGHT_GRAY_CONCRETE,
        Self::CYAN_CONCRETE,
        Self::PURPLE_CONCRETE,
        Self::BLUE_CONCRETE,
        Self::BROWN_CONCRETE,
        Self::GREEN_CONCRETE,
        Self::RED_CONCRETE,
        Self::BLACK_CONCRETE,
        Self::WHITE_CONCRETE_POWDER,
        Self::ORANGE_CONCRETE_POWDER,
        Self::MAGENTA_CONCRETE_POWDER,
        Self::LIGHT_BLUE_CONCRETE_POWDER,
        Self::YELLOW_CONCRETE_POWDER,
        Self::LIME_CONCRETE_POWDER,
        Self::PINK_CONCRETE_POWDER,
        Self::GRAY_CONCRETE_POWDER,
        Self::LIGHT_GRAY_CONCRETE_POWDER,
        Self::CYAN_CONCRETE_POWDER,
        Self::PURPLE_CONCRETE_POWDER,
        Self::BLUE_CONCRETE_POWDER,
        Self::BROWN_CONCRETE_POWDER,
        Self::GREEN_CONCRETE_POWDER,
        Self::RED_CONCRETE_POWDER,
        Self::BLACK_CONCRETE_POWDER,
        Self::TURTLE_EGG,
        Self::SNIFFER_EGG,
        Self::DRIED_GHAST,
        Self::DEAD_TUBE_CORAL_BLOCK,
        Self::DEAD_BRAIN_CORAL_BLOCK,
        Self::DEAD_BUBBLE_CORAL_BLOCK,
        Self::DEAD_FIRE_CORAL_BLOCK,
        Self::DEAD_HORN_CORAL_BLOCK,
        Self::TUBE_CORAL_BLOCK,
        Self::BRAIN_CORAL_BLOCK,
        Self::BUBBLE_CORAL_BLOCK,
        Self::FIRE_CORAL_BLOCK,
        Self::HORN_CORAL_BLOCK,
        Self::TUBE_CORAL,
        Self::BRAIN_CORAL,
        Self::BUBBLE_CORAL,
        Self::FIRE_CORAL,
        Self::HORN_CORAL,
        Self::DEAD_BRAIN_CORAL,
        Self::DEAD_BUBBLE_CORAL,
        Self::DEAD_FIRE_CORAL,
        Self::DEAD_HORN_CORAL,
        Self::DEAD_TUBE_CORAL,
        Self::TUBE_CORAL_FAN,
        Self::BRAIN_CORAL_FAN,
        Self::BUBBLE_CORAL_FAN,
        Self::FIRE_CORAL_FAN,
        Self::HORN_CORAL_FAN,
        Self::DEAD_TUBE_CORAL_FAN,
        Self::DEAD_BRAIN_CORAL_FAN,
        Self::DEAD_BUBBLE_CORAL_FAN,
        Self::DEAD_FIRE_CORAL_FAN,
        Self::DEAD_HORN_CORAL_FAN,
        Self::BLUE_ICE,
        Self::CONDUIT,
        Self::POLISHED_GRANITE_STAIRS,
        Self::SMOOTH_RED_SANDSTONE_STAIRS,
        Self::MOSSY_STONE_BRICK_STAIRS,
        Self::POLISHED_DIORITE_STAIRS,
        Self::MOSSY_COBBLESTONE_STAIRS,
        Self::END_STONE_BRICK_STAIRS,
        Self::STONE_STAIRS,
        Self::SMOOTH_SANDSTONE_STAIRS,
        Self::SMOOTH_QUARTZ_STAIRS,
        Self::GRANITE_STAIRS,
        Self::ANDESITE_STAIRS,
        Self::RED_NETHER_BRICK_STAIRS,
        Self::POLISHED_ANDESITE_STAIRS,
        Self::DIORITE_STAIRS,
        Self::COBBLED_DEEPSLATE_STAIRS,
        Self::POLISHED_DEEPSLATE_STAIRS,
        Self::DEEPSLATE_BRICK_STAIRS,
        Self::DEEPSLATE_TILE_STAIRS,
        Self::POLISHED_GRANITE_SLAB,
        Self::SMOOTH_RED_SANDSTONE_SLAB,
        Self::MOSSY_STONE_BRICK_SLAB,
        Self::POLISHED_DIORITE_SLAB,
        Self::MOSSY_COBBLESTONE_SLAB,
        Self::END_STONE_BRICK_SLAB,
        Self::SMOOTH_SANDSTONE_SLAB,
        Self::SMOOTH_QUARTZ_SLAB,
        Self::GRANITE_SLAB,
        Self::ANDESITE_SLAB,
        Self::RED_NETHER_BRICK_SLAB,
        Self::POLISHED_ANDESITE_SLAB,
        Self::DIORITE_SLAB,
        Self::COBBLED_DEEPSLATE_SLAB,
        Self::POLISHED_DEEPSLATE_SLAB,
        Self::DEEPSLATE_BRICK_SLAB,
        Self::DEEPSLATE_TILE_SLAB,
        Self::SCAFFOLDING,
        Self::REDSTONE,
        Self::REDSTONE_TORCH,
        Self::REDSTONE_BLOCK,
        Self::REPEATER,
        Self::COMPARATOR,
        Self::PISTON,
        Self::STICKY_PISTON,
        Self::SLIME_BLOCK,
        Self::HONEY_BLOCK,
        Self::OBSERVER,
        Self::HOPPER,
        Self::DISPENSER,
        Self::DROPPER,
        Self::LECTERN,
        Self::TARGET,
        Self::LEVER,
        Self::LIGHTNING_ROD,
        Self::EXPOSED_LIGHTNING_ROD,
        Self::WEATHERED_LIGHTNING_ROD,
        Self::OXIDIZED_LIGHTNING_ROD,
        Self::WAXED_LIGHTNING_ROD,
        Self::WAXED_EXPOSED_LIGHTNING_ROD,
        Self::WAXED_WEATHERED_LIGHTNING_ROD,
        Self::WAXED_OXIDIZED_LIGHTNING_ROD,
        Self::DAYLIGHT_DETECTOR,
        Self::SCULK_SENSOR,
        Self::CALIBRATED_SCULK_SENSOR,
        Self::TRIPWIRE_HOOK,
        Self::TRAPPED_CHEST,
        Self::TNT,
        Self::REDSTONE_LAMP,
        Self::NOTE_BLOCK,
        Self::STONE_BUTTON,
        Self::POLISHED_BLACKSTONE_BUTTON,
        Self::OAK_BUTTON,
        Self::SPRUCE_BUTTON,
        Self::BIRCH_BUTTON,
        Self::JUNGLE_BUTTON,
        Self::ACACIA_BUTTON,
        Self::CHERRY_BUTTON,
        Self::DARK_OAK_BUTTON,
        Self::PALE_OAK_BUTTON,
        Self::MANGROVE_BUTTON,
        Self::BAMBOO_BUTTON,
        Self::CRIMSON_BUTTON,
        Self::WARPED_BUTTON,
        Self::STONE_PRESSURE_PLATE,
        Self::POLISHED_BLACKSTONE_PRESSURE_PLATE,
        Self::LIGHT_WEIGHTED_PRESSURE_PLATE,
        Self::HEAVY_WEIGHTED_PRESSURE_PLATE,
        Self::OAK_PRESSURE_PLATE,
        Self::SPRUCE_PRESSURE_PLATE,
        Self::BIRCH_PRESSURE_PLATE,
        Self::JUNGLE_PRESSURE_PLATE,
        Self::ACACIA_PRESSURE_PLATE,
        Self::CHERRY_PRESSURE_PLATE,
        Self::DARK_OAK_PRESSURE_PLATE,
        Self::PALE_OAK_PRESSURE_PLATE,
        Self::MANGROVE_PRESSURE_PLATE,
        Self::BAMBOO_PRESSURE_PLATE,
        Self::CRIMSON_PRESSURE_PLATE,
        Self::WARPED_PRESSURE_PLATE,
        Self::IRON_DOOR,
        Self::OAK_DOOR,
        Self::SPRUCE_DOOR,
        Self::BIRCH_DOOR,
        Self::JUNGLE_DOOR,
        Self::ACACIA_DOOR,
        Self::CHERRY_DOOR,
        Self::DARK_OAK_DOOR,
        Self::PALE_OAK_DOOR,
        Self::MANGROVE_DOOR,
        Self::BAMBOO_DOOR,
        Self::CRIMSON_DOOR,
        Self::WARPED_DOOR,
        Self::COPPER_DOOR,
        Self::EXPOSED_COPPER_DOOR,
        Self::WEATHERED_COPPER_DOOR,
        Self::OXIDIZED_COPPER_DOOR,
        Self::WAXED_COPPER_DOOR,
        Self::WAXED_EXPOSED_COPPER_DOOR,
        Self::WAXED_WEATHERED_COPPER_DOOR,
        Self::WAXED_OXIDIZED_COPPER_DOOR,
        Self::IRON_TRAPDOOR,
        Self::OAK_TRAPDOOR,
        Self::SPRUCE_TRAPDOOR,
        Self::BIRCH_TRAPDOOR,
        Self::JUNGLE_TRAPDOOR,
        Self::ACACIA_TRAPDOOR,
        Self::CHERRY_TRAPDOOR,
        Self::DARK_OAK_TRAPDOOR,
        Self::PALE_OAK_TRAPDOOR,
        Self::MANGROVE_TRAPDOOR,
        Self::BAMBOO_TRAPDOOR,
        Self::CRIMSON_TRAPDOOR,
        Self::WARPED_TRAPDOOR,
        Self::COPPER_TRAPDOOR,
        Self::EXPOSED_COPPER_TRAPDOOR,
        Self::WEATHERED_COPPER_TRAPDOOR,
        Self::OXIDIZED_COPPER_TRAPDOOR,
        Self::WAXED_COPPER_TRAPDOOR,
        Self::WAXED_EXPOSED_COPPER_TRAPDOOR,
        Self::WAXED_WEATHERED_COPPER_TRAPDOOR,
        Self::WAXED_OXIDIZED_COPPER_TRAPDOOR,
        Self::OAK_FENCE_GATE,
        Self::SPRUCE_FENCE_GATE,
        Self::BIRCH_FENCE_GATE,
        Self::JUNGLE_FENCE_GATE,
        Self::ACACIA_FENCE_GATE,
        Self::CHERRY_FENCE_GATE,
        Self::DARK_OAK_FENCE_GATE,
        Self::PALE_OAK_FENCE_GATE,
        Self::MANGROVE_FENCE_GATE,
        Self::BAMBOO_FENCE_GATE,
        Self::CRIMSON_FENCE_GATE,
        Self::WARPED_FENCE_GATE,
        Self::POWERED_RAIL,
        Self::DETECTOR_RAIL,
        Self::RAIL,
        Self::ACTIVATOR_RAIL,
        Self::SADDLE,
        Self::WHITE_HARNESS,
        Self::ORANGE_HARNESS,
        Self::MAGENTA_HARNESS,
        Self::LIGHT_BLUE_HARNESS,
        Self::YELLOW_HARNESS,
        Self::LIME_HARNESS,
        Self::PINK_HARNESS,
        Self::GRAY_HARNESS,
        Self::LIGHT_GRAY_HARNESS,
        Self::CYAN_HARNESS,
        Self::PURPLE_HARNESS,
        Self::BLUE_HARNESS,
        Self::BROWN_HARNESS,
        Self::GREEN_HARNESS,
        Self::RED_HARNESS,
        Self::BLACK_HARNESS,
        Self::MINECART,
        Self::CHEST_MINECART,
        Self::FURNACE_MINECART,
        Self::TNT_MINECART,
        Self::HOPPER_MINECART,
        Self::CARROT_ON_A_STICK,
        Self::WARPED_FUNGUS_ON_A_STICK,
        Self::PHANTOM_MEMBRANE,
        Self::ELYTRA,
        Self::OAK_BOAT,
        Self::OAK_CHEST_BOAT,
        Self::SPRUCE_BOAT,
        Self::SPRUCE_CHEST_BOAT,
        Self::BIRCH_BOAT,
        Self::BIRCH_CHEST_BOAT,
        Self::JUNGLE_BOAT,
        Self::JUNGLE_CHEST_BOAT,
        Self::ACACIA_BOAT,
        Self::ACACIA_CHEST_BOAT,
        Self::CHERRY_BOAT,
        Self::CHERRY_CHEST_BOAT,
        Self::DARK_OAK_BOAT,
        Self::DARK_OAK_CHEST_BOAT,
        Self::PALE_OAK_BOAT,
        Self::PALE_OAK_CHEST_BOAT,
        Self::MANGROVE_BOAT,
        Self::MANGROVE_CHEST_BOAT,
        Self::BAMBOO_RAFT,
        Self::BAMBOO_CHEST_RAFT,
        Self::STRUCTURE_BLOCK,
        Self::JIGSAW,
        Self::TEST_BLOCK,
        Self::TEST_INSTANCE_BLOCK,
        Self::TURTLE_HELMET,
        Self::TURTLE_SCUTE,
        Self::ARMADILLO_SCUTE,
        Self::WOLF_ARMOR,
        Self::FLINT_AND_STEEL,
        Self::BOWL,
        Self::APPLE,
        Self::BOW,
        Self::ARROW,
        Self::COAL,
        Self::CHARCOAL,
        Self::DIAMOND,
        Self::EMERALD,
        Self::LAPIS_LAZULI,
        Self::QUARTZ,
        Self::AMETHYST_SHARD,
        Self::RAW_IRON,
        Self::IRON_INGOT,
        Self::RAW_COPPER,
        Self::COPPER_INGOT,
        Self::RAW_GOLD,
        Self::GOLD_INGOT,
        Self::NETHERITE_INGOT,
        Self::NETHERITE_SCRAP,
        Self::WOODEN_SWORD,
        Self::WOODEN_SHOVEL,
        Self::WOODEN_PICKAXE,
        Self::WOODEN_AXE,
        Self::WOODEN_HOE,
        Self::COPPER_SWORD,
        Self::COPPER_SHOVEL,
        Self::COPPER_PICKAXE,
        Self::COPPER_AXE,
        Self::COPPER_HOE,
        Self::STONE_SWORD,
        Self::STONE_SHOVEL,
        Self::STONE_PICKAXE,
        Self::STONE_AXE,
        Self::STONE_HOE,
        Self::GOLDEN_SWORD,
        Self::GOLDEN_SHOVEL,
        Self::GOLDEN_PICKAXE,
        Self::GOLDEN_AXE,
        Self::GOLDEN_HOE,
        Self::IRON_SWORD,
        Self::IRON_SHOVEL,
        Self::IRON_PICKAXE,
        Self::IRON_AXE,
        Self::IRON_HOE,
        Self::DIAMOND_SWORD,
        Self::DIAMOND_SHOVEL,
        Self::DIAMOND_PICKAXE,
        Self::DIAMOND_AXE,
        Self::DIAMOND_HOE,
        Self::NETHERITE_SWORD,
        Self::NETHERITE_SHOVEL,
        Self::NETHERITE_PICKAXE,
        Self::NETHERITE_AXE,
        Self::NETHERITE_HOE,
        Self::STICK,
        Self::MUSHROOM_STEW,
        Self::STRING,
        Self::FEATHER,
        Self::GUNPOWDER,
        Self::WHEAT_SEEDS,
        Self::WHEAT,
        Self::BREAD,
        Self::LEATHER_HELMET,
        Self::LEATHER_CHESTPLATE,
        Self::LEATHER_LEGGINGS,
        Self::LEATHER_BOOTS,
        Self::COPPER_HELMET,
        Self::COPPER_CHESTPLATE,
        Self::COPPER_LEGGINGS,
        Self::COPPER_BOOTS,
        Self::CHAINMAIL_HELMET,
        Self::CHAINMAIL_CHESTPLATE,
        Self::CHAINMAIL_LEGGINGS,
        Self::CHAINMAIL_BOOTS,
        Self::IRON_HELMET,
        Self::IRON_CHESTPLATE,
        Self::IRON_LEGGINGS,
        Self::IRON_BOOTS,
        Self::DIAMOND_HELMET,
        Self::DIAMOND_CHESTPLATE,
        Self::DIAMOND_LEGGINGS,
        Self::DIAMOND_BOOTS,
        Self::GOLDEN_HELMET,
        Self::GOLDEN_CHESTPLATE,
        Self::GOLDEN_LEGGINGS,
        Self::GOLDEN_BOOTS,
        Self::NETHERITE_HELMET,
        Self::NETHERITE_CHESTPLATE,
        Self::NETHERITE_LEGGINGS,
        Self::NETHERITE_BOOTS,
        Self::FLINT,
        Self::PORKCHOP,
        Self::COOKED_PORKCHOP,
        Self::PAINTING,
        Self::GOLDEN_APPLE,
        Self::ENCHANTED_GOLDEN_APPLE,
        Self::OAK_SIGN,
        Self::SPRUCE_SIGN,
        Self::BIRCH_SIGN,
        Self::JUNGLE_SIGN,
        Self::ACACIA_SIGN,
        Self::CHERRY_SIGN,
        Self::DARK_OAK_SIGN,
        Self::PALE_OAK_SIGN,
        Self::MANGROVE_SIGN,
        Self::BAMBOO_SIGN,
        Self::CRIMSON_SIGN,
        Self::WARPED_SIGN,
        Self::OAK_HANGING_SIGN,
        Self::SPRUCE_HANGING_SIGN,
        Self::BIRCH_HANGING_SIGN,
        Self::JUNGLE_HANGING_SIGN,
        Self::ACACIA_HANGING_SIGN,
        Self::CHERRY_HANGING_SIGN,
        Self::DARK_OAK_HANGING_SIGN,
        Self::PALE_OAK_HANGING_SIGN,
        Self::MANGROVE_HANGING_SIGN,
        Self::BAMBOO_HANGING_SIGN,
        Self::CRIMSON_HANGING_SIGN,
        Self::WARPED_HANGING_SIGN,
        Self::BUCKET,
        Self::WATER_BUCKET,
        Self::LAVA_BUCKET,
        Self::POWDER_SNOW_BUCKET,
        Self::SNOWBALL,
        Self::LEATHER,
        Self::MILK_BUCKET,
        Self::PUFFERFISH_BUCKET,
        Self::SALMON_BUCKET,
        Self::COD_BUCKET,
        Self::TROPICAL_FISH_BUCKET,
        Self::AXOLOTL_BUCKET,
        Self::TADPOLE_BUCKET,
        Self::BRICK,
        Self::CLAY_BALL,
        Self::DRIED_KELP_BLOCK,
        Self::PAPER,
        Self::BOOK,
        Self::SLIME_BALL,
        Self::EGG,
        Self::BLUE_EGG,
        Self::BROWN_EGG,
        Self::COMPASS,
        Self::RECOVERY_COMPASS,
        Self::BUNDLE,
        Self::WHITE_BUNDLE,
        Self::ORANGE_BUNDLE,
        Self::MAGENTA_BUNDLE,
        Self::LIGHT_BLUE_BUNDLE,
        Self::YELLOW_BUNDLE,
        Self::LIME_BUNDLE,
        Self::PINK_BUNDLE,
        Self::GRAY_BUNDLE,
        Self::LIGHT_GRAY_BUNDLE,
        Self::CYAN_BUNDLE,
        Self::PURPLE_BUNDLE,
        Self::BLUE_BUNDLE,
        Self::BROWN_BUNDLE,
        Self::GREEN_BUNDLE,
        Self::RED_BUNDLE,
        Self::BLACK_BUNDLE,
        Self::FISHING_ROD,
        Self::CLOCK,
        Self::SPYGLASS,
        Self::GLOWSTONE_DUST,
        Self::COD,
        Self::SALMON,
        Self::TROPICAL_FISH,
        Self::PUFFERFISH,
        Self::COOKED_COD,
        Self::COOKED_SALMON,
        Self::INK_SAC,
        Self::GLOW_INK_SAC,
        Self::COCOA_BEANS,
        Self::WHITE_DYE,
        Self::ORANGE_DYE,
        Self::MAGENTA_DYE,
        Self::LIGHT_BLUE_DYE,
        Self::YELLOW_DYE,
        Self::LIME_DYE,
        Self::PINK_DYE,
        Self::GRAY_DYE,
        Self::LIGHT_GRAY_DYE,
        Self::CYAN_DYE,
        Self::PURPLE_DYE,
        Self::BLUE_DYE,
        Self::BROWN_DYE,
        Self::GREEN_DYE,
        Self::RED_DYE,
        Self::BLACK_DYE,
        Self::BONE_MEAL,
        Self::BONE,
        Self::SUGAR,
        Self::CAKE,
        Self::WHITE_BED,
        Self::ORANGE_BED,
        Self::MAGENTA_BED,
        Self::LIGHT_BLUE_BED,
        Self::YELLOW_BED,
        Self::LIME_BED,
        Self::PINK_BED,
        Self::GRAY_BED,
        Self::LIGHT_GRAY_BED,
        Self::CYAN_BED,
        Self::PURPLE_BED,
        Self::BLUE_BED,
        Self::BROWN_BED,
        Self::GREEN_BED,
        Self::RED_BED,
        Self::BLACK_BED,
        Self::COOKIE,
        Self::CRAFTER,
        Self::FILLED_MAP,
        Self::SHEARS,
        Self::MELON_SLICE,
        Self::DRIED_KELP,
        Self::PUMPKIN_SEEDS,
        Self::MELON_SEEDS,
        Self::BEEF,
        Self::COOKED_BEEF,
        Self::CHICKEN,
        Self::COOKED_CHICKEN,
        Self::ROTTEN_FLESH,
        Self::ENDER_PEARL,
        Self::BLAZE_ROD,
        Self::GHAST_TEAR,
        Self::GOLD_NUGGET,
        Self::NETHER_WART,
        Self::GLASS_BOTTLE,
        Self::POTION,
        Self::SPIDER_EYE,
        Self::FERMENTED_SPIDER_EYE,
        Self::BLAZE_POWDER,
        Self::MAGMA_CREAM,
        Self::BREWING_STAND,
        Self::CAULDRON,
        Self::ENDER_EYE,
        Self::GLISTERING_MELON_SLICE,
        Self::CHICKEN_SPAWN_EGG,
        Self::COW_SPAWN_EGG,
        Self::PIG_SPAWN_EGG,
        Self::SHEEP_SPAWN_EGG,
        Self::CAMEL_SPAWN_EGG,
        Self::DONKEY_SPAWN_EGG,
        Self::HORSE_SPAWN_EGG,
        Self::MULE_SPAWN_EGG,
        Self::CAT_SPAWN_EGG,
        Self::PARROT_SPAWN_EGG,
        Self::WOLF_SPAWN_EGG,
        Self::ARMADILLO_SPAWN_EGG,
        Self::BAT_SPAWN_EGG,
        Self::BEE_SPAWN_EGG,
        Self::FOX_SPAWN_EGG,
        Self::GOAT_SPAWN_EGG,
        Self::LLAMA_SPAWN_EGG,
        Self::OCELOT_SPAWN_EGG,
        Self::PANDA_SPAWN_EGG,
        Self::POLAR_BEAR_SPAWN_EGG,
        Self::RABBIT_SPAWN_EGG,
        Self::AXOLOTL_SPAWN_EGG,
        Self::COD_SPAWN_EGG,
        Self::DOLPHIN_SPAWN_EGG,
        Self::FROG_SPAWN_EGG,
        Self::GLOW_SQUID_SPAWN_EGG,
        Self::NAUTILUS_SPAWN_EGG,
        Self::PUFFERFISH_SPAWN_EGG,
        Self::SALMON_SPAWN_EGG,
        Self::SQUID_SPAWN_EGG,
        Self::TADPOLE_SPAWN_EGG,
        Self::TROPICAL_FISH_SPAWN_EGG,
        Self::TURTLE_SPAWN_EGG,
        Self::ALLAY_SPAWN_EGG,
        Self::MOOSHROOM_SPAWN_EGG,
        Self::SNIFFER_SPAWN_EGG,
        Self::COPPER_GOLEM_SPAWN_EGG,
        Self::IRON_GOLEM_SPAWN_EGG,
        Self::SNOW_GOLEM_SPAWN_EGG,
        Self::TRADER_LLAMA_SPAWN_EGG,
        Self::VILLAGER_SPAWN_EGG,
        Self::WANDERING_TRADER_SPAWN_EGG,
        Self::BOGGED_SPAWN_EGG,
        Self::CAMEL_HUSK_SPAWN_EGG,
        Self::DROWNED_SPAWN_EGG,
        Self::HUSK_SPAWN_EGG,
        Self::PARCHED_SPAWN_EGG,
        Self::SKELETON_SPAWN_EGG,
        Self::SKELETON_HORSE_SPAWN_EGG,
        Self::STRAY_SPAWN_EGG,
        Self::WITHER_SPAWN_EGG,
        Self::WITHER_SKELETON_SPAWN_EGG,
        Self::ZOMBIE_SPAWN_EGG,
        Self::ZOMBIE_HORSE_SPAWN_EGG,
        Self::ZOMBIE_NAUTILUS_SPAWN_EGG,
        Self::ZOMBIE_VILLAGER_SPAWN_EGG,
        Self::CAVE_SPIDER_SPAWN_EGG,
        Self::SPIDER_SPAWN_EGG,
        Self::BREEZE_SPAWN_EGG,
        Self::CREAKING_SPAWN_EGG,
        Self::CREEPER_SPAWN_EGG,
        Self::ELDER_GUARDIAN_SPAWN_EGG,
        Self::GUARDIAN_SPAWN_EGG,
        Self::PHANTOM_SPAWN_EGG,
        Self::SILVERFISH_SPAWN_EGG,
        Self::SLIME_SPAWN_EGG,
        Self::WARDEN_SPAWN_EGG,
        Self::WITCH_SPAWN_EGG,
        Self::EVOKER_SPAWN_EGG,
        Self::PILLAGER_SPAWN_EGG,
        Self::RAVAGER_SPAWN_EGG,
        Self::VINDICATOR_SPAWN_EGG,
        Self::VEX_SPAWN_EGG,
        Self::BLAZE_SPAWN_EGG,
        Self::GHAST_SPAWN_EGG,
        Self::HAPPY_GHAST_SPAWN_EGG,
        Self::HOGLIN_SPAWN_EGG,
        Self::MAGMA_CUBE_SPAWN_EGG,
        Self::PIGLIN_SPAWN_EGG,
        Self::PIGLIN_BRUTE_SPAWN_EGG,
        Self::STRIDER_SPAWN_EGG,
        Self::ZOGLIN_SPAWN_EGG,
        Self::ZOMBIFIED_PIGLIN_SPAWN_EGG,
        Self::ENDER_DRAGON_SPAWN_EGG,
        Self::ENDERMAN_SPAWN_EGG,
        Self::ENDERMITE_SPAWN_EGG,
        Self::SHULKER_SPAWN_EGG,
        Self::EXPERIENCE_BOTTLE,
        Self::FIRE_CHARGE,
        Self::WIND_CHARGE,
        Self::WRITABLE_BOOK,
        Self::WRITTEN_BOOK,
        Self::BREEZE_ROD,
        Self::MACE,
        Self::ITEM_FRAME,
        Self::GLOW_ITEM_FRAME,
        Self::FLOWER_POT,
        Self::CARROT,
        Self::POTATO,
        Self::BAKED_POTATO,
        Self::POISONOUS_POTATO,
        Self::MAP,
        Self::GOLDEN_CARROT,
        Self::SKELETON_SKULL,
        Self::WITHER_SKELETON_SKULL,
        Self::PLAYER_HEAD,
        Self::ZOMBIE_HEAD,
        Self::CREEPER_HEAD,
        Self::DRAGON_HEAD,
        Self::PIGLIN_HEAD,
        Self::NETHER_STAR,
        Self::PUMPKIN_PIE,
        Self::FIREWORK_ROCKET,
        Self::FIREWORK_STAR,
        Self::ENCHANTED_BOOK,
        Self::NETHER_BRICK,
        Self::RESIN_BRICK,
        Self::PRISMARINE_SHARD,
        Self::PRISMARINE_CRYSTALS,
        Self::RABBIT,
        Self::COOKED_RABBIT,
        Self::RABBIT_STEW,
        Self::RABBIT_FOOT,
        Self::RABBIT_HIDE,
        Self::ARMOR_STAND,
        Self::COPPER_HORSE_ARMOR,
        Self::IRON_HORSE_ARMOR,
        Self::GOLDEN_HORSE_ARMOR,
        Self::DIAMOND_HORSE_ARMOR,
        Self::NETHERITE_HORSE_ARMOR,
        Self::LEATHER_HORSE_ARMOR,
        Self::LEAD,
        Self::NAME_TAG,
        Self::COMMAND_BLOCK_MINECART,
        Self::MUTTON,
        Self::COOKED_MUTTON,
        Self::WHITE_BANNER,
        Self::ORANGE_BANNER,
        Self::MAGENTA_BANNER,
        Self::LIGHT_BLUE_BANNER,
        Self::YELLOW_BANNER,
        Self::LIME_BANNER,
        Self::PINK_BANNER,
        Self::GRAY_BANNER,
        Self::LIGHT_GRAY_BANNER,
        Self::CYAN_BANNER,
        Self::PURPLE_BANNER,
        Self::BLUE_BANNER,
        Self::BROWN_BANNER,
        Self::GREEN_BANNER,
        Self::RED_BANNER,
        Self::BLACK_BANNER,
        Self::END_CRYSTAL,
        Self::CHORUS_FRUIT,
        Self::POPPED_CHORUS_FRUIT,
        Self::TORCHFLOWER_SEEDS,
        Self::PITCHER_POD,
        Self::BEETROOT,
        Self::BEETROOT_SEEDS,
        Self::BEETROOT_SOUP,
        Self::DRAGON_BREATH,
        Self::SPLASH_POTION,
        Self::SPECTRAL_ARROW,
        Self::TIPPED_ARROW,
        Self::LINGERING_POTION,
        Self::SHIELD,
        Self::WOODEN_SPEAR,
        Self::STONE_SPEAR,
        Self::COPPER_SPEAR,
        Self::IRON_SPEAR,
        Self::GOLDEN_SPEAR,
        Self::DIAMOND_SPEAR,
        Self::NETHERITE_SPEAR,
        Self::TOTEM_OF_UNDYING,
        Self::SHULKER_SHELL,
        Self::IRON_NUGGET,
        Self::COPPER_NUGGET,
        Self::KNOWLEDGE_BOOK,
        Self::DEBUG_STICK,
        Self::MUSIC_DISC_13,
        Self::MUSIC_DISC_CAT,
        Self::MUSIC_DISC_BLOCKS,
        Self::MUSIC_DISC_CHIRP,
        Self::MUSIC_DISC_CREATOR,
        Self::MUSIC_DISC_CREATOR_MUSIC_BOX,
        Self::MUSIC_DISC_FAR,
        Self::MUSIC_DISC_LAVA_CHICKEN,
        Self::MUSIC_DISC_MALL,
        Self::MUSIC_DISC_MELLOHI,
        Self::MUSIC_DISC_STAL,
        Self::MUSIC_DISC_STRAD,
        Self::MUSIC_DISC_WARD,
        Self::MUSIC_DISC_11,
        Self::MUSIC_DISC_WAIT,
        Self::MUSIC_DISC_OTHERSIDE,
        Self::MUSIC_DISC_RELIC,
        Self::MUSIC_DISC_5,
        Self::MUSIC_DISC_PIGSTEP,
        Self::MUSIC_DISC_PRECIPICE,
        Self::MUSIC_DISC_TEARS,
        Self::DISC_FRAGMENT_5,
        Self::TRIDENT,
        Self::NAUTILUS_SHELL,
        Self::IRON_NAUTILUS_ARMOR,
        Self::GOLDEN_NAUTILUS_ARMOR,
        Self::DIAMOND_NAUTILUS_ARMOR,
        Self::NETHERITE_NAUTILUS_ARMOR,
        Self::COPPER_NAUTILUS_ARMOR,
        Self::HEART_OF_THE_SEA,
        Self::CROSSBOW,
        Self::SUSPICIOUS_STEW,
        Self::LOOM,
        Self::FLOWER_BANNER_PATTERN,
        Self::CREEPER_BANNER_PATTERN,
        Self::SKULL_BANNER_PATTERN,
        Self::MOJANG_BANNER_PATTERN,
        Self::GLOBE_BANNER_PATTERN,
        Self::PIGLIN_BANNER_PATTERN,
        Self::FLOW_BANNER_PATTERN,
        Self::GUSTER_BANNER_PATTERN,
        Self::FIELD_MASONED_BANNER_PATTERN,
        Self::BORDURE_INDENTED_BANNER_PATTERN,
        Self::GOAT_HORN,
        Self::COMPOSTER,
        Self::BARREL,
        Self::SMOKER,
        Self::BLAST_FURNACE,
        Self::CARTOGRAPHY_TABLE,
        Self::FLETCHING_TABLE,
        Self::GRINDSTONE,
        Self::SMITHING_TABLE,
        Self::STONECUTTER,
        Self::BELL,
        Self::LANTERN,
        Self::SOUL_LANTERN,
        Self::COPPER_LANTERN,
        Self::EXPOSED_COPPER_LANTERN,
        Self::WEATHERED_COPPER_LANTERN,
        Self::OXIDIZED_COPPER_LANTERN,
        Self::WAXED_COPPER_LANTERN,
        Self::WAXED_EXPOSED_COPPER_LANTERN,
        Self::WAXED_WEATHERED_COPPER_LANTERN,
        Self::WAXED_OXIDIZED_COPPER_LANTERN,
        Self::SWEET_BERRIES,
        Self::GLOW_BERRIES,
        Self::CAMPFIRE,
        Self::SOUL_CAMPFIRE,
        Self::SHROOMLIGHT,
        Self::HONEYCOMB,
        Self::BEE_NEST,
        Self::BEEHIVE,
        Self::HONEY_BOTTLE,
        Self::HONEYCOMB_BLOCK,
        Self::LODESTONE,
        Self::CRYING_OBSIDIAN,
        Self::BLACKSTONE,
        Self::BLACKSTONE_SLAB,
        Self::BLACKSTONE_STAIRS,
        Self::GILDED_BLACKSTONE,
        Self::POLISHED_BLACKSTONE,
        Self::POLISHED_BLACKSTONE_SLAB,
        Self::POLISHED_BLACKSTONE_STAIRS,
        Self::CHISELED_POLISHED_BLACKSTONE,
        Self::POLISHED_BLACKSTONE_BRICKS,
        Self::POLISHED_BLACKSTONE_BRICK_SLAB,
        Self::POLISHED_BLACKSTONE_BRICK_STAIRS,
        Self::CRACKED_POLISHED_BLACKSTONE_BRICKS,
        Self::RESPAWN_ANCHOR,
        Self::CANDLE,
        Self::WHITE_CANDLE,
        Self::ORANGE_CANDLE,
        Self::MAGENTA_CANDLE,
        Self::LIGHT_BLUE_CANDLE,
        Self::YELLOW_CANDLE,
        Self::LIME_CANDLE,
        Self::PINK_CANDLE,
        Self::GRAY_CANDLE,
        Self::LIGHT_GRAY_CANDLE,
        Self::CYAN_CANDLE,
        Self::PURPLE_CANDLE,
        Self::BLUE_CANDLE,
        Self::BROWN_CANDLE,
        Self::GREEN_CANDLE,
        Self::RED_CANDLE,
        Self::BLACK_CANDLE,
        Self::SMALL_AMETHYST_BUD,
        Self::MEDIUM_AMETHYST_BUD,
        Self::LARGE_AMETHYST_BUD,
        Self::AMETHYST_CLUSTER,
        Self::POINTED_DRIPSTONE,
        Self::OCHRE_FROGLIGHT,
        Self::VERDANT_FROGLIGHT,
        Self::PEARLESCENT_FROGLIGHT,
        Self::FROGSPAWN,
        Self::ECHO_SHARD,
        Self::BRUSH,
        Self::NETHERITE_UPGRADE_SMITHING_TEMPLATE,
        Self::SENTRY_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::DUNE_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::COAST_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::WILD_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::WARD_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::EYE_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::VEX_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::TIDE_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::SNOUT_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::RIB_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::SPIRE_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::WAYFINDER_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::SHAPER_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::SILENCE_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::RAISER_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::HOST_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::FLOW_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::BOLT_ARMOR_TRIM_SMITHING_TEMPLATE,
        Self::ANGLER_POTTERY_SHERD,
        Self::ARCHER_POTTERY_SHERD,
        Self::ARMS_UP_POTTERY_SHERD,
        Self::BLADE_POTTERY_SHERD,
        Self::BREWER_POTTERY_SHERD,
        Self::BURN_POTTERY_SHERD,
        Self::DANGER_POTTERY_SHERD,
        Self::EXPLORER_POTTERY_SHERD,
        Self::FLOW_POTTERY_SHERD,
        Self::FRIEND_POTTERY_SHERD,
        Self::GUSTER_POTTERY_SHERD,
        Self::HEART_POTTERY_SHERD,
        Self::HEARTBREAK_POTTERY_SHERD,
        Self::HOWL_POTTERY_SHERD,
        Self::MINER_POTTERY_SHERD,
        Self::MOURNER_POTTERY_SHERD,
        Self::PLENTY_POTTERY_SHERD,
        Self::PRIZE_POTTERY_SHERD,
        Self::SCRAPE_POTTERY_SHERD,
        Self::SHEAF_POTTERY_SHERD,
        Self::SHELTER_POTTERY_SHERD,
        Self::SKULL_POTTERY_SHERD,
        Self::SNORT_POTTERY_SHERD,
        Self::COPPER_GRATE,
        Self::EXPOSED_COPPER_GRATE,
        Self::WEATHERED_COPPER_GRATE,
        Self::OXIDIZED_COPPER_GRATE,
        Self::WAXED_COPPER_GRATE,
        Self::WAXED_EXPOSED_COPPER_GRATE,
        Self::WAXED_WEATHERED_COPPER_GRATE,
        Self::WAXED_OXIDIZED_COPPER_GRATE,
        Self::COPPER_BULB,
        Self::EXPOSED_COPPER_BULB,
        Self::WEATHERED_COPPER_BULB,
        Self::OXIDIZED_COPPER_BULB,
        Self::WAXED_COPPER_BULB,
        Self::WAXED_EXPOSED_COPPER_BULB,
        Self::WAXED_WEATHERED_COPPER_BULB,
        Self::WAXED_OXIDIZED_COPPER_BULB,
        Self::COPPER_CHEST,
        Self::EXPOSED_COPPER_CHEST,
        Self::WEATHERED_COPPER_CHEST,
        Self::OXIDIZED_COPPER_CHEST,
        Self::WAXED_COPPER_CHEST,
        Self::WAXED_EXPOSED_COPPER_CHEST,
        Self::WAXED_WEATHERED_COPPER_CHEST,
        Self::WAXED_OXIDIZED_COPPER_CHEST,
        Self::COPPER_GOLEM_STATUE,
        Self::EXPOSED_COPPER_GOLEM_STATUE,
        Self::WEATHERED_COPPER_GOLEM_STATUE,
        Self::OXIDIZED_COPPER_GOLEM_STATUE,
        Self::WAXED_COPPER_GOLEM_STATUE,
        Self::WAXED_EXPOSED_COPPER_GOLEM_STATUE,
        Self::WAXED_WEATHERED_COPPER_GOLEM_STATUE,
        Self::WAXED_OXIDIZED_COPPER_GOLEM_STATUE,
        Self::TRIAL_SPAWNER,
        Self::TRIAL_KEY,
        Self::OMINOUS_TRIAL_KEY,
        Self::VAULT,
        Self::OMINOUS_BOTTLE,
    ];
    pub fn from_id(id: i32) -> Option<Self> {
        Self::ALL.iter().find(|material| material.id() == id).cloned()
    }
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.iter().find(|material| material.key().path == key || material.key().to_string() == key).cloned()
    }
}
