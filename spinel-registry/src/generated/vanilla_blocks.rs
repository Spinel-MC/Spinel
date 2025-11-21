use crate::types::Identifier;
use crate::{
    blocks::properties::{self, BlockStateProperties, NoteBlockInstrument},
    blocks::{
        behaviour::{BlockBehaviourProperties, PushReaction},
        offset, Block, BlockRegistry,
    },
};
use std::sync::LazyLock;
pub static AIR: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("air"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_air(true)
            .replaceable(true),
        &[],
    )
});
pub static STONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("stone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static GRANITE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("granite"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POLISHED_GRANITE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("polished_granite"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DIORITE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("diorite"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POLISHED_DIORITE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("polished_diorite"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static ANDESITE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("andesite"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POLISHED_ANDESITE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("polished_andesite"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static GRASS_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("grass_block") , BlockBehaviourProperties :: new () . explosion_resistance (0.6f32) . is_randomly_ticking (true) . destroy_time (0.6f32) , & [& BlockStateProperties :: SNOWY] ,) . with_default_state (offset ! (BlockStateProperties :: SNOWY => BlockStateProperties :: SNOWY . index_of (false)))
});
pub static DIRT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dirt"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32),
        &[],
    )
});
pub static COARSE_DIRT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("coarse_dirt"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32),
        &[],
    )
});
pub static PODZOL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("podzol") , BlockBehaviourProperties :: new () . explosion_resistance (0.5f32) . destroy_time (0.5f32) , & [& BlockStateProperties :: SNOWY] ,) . with_default_state (offset ! (BlockStateProperties :: SNOWY => BlockStateProperties :: SNOWY . index_of (false)))
});
pub static COBBLESTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cobblestone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static OAK_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("oak_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static SPRUCE_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("spruce_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static BIRCH_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("birch_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static JUNGLE_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("jungle_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static ACACIA_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("acacia_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static CHERRY_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cherry_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static DARK_OAK_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dark_oak_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static PALE_OAK_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static PALE_OAK_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pale_oak_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static MANGROVE_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("mangrove_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static BAMBOO_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bamboo_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static BAMBOO_MOSAIC: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bamboo_mosaic"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static OAK_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("oak_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::STAGE],
    )
    .with_default_state(offset ! (BlockStateProperties :: STAGE => 0usize))
});
pub static SPRUCE_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("spruce_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::STAGE],
    )
    .with_default_state(offset ! (BlockStateProperties :: STAGE => 0usize))
});
pub static BIRCH_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("birch_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::STAGE],
    )
    .with_default_state(offset ! (BlockStateProperties :: STAGE => 0usize))
});
pub static JUNGLE_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("jungle_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::STAGE],
    )
    .with_default_state(offset ! (BlockStateProperties :: STAGE => 0usize))
});
pub static ACACIA_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("acacia_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::STAGE],
    )
    .with_default_state(offset ! (BlockStateProperties :: STAGE => 0usize))
});
pub static CHERRY_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cherry_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::STAGE],
    )
    .with_default_state(offset ! (BlockStateProperties :: STAGE => 0usize))
});
pub static DARK_OAK_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dark_oak_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::STAGE],
    )
    .with_default_state(offset ! (BlockStateProperties :: STAGE => 0usize))
});
pub static PALE_OAK_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pale_oak_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::STAGE],
    )
    .with_default_state(offset ! (BlockStateProperties :: STAGE => 0usize))
});
pub static MANGROVE_PROPAGULE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_propagule") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: AGE_4 , & BlockStateProperties :: HANGING , & BlockStateProperties :: STAGE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AGE_4 => 0usize , BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: STAGE => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BEDROCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bedrock"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3600000f32)
            .destroy_time(-1f32)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static WATER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("water"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(100f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(100f32)
            .liquid(true)
            .replaceable(true),
        &[&BlockStateProperties::LEVEL],
    )
    .with_default_state(offset ! (BlockStateProperties :: LEVEL => 0usize))
});
pub static LAVA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lava"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(100f32)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(100f32)
            .liquid(true)
            .replaceable(true),
        &[&BlockStateProperties::LEVEL],
    )
    .with_default_state(offset ! (BlockStateProperties :: LEVEL => 0usize))
});
pub static SAND: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("sand"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static SUSPICIOUS_SAND: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("suspicious_sand"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.25f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.25f32)
            .instrument(NoteBlockInstrument::Snare),
        &[&BlockStateProperties::DUSTED],
    )
    .with_default_state(offset ! (BlockStateProperties :: DUSTED => 0usize))
});
pub static RED_SAND: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_sand"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static GRAVEL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gravel"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.6f32)
            .destroy_time(0.6f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static SUSPICIOUS_GRAVEL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("suspicious_gravel"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.25f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.25f32)
            .instrument(NoteBlockInstrument::Snare),
        &[&BlockStateProperties::DUSTED],
    )
    .with_default_state(offset ! (BlockStateProperties :: DUSTED => 0usize))
});
pub static GOLD_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gold_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_GOLD_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_gold_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(4.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static IRON_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("iron_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_IRON_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_iron_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(4.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static COAL_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("coal_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_COAL_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_coal_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(4.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static NETHER_GOLD_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("nether_gold_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static OAK_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static SPRUCE_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static BIRCH_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static JUNGLE_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static ACACIA_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static CHERRY_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static DARK_OAK_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static PALE_OAK_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static MANGROVE_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static MANGROVE_ROOTS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_roots") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.7f32) . destroy_time (0.7f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MUDDY_MANGROVE_ROOTS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("muddy_mangrove_roots") , BlockBehaviourProperties :: new () . explosion_resistance (0.7f32) . destroy_time (0.7f32) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static BAMBOO_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_block") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_SPRUCE_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_spruce_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_BIRCH_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_birch_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_JUNGLE_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_jungle_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_ACACIA_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_acacia_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_CHERRY_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_cherry_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_DARK_OAK_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_dark_oak_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_PALE_OAK_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_pale_oak_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_OAK_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_oak_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_MANGROVE_LOG: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_mangrove_log") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_BAMBOO_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_bamboo_block") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static OAK_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static SPRUCE_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static BIRCH_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static JUNGLE_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static ACACIA_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static CHERRY_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static DARK_OAK_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static MANGROVE_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_OAK_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_oak_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_SPRUCE_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_spruce_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_BIRCH_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_birch_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_JUNGLE_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_jungle_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_ACACIA_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_acacia_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_CHERRY_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_cherry_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_DARK_OAK_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_dark_oak_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_PALE_OAK_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_pale_oak_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_MANGROVE_WOOD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_mangrove_wood") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static OAK_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPRUCE_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ACACIA_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static AZALEA_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("azalea_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static FLOWERING_AZALEA_LEAVES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("flowering_azalea_leaves") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: DISTANCE , & BlockStateProperties :: PERSISTENT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DISTANCE => 7usize , BlockStateProperties :: PERSISTENT => BlockStateProperties :: PERSISTENT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPONGE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("sponge"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.6f32)
            .destroy_time(0.6f32),
        &[],
    )
});
pub static WET_SPONGE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("wet_sponge"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.6f32)
            .destroy_time(0.6f32),
        &[],
    )
});
pub static GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static LAPIS_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lapis_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_LAPIS_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_lapis_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(4.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static LAPIS_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lapis_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static DISPENSER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dispenser") , BlockBehaviourProperties :: new () . explosion_resistance (3.5f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: TRIGGERED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: TRIGGERED => BlockStateProperties :: TRIGGERED . index_of (false)))
});
pub static SANDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("sandstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CHISELED_SANDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_sandstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CUT_SANDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cut_sandstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static NOTE_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("note_block") , BlockBehaviourProperties :: new () . explosion_resistance (0.8f32) . destroy_time (0.8f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: NOTEBLOCK_INSTRUMENT , & BlockStateProperties :: NOTE , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: NOTEBLOCK_INSTRUMENT => BlockStateProperties :: NOTEBLOCK_INSTRUMENT . get_internal_index_const (& properties :: NoteBlockInstrument :: Harp) , BlockStateProperties :: NOTE => 0usize , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WHITE_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("white_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static ORANGE_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("orange_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static MAGENTA_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("magenta_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static LIGHT_BLUE_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_blue_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static YELLOW_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("yellow_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static LIME_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lime_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static PINK_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pink_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static GRAY_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("gray_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static LIGHT_GRAY_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_gray_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static CYAN_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cyan_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static PURPLE_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purple_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static BLUE_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blue_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static BROWN_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brown_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static GREEN_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("green_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static RED_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static BLACK_BED: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("black_bed") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OCCUPIED , & BlockStateProperties :: BED_PART] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OCCUPIED => BlockStateProperties :: OCCUPIED . index_of (false) , BlockStateProperties :: BED_PART => BlockStateProperties :: BED_PART . get_internal_index_const (& properties :: BedPart :: Foot)))
});
pub static POWERED_RAIL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("powered_rail") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.7f32) . destroy_time (0.7f32) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: RAIL_SHAPE_STRAIGHT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: RAIL_SHAPE_STRAIGHT => BlockStateProperties :: RAIL_SHAPE_STRAIGHT . get_internal_index_const (& properties :: RailShape :: NorthSouth) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DETECTOR_RAIL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("detector_rail") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.7f32) . destroy_time (0.7f32) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: RAIL_SHAPE_STRAIGHT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: RAIL_SHAPE_STRAIGHT => BlockStateProperties :: RAIL_SHAPE_STRAIGHT . get_internal_index_const (& properties :: RailShape :: NorthSouth) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static STICKY_PISTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sticky_piston") , BlockBehaviourProperties :: new () . explosion_resistance (1.5f32) . push_reaction (PushReaction :: Block) . destroy_time (1.5f32) , & [& BlockStateProperties :: EXTENDED , & BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: EXTENDED => BlockStateProperties :: EXTENDED . index_of (false) , BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static COBWEB: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cobweb"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(4f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(4f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static SHORT_GRASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("short_grass"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true)
            .replaceable(true),
        &[],
    )
});
pub static FERN: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("fern"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true)
            .replaceable(true),
        &[],
    )
});
pub static DEAD_BUSH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dead_bush"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true)
            .replaceable(true),
        &[],
    )
});
pub static BUSH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bush"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true)
            .replaceable(true),
        &[],
    )
});
pub static SHORT_DRY_GRASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("short_dry_grass"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true)
            .replaceable(true),
        &[],
    )
});
pub static TALL_DRY_GRASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("tall_dry_grass"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true)
            .replaceable(true),
        &[],
    )
});
pub static SEAGRASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("seagrass"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .replaceable(true),
        &[],
    )
});
pub static TALL_SEAGRASS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tall_seagrass") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . replaceable (true) , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static PISTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("piston") , BlockBehaviourProperties :: new () . explosion_resistance (1.5f32) . push_reaction (PushReaction :: Block) . destroy_time (1.5f32) , & [& BlockStateProperties :: EXTENDED , & BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: EXTENDED => BlockStateProperties :: EXTENDED . index_of (false) , BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static PISTON_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("piston_head") , BlockBehaviourProperties :: new () . explosion_resistance (1.5f32) . push_reaction (PushReaction :: Block) . destroy_time (1.5f32) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: SHORT , & BlockStateProperties :: PISTON_TYPE] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: SHORT => BlockStateProperties :: SHORT . index_of (false) , BlockStateProperties :: PISTON_TYPE => BlockStateProperties :: PISTON_TYPE . get_internal_index_const (& properties :: PistonType :: Normal)))
});
pub static WHITE_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static ORANGE_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static MAGENTA_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magenta_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static LIGHT_BLUE_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_blue_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static YELLOW_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("yellow_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static LIME_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lime_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static PINK_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static GRAY_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gray_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static LIGHT_GRAY_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_gray_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static CYAN_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cyan_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static PURPLE_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purple_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static BLUE_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static BROWN_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static GREEN_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("green_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static RED_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static BLACK_WOOL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("black_wool"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Guitar),
        &[],
    )
});
pub static MOVING_PISTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("moving_piston") , BlockBehaviourProperties :: new () . can_occlude (false) . force_solid_on (true) . push_reaction (PushReaction :: Block) . dynamic_shape (true) . destroy_time (- 1f32) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: PISTON_TYPE] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: PISTON_TYPE => BlockStateProperties :: PISTON_TYPE . get_internal_index_const (& properties :: PistonType :: Normal)))
});
pub static DANDELION: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dandelion"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static TORCHFLOWER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("torchflower"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POPPY: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("poppy"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static BLUE_ORCHID: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_orchid"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static ALLIUM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("allium"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static AZURE_BLUET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("azure_bluet"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static RED_TULIP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_tulip"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static ORANGE_TULIP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_tulip"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static WHITE_TULIP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_tulip"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static PINK_TULIP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_tulip"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static OXEYE_DAISY: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("oxeye_daisy"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static CORNFLOWER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cornflower"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static WITHER_ROSE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("wither_rose"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static LILY_OF_THE_VALLEY: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lily_of_the_valley"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static BROWN_MUSHROOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_mushroom"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static RED_MUSHROOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_mushroom"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static GOLD_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gold_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Bell),
        &[],
    )
});
pub static IRON_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("iron_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::IronXylophone),
        &[],
    )
});
pub static BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static TNT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tnt") , BlockBehaviourProperties :: new () . ignited_by_lava (true) , & [& BlockStateProperties :: UNSTABLE] ,) . with_default_state (offset ! (BlockStateProperties :: UNSTABLE => BlockStateProperties :: UNSTABLE . index_of (false)))
});
pub static BOOKSHELF: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bookshelf"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.5f32)
            .destroy_time(1.5f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static CHISELED_BOOKSHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("chiseled_bookshelf") , BlockBehaviourProperties :: new () . explosion_resistance (1.5f32) . destroy_time (1.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: SLOT_0_OCCUPIED , & BlockStateProperties :: SLOT_1_OCCUPIED , & BlockStateProperties :: SLOT_2_OCCUPIED , & BlockStateProperties :: SLOT_3_OCCUPIED , & BlockStateProperties :: SLOT_4_OCCUPIED , & BlockStateProperties :: SLOT_5_OCCUPIED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: SLOT_0_OCCUPIED => BlockStateProperties :: SLOT_0_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_1_OCCUPIED => BlockStateProperties :: SLOT_1_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_2_OCCUPIED => BlockStateProperties :: SLOT_2_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_3_OCCUPIED => BlockStateProperties :: SLOT_3_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_4_OCCUPIED => BlockStateProperties :: SLOT_4_OCCUPIED . index_of (false) , BlockStateProperties :: SLOT_5_OCCUPIED => BlockStateProperties :: SLOT_5_OCCUPIED . index_of (false)))
});
pub static ACACIA_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CRIMSON_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OAK_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPRUCE_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WARPED_SHELF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_shelf") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: SIDE_CHAIN_PART , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SIDE_CHAIN_PART => BlockStateProperties :: SIDE_CHAIN_PART . get_internal_index_const (& properties :: SideChainPart :: Unconnected) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MOSSY_COBBLESTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("mossy_cobblestone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static OBSIDIAN: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("obsidian"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1200f32)
            .destroy_time(50f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static TORCH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("torch"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static WALL_TORCH: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("wall_torch") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static FIRE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("fire") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . replaceable (true) , & [& BlockStateProperties :: AGE_15 , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: AGE_15 => 0usize , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static SOUL_FIRE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("soul_fire"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .replaceable(true),
        &[],
    )
});
pub static SPAWNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("spawner"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(5f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CREAKING_HEART: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("creaking_heart") , BlockBehaviourProperties :: new () . explosion_resistance (10f32) . destroy_time (10f32) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: CREAKING_HEART_STATE , & BlockStateProperties :: NATURAL] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: CREAKING_HEART_STATE => BlockStateProperties :: CREAKING_HEART_STATE . get_internal_index_const (& properties :: CreakingHeartState :: Uprooted) , BlockStateProperties :: NATURAL => BlockStateProperties :: NATURAL . index_of (false)))
});
pub static OAK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("chest") , BlockBehaviourProperties :: new () . explosion_resistance (2.5f32) . destroy_time (2.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static REDSTONE_WIRE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("redstone_wire") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: EAST_REDSTONE , & BlockStateProperties :: NORTH_REDSTONE , & BlockStateProperties :: POWER , & BlockStateProperties :: SOUTH_REDSTONE , & BlockStateProperties :: WEST_REDSTONE] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_REDSTONE => BlockStateProperties :: EAST_REDSTONE . get_internal_index_const (& properties :: RedstoneSide :: None) , BlockStateProperties :: NORTH_REDSTONE => BlockStateProperties :: NORTH_REDSTONE . get_internal_index_const (& properties :: RedstoneSide :: None) , BlockStateProperties :: POWER => 0usize , BlockStateProperties :: SOUTH_REDSTONE => BlockStateProperties :: SOUTH_REDSTONE . get_internal_index_const (& properties :: RedstoneSide :: None) , BlockStateProperties :: WEST_REDSTONE => BlockStateProperties :: WEST_REDSTONE . get_internal_index_const (& properties :: RedstoneSide :: None)))
});
pub static DIAMOND_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("diamond_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_DIAMOND_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_diamond_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(4.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DIAMOND_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("diamond_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static CRAFTING_TABLE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("crafting_table"),
        BlockBehaviourProperties::new()
            .explosion_resistance(2.5f32)
            .destroy_time(2.5f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static WHEAT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("wheat"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_7],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_7 => 0usize))
});
pub static FARMLAND: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("farmland"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.6f32)
            .is_randomly_ticking(true)
            .destroy_time(0.6f32),
        &[&BlockStateProperties::MOISTURE],
    )
    .with_default_state(offset ! (BlockStateProperties :: MOISTURE => 0usize))
});
pub static FURNACE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("furnace") , BlockBehaviourProperties :: new () . explosion_resistance (3.5f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)))
});
pub static OAK_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPRUCE_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ACACIA_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OAK_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static LADDER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("ladder") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.4f32) . force_solid_off (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.4f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static RAIL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("rail") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.7f32) . destroy_time (0.7f32) , & [& BlockStateProperties :: RAIL_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: RAIL_SHAPE => BlockStateProperties :: RAIL_SHAPE . get_internal_index_const (& properties :: RailShape :: NorthSouth) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COBBLESTONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cobblestone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OAK_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPRUCE_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ACACIA_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OAK_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPRUCE_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ACACIA_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CRIMSON_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WARPED_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OAK_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPRUCE_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ACACIA_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CRIMSON_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WARPED_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_WALL_HANGING_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_wall_hanging_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LEVER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lever") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static STONE_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stone_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static IRON_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("iron_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (5f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static OAK_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static SPRUCE_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BIRCH_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static JUNGLE_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static ACACIA_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CHERRY_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static DARK_OAK_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static PALE_OAK_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static MANGROVE_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BAMBOO_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static REDSTONE_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("redstone_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .is_randomly_ticking(true)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static DEEPSLATE_REDSTONE_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_redstone_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .is_randomly_ticking(true)
            .destroy_time(4.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static REDSTONE_TORCH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("redstone_torch"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (true)),
    )
});
pub static REDSTONE_WALL_TORCH: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("redstone_wall_torch") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (true)))
});
pub static STONE_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stone_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static SNOW: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("snow"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .is_randomly_ticking(true)
            .force_solid_off(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.1f32)
            .requires_correct_tool_for_drops(true)
            .replaceable(true),
        &[&BlockStateProperties::LAYERS],
    )
    .with_default_state(offset ! (BlockStateProperties :: LAYERS => 1usize))
});
pub static ICE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("ice"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.5f32)
            .is_randomly_ticking(true)
            .friction(0.98f32)
            .destroy_time(0.5f32),
        &[],
    )
});
pub static SNOW_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("snow_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.2f32)
            .destroy_time(0.2f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static CACTUS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cactus"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.4f32)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.4f32),
        &[&BlockStateProperties::AGE_15],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_15 => 0usize))
});
pub static CACTUS_FLOWER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cactus_flower"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true),
        &[],
    )
});
pub static CLAY: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("clay"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.6f32)
            .destroy_time(0.6f32)
            .instrument(NoteBlockInstrument::Flute),
        &[],
    )
});
pub static SUGAR_CANE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("sugar_cane"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_15],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_15 => 0usize))
});
pub static JUKEBOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jukebox") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HAS_RECORD] ,) . with_default_state (offset ! (BlockStateProperties :: HAS_RECORD => BlockStateProperties :: HAS_RECORD . index_of (false)))
});
pub static OAK_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static NETHERRACK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("netherrack"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.4f32)
            .destroy_time(0.4f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static SOUL_SAND: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("soul_sand"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .speed_factor(0.4f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::CowBell),
        &[],
    )
});
pub static SOUL_SOIL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("soul_soil"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32),
        &[],
    )
});
pub static BASALT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("basalt") , BlockBehaviourProperties :: new () . explosion_resistance (4.2f32) . destroy_time (1.25f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static POLISHED_BASALT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_basalt") , BlockBehaviourProperties :: new () . explosion_resistance (4.2f32) . destroy_time (1.25f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static SOUL_TORCH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("soul_torch"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static SOUL_WALL_TORCH: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("soul_wall_torch") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static COPPER_TORCH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("copper_torch"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static COPPER_WALL_TORCH: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_wall_torch") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static GLOWSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("glowstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Pling),
        &[],
    )
});
pub static NETHER_PORTAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("nether_portal") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . is_randomly_ticking (true) . push_reaction (PushReaction :: Block) . destroy_time (- 1f32) , & [& BlockStateProperties :: HORIZONTAL_AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_AXIS => BlockStateProperties :: HORIZONTAL_AXIS . get_internal_index_const (& properties :: Axis :: X)))
});
pub static CARVED_PUMPKIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("carved_pumpkin") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static JACK_O_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jack_o_lantern") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::BITES],
    )
    .with_default_state(offset ! (BlockStateProperties :: BITES => 0usize))
});
pub static REPEATER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("repeater") , BlockBehaviourProperties :: new () . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: DELAY , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LOCKED , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: DELAY => 1usize , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LOCKED => BlockStateProperties :: LOCKED . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WHITE_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static ORANGE_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static MAGENTA_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magenta_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static LIGHT_BLUE_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_blue_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static YELLOW_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("yellow_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static LIME_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lime_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static PINK_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static GRAY_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gray_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static LIGHT_GRAY_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_gray_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static CYAN_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cyan_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static PURPLE_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purple_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static BLUE_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static BROWN_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static GREEN_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("green_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static RED_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static BLACK_STAINED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("black_stained_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static OAK_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPRUCE_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ACACIA_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static MOSSY_STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("mossy_stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CRACKED_STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cracked_stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CHISELED_STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PACKED_MUD: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("packed_mud"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(1f32),
        &[],
    )
});
pub static MUD_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("mud_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static INFESTED_STONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("infested_stone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.75f32)
            .destroy_time(0.75f32),
        &[],
    )
});
pub static INFESTED_COBBLESTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("infested_cobblestone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.75f32)
            .destroy_time(1f32),
        &[],
    )
});
pub static INFESTED_STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("infested_stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.75f32)
            .destroy_time(0.75f32),
        &[],
    )
});
pub static INFESTED_MOSSY_STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("infested_mossy_stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.75f32)
            .destroy_time(0.75f32),
        &[],
    )
});
pub static INFESTED_CRACKED_STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("infested_cracked_stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.75f32)
            .destroy_time(0.75f32),
        &[],
    )
});
pub static INFESTED_CHISELED_STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("infested_chiseled_stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.75f32)
            .destroy_time(0.75f32),
        &[],
    )
});
pub static BROWN_MUSHROOM_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brown_mushroom_block") , BlockBehaviourProperties :: new () . explosion_resistance (0.2f32) . destroy_time (0.2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (true) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (true) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (true) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (true) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (true)))
});
pub static RED_MUSHROOM_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_mushroom_block") , BlockBehaviourProperties :: new () . explosion_resistance (0.2f32) . destroy_time (0.2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (true) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (true) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (true) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (true) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (true)))
});
pub static MUSHROOM_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mushroom_stem") , BlockBehaviourProperties :: new () . explosion_resistance (0.2f32) . destroy_time (0.2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (true) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (true) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (true) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (true) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (true)))
});
pub static IRON_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("iron_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static COPPER_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static EXPOSED_COPPER_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static WEATHERED_COPPER_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static OXIDIZED_COPPER_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static WAXED_COPPER_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_BARS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_bars") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static IRON_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("iron_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COPPER_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_COPPER_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_COPPER_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OXIDIZED_COPPER_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_COPPER_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_CHAIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_chain") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: AXIS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static PUMPKIN: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pumpkin"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(1f32)
            .instrument(NoteBlockInstrument::Didgeridoo),
        &[],
    )
});
pub static MELON: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("melon"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(1f32),
        &[],
    )
});
pub static ATTACHED_PUMPKIN_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("attached_pumpkin_stem") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static ATTACHED_MELON_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("attached_melon_stem") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static PUMPKIN_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pumpkin_stem"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_7],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_7 => 0usize))
});
pub static MELON_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("melon_stem"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_7],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_7 => 0usize))
});
pub static VINE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("vine") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.2f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) . replaceable (true) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static GLOW_LICHEN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("glow_lichen") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.2f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) . ignited_by_lava (true) . replaceable (true) , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static RESIN_CLUMP: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("resin_clump") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) . replaceable (true) , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static OAK_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static STONE_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stone_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MUD_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mud_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MYCELIUM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mycelium") , BlockBehaviourProperties :: new () . explosion_resistance (0.6f32) . is_randomly_ticking (true) . destroy_time (0.6f32) , & [& BlockStateProperties :: SNOWY] ,) . with_default_state (offset ! (BlockStateProperties :: SNOWY => BlockStateProperties :: SNOWY . index_of (false)))
});
pub static LILY_PAD: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lily_pad"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static RESIN_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("resin_block"),
        BlockBehaviourProperties::new().instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RESIN_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("resin_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RESIN_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("resin_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static RESIN_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("resin_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static RESIN_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("resin_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static CHISELED_RESIN_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_resin_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static NETHER_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("nether_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static NETHER_BRICK_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("nether_brick_fence") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static NETHER_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("nether_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static NETHER_WART: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("nether_wart"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_3],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_3 => 0usize))
});
pub static ENCHANTING_TABLE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("enchanting_table"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1200f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BREWING_STAND: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brewing_stand") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.5f32) . destroy_time (0.5f32) , & [& BlockStateProperties :: HAS_BOTTLE_0 , & BlockStateProperties :: HAS_BOTTLE_1 , & BlockStateProperties :: HAS_BOTTLE_2] ,) . with_default_state (offset ! (BlockStateProperties :: HAS_BOTTLE_0 => BlockStateProperties :: HAS_BOTTLE_0 . index_of (false) , BlockStateProperties :: HAS_BOTTLE_1 => BlockStateProperties :: HAS_BOTTLE_1 . index_of (false) , BlockStateProperties :: HAS_BOTTLE_2 => BlockStateProperties :: HAS_BOTTLE_2 . index_of (false)))
});
pub static CAULDRON: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cauldron"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(2f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WATER_CAULDRON: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("water_cauldron"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(2f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true),
        &[&BlockStateProperties::LEVEL_CAULDRON],
    )
    .with_default_state(offset ! (BlockStateProperties :: LEVEL_CAULDRON => 1usize))
});
pub static LAVA_CAULDRON: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lava_cauldron"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(2f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static POWDER_SNOW_CAULDRON: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("powder_snow_cauldron"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(2f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true),
        &[&BlockStateProperties::LEVEL_CAULDRON],
    )
    .with_default_state(offset ! (BlockStateProperties :: LEVEL_CAULDRON => 1usize))
});
pub static END_PORTAL: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("end_portal"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(3600000f32)
            .push_reaction(PushReaction::Block)
            .destroy_time(-1f32),
        &[],
    )
});
pub static END_PORTAL_FRAME: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("end_portal_frame") , BlockBehaviourProperties :: new () . explosion_resistance (3600000f32) . destroy_time (- 1f32) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EYE , & BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: EYE => BlockStateProperties :: EYE . index_of (false) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static END_STONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("end_stone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(9f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DRAGON_EGG: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dragon_egg"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(9f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(3f32),
        &[],
    )
});
pub static REDSTONE_LAMP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("redstone_lamp"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static COCOA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cocoa") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) , & [& BlockStateProperties :: AGE_2 , & BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: AGE_2 => 0usize , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static SANDSTONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sandstone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (0.8f32) . destroy_time (0.8f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EMERALD_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("emerald_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_EMERALD_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_emerald_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(4.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static ENDER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("ender_chest") , BlockBehaviourProperties :: new () . explosion_resistance (600f32) . destroy_time (22.5f32) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static TRIPWIRE_HOOK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tripwire_hook") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static TRIPWIRE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tripwire") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: ATTACHED , & BlockStateProperties :: DISARMED , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: POWERED , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACHED => BlockStateProperties :: ATTACHED . index_of (false) , BlockStateProperties :: DISARMED => BlockStateProperties :: DISARMED . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static EMERALD_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("emerald_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Bit),
        &[],
    )
});
pub static SPRUCE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COMMAND_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("command_block") , BlockBehaviourProperties :: new () . explosion_resistance (3600000f32) . destroy_time (- 1f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: CONDITIONAL , & BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: CONDITIONAL => BlockStateProperties :: CONDITIONAL . index_of (false) , BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BEACON: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("beacon"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static COBBLESTONE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cobblestone_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static MOSSY_COBBLESTONE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mossy_cobblestone_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static FLOWER_POT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("flower_pot"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_TORCHFLOWER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_torchflower"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_OAK_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_oak_sapling"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_SPRUCE_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_spruce_sapling"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_BIRCH_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_birch_sapling"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_JUNGLE_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_jungle_sapling"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_ACACIA_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_acacia_sapling"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_CHERRY_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_cherry_sapling"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_DARK_OAK_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_dark_oak_sapling"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_PALE_OAK_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_pale_oak_sapling"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_MANGROVE_PROPAGULE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_mangrove_propagule"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_FERN: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_fern"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_DANDELION: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_dandelion"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_POPPY: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_poppy"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_BLUE_ORCHID: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_blue_orchid"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_ALLIUM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_allium"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_AZURE_BLUET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_azure_bluet"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_RED_TULIP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_red_tulip"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_ORANGE_TULIP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_orange_tulip"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_WHITE_TULIP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_white_tulip"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_PINK_TULIP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_pink_tulip"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_OXEYE_DAISY: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_oxeye_daisy"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_CORNFLOWER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_cornflower"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_LILY_OF_THE_VALLEY: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_lily_of_the_valley"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_WITHER_ROSE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_wither_rose"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_RED_MUSHROOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_red_mushroom"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_BROWN_MUSHROOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_brown_mushroom"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_DEAD_BUSH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_dead_bush"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_CACTUS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_cactus"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static CARROTS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("carrots"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_7],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_7 => 0usize))
});
pub static POTATOES: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potatoes"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_7],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_7 => 0usize))
});
pub static OAK_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static SPRUCE_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BIRCH_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static JUNGLE_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static ACACIA_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CHERRY_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static DARK_OAK_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static PALE_OAK_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static MANGROVE_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BAMBOO_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static SKELETON_SKULL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("skeleton_skull") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Skeleton) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static SKELETON_WALL_SKULL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("skeleton_wall_skull") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WITHER_SKELETON_SKULL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("wither_skeleton_skull") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) . instrument (NoteBlockInstrument :: WitherSkeleton) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static WITHER_SKELETON_WALL_SKULL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("wither_skeleton_wall_skull") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static ZOMBIE_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("zombie_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Zombie) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static ZOMBIE_WALL_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("zombie_wall_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static PLAYER_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("player_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) . instrument (NoteBlockInstrument :: CustomHead) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static PLAYER_WALL_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("player_wall_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CREEPER_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("creeper_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Creeper) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static CREEPER_WALL_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("creeper_wall_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static DRAGON_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dragon_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Dragon) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static DRAGON_WALL_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dragon_wall_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static PIGLIN_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("piglin_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Piglin) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: ROTATION_16] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static PIGLIN_WALL_HEAD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("piglin_wall_head") , BlockBehaviourProperties :: new () . explosion_resistance (1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static ANVIL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("anvil") , BlockBehaviourProperties :: new () . explosion_resistance (1200f32) . push_reaction (PushReaction :: Block) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static CHIPPED_ANVIL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("chipped_anvil") , BlockBehaviourProperties :: new () . explosion_resistance (1200f32) . push_reaction (PushReaction :: Block) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static DAMAGED_ANVIL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("damaged_anvil") , BlockBehaviourProperties :: new () . explosion_resistance (1200f32) . push_reaction (PushReaction :: Block) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static TRAPPED_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("trapped_chest") , BlockBehaviourProperties :: new () . explosion_resistance (2.5f32) . destroy_time (2.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LIGHT_WEIGHTED_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_weighted_pressure_plate"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::POWER],
    )
    .with_default_state(offset ! (BlockStateProperties :: POWER => 0usize))
});
pub static HEAVY_WEIGHTED_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("heavy_weighted_pressure_plate"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::POWER],
    )
    .with_default_state(offset ! (BlockStateProperties :: POWER => 0usize))
});
pub static COMPARATOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("comparator") , BlockBehaviourProperties :: new () . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: MODE_COMPARATOR , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: MODE_COMPARATOR => BlockStateProperties :: MODE_COMPARATOR . get_internal_index_const (& properties :: ComparatorMode :: Compare) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static DAYLIGHT_DETECTOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("daylight_detector") , BlockBehaviourProperties :: new () . explosion_resistance (0.2f32) . destroy_time (0.2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: INVERTED , & BlockStateProperties :: POWER] ,) . with_default_state (offset ! (BlockStateProperties :: INVERTED => BlockStateProperties :: INVERTED . index_of (false) , BlockStateProperties :: POWER => 0usize))
});
pub static REDSTONE_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("redstone_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static NETHER_QUARTZ_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("nether_quartz_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static HOPPER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("hopper") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (4.8f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: ENABLED , & BlockStateProperties :: FACING_HOPPER] ,) . with_default_state (offset ! (BlockStateProperties :: ENABLED => BlockStateProperties :: ENABLED . index_of (true) , BlockStateProperties :: FACING_HOPPER => BlockStateProperties :: FACING_HOPPER . get_internal_index_const (& properties :: Direction :: Down)))
});
pub static QUARTZ_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("quartz_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CHISELED_QUARTZ_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_quartz_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static QUARTZ_PILLAR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("quartz_pillar") , BlockBehaviourProperties :: new () . explosion_resistance (0.8f32) . destroy_time (0.8f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static QUARTZ_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("quartz_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (0.8f32) . destroy_time (0.8f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ACTIVATOR_RAIL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("activator_rail") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.7f32) . destroy_time (0.7f32) , & [& BlockStateProperties :: POWERED , & BlockStateProperties :: RAIL_SHAPE_STRAIGHT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: RAIL_SHAPE_STRAIGHT => BlockStateProperties :: RAIL_SHAPE_STRAIGHT . get_internal_index_const (& properties :: RailShape :: NorthSouth) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DROPPER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dropper") , BlockBehaviourProperties :: new () . explosion_resistance (3.5f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: TRIGGERED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: TRIGGERED => BlockStateProperties :: TRIGGERED . index_of (false)))
});
pub static WHITE_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static ORANGE_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static MAGENTA_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magenta_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static LIGHT_BLUE_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_blue_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static YELLOW_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("yellow_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static LIME_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lime_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PINK_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static GRAY_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gray_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static LIGHT_GRAY_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_gray_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CYAN_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cyan_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PURPLE_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purple_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BLUE_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BROWN_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static GREEN_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("green_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BLACK_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("black_terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static WHITE_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("white_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static ORANGE_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("orange_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static MAGENTA_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("magenta_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static LIGHT_BLUE_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_blue_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static YELLOW_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("yellow_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static LIME_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lime_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static PINK_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pink_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static GRAY_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("gray_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static LIGHT_GRAY_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_gray_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static CYAN_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cyan_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static PURPLE_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purple_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static BLUE_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blue_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static BROWN_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brown_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static GREEN_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("green_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static RED_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static BLACK_STAINED_GLASS_PANE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("black_stained_glass_pane") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.3f32) . destroy_time (0.3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static ACACIA_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_MOSAIC_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_mosaic_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SLIME_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("slime_block"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .friction(0.8f32),
        &[],
    )
});
pub static BARRIER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("barrier") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3600000.8f32) . push_reaction (PushReaction :: Block) . destroy_time (- 1f32) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LIGHT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3600000.8f32) . destroy_time (- 1f32) . replaceable (true) , & [& BlockStateProperties :: LEVEL , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: LEVEL => 15usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static IRON_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("iron_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (5f32) . destroy_time (5f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PRISMARINE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("prismarine"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PRISMARINE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("prismarine_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DARK_PRISMARINE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dark_prismarine"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PRISMARINE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("prismarine_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PRISMARINE_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("prismarine_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_PRISMARINE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_prismarine_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PRISMARINE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("prismarine_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PRISMARINE_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("prismarine_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_PRISMARINE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_prismarine_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SEA_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("sea_lantern"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static HAY_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("hay_block") , BlockBehaviourProperties :: new () . explosion_resistance (0.5f32) . destroy_time (0.5f32) . instrument (NoteBlockInstrument :: Banjo) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static WHITE_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static ORANGE_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static MAGENTA_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magenta_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static LIGHT_BLUE_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_blue_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static YELLOW_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("yellow_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static LIME_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lime_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static PINK_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static GRAY_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gray_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static LIGHT_GRAY_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_gray_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static CYAN_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cyan_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static PURPLE_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purple_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static BLUE_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static BROWN_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static GREEN_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("green_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static RED_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static BLACK_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("black_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("terracotta"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static COAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("coal_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PACKED_ICE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("packed_ice"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .friction(0.98f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Chime),
        &[],
    )
});
pub static SUNFLOWER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sunflower") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static LILAC: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lilac") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static ROSE_BUSH: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("rose_bush") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static PEONY: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("peony") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static TALL_GRASS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tall_grass") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) . replaceable (true) , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static LARGE_FERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("large_fern") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) . replaceable (true) , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static WHITE_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static ORANGE_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static MAGENTA_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magenta_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static LIGHT_BLUE_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_blue_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static YELLOW_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("yellow_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static LIME_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lime_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static PINK_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static GRAY_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gray_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static LIGHT_GRAY_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_gray_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static CYAN_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cyan_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static PURPLE_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purple_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static BLUE_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static BROWN_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static GREEN_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("green_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static RED_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static BLACK_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("black_banner"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .force_solid_on(true)
            .destroy_time(1f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::ROTATION_16],
    )
    .with_default_state(offset ! (BlockStateProperties :: ROTATION_16 => 0usize))
});
pub static WHITE_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("white_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static ORANGE_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("orange_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static MAGENTA_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("magenta_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static LIGHT_BLUE_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_blue_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static YELLOW_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("yellow_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static LIME_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lime_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static PINK_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pink_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static GRAY_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("gray_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static LIGHT_GRAY_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_gray_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static CYAN_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cyan_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static PURPLE_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purple_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BLUE_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blue_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BROWN_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brown_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static GREEN_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("green_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static RED_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BLACK_WALL_BANNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("black_wall_banner") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static RED_SANDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_sandstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CHISELED_RED_SANDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_red_sandstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CUT_RED_SANDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cut_red_sandstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RED_SANDSTONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_sandstone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (0.8f32) . destroy_time (0.8f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OAK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oak_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SPRUCE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIRCH_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static JUNGLE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ACACIA_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CHERRY_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DARK_OAK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_OAK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MANGROVE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BAMBOO_MOSAIC_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_mosaic_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static STONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMOOTH_STONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("smooth_stone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SANDSTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sandstone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CUT_SANDSTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cut_sandstone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PETRIFIED_OAK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("petrified_oak_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COBBLESTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cobblestone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static STONE_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stone_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MUD_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mud_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static NETHER_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("nether_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static QUARTZ_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("quartz_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static RED_SANDSTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_sandstone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CUT_RED_SANDSTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cut_red_sandstone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PURPUR_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purpur_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMOOTH_STONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("smooth_stone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static SMOOTH_SANDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("smooth_sandstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static SMOOTH_QUARTZ: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("smooth_quartz"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static SMOOTH_RED_SANDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("smooth_red_sandstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static SPRUCE_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BIRCH_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static JUNGLE_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static ACACIA_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CHERRY_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static DARK_OAK_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static PALE_OAK_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static MANGROVE_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BAMBOO_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static SPRUCE_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static BIRCH_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static JUNGLE_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static ACACIA_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static CHERRY_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static DARK_OAK_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static PALE_OAK_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static MANGROVE_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static BAMBOO_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static SPRUCE_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("spruce_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BIRCH_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("birch_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static JUNGLE_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jungle_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static ACACIA_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("acacia_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CHERRY_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cherry_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static DARK_OAK_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dark_oak_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static PALE_OAK_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_oak_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static MANGROVE_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mangrove_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static BAMBOO_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static END_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("end_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . force_solid_off (true) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static CHORUS_PLANT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("chorus_plant") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.4f32) . force_solid_off (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.4f32) , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static CHORUS_FLOWER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chorus_flower"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.4f32)
            .is_randomly_ticking(true)
            .force_solid_off(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.4f32),
        &[&BlockStateProperties::AGE_5],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_5 => 0usize))
});
pub static PURPUR_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purpur_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PURPUR_PILLAR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purpur_pillar") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static PURPUR_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purpur_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static END_STONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("end_stone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(9f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static TORCHFLOWER_CROP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("torchflower_crop"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_1],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_1 => 0usize))
});
pub static PITCHER_CROP: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pitcher_crop") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: AGE_4 , & BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: AGE_4 => 0usize , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static PITCHER_PLANT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pitcher_plant") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) , & [& BlockStateProperties :: DOUBLE_BLOCK_HALF] ,) . with_default_state (offset ! (BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower)))
});
pub static BEETROOTS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("beetroots"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_3],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_3 => 0usize))
});
pub static DIRT_PATH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dirt_path"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.65f32)
            .destroy_time(0.65f32),
        &[],
    )
});
pub static END_GATEWAY: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("end_gateway"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(3600000f32)
            .push_reaction(PushReaction::Block)
            .destroy_time(-1f32),
        &[],
    )
});
pub static REPEATING_COMMAND_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("repeating_command_block") , BlockBehaviourProperties :: new () . explosion_resistance (3600000f32) . destroy_time (- 1f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: CONDITIONAL , & BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: CONDITIONAL => BlockStateProperties :: CONDITIONAL . index_of (false) , BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static CHAIN_COMMAND_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("chain_command_block") , BlockBehaviourProperties :: new () . explosion_resistance (3600000f32) . destroy_time (- 1f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: CONDITIONAL , & BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: CONDITIONAL => BlockStateProperties :: CONDITIONAL . index_of (false) , BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static FROSTED_ICE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("frosted_ice"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.5f32)
            .friction(0.98f32)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::AGE_3],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_3 => 0usize))
});
pub static MAGMA_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magma_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static NETHER_WART_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("nether_wart_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1f32)
            .destroy_time(1f32),
        &[],
    )
});
pub static RED_NETHER_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_nether_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BONE_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bone_block") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Xylophone) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRUCTURE_VOID: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("structure_void"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .replaceable(true),
        &[],
    )
});
pub static OBSERVER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("observer") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: South) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static WHITE_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("white_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static ORANGE_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("orange_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static MAGENTA_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("magenta_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static LIGHT_BLUE_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_blue_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static YELLOW_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("yellow_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static LIME_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lime_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static PINK_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pink_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static GRAY_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("gray_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static LIGHT_GRAY_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_gray_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static CYAN_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cyan_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static PURPLE_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purple_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static BLUE_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blue_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static BROWN_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brown_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static GREEN_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("green_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static RED_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static BLACK_SHULKER_BOX: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("black_shulker_box") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (2f32) , & [& BlockStateProperties :: FACING] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up)))
});
pub static WHITE_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("white_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static ORANGE_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("orange_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static MAGENTA_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("magenta_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static LIGHT_BLUE_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_blue_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static YELLOW_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("yellow_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static LIME_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lime_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static PINK_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pink_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static GRAY_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("gray_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static LIGHT_GRAY_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_gray_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static CYAN_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cyan_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static PURPLE_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purple_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BLUE_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blue_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BROWN_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brown_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static GREEN_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("green_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static RED_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BLACK_GLAZED_TERRACOTTA: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("black_glazed_terracotta") , BlockBehaviourProperties :: new () . explosion_resistance (1.4f32) . push_reaction (PushReaction :: PushOnly) . destroy_time (1.4f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static WHITE_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static ORANGE_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static MAGENTA_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magenta_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static LIGHT_BLUE_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_blue_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static YELLOW_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("yellow_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static LIME_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lime_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PINK_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static GRAY_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gray_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static LIGHT_GRAY_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_gray_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CYAN_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cyan_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static PURPLE_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purple_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BLUE_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BROWN_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static GREEN_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("green_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RED_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BLACK_CONCRETE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("black_concrete"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.8f32)
            .destroy_time(1.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static WHITE_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static ORANGE_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static MAGENTA_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magenta_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static LIGHT_BLUE_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_blue_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static YELLOW_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("yellow_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static LIME_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lime_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static PINK_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static GRAY_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gray_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static LIGHT_GRAY_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_gray_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static CYAN_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cyan_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static PURPLE_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purple_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static BLUE_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static BROWN_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static GREEN_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("green_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static RED_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static BLACK_CONCRETE_POWDER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("black_concrete_powder"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32)
            .instrument(NoteBlockInstrument::Snare),
        &[],
    )
});
pub static KELP: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("kelp"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_25],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_25 => 0usize))
});
pub static KELP_PLANT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("kelp_plant"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static DRIED_KELP_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dried_kelp_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(2.5f32)
            .destroy_time(0.5f32),
        &[],
    )
});
pub static TURTLE_EGG: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("turtle_egg"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.5f32)
            .is_randomly_ticking(true)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::EGGS, &BlockStateProperties::HATCH],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: EGGS => 1usize , BlockStateProperties :: HATCH => 0usize),
    )
});
pub static SNIFFER_EGG: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("sniffer_egg"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::HATCH],
    )
    .with_default_state(offset ! (BlockStateProperties :: HATCH => 0usize))
});
pub static DRIED_GHAST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dried_ghast") , BlockBehaviourProperties :: new () . can_occlude (false) . is_randomly_ticking (true) . force_solid_on (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DRIED_GHAST_HYDRATION_LEVELS , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DRIED_GHAST_HYDRATION_LEVELS => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DEAD_TUBE_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dead_tube_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .force_solid_on(true)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEAD_BRAIN_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dead_brain_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .force_solid_on(true)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEAD_BUBBLE_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dead_bubble_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .force_solid_on(true)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEAD_FIRE_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dead_fire_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .force_solid_on(true)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEAD_HORN_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dead_horn_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .force_solid_on(true)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static TUBE_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("tube_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BRAIN_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brain_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BUBBLE_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bubble_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static FIRE_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("fire_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static HORN_CORAL_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("horn_coral_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEAD_TUBE_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_tube_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_BRAIN_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_brain_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_BUBBLE_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_bubble_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_FIRE_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_fire_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_HORN_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_horn_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static TUBE_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tube_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static BRAIN_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brain_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static BUBBLE_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bubble_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static FIRE_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("fire_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static HORN_CORAL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("horn_coral") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_TUBE_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_tube_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_BRAIN_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_brain_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_BUBBLE_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_bubble_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_FIRE_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_fire_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_HORN_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_horn_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static TUBE_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tube_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static BRAIN_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brain_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static BUBBLE_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bubble_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static FIRE_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("fire_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static HORN_CORAL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("horn_coral_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_TUBE_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_tube_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_BRAIN_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_brain_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_BUBBLE_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_bubble_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_FIRE_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_fire_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static DEAD_HORN_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("dead_horn_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . force_solid_on (true) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static TUBE_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tube_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static BRAIN_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brain_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static BUBBLE_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bubble_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static FIRE_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("fire_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static HORN_CORAL_WALL_FAN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("horn_coral_wall_fan") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static SEA_PICKLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sea_pickle") , BlockBehaviourProperties :: new () . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: PICKLES , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: PICKLES => 1usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static BLUE_ICE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_ice"),
        BlockBehaviourProperties::new()
            .explosion_resistance(2.8f32)
            .friction(0.989f32)
            .destroy_time(2.8f32),
        &[],
    )
});
pub static CONDUIT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("conduit") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (3f32) . instrument (NoteBlockInstrument :: Hat) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (true)))
});
pub static BAMBOO_SAPLING: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bamboo_sapling"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .explosion_resistance(1f32)
            .is_randomly_ticking(true)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static BAMBOO: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bamboo") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (1f32) . is_randomly_ticking (true) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (1f32) . ignited_by_lava (true) , & [& BlockStateProperties :: AGE_1 , & BlockStateProperties :: BAMBOO_LEAVES , & BlockStateProperties :: STAGE] ,) . with_default_state (offset ! (BlockStateProperties :: AGE_1 => 0usize , BlockStateProperties :: BAMBOO_LEAVES => BlockStateProperties :: BAMBOO_LEAVES . get_internal_index_const (& properties :: BambooLeaves :: None) , BlockStateProperties :: STAGE => 0usize))
});
pub static POTTED_BAMBOO: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_bamboo"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static VOID_AIR: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("void_air"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_air(true)
            .replaceable(true),
        &[],
    )
});
pub static CAVE_AIR: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cave_air"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_air(true)
            .replaceable(true),
        &[],
    )
});
pub static BUBBLE_COLUMN: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("bubble_column"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .liquid(true)
            .replaceable(true),
        &[&BlockStateProperties::DRAG],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: DRAG => BlockStateProperties :: DRAG . index_of (true)),
    )
});
pub static POLISHED_GRANITE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_granite_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMOOTH_RED_SANDSTONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("smooth_red_sandstone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MOSSY_STONE_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mossy_stone_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_DIORITE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_diorite_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MOSSY_COBBLESTONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mossy_cobblestone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static END_STONE_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("end_stone_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (9f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static STONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMOOTH_SANDSTONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("smooth_sandstone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMOOTH_QUARTZ_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("smooth_quartz_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static GRANITE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("granite_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ANDESITE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("andesite_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static RED_NETHER_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_nether_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_ANDESITE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_andesite_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DIORITE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("diorite_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_GRANITE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_granite_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMOOTH_RED_SANDSTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("smooth_red_sandstone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MOSSY_STONE_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mossy_stone_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_DIORITE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_diorite_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MOSSY_COBBLESTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mossy_cobblestone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static END_STONE_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("end_stone_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (9f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMOOTH_SANDSTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("smooth_sandstone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMOOTH_QUARTZ_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("smooth_quartz_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static GRANITE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("granite_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ANDESITE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("andesite_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static RED_NETHER_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_nether_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_ANDESITE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_andesite_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DIORITE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("diorite_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static PRISMARINE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("prismarine_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static RED_SANDSTONE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_sandstone_wall") , BlockBehaviourProperties :: new () . explosion_resistance (0.8f32) . force_solid_on (true) . destroy_time (0.8f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static MOSSY_STONE_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mossy_stone_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static GRANITE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("granite_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static STONE_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stone_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static MUD_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("mud_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static NETHER_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("nether_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static ANDESITE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("andesite_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static RED_NETHER_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_nether_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static SANDSTONE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sandstone_wall") , BlockBehaviourProperties :: new () . explosion_resistance (0.8f32) . force_solid_on (true) . destroy_time (0.8f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static END_STONE_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("end_stone_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (9f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static DIORITE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("diorite_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static SCAFFOLDING: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("scaffolding") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) , & [& BlockStateProperties :: BOTTOM , & BlockStateProperties :: STABILITY_DISTANCE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: BOTTOM => BlockStateProperties :: BOTTOM . index_of (false) , BlockStateProperties :: STABILITY_DISTANCE => 7usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LOOM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("loom") , BlockBehaviourProperties :: new () . explosion_resistance (2.5f32) . destroy_time (2.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BARREL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("barrel") , BlockBehaviourProperties :: new () . explosion_resistance (2.5f32) . destroy_time (2.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: OPEN] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false)))
});
pub static SMOKER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("smoker") , BlockBehaviourProperties :: new () . explosion_resistance (3.5f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)))
});
pub static BLAST_FURNACE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blast_furnace") , BlockBehaviourProperties :: new () . explosion_resistance (3.5f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)))
});
pub static CARTOGRAPHY_TABLE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cartography_table"),
        BlockBehaviourProperties::new()
            .explosion_resistance(2.5f32)
            .destroy_time(2.5f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static FLETCHING_TABLE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("fletching_table"),
        BlockBehaviourProperties::new()
            .explosion_resistance(2.5f32)
            .destroy_time(2.5f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static GRINDSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("grindstone") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Block) . destroy_time (2f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static LECTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lectern") , BlockBehaviourProperties :: new () . explosion_resistance (2.5f32) . destroy_time (2.5f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HAS_BOOK , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HAS_BOOK => BlockStateProperties :: HAS_BOOK . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static SMITHING_TABLE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("smithing_table"),
        BlockBehaviourProperties::new()
            .explosion_resistance(2.5f32)
            .destroy_time(2.5f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static STONECUTTER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stonecutter") , BlockBehaviourProperties :: new () . explosion_resistance (3.5f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North)))
});
pub static BELL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bell") , BlockBehaviourProperties :: new () . explosion_resistance (5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (5f32) , & [& BlockStateProperties :: BELL_ATTACHMENT , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: BELL_ATTACHMENT => BlockStateProperties :: BELL_ATTACHMENT . get_internal_index_const (& properties :: BellAttachType :: Floor) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SOUL_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("soul_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COPPER_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_COPPER_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_COPPER_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OXIDIZED_COPPER_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_COPPER_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_LANTERN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_lantern") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (3.5f32) , & [& BlockStateProperties :: HANGING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HANGING => BlockStateProperties :: HANGING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CAMPFIRE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("campfire") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT , & BlockStateProperties :: SIGNAL_FIRE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (true) , BlockStateProperties :: SIGNAL_FIRE => BlockStateProperties :: SIGNAL_FIRE . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SOUL_CAMPFIRE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("soul_campfire") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (2f32) . destroy_time (2f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LIT , & BlockStateProperties :: SIGNAL_FIRE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (true) , BlockStateProperties :: SIGNAL_FIRE => BlockStateProperties :: SIGNAL_FIRE . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SWEET_BERRY_BUSH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("sweet_berry_bush"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_3],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_3 => 0usize))
});
pub static WARPED_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_stem") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_WARPED_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_warped_stem") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static WARPED_HYPHAE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_hyphae") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_WARPED_HYPHAE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_warped_hyphae") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static WARPED_NYLIUM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("warped_nylium"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.4f32)
            .is_randomly_ticking(true)
            .destroy_time(0.4f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static WARPED_FUNGUS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("warped_fungus"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static WARPED_WART_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("warped_wart_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1f32)
            .destroy_time(1f32),
        &[],
    )
});
pub static WARPED_ROOTS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("warped_roots"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .replaceable(true),
        &[],
    )
});
pub static NETHER_SPROUTS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("nether_sprouts"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .replaceable(true),
        &[],
    )
});
pub static CRIMSON_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_stem") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_CRIMSON_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_crimson_stem") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static CRIMSON_HYPHAE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_hyphae") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static STRIPPED_CRIMSON_HYPHAE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("stripped_crimson_hyphae") , BlockBehaviourProperties :: new () . explosion_resistance (2f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static CRIMSON_NYLIUM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("crimson_nylium"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.4f32)
            .is_randomly_ticking(true)
            .destroy_time(0.4f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CRIMSON_FUNGUS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("crimson_fungus"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static SHROOMLIGHT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("shroomlight"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1f32)
            .destroy_time(1f32),
        &[],
    )
});
pub static WEEPING_VINES: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("weeping_vines"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_25],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_25 => 0usize))
});
pub static WEEPING_VINES_PLANT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("weeping_vines_plant"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static TWISTING_VINES: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("twisting_vines"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[&BlockStateProperties::AGE_25],
    )
    .with_default_state(offset ! (BlockStateProperties :: AGE_25 => 0usize))
});
pub static TWISTING_VINES_PLANT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("twisting_vines_plant"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static CRIMSON_ROOTS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("crimson_roots"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .replaceable(true),
        &[],
    )
});
pub static CRIMSON_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("crimson_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static WARPED_PLANKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("warped_planks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(2f32)
            .instrument(NoteBlockInstrument::Bass),
        &[],
    )
});
pub static CRIMSON_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WARPED_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_slab") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CRIMSON_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WARPED_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CRIMSON_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static WARPED_FENCE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_fence") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static CRIMSON_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WARPED_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . destroy_time (3f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CRIMSON_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WARPED_FENCE_GATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_fence_gate") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . force_solid_on (true) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: IN_WALL , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: IN_WALL => BlockStateProperties :: IN_WALL . index_of (false) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CRIMSON_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WARPED_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (2f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CRIMSON_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WARPED_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CRIMSON_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WARPED_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static CRIMSON_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WARPED_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: ROTATION_16 , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: ROTATION_16 => 0usize , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CRIMSON_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crimson_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WARPED_WALL_SIGN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("warped_wall_sign") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (1f32) . force_solid_on (true) . destroy_time (1f32) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static STRUCTURE_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("structure_block") , BlockBehaviourProperties :: new () . explosion_resistance (3600000f32) . destroy_time (- 1f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: STRUCTUREBLOCK_MODE] ,) . with_default_state (offset ! (BlockStateProperties :: STRUCTUREBLOCK_MODE => BlockStateProperties :: STRUCTUREBLOCK_MODE . get_internal_index_const (& properties :: StructureMode :: Load)))
});
pub static JIGSAW: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("jigsaw") , BlockBehaviourProperties :: new () . explosion_resistance (3600000f32) . destroy_time (- 1f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: ORIENTATION] ,) . with_default_state (offset ! (BlockStateProperties :: ORIENTATION => BlockStateProperties :: ORIENTATION . get_internal_index_const (& properties :: FrontAndTop :: NorthUp)))
});
pub static TEST_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("test_block") , BlockBehaviourProperties :: new () . explosion_resistance (3600000f32) . destroy_time (- 1f32) , & [& BlockStateProperties :: TEST_BLOCK_MODE] ,) . with_default_state (offset ! (BlockStateProperties :: TEST_BLOCK_MODE => BlockStateProperties :: TEST_BLOCK_MODE . get_internal_index_const (& properties :: TestBlockMode :: Start)))
});
pub static TEST_INSTANCE_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("test_instance_block"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(3600000f32)
            .destroy_time(-1f32),
        &[],
    )
});
pub static COMPOSTER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("composter"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.6f32)
            .destroy_time(0.6f32)
            .ignited_by_lava(true)
            .instrument(NoteBlockInstrument::Bass),
        &[&BlockStateProperties::LEVEL_COMPOSTER],
    )
    .with_default_state(offset ! (BlockStateProperties :: LEVEL_COMPOSTER => 0usize))
});
pub static TARGET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("target"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::POWER],
    )
    .with_default_state(offset ! (BlockStateProperties :: POWER => 0usize))
});
pub static BEE_NEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("bee_nest") , BlockBehaviourProperties :: new () . explosion_resistance (0.3f32) . destroy_time (0.3f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LEVEL_HONEY] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LEVEL_HONEY => 0usize))
});
pub static BEEHIVE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("beehive") , BlockBehaviourProperties :: new () . explosion_resistance (0.6f32) . destroy_time (0.6f32) . ignited_by_lava (true) . instrument (NoteBlockInstrument :: Bass) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: LEVEL_HONEY] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: LEVEL_HONEY => 0usize))
});
pub static HONEY_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("honey_block"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .speed_factor(0.4f32)
            .jump_factor(0.5f32),
        &[],
    )
});
pub static HONEYCOMB_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("honeycomb_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.6f32)
            .destroy_time(0.6f32),
        &[],
    )
});
pub static NETHERITE_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("netherite_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1200f32)
            .destroy_time(50f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static ANCIENT_DEBRIS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("ancient_debris"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1200f32)
            .destroy_time(30f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static CRYING_OBSIDIAN: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("crying_obsidian"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1200f32)
            .destroy_time(50f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RESPAWN_ANCHOR: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("respawn_anchor"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1200f32)
            .destroy_time(50f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[&BlockStateProperties::RESPAWN_ANCHOR_CHARGES],
    )
    .with_default_state(offset ! (BlockStateProperties :: RESPAWN_ANCHOR_CHARGES => 0usize))
});
pub static POTTED_CRIMSON_FUNGUS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_crimson_fungus"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_WARPED_FUNGUS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_warped_fungus"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_CRIMSON_ROOTS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_crimson_roots"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_WARPED_ROOTS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_warped_roots"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static LODESTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lodestone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3.5f32)
            .push_reaction(PushReaction::Block)
            .destroy_time(3.5f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static BLACKSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blackstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static BLACKSTONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blackstone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BLACKSTONE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blackstone_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static BLACKSTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blackstone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_BLACKSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("polished_blackstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POLISHED_BLACKSTONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("polished_blackstone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CRACKED_POLISHED_BLACKSTONE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cracked_polished_blackstone_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CHISELED_POLISHED_BLACKSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_polished_blackstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POLISHED_BLACKSTONE_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_blackstone_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_BLACKSTONE_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_blackstone_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_BLACKSTONE_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_blackstone_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static GILDED_BLACKSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gilded_blackstone"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POLISHED_BLACKSTONE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_blackstone_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_BLACKSTONE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_blackstone_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_BLACKSTONE_PRESSURE_PLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_blackstone_pressure_plate") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static POLISHED_BLACKSTONE_BUTTON: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_blackstone_button") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.5f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.5f32) , & [& BlockStateProperties :: ATTACH_FACE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: ATTACH_FACE => BlockStateProperties :: ATTACH_FACE . get_internal_index_const (& properties :: AttachFace :: Wall) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static POLISHED_BLACKSTONE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_blackstone_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (2f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static CHISELED_NETHER_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_nether_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CRACKED_NETHER_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cracked_nether_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(2f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static QUARTZ_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("quartz_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.8f32)
            .destroy_time(0.8f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WHITE_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("white_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ORANGE_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("orange_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MAGENTA_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("magenta_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LIGHT_BLUE_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_blue_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static YELLOW_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("yellow_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LIME_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lime_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PINK_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pink_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static GRAY_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("gray_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LIGHT_GRAY_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("light_gray_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CYAN_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cyan_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PURPLE_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("purple_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BLUE_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("blue_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BROWN_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("brown_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static GREEN_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("green_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static RED_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("red_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BLACK_CANDLE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("black_candle") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: CANDLES , & BlockStateProperties :: LIT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CANDLES => 1usize , BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static WHITE_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("white_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static ORANGE_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("orange_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static MAGENTA_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("magenta_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static LIGHT_BLUE_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_blue_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static YELLOW_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("yellow_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static LIME_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("lime_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static PINK_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pink_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static GRAY_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("gray_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static LIGHT_GRAY_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("light_gray_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static CYAN_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cyan_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static PURPLE_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("purple_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static BLUE_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("blue_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static BROWN_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("brown_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static GREEN_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("green_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static RED_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("red_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static BLACK_CANDLE_CAKE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("black_candle_cake"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .force_solid_on(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.5f32),
        &[&BlockStateProperties::LIT],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false)),
    )
});
pub static AMETHYST_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("amethyst_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.5f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static BUDDING_AMETHYST: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("budding_amethyst"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1.5f32)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static AMETHYST_CLUSTER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("amethyst_cluster") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (1.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (1.5f32) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LARGE_AMETHYST_BUD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("large_amethyst_bud") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (1.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (1.5f32) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static MEDIUM_AMETHYST_BUD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("medium_amethyst_bud") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (1.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (1.5f32) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMALL_AMETHYST_BUD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("small_amethyst_bud") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (1.5f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (1.5f32) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static TUFF: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("tuff"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static TUFF_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tuff_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static TUFF_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tuff_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static TUFF_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tuff_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static POLISHED_TUFF: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("polished_tuff"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POLISHED_TUFF_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_tuff_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_TUFF_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_tuff_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_TUFF_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_tuff_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static CHISELED_TUFF: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_tuff"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static TUFF_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("tuff_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static TUFF_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tuff_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static TUFF_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tuff_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static TUFF_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("tuff_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (1.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static CHISELED_TUFF_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_tuff_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CALCITE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("calcite"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.75f32)
            .destroy_time(0.75f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static TINTED_GLASS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("tinted_glass"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.3f32)
            .destroy_time(0.3f32)
            .instrument(NoteBlockInstrument::Hat),
        &[],
    )
});
pub static POWDER_SNOW: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("powder_snow"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .explosion_resistance(0.25f32)
            .dynamic_shape(true)
            .destroy_time(0.25f32),
        &[],
    )
});
pub static SCULK_SENSOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sculk_sensor") , BlockBehaviourProperties :: new () . explosion_resistance (1.5f32) . destroy_time (1.5f32) , & [& BlockStateProperties :: POWER , & BlockStateProperties :: SCULK_SENSOR_PHASE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: POWER => 0usize , BlockStateProperties :: SCULK_SENSOR_PHASE => BlockStateProperties :: SCULK_SENSOR_PHASE . get_internal_index_const (& properties :: SculkSensorPhase :: Inactive) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CALIBRATED_SCULK_SENSOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("calibrated_sculk_sensor") , BlockBehaviourProperties :: new () . explosion_resistance (1.5f32) . destroy_time (1.5f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: POWER , & BlockStateProperties :: SCULK_SENSOR_PHASE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: POWER => 0usize , BlockStateProperties :: SCULK_SENSOR_PHASE => BlockStateProperties :: SCULK_SENSOR_PHASE . get_internal_index_const (& properties :: SculkSensorPhase :: Inactive) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SCULK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("sculk"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.2f32)
            .destroy_time(0.2f32),
        &[],
    )
});
pub static SCULK_VEIN: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sculk_vein") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.2f32) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.2f32) , & [& BlockStateProperties :: DOWN , & BlockStateProperties :: EAST , & BlockStateProperties :: NORTH , & BlockStateProperties :: SOUTH , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST] ,) . with_default_state (offset ! (BlockStateProperties :: DOWN => BlockStateProperties :: DOWN . index_of (false) , BlockStateProperties :: EAST => BlockStateProperties :: EAST . index_of (false) , BlockStateProperties :: NORTH => BlockStateProperties :: NORTH . index_of (false) , BlockStateProperties :: SOUTH => BlockStateProperties :: SOUTH . index_of (false) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST => BlockStateProperties :: WEST . index_of (false)))
});
pub static SCULK_CATALYST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sculk_catalyst") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (3f32) , & [& BlockStateProperties :: BLOOM] ,) . with_default_state (offset ! (BlockStateProperties :: BLOOM => BlockStateProperties :: BLOOM . index_of (false)))
});
pub static SCULK_SHRIEKER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("sculk_shrieker") , BlockBehaviourProperties :: new () . explosion_resistance (3f32) . destroy_time (3f32) , & [& BlockStateProperties :: CAN_SUMMON , & BlockStateProperties :: SHRIEKING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CAN_SUMMON => BlockStateProperties :: CAN_SUMMON . index_of (false) , BlockStateProperties :: SHRIEKING => BlockStateProperties :: SHRIEKING . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COPPER_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("copper_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static EXPOSED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("exposed_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WEATHERED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("weathered_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static OXIDIZED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("oxidized_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static COPPER_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("copper_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_COPPER_ORE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_copper_ore"),
        BlockBehaviourProperties::new()
            .explosion_resistance(3f32)
            .destroy_time(4.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static OXIDIZED_CUT_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("oxidized_cut_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WEATHERED_CUT_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("weathered_cut_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static EXPOSED_CUT_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("exposed_cut_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static CUT_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cut_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static OXIDIZED_CHISELED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("oxidized_chiseled_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WEATHERED_CHISELED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("weathered_chiseled_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static EXPOSED_CHISELED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("exposed_chiseled_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static CHISELED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_OXIDIZED_CHISELED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_oxidized_chiseled_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_WEATHERED_CHISELED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_weathered_chiseled_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_EXPOSED_CHISELED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_exposed_chiseled_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_CHISELED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_chiseled_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static OXIDIZED_CUT_COPPER_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_cut_copper_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_CUT_COPPER_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_cut_copper_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_CUT_COPPER_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_cut_copper_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CUT_COPPER_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cut_copper_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OXIDIZED_CUT_COPPER_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_cut_copper_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_CUT_COPPER_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_cut_copper_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_CUT_COPPER_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_cut_copper_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CUT_COPPER_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cut_copper_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_COPPER_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_copper_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_WEATHERED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_weathered_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_EXPOSED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_exposed_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_OXIDIZED_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_oxidized_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_OXIDIZED_CUT_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_oxidized_cut_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_WEATHERED_CUT_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_weathered_cut_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_EXPOSED_CUT_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_exposed_cut_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_CUT_COPPER: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("waxed_cut_copper"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3f32)
            .requires_correct_tool_for_drops(true),
        &[],
    )
});
pub static WAXED_OXIDIZED_CUT_COPPER_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_cut_copper_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_CUT_COPPER_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_cut_copper_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_CUT_COPPER_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_cut_copper_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_CUT_COPPER_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_cut_copper_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_OXIDIZED_CUT_COPPER_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_cut_copper_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_CUT_COPPER_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_cut_copper_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_CUT_COPPER_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_cut_copper_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_CUT_COPPER_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_cut_copper_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COPPER_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static EXPOSED_COPPER_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static OXIDIZED_COPPER_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WEATHERED_COPPER_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WAXED_COPPER_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_DOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_door") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: DOOR_HINGE , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: DOOR_HINGE => BlockStateProperties :: DOOR_HINGE . get_internal_index_const (& properties :: DoorHingeSide :: Left) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static COPPER_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_COPPER_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OXIDIZED_COPPER_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_COPPER_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_COPPER_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_TRAPDOOR: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_trapdoor") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: OPEN , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: OPEN => BlockStateProperties :: OPEN . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COPPER_GRATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_grate") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_COPPER_GRATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_grate") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_COPPER_GRATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_grate") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OXIDIZED_COPPER_GRATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_grate") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_COPPER_GRATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_grate") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_GRATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_grate") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_GRATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_grate") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_GRATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_grate") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COPPER_BULB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_bulb") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static EXPOSED_COPPER_BULB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_bulb") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WEATHERED_COPPER_BULB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_bulb") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static OXIDIZED_COPPER_BULB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_bulb") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WAXED_COPPER_BULB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_bulb") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_BULB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_bulb") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_BULB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_bulb") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_BULB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_bulb") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: LIT , & BlockStateProperties :: POWERED] ,) . with_default_state (offset ! (BlockStateProperties :: LIT => BlockStateProperties :: LIT . index_of (false) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false)))
});
pub static COPPER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_chest") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_COPPER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_chest") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_COPPER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_chest") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OXIDIZED_COPPER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_chest") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_COPPER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_chest") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_chest") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_chest") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_CHEST: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_chest") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: CHEST_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: CHEST_TYPE => BlockStateProperties :: CHEST_TYPE . get_internal_index_const (& properties :: ChestType :: Single) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COPPER_GOLEM_STATUE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("copper_golem_statue") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => BlockStateProperties :: COPPER_GOLEM_POSE . get_internal_index_const (& properties :: Pose :: Standing) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_COPPER_GOLEM_STATUE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_copper_golem_statue") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => BlockStateProperties :: COPPER_GOLEM_POSE . get_internal_index_const (& properties :: Pose :: Standing) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_COPPER_GOLEM_STATUE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_copper_golem_statue") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => BlockStateProperties :: COPPER_GOLEM_POSE . get_internal_index_const (& properties :: Pose :: Standing) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OXIDIZED_COPPER_GOLEM_STATUE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_copper_golem_statue") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => BlockStateProperties :: COPPER_GOLEM_POSE . get_internal_index_const (& properties :: Pose :: Standing) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_COPPER_GOLEM_STATUE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_copper_golem_statue") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => BlockStateProperties :: COPPER_GOLEM_POSE . get_internal_index_const (& properties :: Pose :: Standing) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_COPPER_GOLEM_STATUE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_copper_golem_statue") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => BlockStateProperties :: COPPER_GOLEM_POSE . get_internal_index_const (& properties :: Pose :: Standing) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_COPPER_GOLEM_STATUE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_copper_golem_statue") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => BlockStateProperties :: COPPER_GOLEM_POSE . get_internal_index_const (& properties :: Pose :: Standing) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_OXIDIZED_COPPER_GOLEM_STATUE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_copper_golem_statue") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . push_reaction (PushReaction :: Destroy) . destroy_time (3f32) , & [& BlockStateProperties :: COPPER_GOLEM_POSE , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: COPPER_GOLEM_POSE => BlockStateProperties :: COPPER_GOLEM_POSE . get_internal_index_const (& properties :: Pose :: Standing) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static LIGHTNING_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("lightning_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static EXPOSED_LIGHTNING_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("exposed_lightning_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WEATHERED_LIGHTNING_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("weathered_lightning_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static OXIDIZED_LIGHTNING_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("oxidized_lightning_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_LIGHTNING_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_lightning_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_EXPOSED_LIGHTNING_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_exposed_lightning_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_WEATHERED_LIGHTNING_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_weathered_lightning_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static WAXED_OXIDIZED_LIGHTNING_ROD: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("waxed_oxidized_lightning_rod") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3f32) . requires_correct_tool_for_drops (true) , & [& BlockStateProperties :: FACING , & BlockStateProperties :: POWERED , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: FACING => BlockStateProperties :: FACING . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: POWERED => BlockStateProperties :: POWERED . index_of (false) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POINTED_DRIPSTONE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pointed_dripstone") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (3f32) . is_randomly_ticking (true) . force_solid_on (true) . push_reaction (PushReaction :: Destroy) . dynamic_shape (true) . destroy_time (1.5f32) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: DRIPSTONE_THICKNESS , & BlockStateProperties :: VERTICAL_DIRECTION , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: DRIPSTONE_THICKNESS => BlockStateProperties :: DRIPSTONE_THICKNESS . get_internal_index_const (& properties :: DripstoneThickness :: Tip) , BlockStateProperties :: VERTICAL_DIRECTION => BlockStateProperties :: VERTICAL_DIRECTION . get_internal_index_const (& properties :: Direction :: Up) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DRIPSTONE_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("dripstone_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1f32)
            .destroy_time(1.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CAVE_VINES: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cave_vines") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . is_randomly_ticking (true) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: AGE_25 , & BlockStateProperties :: BERRIES] ,) . with_default_state (offset ! (BlockStateProperties :: AGE_25 => 0usize , BlockStateProperties :: BERRIES => BlockStateProperties :: BERRIES . index_of (false)))
});
pub static CAVE_VINES_PLANT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cave_vines_plant") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: BERRIES] ,) . with_default_state (offset ! (BlockStateProperties :: BERRIES => BlockStateProperties :: BERRIES . index_of (false)))
});
pub static SPORE_BLOSSOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("spore_blossom"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static AZALEA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("azalea"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .force_solid_off(true)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static FLOWERING_AZALEA: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("flowering_azalea"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .force_solid_off(true)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static MOSS_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("moss_carpet"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.1f32),
        &[],
    )
});
pub static PINK_PETALS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pink_petals") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: FLOWER_AMOUNT] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: FLOWER_AMOUNT => 1usize))
});
pub static WILDFLOWERS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("wildflowers") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: FLOWER_AMOUNT] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: FLOWER_AMOUNT => 1usize))
});
pub static LEAF_LITTER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("leaf_litter") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . replaceable (true) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: SEGMENT_AMOUNT] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: SEGMENT_AMOUNT => 1usize))
});
pub static MOSS_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("moss_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.1f32),
        &[],
    )
});
pub static BIG_DRIPLEAF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("big_dripleaf") , BlockBehaviourProperties :: new () . explosion_resistance (0.1f32) . force_solid_off (true) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: TILT , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: TILT => BlockStateProperties :: TILT . get_internal_index_const (& properties :: Tilt :: None) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static BIG_DRIPLEAF_STEM: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("big_dripleaf_stem") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static SMALL_DRIPLEAF: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("small_dripleaf") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: DOUBLE_BLOCK_HALF , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: DOUBLE_BLOCK_HALF => BlockStateProperties :: DOUBLE_BLOCK_HALF . get_internal_index_const (& properties :: DoubleBlockHalf :: Lower) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static HANGING_ROOTS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("hanging_roots") , BlockBehaviourProperties :: new () . has_collision (false) . can_occlude (false) . push_reaction (PushReaction :: Destroy) . ignited_by_lava (true) . replaceable (true) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static ROOTED_DIRT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("rooted_dirt"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32),
        &[],
    )
});
pub static MUD: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("mud"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.5f32)
            .destroy_time(0.5f32),
        &[],
    )
});
pub static DEEPSLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("deepslate") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static COBBLED_DEEPSLATE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cobbled_deepslate"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static COBBLED_DEEPSLATE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cobbled_deepslate_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COBBLED_DEEPSLATE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cobbled_deepslate_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static COBBLED_DEEPSLATE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("cobbled_deepslate_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static POLISHED_DEEPSLATE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("polished_deepslate"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POLISHED_DEEPSLATE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_deepslate_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_DEEPSLATE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_deepslate_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static POLISHED_DEEPSLATE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("polished_deepslate_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static DEEPSLATE_TILES: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_tiles"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_TILE_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("deepslate_tile_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DEEPSLATE_TILE_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("deepslate_tile_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DEEPSLATE_TILE_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("deepslate_tile_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static DEEPSLATE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("deepslate_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DEEPSLATE_BRICK_STAIRS: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("deepslate_brick_stairs") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: HALF , & BlockStateProperties :: STAIRS_SHAPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: HALF => BlockStateProperties :: HALF . get_internal_index_const (& properties :: Half :: Bottom) , BlockStateProperties :: STAIRS_SHAPE => BlockStateProperties :: STAIRS_SHAPE . get_internal_index_const (& properties :: StairsShape :: Straight) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DEEPSLATE_BRICK_SLAB: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("deepslate_brick_slab") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: SLAB_TYPE , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: SLAB_TYPE => BlockStateProperties :: SLAB_TYPE . get_internal_index_const (& properties :: SlabType :: Bottom) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static DEEPSLATE_BRICK_WALL: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("deepslate_brick_wall") , BlockBehaviourProperties :: new () . explosion_resistance (6f32) . force_solid_on (true) . destroy_time (3.5f32) . requires_correct_tool_for_drops (true) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: UP , & BlockStateProperties :: WATERLOGGED , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: UP => BlockStateProperties :: UP . index_of (true) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static CHISELED_DEEPSLATE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("chiseled_deepslate"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CRACKED_DEEPSLATE_BRICKS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cracked_deepslate_bricks"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static CRACKED_DEEPSLATE_TILES: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("cracked_deepslate_tiles"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(3.5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static INFESTED_DEEPSLATE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("infested_deepslate") , BlockBehaviourProperties :: new () . explosion_resistance (0.75f32) . destroy_time (1.5f32) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static SMOOTH_BASALT: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("smooth_basalt"),
        BlockBehaviourProperties::new()
            .explosion_resistance(4.2f32)
            .destroy_time(1.25f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RAW_IRON_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("raw_iron_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RAW_COPPER_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("raw_copper_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static RAW_GOLD_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("raw_gold_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(6f32)
            .destroy_time(5f32)
            .requires_correct_tool_for_drops(true)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static POTTED_AZALEA_BUSH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_azalea_bush"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_FLOWERING_AZALEA_BUSH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_flowering_azalea_bush"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static OCHRE_FROGLIGHT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("ochre_froglight") , BlockBehaviourProperties :: new () . explosion_resistance (0.3f32) . destroy_time (0.3f32) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static VERDANT_FROGLIGHT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("verdant_froglight") , BlockBehaviourProperties :: new () . explosion_resistance (0.3f32) . destroy_time (0.3f32) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static PEARLESCENT_FROGLIGHT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pearlescent_froglight") , BlockBehaviourProperties :: new () . explosion_resistance (0.3f32) . destroy_time (0.3f32) , & [& BlockStateProperties :: AXIS] ,) . with_default_state (offset ! (BlockStateProperties :: AXIS => BlockStateProperties :: AXIS . get_internal_index_const (& properties :: Axis :: Y)))
});
pub static FROGSPAWN: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("frogspawn"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static REINFORCED_DEEPSLATE: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("reinforced_deepslate"),
        BlockBehaviourProperties::new()
            .explosion_resistance(1200f32)
            .destroy_time(55f32)
            .instrument(NoteBlockInstrument::Basedrum),
        &[],
    )
});
pub static DECORATED_POT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("decorated_pot") , BlockBehaviourProperties :: new () . can_occlude (false) . push_reaction (PushReaction :: Destroy) , & [& BlockStateProperties :: CRACKED , & BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: CRACKED => BlockStateProperties :: CRACKED . index_of (false) , BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static CRAFTER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("crafter") , BlockBehaviourProperties :: new () . explosion_resistance (3.5f32) . destroy_time (1.5f32) , & [& BlockStateProperties :: CRAFTING , & BlockStateProperties :: ORIENTATION , & BlockStateProperties :: TRIGGERED] ,) . with_default_state (offset ! (BlockStateProperties :: CRAFTING => BlockStateProperties :: CRAFTING . index_of (false) , BlockStateProperties :: ORIENTATION => BlockStateProperties :: ORIENTATION . get_internal_index_const (& properties :: FrontAndTop :: NorthUp) , BlockStateProperties :: TRIGGERED => BlockStateProperties :: TRIGGERED . index_of (false)))
});
pub static TRIAL_SPAWNER: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("trial_spawner") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (50f32) . destroy_time (50f32) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: OMINOUS , & BlockStateProperties :: TRIAL_SPAWNER_STATE] ,) . with_default_state (offset ! (BlockStateProperties :: OMINOUS => BlockStateProperties :: OMINOUS . index_of (false) , BlockStateProperties :: TRIAL_SPAWNER_STATE => BlockStateProperties :: TRIAL_SPAWNER_STATE . get_internal_index_const (& properties :: TrialSpawnerState :: Inactive)))
});
pub static VAULT: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("vault") , BlockBehaviourProperties :: new () . can_occlude (false) . explosion_resistance (50f32) . destroy_time (50f32) . instrument (NoteBlockInstrument :: Basedrum) , & [& BlockStateProperties :: HORIZONTAL_FACING , & BlockStateProperties :: OMINOUS , & BlockStateProperties :: VAULT_STATE] ,) . with_default_state (offset ! (BlockStateProperties :: HORIZONTAL_FACING => BlockStateProperties :: HORIZONTAL_FACING . get_internal_index_const (& properties :: Direction :: North) , BlockStateProperties :: OMINOUS => BlockStateProperties :: OMINOUS . index_of (false) , BlockStateProperties :: VAULT_STATE => BlockStateProperties :: VAULT_STATE . get_internal_index_const (& properties :: VaultState :: Inactive)))
});
pub static HEAVY_CORE: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("heavy_core") , BlockBehaviourProperties :: new () . explosion_resistance (1200f32) . destroy_time (10f32) . instrument (NoteBlockInstrument :: Snare) , & [& BlockStateProperties :: WATERLOGGED] ,) . with_default_state (offset ! (BlockStateProperties :: WATERLOGGED => BlockStateProperties :: WATERLOGGED . index_of (false)))
});
pub static PALE_MOSS_BLOCK: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pale_moss_block"),
        BlockBehaviourProperties::new()
            .explosion_resistance(0.1f32)
            .push_reaction(PushReaction::Destroy)
            .destroy_time(0.1f32)
            .ignited_by_lava(true),
        &[],
    )
});
pub static PALE_MOSS_CARPET: LazyLock<Block> = LazyLock::new(|| {
    Block :: new (Identifier :: vanilla_static ("pale_moss_carpet") , BlockBehaviourProperties :: new () . explosion_resistance (0.1f32) . push_reaction (PushReaction :: Destroy) . destroy_time (0.1f32) . ignited_by_lava (true) , & [& BlockStateProperties :: BOTTOM , & BlockStateProperties :: EAST_WALL , & BlockStateProperties :: NORTH_WALL , & BlockStateProperties :: SOUTH_WALL , & BlockStateProperties :: WEST_WALL] ,) . with_default_state (offset ! (BlockStateProperties :: BOTTOM => BlockStateProperties :: BOTTOM . index_of (true) , BlockStateProperties :: EAST_WALL => BlockStateProperties :: EAST_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: NORTH_WALL => BlockStateProperties :: NORTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: SOUTH_WALL => BlockStateProperties :: SOUTH_WALL . get_internal_index_const (& properties :: WallSide :: None) , BlockStateProperties :: WEST_WALL => BlockStateProperties :: WEST_WALL . get_internal_index_const (& properties :: WallSide :: None)))
});
pub static PALE_HANGING_MOSS: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("pale_hanging_moss"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true),
        &[&BlockStateProperties::TIP],
    )
    .with_default_state(
        offset ! (BlockStateProperties :: TIP => BlockStateProperties :: TIP . index_of (true)),
    )
});
pub static OPEN_EYEBLOSSOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("open_eyeblossom"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static CLOSED_EYEBLOSSOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("closed_eyeblossom"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_OPEN_EYEBLOSSOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_open_eyeblossom"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static POTTED_CLOSED_EYEBLOSSOM: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("potted_closed_eyeblossom"),
        BlockBehaviourProperties::new()
            .can_occlude(false)
            .is_randomly_ticking(true)
            .push_reaction(PushReaction::Destroy),
        &[],
    )
});
pub static FIREFLY_BUSH: LazyLock<Block> = LazyLock::new(|| {
    Block::new(
        Identifier::vanilla_static("firefly_bush"),
        BlockBehaviourProperties::new()
            .has_collision(false)
            .can_occlude(false)
            .push_reaction(PushReaction::Destroy)
            .ignited_by_lava(true),
        &[],
    )
});
pub fn register_blocks(registry: &mut BlockRegistry) {
    registry.register(&*AIR);
    registry.register(&*STONE);
    registry.register(&*GRANITE);
    registry.register(&*POLISHED_GRANITE);
    registry.register(&*DIORITE);
    registry.register(&*POLISHED_DIORITE);
    registry.register(&*ANDESITE);
    registry.register(&*POLISHED_ANDESITE);
    registry.register(&*GRASS_BLOCK);
    registry.register(&*DIRT);
    registry.register(&*COARSE_DIRT);
    registry.register(&*PODZOL);
    registry.register(&*COBBLESTONE);
    registry.register(&*OAK_PLANKS);
    registry.register(&*SPRUCE_PLANKS);
    registry.register(&*BIRCH_PLANKS);
    registry.register(&*JUNGLE_PLANKS);
    registry.register(&*ACACIA_PLANKS);
    registry.register(&*CHERRY_PLANKS);
    registry.register(&*DARK_OAK_PLANKS);
    registry.register(&*PALE_OAK_WOOD);
    registry.register(&*PALE_OAK_PLANKS);
    registry.register(&*MANGROVE_PLANKS);
    registry.register(&*BAMBOO_PLANKS);
    registry.register(&*BAMBOO_MOSAIC);
    registry.register(&*OAK_SAPLING);
    registry.register(&*SPRUCE_SAPLING);
    registry.register(&*BIRCH_SAPLING);
    registry.register(&*JUNGLE_SAPLING);
    registry.register(&*ACACIA_SAPLING);
    registry.register(&*CHERRY_SAPLING);
    registry.register(&*DARK_OAK_SAPLING);
    registry.register(&*PALE_OAK_SAPLING);
    registry.register(&*MANGROVE_PROPAGULE);
    registry.register(&*BEDROCK);
    registry.register(&*WATER);
    registry.register(&*LAVA);
    registry.register(&*SAND);
    registry.register(&*SUSPICIOUS_SAND);
    registry.register(&*RED_SAND);
    registry.register(&*GRAVEL);
    registry.register(&*SUSPICIOUS_GRAVEL);
    registry.register(&*GOLD_ORE);
    registry.register(&*DEEPSLATE_GOLD_ORE);
    registry.register(&*IRON_ORE);
    registry.register(&*DEEPSLATE_IRON_ORE);
    registry.register(&*COAL_ORE);
    registry.register(&*DEEPSLATE_COAL_ORE);
    registry.register(&*NETHER_GOLD_ORE);
    registry.register(&*OAK_LOG);
    registry.register(&*SPRUCE_LOG);
    registry.register(&*BIRCH_LOG);
    registry.register(&*JUNGLE_LOG);
    registry.register(&*ACACIA_LOG);
    registry.register(&*CHERRY_LOG);
    registry.register(&*DARK_OAK_LOG);
    registry.register(&*PALE_OAK_LOG);
    registry.register(&*MANGROVE_LOG);
    registry.register(&*MANGROVE_ROOTS);
    registry.register(&*MUDDY_MANGROVE_ROOTS);
    registry.register(&*BAMBOO_BLOCK);
    registry.register(&*STRIPPED_SPRUCE_LOG);
    registry.register(&*STRIPPED_BIRCH_LOG);
    registry.register(&*STRIPPED_JUNGLE_LOG);
    registry.register(&*STRIPPED_ACACIA_LOG);
    registry.register(&*STRIPPED_CHERRY_LOG);
    registry.register(&*STRIPPED_DARK_OAK_LOG);
    registry.register(&*STRIPPED_PALE_OAK_LOG);
    registry.register(&*STRIPPED_OAK_LOG);
    registry.register(&*STRIPPED_MANGROVE_LOG);
    registry.register(&*STRIPPED_BAMBOO_BLOCK);
    registry.register(&*OAK_WOOD);
    registry.register(&*SPRUCE_WOOD);
    registry.register(&*BIRCH_WOOD);
    registry.register(&*JUNGLE_WOOD);
    registry.register(&*ACACIA_WOOD);
    registry.register(&*CHERRY_WOOD);
    registry.register(&*DARK_OAK_WOOD);
    registry.register(&*MANGROVE_WOOD);
    registry.register(&*STRIPPED_OAK_WOOD);
    registry.register(&*STRIPPED_SPRUCE_WOOD);
    registry.register(&*STRIPPED_BIRCH_WOOD);
    registry.register(&*STRIPPED_JUNGLE_WOOD);
    registry.register(&*STRIPPED_ACACIA_WOOD);
    registry.register(&*STRIPPED_CHERRY_WOOD);
    registry.register(&*STRIPPED_DARK_OAK_WOOD);
    registry.register(&*STRIPPED_PALE_OAK_WOOD);
    registry.register(&*STRIPPED_MANGROVE_WOOD);
    registry.register(&*OAK_LEAVES);
    registry.register(&*SPRUCE_LEAVES);
    registry.register(&*BIRCH_LEAVES);
    registry.register(&*JUNGLE_LEAVES);
    registry.register(&*ACACIA_LEAVES);
    registry.register(&*CHERRY_LEAVES);
    registry.register(&*DARK_OAK_LEAVES);
    registry.register(&*PALE_OAK_LEAVES);
    registry.register(&*MANGROVE_LEAVES);
    registry.register(&*AZALEA_LEAVES);
    registry.register(&*FLOWERING_AZALEA_LEAVES);
    registry.register(&*SPONGE);
    registry.register(&*WET_SPONGE);
    registry.register(&*GLASS);
    registry.register(&*LAPIS_ORE);
    registry.register(&*DEEPSLATE_LAPIS_ORE);
    registry.register(&*LAPIS_BLOCK);
    registry.register(&*DISPENSER);
    registry.register(&*SANDSTONE);
    registry.register(&*CHISELED_SANDSTONE);
    registry.register(&*CUT_SANDSTONE);
    registry.register(&*NOTE_BLOCK);
    registry.register(&*WHITE_BED);
    registry.register(&*ORANGE_BED);
    registry.register(&*MAGENTA_BED);
    registry.register(&*LIGHT_BLUE_BED);
    registry.register(&*YELLOW_BED);
    registry.register(&*LIME_BED);
    registry.register(&*PINK_BED);
    registry.register(&*GRAY_BED);
    registry.register(&*LIGHT_GRAY_BED);
    registry.register(&*CYAN_BED);
    registry.register(&*PURPLE_BED);
    registry.register(&*BLUE_BED);
    registry.register(&*BROWN_BED);
    registry.register(&*GREEN_BED);
    registry.register(&*RED_BED);
    registry.register(&*BLACK_BED);
    registry.register(&*POWERED_RAIL);
    registry.register(&*DETECTOR_RAIL);
    registry.register(&*STICKY_PISTON);
    registry.register(&*COBWEB);
    registry.register(&*SHORT_GRASS);
    registry.register(&*FERN);
    registry.register(&*DEAD_BUSH);
    registry.register(&*BUSH);
    registry.register(&*SHORT_DRY_GRASS);
    registry.register(&*TALL_DRY_GRASS);
    registry.register(&*SEAGRASS);
    registry.register(&*TALL_SEAGRASS);
    registry.register(&*PISTON);
    registry.register(&*PISTON_HEAD);
    registry.register(&*WHITE_WOOL);
    registry.register(&*ORANGE_WOOL);
    registry.register(&*MAGENTA_WOOL);
    registry.register(&*LIGHT_BLUE_WOOL);
    registry.register(&*YELLOW_WOOL);
    registry.register(&*LIME_WOOL);
    registry.register(&*PINK_WOOL);
    registry.register(&*GRAY_WOOL);
    registry.register(&*LIGHT_GRAY_WOOL);
    registry.register(&*CYAN_WOOL);
    registry.register(&*PURPLE_WOOL);
    registry.register(&*BLUE_WOOL);
    registry.register(&*BROWN_WOOL);
    registry.register(&*GREEN_WOOL);
    registry.register(&*RED_WOOL);
    registry.register(&*BLACK_WOOL);
    registry.register(&*MOVING_PISTON);
    registry.register(&*DANDELION);
    registry.register(&*TORCHFLOWER);
    registry.register(&*POPPY);
    registry.register(&*BLUE_ORCHID);
    registry.register(&*ALLIUM);
    registry.register(&*AZURE_BLUET);
    registry.register(&*RED_TULIP);
    registry.register(&*ORANGE_TULIP);
    registry.register(&*WHITE_TULIP);
    registry.register(&*PINK_TULIP);
    registry.register(&*OXEYE_DAISY);
    registry.register(&*CORNFLOWER);
    registry.register(&*WITHER_ROSE);
    registry.register(&*LILY_OF_THE_VALLEY);
    registry.register(&*BROWN_MUSHROOM);
    registry.register(&*RED_MUSHROOM);
    registry.register(&*GOLD_BLOCK);
    registry.register(&*IRON_BLOCK);
    registry.register(&*BRICKS);
    registry.register(&*TNT);
    registry.register(&*BOOKSHELF);
    registry.register(&*CHISELED_BOOKSHELF);
    registry.register(&*ACACIA_SHELF);
    registry.register(&*BAMBOO_SHELF);
    registry.register(&*BIRCH_SHELF);
    registry.register(&*CHERRY_SHELF);
    registry.register(&*CRIMSON_SHELF);
    registry.register(&*DARK_OAK_SHELF);
    registry.register(&*JUNGLE_SHELF);
    registry.register(&*MANGROVE_SHELF);
    registry.register(&*OAK_SHELF);
    registry.register(&*PALE_OAK_SHELF);
    registry.register(&*SPRUCE_SHELF);
    registry.register(&*WARPED_SHELF);
    registry.register(&*MOSSY_COBBLESTONE);
    registry.register(&*OBSIDIAN);
    registry.register(&*TORCH);
    registry.register(&*WALL_TORCH);
    registry.register(&*FIRE);
    registry.register(&*SOUL_FIRE);
    registry.register(&*SPAWNER);
    registry.register(&*CREAKING_HEART);
    registry.register(&*OAK_STAIRS);
    registry.register(&*CHEST);
    registry.register(&*REDSTONE_WIRE);
    registry.register(&*DIAMOND_ORE);
    registry.register(&*DEEPSLATE_DIAMOND_ORE);
    registry.register(&*DIAMOND_BLOCK);
    registry.register(&*CRAFTING_TABLE);
    registry.register(&*WHEAT);
    registry.register(&*FARMLAND);
    registry.register(&*FURNACE);
    registry.register(&*OAK_SIGN);
    registry.register(&*SPRUCE_SIGN);
    registry.register(&*BIRCH_SIGN);
    registry.register(&*ACACIA_SIGN);
    registry.register(&*CHERRY_SIGN);
    registry.register(&*JUNGLE_SIGN);
    registry.register(&*DARK_OAK_SIGN);
    registry.register(&*PALE_OAK_SIGN);
    registry.register(&*MANGROVE_SIGN);
    registry.register(&*BAMBOO_SIGN);
    registry.register(&*OAK_DOOR);
    registry.register(&*LADDER);
    registry.register(&*RAIL);
    registry.register(&*COBBLESTONE_STAIRS);
    registry.register(&*OAK_WALL_SIGN);
    registry.register(&*SPRUCE_WALL_SIGN);
    registry.register(&*BIRCH_WALL_SIGN);
    registry.register(&*ACACIA_WALL_SIGN);
    registry.register(&*CHERRY_WALL_SIGN);
    registry.register(&*JUNGLE_WALL_SIGN);
    registry.register(&*DARK_OAK_WALL_SIGN);
    registry.register(&*PALE_OAK_WALL_SIGN);
    registry.register(&*MANGROVE_WALL_SIGN);
    registry.register(&*BAMBOO_WALL_SIGN);
    registry.register(&*OAK_HANGING_SIGN);
    registry.register(&*SPRUCE_HANGING_SIGN);
    registry.register(&*BIRCH_HANGING_SIGN);
    registry.register(&*ACACIA_HANGING_SIGN);
    registry.register(&*CHERRY_HANGING_SIGN);
    registry.register(&*JUNGLE_HANGING_SIGN);
    registry.register(&*DARK_OAK_HANGING_SIGN);
    registry.register(&*PALE_OAK_HANGING_SIGN);
    registry.register(&*CRIMSON_HANGING_SIGN);
    registry.register(&*WARPED_HANGING_SIGN);
    registry.register(&*MANGROVE_HANGING_SIGN);
    registry.register(&*BAMBOO_HANGING_SIGN);
    registry.register(&*OAK_WALL_HANGING_SIGN);
    registry.register(&*SPRUCE_WALL_HANGING_SIGN);
    registry.register(&*BIRCH_WALL_HANGING_SIGN);
    registry.register(&*ACACIA_WALL_HANGING_SIGN);
    registry.register(&*CHERRY_WALL_HANGING_SIGN);
    registry.register(&*JUNGLE_WALL_HANGING_SIGN);
    registry.register(&*DARK_OAK_WALL_HANGING_SIGN);
    registry.register(&*PALE_OAK_WALL_HANGING_SIGN);
    registry.register(&*MANGROVE_WALL_HANGING_SIGN);
    registry.register(&*CRIMSON_WALL_HANGING_SIGN);
    registry.register(&*WARPED_WALL_HANGING_SIGN);
    registry.register(&*BAMBOO_WALL_HANGING_SIGN);
    registry.register(&*LEVER);
    registry.register(&*STONE_PRESSURE_PLATE);
    registry.register(&*IRON_DOOR);
    registry.register(&*OAK_PRESSURE_PLATE);
    registry.register(&*SPRUCE_PRESSURE_PLATE);
    registry.register(&*BIRCH_PRESSURE_PLATE);
    registry.register(&*JUNGLE_PRESSURE_PLATE);
    registry.register(&*ACACIA_PRESSURE_PLATE);
    registry.register(&*CHERRY_PRESSURE_PLATE);
    registry.register(&*DARK_OAK_PRESSURE_PLATE);
    registry.register(&*PALE_OAK_PRESSURE_PLATE);
    registry.register(&*MANGROVE_PRESSURE_PLATE);
    registry.register(&*BAMBOO_PRESSURE_PLATE);
    registry.register(&*REDSTONE_ORE);
    registry.register(&*DEEPSLATE_REDSTONE_ORE);
    registry.register(&*REDSTONE_TORCH);
    registry.register(&*REDSTONE_WALL_TORCH);
    registry.register(&*STONE_BUTTON);
    registry.register(&*SNOW);
    registry.register(&*ICE);
    registry.register(&*SNOW_BLOCK);
    registry.register(&*CACTUS);
    registry.register(&*CACTUS_FLOWER);
    registry.register(&*CLAY);
    registry.register(&*SUGAR_CANE);
    registry.register(&*JUKEBOX);
    registry.register(&*OAK_FENCE);
    registry.register(&*NETHERRACK);
    registry.register(&*SOUL_SAND);
    registry.register(&*SOUL_SOIL);
    registry.register(&*BASALT);
    registry.register(&*POLISHED_BASALT);
    registry.register(&*SOUL_TORCH);
    registry.register(&*SOUL_WALL_TORCH);
    registry.register(&*COPPER_TORCH);
    registry.register(&*COPPER_WALL_TORCH);
    registry.register(&*GLOWSTONE);
    registry.register(&*NETHER_PORTAL);
    registry.register(&*CARVED_PUMPKIN);
    registry.register(&*JACK_O_LANTERN);
    registry.register(&*CAKE);
    registry.register(&*REPEATER);
    registry.register(&*WHITE_STAINED_GLASS);
    registry.register(&*ORANGE_STAINED_GLASS);
    registry.register(&*MAGENTA_STAINED_GLASS);
    registry.register(&*LIGHT_BLUE_STAINED_GLASS);
    registry.register(&*YELLOW_STAINED_GLASS);
    registry.register(&*LIME_STAINED_GLASS);
    registry.register(&*PINK_STAINED_GLASS);
    registry.register(&*GRAY_STAINED_GLASS);
    registry.register(&*LIGHT_GRAY_STAINED_GLASS);
    registry.register(&*CYAN_STAINED_GLASS);
    registry.register(&*PURPLE_STAINED_GLASS);
    registry.register(&*BLUE_STAINED_GLASS);
    registry.register(&*BROWN_STAINED_GLASS);
    registry.register(&*GREEN_STAINED_GLASS);
    registry.register(&*RED_STAINED_GLASS);
    registry.register(&*BLACK_STAINED_GLASS);
    registry.register(&*OAK_TRAPDOOR);
    registry.register(&*SPRUCE_TRAPDOOR);
    registry.register(&*BIRCH_TRAPDOOR);
    registry.register(&*JUNGLE_TRAPDOOR);
    registry.register(&*ACACIA_TRAPDOOR);
    registry.register(&*CHERRY_TRAPDOOR);
    registry.register(&*DARK_OAK_TRAPDOOR);
    registry.register(&*PALE_OAK_TRAPDOOR);
    registry.register(&*MANGROVE_TRAPDOOR);
    registry.register(&*BAMBOO_TRAPDOOR);
    registry.register(&*STONE_BRICKS);
    registry.register(&*MOSSY_STONE_BRICKS);
    registry.register(&*CRACKED_STONE_BRICKS);
    registry.register(&*CHISELED_STONE_BRICKS);
    registry.register(&*PACKED_MUD);
    registry.register(&*MUD_BRICKS);
    registry.register(&*INFESTED_STONE);
    registry.register(&*INFESTED_COBBLESTONE);
    registry.register(&*INFESTED_STONE_BRICKS);
    registry.register(&*INFESTED_MOSSY_STONE_BRICKS);
    registry.register(&*INFESTED_CRACKED_STONE_BRICKS);
    registry.register(&*INFESTED_CHISELED_STONE_BRICKS);
    registry.register(&*BROWN_MUSHROOM_BLOCK);
    registry.register(&*RED_MUSHROOM_BLOCK);
    registry.register(&*MUSHROOM_STEM);
    registry.register(&*IRON_BARS);
    registry.register(&*COPPER_BARS);
    registry.register(&*EXPOSED_COPPER_BARS);
    registry.register(&*WEATHERED_COPPER_BARS);
    registry.register(&*OXIDIZED_COPPER_BARS);
    registry.register(&*WAXED_COPPER_BARS);
    registry.register(&*WAXED_EXPOSED_COPPER_BARS);
    registry.register(&*WAXED_WEATHERED_COPPER_BARS);
    registry.register(&*WAXED_OXIDIZED_COPPER_BARS);
    registry.register(&*IRON_CHAIN);
    registry.register(&*COPPER_CHAIN);
    registry.register(&*EXPOSED_COPPER_CHAIN);
    registry.register(&*WEATHERED_COPPER_CHAIN);
    registry.register(&*OXIDIZED_COPPER_CHAIN);
    registry.register(&*WAXED_COPPER_CHAIN);
    registry.register(&*WAXED_EXPOSED_COPPER_CHAIN);
    registry.register(&*WAXED_WEATHERED_COPPER_CHAIN);
    registry.register(&*WAXED_OXIDIZED_COPPER_CHAIN);
    registry.register(&*GLASS_PANE);
    registry.register(&*PUMPKIN);
    registry.register(&*MELON);
    registry.register(&*ATTACHED_PUMPKIN_STEM);
    registry.register(&*ATTACHED_MELON_STEM);
    registry.register(&*PUMPKIN_STEM);
    registry.register(&*MELON_STEM);
    registry.register(&*VINE);
    registry.register(&*GLOW_LICHEN);
    registry.register(&*RESIN_CLUMP);
    registry.register(&*OAK_FENCE_GATE);
    registry.register(&*BRICK_STAIRS);
    registry.register(&*STONE_BRICK_STAIRS);
    registry.register(&*MUD_BRICK_STAIRS);
    registry.register(&*MYCELIUM);
    registry.register(&*LILY_PAD);
    registry.register(&*RESIN_BLOCK);
    registry.register(&*RESIN_BRICKS);
    registry.register(&*RESIN_BRICK_STAIRS);
    registry.register(&*RESIN_BRICK_SLAB);
    registry.register(&*RESIN_BRICK_WALL);
    registry.register(&*CHISELED_RESIN_BRICKS);
    registry.register(&*NETHER_BRICKS);
    registry.register(&*NETHER_BRICK_FENCE);
    registry.register(&*NETHER_BRICK_STAIRS);
    registry.register(&*NETHER_WART);
    registry.register(&*ENCHANTING_TABLE);
    registry.register(&*BREWING_STAND);
    registry.register(&*CAULDRON);
    registry.register(&*WATER_CAULDRON);
    registry.register(&*LAVA_CAULDRON);
    registry.register(&*POWDER_SNOW_CAULDRON);
    registry.register(&*END_PORTAL);
    registry.register(&*END_PORTAL_FRAME);
    registry.register(&*END_STONE);
    registry.register(&*DRAGON_EGG);
    registry.register(&*REDSTONE_LAMP);
    registry.register(&*COCOA);
    registry.register(&*SANDSTONE_STAIRS);
    registry.register(&*EMERALD_ORE);
    registry.register(&*DEEPSLATE_EMERALD_ORE);
    registry.register(&*ENDER_CHEST);
    registry.register(&*TRIPWIRE_HOOK);
    registry.register(&*TRIPWIRE);
    registry.register(&*EMERALD_BLOCK);
    registry.register(&*SPRUCE_STAIRS);
    registry.register(&*BIRCH_STAIRS);
    registry.register(&*JUNGLE_STAIRS);
    registry.register(&*COMMAND_BLOCK);
    registry.register(&*BEACON);
    registry.register(&*COBBLESTONE_WALL);
    registry.register(&*MOSSY_COBBLESTONE_WALL);
    registry.register(&*FLOWER_POT);
    registry.register(&*POTTED_TORCHFLOWER);
    registry.register(&*POTTED_OAK_SAPLING);
    registry.register(&*POTTED_SPRUCE_SAPLING);
    registry.register(&*POTTED_BIRCH_SAPLING);
    registry.register(&*POTTED_JUNGLE_SAPLING);
    registry.register(&*POTTED_ACACIA_SAPLING);
    registry.register(&*POTTED_CHERRY_SAPLING);
    registry.register(&*POTTED_DARK_OAK_SAPLING);
    registry.register(&*POTTED_PALE_OAK_SAPLING);
    registry.register(&*POTTED_MANGROVE_PROPAGULE);
    registry.register(&*POTTED_FERN);
    registry.register(&*POTTED_DANDELION);
    registry.register(&*POTTED_POPPY);
    registry.register(&*POTTED_BLUE_ORCHID);
    registry.register(&*POTTED_ALLIUM);
    registry.register(&*POTTED_AZURE_BLUET);
    registry.register(&*POTTED_RED_TULIP);
    registry.register(&*POTTED_ORANGE_TULIP);
    registry.register(&*POTTED_WHITE_TULIP);
    registry.register(&*POTTED_PINK_TULIP);
    registry.register(&*POTTED_OXEYE_DAISY);
    registry.register(&*POTTED_CORNFLOWER);
    registry.register(&*POTTED_LILY_OF_THE_VALLEY);
    registry.register(&*POTTED_WITHER_ROSE);
    registry.register(&*POTTED_RED_MUSHROOM);
    registry.register(&*POTTED_BROWN_MUSHROOM);
    registry.register(&*POTTED_DEAD_BUSH);
    registry.register(&*POTTED_CACTUS);
    registry.register(&*CARROTS);
    registry.register(&*POTATOES);
    registry.register(&*OAK_BUTTON);
    registry.register(&*SPRUCE_BUTTON);
    registry.register(&*BIRCH_BUTTON);
    registry.register(&*JUNGLE_BUTTON);
    registry.register(&*ACACIA_BUTTON);
    registry.register(&*CHERRY_BUTTON);
    registry.register(&*DARK_OAK_BUTTON);
    registry.register(&*PALE_OAK_BUTTON);
    registry.register(&*MANGROVE_BUTTON);
    registry.register(&*BAMBOO_BUTTON);
    registry.register(&*SKELETON_SKULL);
    registry.register(&*SKELETON_WALL_SKULL);
    registry.register(&*WITHER_SKELETON_SKULL);
    registry.register(&*WITHER_SKELETON_WALL_SKULL);
    registry.register(&*ZOMBIE_HEAD);
    registry.register(&*ZOMBIE_WALL_HEAD);
    registry.register(&*PLAYER_HEAD);
    registry.register(&*PLAYER_WALL_HEAD);
    registry.register(&*CREEPER_HEAD);
    registry.register(&*CREEPER_WALL_HEAD);
    registry.register(&*DRAGON_HEAD);
    registry.register(&*DRAGON_WALL_HEAD);
    registry.register(&*PIGLIN_HEAD);
    registry.register(&*PIGLIN_WALL_HEAD);
    registry.register(&*ANVIL);
    registry.register(&*CHIPPED_ANVIL);
    registry.register(&*DAMAGED_ANVIL);
    registry.register(&*TRAPPED_CHEST);
    registry.register(&*LIGHT_WEIGHTED_PRESSURE_PLATE);
    registry.register(&*HEAVY_WEIGHTED_PRESSURE_PLATE);
    registry.register(&*COMPARATOR);
    registry.register(&*DAYLIGHT_DETECTOR);
    registry.register(&*REDSTONE_BLOCK);
    registry.register(&*NETHER_QUARTZ_ORE);
    registry.register(&*HOPPER);
    registry.register(&*QUARTZ_BLOCK);
    registry.register(&*CHISELED_QUARTZ_BLOCK);
    registry.register(&*QUARTZ_PILLAR);
    registry.register(&*QUARTZ_STAIRS);
    registry.register(&*ACTIVATOR_RAIL);
    registry.register(&*DROPPER);
    registry.register(&*WHITE_TERRACOTTA);
    registry.register(&*ORANGE_TERRACOTTA);
    registry.register(&*MAGENTA_TERRACOTTA);
    registry.register(&*LIGHT_BLUE_TERRACOTTA);
    registry.register(&*YELLOW_TERRACOTTA);
    registry.register(&*LIME_TERRACOTTA);
    registry.register(&*PINK_TERRACOTTA);
    registry.register(&*GRAY_TERRACOTTA);
    registry.register(&*LIGHT_GRAY_TERRACOTTA);
    registry.register(&*CYAN_TERRACOTTA);
    registry.register(&*PURPLE_TERRACOTTA);
    registry.register(&*BLUE_TERRACOTTA);
    registry.register(&*BROWN_TERRACOTTA);
    registry.register(&*GREEN_TERRACOTTA);
    registry.register(&*RED_TERRACOTTA);
    registry.register(&*BLACK_TERRACOTTA);
    registry.register(&*WHITE_STAINED_GLASS_PANE);
    registry.register(&*ORANGE_STAINED_GLASS_PANE);
    registry.register(&*MAGENTA_STAINED_GLASS_PANE);
    registry.register(&*LIGHT_BLUE_STAINED_GLASS_PANE);
    registry.register(&*YELLOW_STAINED_GLASS_PANE);
    registry.register(&*LIME_STAINED_GLASS_PANE);
    registry.register(&*PINK_STAINED_GLASS_PANE);
    registry.register(&*GRAY_STAINED_GLASS_PANE);
    registry.register(&*LIGHT_GRAY_STAINED_GLASS_PANE);
    registry.register(&*CYAN_STAINED_GLASS_PANE);
    registry.register(&*PURPLE_STAINED_GLASS_PANE);
    registry.register(&*BLUE_STAINED_GLASS_PANE);
    registry.register(&*BROWN_STAINED_GLASS_PANE);
    registry.register(&*GREEN_STAINED_GLASS_PANE);
    registry.register(&*RED_STAINED_GLASS_PANE);
    registry.register(&*BLACK_STAINED_GLASS_PANE);
    registry.register(&*ACACIA_STAIRS);
    registry.register(&*CHERRY_STAIRS);
    registry.register(&*DARK_OAK_STAIRS);
    registry.register(&*PALE_OAK_STAIRS);
    registry.register(&*MANGROVE_STAIRS);
    registry.register(&*BAMBOO_STAIRS);
    registry.register(&*BAMBOO_MOSAIC_STAIRS);
    registry.register(&*SLIME_BLOCK);
    registry.register(&*BARRIER);
    registry.register(&*LIGHT);
    registry.register(&*IRON_TRAPDOOR);
    registry.register(&*PRISMARINE);
    registry.register(&*PRISMARINE_BRICKS);
    registry.register(&*DARK_PRISMARINE);
    registry.register(&*PRISMARINE_STAIRS);
    registry.register(&*PRISMARINE_BRICK_STAIRS);
    registry.register(&*DARK_PRISMARINE_STAIRS);
    registry.register(&*PRISMARINE_SLAB);
    registry.register(&*PRISMARINE_BRICK_SLAB);
    registry.register(&*DARK_PRISMARINE_SLAB);
    registry.register(&*SEA_LANTERN);
    registry.register(&*HAY_BLOCK);
    registry.register(&*WHITE_CARPET);
    registry.register(&*ORANGE_CARPET);
    registry.register(&*MAGENTA_CARPET);
    registry.register(&*LIGHT_BLUE_CARPET);
    registry.register(&*YELLOW_CARPET);
    registry.register(&*LIME_CARPET);
    registry.register(&*PINK_CARPET);
    registry.register(&*GRAY_CARPET);
    registry.register(&*LIGHT_GRAY_CARPET);
    registry.register(&*CYAN_CARPET);
    registry.register(&*PURPLE_CARPET);
    registry.register(&*BLUE_CARPET);
    registry.register(&*BROWN_CARPET);
    registry.register(&*GREEN_CARPET);
    registry.register(&*RED_CARPET);
    registry.register(&*BLACK_CARPET);
    registry.register(&*TERRACOTTA);
    registry.register(&*COAL_BLOCK);
    registry.register(&*PACKED_ICE);
    registry.register(&*SUNFLOWER);
    registry.register(&*LILAC);
    registry.register(&*ROSE_BUSH);
    registry.register(&*PEONY);
    registry.register(&*TALL_GRASS);
    registry.register(&*LARGE_FERN);
    registry.register(&*WHITE_BANNER);
    registry.register(&*ORANGE_BANNER);
    registry.register(&*MAGENTA_BANNER);
    registry.register(&*LIGHT_BLUE_BANNER);
    registry.register(&*YELLOW_BANNER);
    registry.register(&*LIME_BANNER);
    registry.register(&*PINK_BANNER);
    registry.register(&*GRAY_BANNER);
    registry.register(&*LIGHT_GRAY_BANNER);
    registry.register(&*CYAN_BANNER);
    registry.register(&*PURPLE_BANNER);
    registry.register(&*BLUE_BANNER);
    registry.register(&*BROWN_BANNER);
    registry.register(&*GREEN_BANNER);
    registry.register(&*RED_BANNER);
    registry.register(&*BLACK_BANNER);
    registry.register(&*WHITE_WALL_BANNER);
    registry.register(&*ORANGE_WALL_BANNER);
    registry.register(&*MAGENTA_WALL_BANNER);
    registry.register(&*LIGHT_BLUE_WALL_BANNER);
    registry.register(&*YELLOW_WALL_BANNER);
    registry.register(&*LIME_WALL_BANNER);
    registry.register(&*PINK_WALL_BANNER);
    registry.register(&*GRAY_WALL_BANNER);
    registry.register(&*LIGHT_GRAY_WALL_BANNER);
    registry.register(&*CYAN_WALL_BANNER);
    registry.register(&*PURPLE_WALL_BANNER);
    registry.register(&*BLUE_WALL_BANNER);
    registry.register(&*BROWN_WALL_BANNER);
    registry.register(&*GREEN_WALL_BANNER);
    registry.register(&*RED_WALL_BANNER);
    registry.register(&*BLACK_WALL_BANNER);
    registry.register(&*RED_SANDSTONE);
    registry.register(&*CHISELED_RED_SANDSTONE);
    registry.register(&*CUT_RED_SANDSTONE);
    registry.register(&*RED_SANDSTONE_STAIRS);
    registry.register(&*OAK_SLAB);
    registry.register(&*SPRUCE_SLAB);
    registry.register(&*BIRCH_SLAB);
    registry.register(&*JUNGLE_SLAB);
    registry.register(&*ACACIA_SLAB);
    registry.register(&*CHERRY_SLAB);
    registry.register(&*DARK_OAK_SLAB);
    registry.register(&*PALE_OAK_SLAB);
    registry.register(&*MANGROVE_SLAB);
    registry.register(&*BAMBOO_SLAB);
    registry.register(&*BAMBOO_MOSAIC_SLAB);
    registry.register(&*STONE_SLAB);
    registry.register(&*SMOOTH_STONE_SLAB);
    registry.register(&*SANDSTONE_SLAB);
    registry.register(&*CUT_SANDSTONE_SLAB);
    registry.register(&*PETRIFIED_OAK_SLAB);
    registry.register(&*COBBLESTONE_SLAB);
    registry.register(&*BRICK_SLAB);
    registry.register(&*STONE_BRICK_SLAB);
    registry.register(&*MUD_BRICK_SLAB);
    registry.register(&*NETHER_BRICK_SLAB);
    registry.register(&*QUARTZ_SLAB);
    registry.register(&*RED_SANDSTONE_SLAB);
    registry.register(&*CUT_RED_SANDSTONE_SLAB);
    registry.register(&*PURPUR_SLAB);
    registry.register(&*SMOOTH_STONE);
    registry.register(&*SMOOTH_SANDSTONE);
    registry.register(&*SMOOTH_QUARTZ);
    registry.register(&*SMOOTH_RED_SANDSTONE);
    registry.register(&*SPRUCE_FENCE_GATE);
    registry.register(&*BIRCH_FENCE_GATE);
    registry.register(&*JUNGLE_FENCE_GATE);
    registry.register(&*ACACIA_FENCE_GATE);
    registry.register(&*CHERRY_FENCE_GATE);
    registry.register(&*DARK_OAK_FENCE_GATE);
    registry.register(&*PALE_OAK_FENCE_GATE);
    registry.register(&*MANGROVE_FENCE_GATE);
    registry.register(&*BAMBOO_FENCE_GATE);
    registry.register(&*SPRUCE_FENCE);
    registry.register(&*BIRCH_FENCE);
    registry.register(&*JUNGLE_FENCE);
    registry.register(&*ACACIA_FENCE);
    registry.register(&*CHERRY_FENCE);
    registry.register(&*DARK_OAK_FENCE);
    registry.register(&*PALE_OAK_FENCE);
    registry.register(&*MANGROVE_FENCE);
    registry.register(&*BAMBOO_FENCE);
    registry.register(&*SPRUCE_DOOR);
    registry.register(&*BIRCH_DOOR);
    registry.register(&*JUNGLE_DOOR);
    registry.register(&*ACACIA_DOOR);
    registry.register(&*CHERRY_DOOR);
    registry.register(&*DARK_OAK_DOOR);
    registry.register(&*PALE_OAK_DOOR);
    registry.register(&*MANGROVE_DOOR);
    registry.register(&*BAMBOO_DOOR);
    registry.register(&*END_ROD);
    registry.register(&*CHORUS_PLANT);
    registry.register(&*CHORUS_FLOWER);
    registry.register(&*PURPUR_BLOCK);
    registry.register(&*PURPUR_PILLAR);
    registry.register(&*PURPUR_STAIRS);
    registry.register(&*END_STONE_BRICKS);
    registry.register(&*TORCHFLOWER_CROP);
    registry.register(&*PITCHER_CROP);
    registry.register(&*PITCHER_PLANT);
    registry.register(&*BEETROOTS);
    registry.register(&*DIRT_PATH);
    registry.register(&*END_GATEWAY);
    registry.register(&*REPEATING_COMMAND_BLOCK);
    registry.register(&*CHAIN_COMMAND_BLOCK);
    registry.register(&*FROSTED_ICE);
    registry.register(&*MAGMA_BLOCK);
    registry.register(&*NETHER_WART_BLOCK);
    registry.register(&*RED_NETHER_BRICKS);
    registry.register(&*BONE_BLOCK);
    registry.register(&*STRUCTURE_VOID);
    registry.register(&*OBSERVER);
    registry.register(&*SHULKER_BOX);
    registry.register(&*WHITE_SHULKER_BOX);
    registry.register(&*ORANGE_SHULKER_BOX);
    registry.register(&*MAGENTA_SHULKER_BOX);
    registry.register(&*LIGHT_BLUE_SHULKER_BOX);
    registry.register(&*YELLOW_SHULKER_BOX);
    registry.register(&*LIME_SHULKER_BOX);
    registry.register(&*PINK_SHULKER_BOX);
    registry.register(&*GRAY_SHULKER_BOX);
    registry.register(&*LIGHT_GRAY_SHULKER_BOX);
    registry.register(&*CYAN_SHULKER_BOX);
    registry.register(&*PURPLE_SHULKER_BOX);
    registry.register(&*BLUE_SHULKER_BOX);
    registry.register(&*BROWN_SHULKER_BOX);
    registry.register(&*GREEN_SHULKER_BOX);
    registry.register(&*RED_SHULKER_BOX);
    registry.register(&*BLACK_SHULKER_BOX);
    registry.register(&*WHITE_GLAZED_TERRACOTTA);
    registry.register(&*ORANGE_GLAZED_TERRACOTTA);
    registry.register(&*MAGENTA_GLAZED_TERRACOTTA);
    registry.register(&*LIGHT_BLUE_GLAZED_TERRACOTTA);
    registry.register(&*YELLOW_GLAZED_TERRACOTTA);
    registry.register(&*LIME_GLAZED_TERRACOTTA);
    registry.register(&*PINK_GLAZED_TERRACOTTA);
    registry.register(&*GRAY_GLAZED_TERRACOTTA);
    registry.register(&*LIGHT_GRAY_GLAZED_TERRACOTTA);
    registry.register(&*CYAN_GLAZED_TERRACOTTA);
    registry.register(&*PURPLE_GLAZED_TERRACOTTA);
    registry.register(&*BLUE_GLAZED_TERRACOTTA);
    registry.register(&*BROWN_GLAZED_TERRACOTTA);
    registry.register(&*GREEN_GLAZED_TERRACOTTA);
    registry.register(&*RED_GLAZED_TERRACOTTA);
    registry.register(&*BLACK_GLAZED_TERRACOTTA);
    registry.register(&*WHITE_CONCRETE);
    registry.register(&*ORANGE_CONCRETE);
    registry.register(&*MAGENTA_CONCRETE);
    registry.register(&*LIGHT_BLUE_CONCRETE);
    registry.register(&*YELLOW_CONCRETE);
    registry.register(&*LIME_CONCRETE);
    registry.register(&*PINK_CONCRETE);
    registry.register(&*GRAY_CONCRETE);
    registry.register(&*LIGHT_GRAY_CONCRETE);
    registry.register(&*CYAN_CONCRETE);
    registry.register(&*PURPLE_CONCRETE);
    registry.register(&*BLUE_CONCRETE);
    registry.register(&*BROWN_CONCRETE);
    registry.register(&*GREEN_CONCRETE);
    registry.register(&*RED_CONCRETE);
    registry.register(&*BLACK_CONCRETE);
    registry.register(&*WHITE_CONCRETE_POWDER);
    registry.register(&*ORANGE_CONCRETE_POWDER);
    registry.register(&*MAGENTA_CONCRETE_POWDER);
    registry.register(&*LIGHT_BLUE_CONCRETE_POWDER);
    registry.register(&*YELLOW_CONCRETE_POWDER);
    registry.register(&*LIME_CONCRETE_POWDER);
    registry.register(&*PINK_CONCRETE_POWDER);
    registry.register(&*GRAY_CONCRETE_POWDER);
    registry.register(&*LIGHT_GRAY_CONCRETE_POWDER);
    registry.register(&*CYAN_CONCRETE_POWDER);
    registry.register(&*PURPLE_CONCRETE_POWDER);
    registry.register(&*BLUE_CONCRETE_POWDER);
    registry.register(&*BROWN_CONCRETE_POWDER);
    registry.register(&*GREEN_CONCRETE_POWDER);
    registry.register(&*RED_CONCRETE_POWDER);
    registry.register(&*BLACK_CONCRETE_POWDER);
    registry.register(&*KELP);
    registry.register(&*KELP_PLANT);
    registry.register(&*DRIED_KELP_BLOCK);
    registry.register(&*TURTLE_EGG);
    registry.register(&*SNIFFER_EGG);
    registry.register(&*DRIED_GHAST);
    registry.register(&*DEAD_TUBE_CORAL_BLOCK);
    registry.register(&*DEAD_BRAIN_CORAL_BLOCK);
    registry.register(&*DEAD_BUBBLE_CORAL_BLOCK);
    registry.register(&*DEAD_FIRE_CORAL_BLOCK);
    registry.register(&*DEAD_HORN_CORAL_BLOCK);
    registry.register(&*TUBE_CORAL_BLOCK);
    registry.register(&*BRAIN_CORAL_BLOCK);
    registry.register(&*BUBBLE_CORAL_BLOCK);
    registry.register(&*FIRE_CORAL_BLOCK);
    registry.register(&*HORN_CORAL_BLOCK);
    registry.register(&*DEAD_TUBE_CORAL);
    registry.register(&*DEAD_BRAIN_CORAL);
    registry.register(&*DEAD_BUBBLE_CORAL);
    registry.register(&*DEAD_FIRE_CORAL);
    registry.register(&*DEAD_HORN_CORAL);
    registry.register(&*TUBE_CORAL);
    registry.register(&*BRAIN_CORAL);
    registry.register(&*BUBBLE_CORAL);
    registry.register(&*FIRE_CORAL);
    registry.register(&*HORN_CORAL);
    registry.register(&*DEAD_TUBE_CORAL_FAN);
    registry.register(&*DEAD_BRAIN_CORAL_FAN);
    registry.register(&*DEAD_BUBBLE_CORAL_FAN);
    registry.register(&*DEAD_FIRE_CORAL_FAN);
    registry.register(&*DEAD_HORN_CORAL_FAN);
    registry.register(&*TUBE_CORAL_FAN);
    registry.register(&*BRAIN_CORAL_FAN);
    registry.register(&*BUBBLE_CORAL_FAN);
    registry.register(&*FIRE_CORAL_FAN);
    registry.register(&*HORN_CORAL_FAN);
    registry.register(&*DEAD_TUBE_CORAL_WALL_FAN);
    registry.register(&*DEAD_BRAIN_CORAL_WALL_FAN);
    registry.register(&*DEAD_BUBBLE_CORAL_WALL_FAN);
    registry.register(&*DEAD_FIRE_CORAL_WALL_FAN);
    registry.register(&*DEAD_HORN_CORAL_WALL_FAN);
    registry.register(&*TUBE_CORAL_WALL_FAN);
    registry.register(&*BRAIN_CORAL_WALL_FAN);
    registry.register(&*BUBBLE_CORAL_WALL_FAN);
    registry.register(&*FIRE_CORAL_WALL_FAN);
    registry.register(&*HORN_CORAL_WALL_FAN);
    registry.register(&*SEA_PICKLE);
    registry.register(&*BLUE_ICE);
    registry.register(&*CONDUIT);
    registry.register(&*BAMBOO_SAPLING);
    registry.register(&*BAMBOO);
    registry.register(&*POTTED_BAMBOO);
    registry.register(&*VOID_AIR);
    registry.register(&*CAVE_AIR);
    registry.register(&*BUBBLE_COLUMN);
    registry.register(&*POLISHED_GRANITE_STAIRS);
    registry.register(&*SMOOTH_RED_SANDSTONE_STAIRS);
    registry.register(&*MOSSY_STONE_BRICK_STAIRS);
    registry.register(&*POLISHED_DIORITE_STAIRS);
    registry.register(&*MOSSY_COBBLESTONE_STAIRS);
    registry.register(&*END_STONE_BRICK_STAIRS);
    registry.register(&*STONE_STAIRS);
    registry.register(&*SMOOTH_SANDSTONE_STAIRS);
    registry.register(&*SMOOTH_QUARTZ_STAIRS);
    registry.register(&*GRANITE_STAIRS);
    registry.register(&*ANDESITE_STAIRS);
    registry.register(&*RED_NETHER_BRICK_STAIRS);
    registry.register(&*POLISHED_ANDESITE_STAIRS);
    registry.register(&*DIORITE_STAIRS);
    registry.register(&*POLISHED_GRANITE_SLAB);
    registry.register(&*SMOOTH_RED_SANDSTONE_SLAB);
    registry.register(&*MOSSY_STONE_BRICK_SLAB);
    registry.register(&*POLISHED_DIORITE_SLAB);
    registry.register(&*MOSSY_COBBLESTONE_SLAB);
    registry.register(&*END_STONE_BRICK_SLAB);
    registry.register(&*SMOOTH_SANDSTONE_SLAB);
    registry.register(&*SMOOTH_QUARTZ_SLAB);
    registry.register(&*GRANITE_SLAB);
    registry.register(&*ANDESITE_SLAB);
    registry.register(&*RED_NETHER_BRICK_SLAB);
    registry.register(&*POLISHED_ANDESITE_SLAB);
    registry.register(&*DIORITE_SLAB);
    registry.register(&*BRICK_WALL);
    registry.register(&*PRISMARINE_WALL);
    registry.register(&*RED_SANDSTONE_WALL);
    registry.register(&*MOSSY_STONE_BRICK_WALL);
    registry.register(&*GRANITE_WALL);
    registry.register(&*STONE_BRICK_WALL);
    registry.register(&*MUD_BRICK_WALL);
    registry.register(&*NETHER_BRICK_WALL);
    registry.register(&*ANDESITE_WALL);
    registry.register(&*RED_NETHER_BRICK_WALL);
    registry.register(&*SANDSTONE_WALL);
    registry.register(&*END_STONE_BRICK_WALL);
    registry.register(&*DIORITE_WALL);
    registry.register(&*SCAFFOLDING);
    registry.register(&*LOOM);
    registry.register(&*BARREL);
    registry.register(&*SMOKER);
    registry.register(&*BLAST_FURNACE);
    registry.register(&*CARTOGRAPHY_TABLE);
    registry.register(&*FLETCHING_TABLE);
    registry.register(&*GRINDSTONE);
    registry.register(&*LECTERN);
    registry.register(&*SMITHING_TABLE);
    registry.register(&*STONECUTTER);
    registry.register(&*BELL);
    registry.register(&*LANTERN);
    registry.register(&*SOUL_LANTERN);
    registry.register(&*COPPER_LANTERN);
    registry.register(&*EXPOSED_COPPER_LANTERN);
    registry.register(&*WEATHERED_COPPER_LANTERN);
    registry.register(&*OXIDIZED_COPPER_LANTERN);
    registry.register(&*WAXED_COPPER_LANTERN);
    registry.register(&*WAXED_EXPOSED_COPPER_LANTERN);
    registry.register(&*WAXED_WEATHERED_COPPER_LANTERN);
    registry.register(&*WAXED_OXIDIZED_COPPER_LANTERN);
    registry.register(&*CAMPFIRE);
    registry.register(&*SOUL_CAMPFIRE);
    registry.register(&*SWEET_BERRY_BUSH);
    registry.register(&*WARPED_STEM);
    registry.register(&*STRIPPED_WARPED_STEM);
    registry.register(&*WARPED_HYPHAE);
    registry.register(&*STRIPPED_WARPED_HYPHAE);
    registry.register(&*WARPED_NYLIUM);
    registry.register(&*WARPED_FUNGUS);
    registry.register(&*WARPED_WART_BLOCK);
    registry.register(&*WARPED_ROOTS);
    registry.register(&*NETHER_SPROUTS);
    registry.register(&*CRIMSON_STEM);
    registry.register(&*STRIPPED_CRIMSON_STEM);
    registry.register(&*CRIMSON_HYPHAE);
    registry.register(&*STRIPPED_CRIMSON_HYPHAE);
    registry.register(&*CRIMSON_NYLIUM);
    registry.register(&*CRIMSON_FUNGUS);
    registry.register(&*SHROOMLIGHT);
    registry.register(&*WEEPING_VINES);
    registry.register(&*WEEPING_VINES_PLANT);
    registry.register(&*TWISTING_VINES);
    registry.register(&*TWISTING_VINES_PLANT);
    registry.register(&*CRIMSON_ROOTS);
    registry.register(&*CRIMSON_PLANKS);
    registry.register(&*WARPED_PLANKS);
    registry.register(&*CRIMSON_SLAB);
    registry.register(&*WARPED_SLAB);
    registry.register(&*CRIMSON_PRESSURE_PLATE);
    registry.register(&*WARPED_PRESSURE_PLATE);
    registry.register(&*CRIMSON_FENCE);
    registry.register(&*WARPED_FENCE);
    registry.register(&*CRIMSON_TRAPDOOR);
    registry.register(&*WARPED_TRAPDOOR);
    registry.register(&*CRIMSON_FENCE_GATE);
    registry.register(&*WARPED_FENCE_GATE);
    registry.register(&*CRIMSON_STAIRS);
    registry.register(&*WARPED_STAIRS);
    registry.register(&*CRIMSON_BUTTON);
    registry.register(&*WARPED_BUTTON);
    registry.register(&*CRIMSON_DOOR);
    registry.register(&*WARPED_DOOR);
    registry.register(&*CRIMSON_SIGN);
    registry.register(&*WARPED_SIGN);
    registry.register(&*CRIMSON_WALL_SIGN);
    registry.register(&*WARPED_WALL_SIGN);
    registry.register(&*STRUCTURE_BLOCK);
    registry.register(&*JIGSAW);
    registry.register(&*TEST_BLOCK);
    registry.register(&*TEST_INSTANCE_BLOCK);
    registry.register(&*COMPOSTER);
    registry.register(&*TARGET);
    registry.register(&*BEE_NEST);
    registry.register(&*BEEHIVE);
    registry.register(&*HONEY_BLOCK);
    registry.register(&*HONEYCOMB_BLOCK);
    registry.register(&*NETHERITE_BLOCK);
    registry.register(&*ANCIENT_DEBRIS);
    registry.register(&*CRYING_OBSIDIAN);
    registry.register(&*RESPAWN_ANCHOR);
    registry.register(&*POTTED_CRIMSON_FUNGUS);
    registry.register(&*POTTED_WARPED_FUNGUS);
    registry.register(&*POTTED_CRIMSON_ROOTS);
    registry.register(&*POTTED_WARPED_ROOTS);
    registry.register(&*LODESTONE);
    registry.register(&*BLACKSTONE);
    registry.register(&*BLACKSTONE_STAIRS);
    registry.register(&*BLACKSTONE_WALL);
    registry.register(&*BLACKSTONE_SLAB);
    registry.register(&*POLISHED_BLACKSTONE);
    registry.register(&*POLISHED_BLACKSTONE_BRICKS);
    registry.register(&*CRACKED_POLISHED_BLACKSTONE_BRICKS);
    registry.register(&*CHISELED_POLISHED_BLACKSTONE);
    registry.register(&*POLISHED_BLACKSTONE_BRICK_SLAB);
    registry.register(&*POLISHED_BLACKSTONE_BRICK_STAIRS);
    registry.register(&*POLISHED_BLACKSTONE_BRICK_WALL);
    registry.register(&*GILDED_BLACKSTONE);
    registry.register(&*POLISHED_BLACKSTONE_STAIRS);
    registry.register(&*POLISHED_BLACKSTONE_SLAB);
    registry.register(&*POLISHED_BLACKSTONE_PRESSURE_PLATE);
    registry.register(&*POLISHED_BLACKSTONE_BUTTON);
    registry.register(&*POLISHED_BLACKSTONE_WALL);
    registry.register(&*CHISELED_NETHER_BRICKS);
    registry.register(&*CRACKED_NETHER_BRICKS);
    registry.register(&*QUARTZ_BRICKS);
    registry.register(&*CANDLE);
    registry.register(&*WHITE_CANDLE);
    registry.register(&*ORANGE_CANDLE);
    registry.register(&*MAGENTA_CANDLE);
    registry.register(&*LIGHT_BLUE_CANDLE);
    registry.register(&*YELLOW_CANDLE);
    registry.register(&*LIME_CANDLE);
    registry.register(&*PINK_CANDLE);
    registry.register(&*GRAY_CANDLE);
    registry.register(&*LIGHT_GRAY_CANDLE);
    registry.register(&*CYAN_CANDLE);
    registry.register(&*PURPLE_CANDLE);
    registry.register(&*BLUE_CANDLE);
    registry.register(&*BROWN_CANDLE);
    registry.register(&*GREEN_CANDLE);
    registry.register(&*RED_CANDLE);
    registry.register(&*BLACK_CANDLE);
    registry.register(&*CANDLE_CAKE);
    registry.register(&*WHITE_CANDLE_CAKE);
    registry.register(&*ORANGE_CANDLE_CAKE);
    registry.register(&*MAGENTA_CANDLE_CAKE);
    registry.register(&*LIGHT_BLUE_CANDLE_CAKE);
    registry.register(&*YELLOW_CANDLE_CAKE);
    registry.register(&*LIME_CANDLE_CAKE);
    registry.register(&*PINK_CANDLE_CAKE);
    registry.register(&*GRAY_CANDLE_CAKE);
    registry.register(&*LIGHT_GRAY_CANDLE_CAKE);
    registry.register(&*CYAN_CANDLE_CAKE);
    registry.register(&*PURPLE_CANDLE_CAKE);
    registry.register(&*BLUE_CANDLE_CAKE);
    registry.register(&*BROWN_CANDLE_CAKE);
    registry.register(&*GREEN_CANDLE_CAKE);
    registry.register(&*RED_CANDLE_CAKE);
    registry.register(&*BLACK_CANDLE_CAKE);
    registry.register(&*AMETHYST_BLOCK);
    registry.register(&*BUDDING_AMETHYST);
    registry.register(&*AMETHYST_CLUSTER);
    registry.register(&*LARGE_AMETHYST_BUD);
    registry.register(&*MEDIUM_AMETHYST_BUD);
    registry.register(&*SMALL_AMETHYST_BUD);
    registry.register(&*TUFF);
    registry.register(&*TUFF_SLAB);
    registry.register(&*TUFF_STAIRS);
    registry.register(&*TUFF_WALL);
    registry.register(&*POLISHED_TUFF);
    registry.register(&*POLISHED_TUFF_SLAB);
    registry.register(&*POLISHED_TUFF_STAIRS);
    registry.register(&*POLISHED_TUFF_WALL);
    registry.register(&*CHISELED_TUFF);
    registry.register(&*TUFF_BRICKS);
    registry.register(&*TUFF_BRICK_SLAB);
    registry.register(&*TUFF_BRICK_STAIRS);
    registry.register(&*TUFF_BRICK_WALL);
    registry.register(&*CHISELED_TUFF_BRICKS);
    registry.register(&*CALCITE);
    registry.register(&*TINTED_GLASS);
    registry.register(&*POWDER_SNOW);
    registry.register(&*SCULK_SENSOR);
    registry.register(&*CALIBRATED_SCULK_SENSOR);
    registry.register(&*SCULK);
    registry.register(&*SCULK_VEIN);
    registry.register(&*SCULK_CATALYST);
    registry.register(&*SCULK_SHRIEKER);
    registry.register(&*COPPER_BLOCK);
    registry.register(&*EXPOSED_COPPER);
    registry.register(&*WEATHERED_COPPER);
    registry.register(&*OXIDIZED_COPPER);
    registry.register(&*COPPER_ORE);
    registry.register(&*DEEPSLATE_COPPER_ORE);
    registry.register(&*OXIDIZED_CUT_COPPER);
    registry.register(&*WEATHERED_CUT_COPPER);
    registry.register(&*EXPOSED_CUT_COPPER);
    registry.register(&*CUT_COPPER);
    registry.register(&*OXIDIZED_CHISELED_COPPER);
    registry.register(&*WEATHERED_CHISELED_COPPER);
    registry.register(&*EXPOSED_CHISELED_COPPER);
    registry.register(&*CHISELED_COPPER);
    registry.register(&*WAXED_OXIDIZED_CHISELED_COPPER);
    registry.register(&*WAXED_WEATHERED_CHISELED_COPPER);
    registry.register(&*WAXED_EXPOSED_CHISELED_COPPER);
    registry.register(&*WAXED_CHISELED_COPPER);
    registry.register(&*OXIDIZED_CUT_COPPER_STAIRS);
    registry.register(&*WEATHERED_CUT_COPPER_STAIRS);
    registry.register(&*EXPOSED_CUT_COPPER_STAIRS);
    registry.register(&*CUT_COPPER_STAIRS);
    registry.register(&*OXIDIZED_CUT_COPPER_SLAB);
    registry.register(&*WEATHERED_CUT_COPPER_SLAB);
    registry.register(&*EXPOSED_CUT_COPPER_SLAB);
    registry.register(&*CUT_COPPER_SLAB);
    registry.register(&*WAXED_COPPER_BLOCK);
    registry.register(&*WAXED_WEATHERED_COPPER);
    registry.register(&*WAXED_EXPOSED_COPPER);
    registry.register(&*WAXED_OXIDIZED_COPPER);
    registry.register(&*WAXED_OXIDIZED_CUT_COPPER);
    registry.register(&*WAXED_WEATHERED_CUT_COPPER);
    registry.register(&*WAXED_EXPOSED_CUT_COPPER);
    registry.register(&*WAXED_CUT_COPPER);
    registry.register(&*WAXED_OXIDIZED_CUT_COPPER_STAIRS);
    registry.register(&*WAXED_WEATHERED_CUT_COPPER_STAIRS);
    registry.register(&*WAXED_EXPOSED_CUT_COPPER_STAIRS);
    registry.register(&*WAXED_CUT_COPPER_STAIRS);
    registry.register(&*WAXED_OXIDIZED_CUT_COPPER_SLAB);
    registry.register(&*WAXED_WEATHERED_CUT_COPPER_SLAB);
    registry.register(&*WAXED_EXPOSED_CUT_COPPER_SLAB);
    registry.register(&*WAXED_CUT_COPPER_SLAB);
    registry.register(&*COPPER_DOOR);
    registry.register(&*EXPOSED_COPPER_DOOR);
    registry.register(&*OXIDIZED_COPPER_DOOR);
    registry.register(&*WEATHERED_COPPER_DOOR);
    registry.register(&*WAXED_COPPER_DOOR);
    registry.register(&*WAXED_EXPOSED_COPPER_DOOR);
    registry.register(&*WAXED_OXIDIZED_COPPER_DOOR);
    registry.register(&*WAXED_WEATHERED_COPPER_DOOR);
    registry.register(&*COPPER_TRAPDOOR);
    registry.register(&*EXPOSED_COPPER_TRAPDOOR);
    registry.register(&*OXIDIZED_COPPER_TRAPDOOR);
    registry.register(&*WEATHERED_COPPER_TRAPDOOR);
    registry.register(&*WAXED_COPPER_TRAPDOOR);
    registry.register(&*WAXED_EXPOSED_COPPER_TRAPDOOR);
    registry.register(&*WAXED_OXIDIZED_COPPER_TRAPDOOR);
    registry.register(&*WAXED_WEATHERED_COPPER_TRAPDOOR);
    registry.register(&*COPPER_GRATE);
    registry.register(&*EXPOSED_COPPER_GRATE);
    registry.register(&*WEATHERED_COPPER_GRATE);
    registry.register(&*OXIDIZED_COPPER_GRATE);
    registry.register(&*WAXED_COPPER_GRATE);
    registry.register(&*WAXED_EXPOSED_COPPER_GRATE);
    registry.register(&*WAXED_WEATHERED_COPPER_GRATE);
    registry.register(&*WAXED_OXIDIZED_COPPER_GRATE);
    registry.register(&*COPPER_BULB);
    registry.register(&*EXPOSED_COPPER_BULB);
    registry.register(&*WEATHERED_COPPER_BULB);
    registry.register(&*OXIDIZED_COPPER_BULB);
    registry.register(&*WAXED_COPPER_BULB);
    registry.register(&*WAXED_EXPOSED_COPPER_BULB);
    registry.register(&*WAXED_WEATHERED_COPPER_BULB);
    registry.register(&*WAXED_OXIDIZED_COPPER_BULB);
    registry.register(&*COPPER_CHEST);
    registry.register(&*EXPOSED_COPPER_CHEST);
    registry.register(&*WEATHERED_COPPER_CHEST);
    registry.register(&*OXIDIZED_COPPER_CHEST);
    registry.register(&*WAXED_COPPER_CHEST);
    registry.register(&*WAXED_EXPOSED_COPPER_CHEST);
    registry.register(&*WAXED_WEATHERED_COPPER_CHEST);
    registry.register(&*WAXED_OXIDIZED_COPPER_CHEST);
    registry.register(&*COPPER_GOLEM_STATUE);
    registry.register(&*EXPOSED_COPPER_GOLEM_STATUE);
    registry.register(&*WEATHERED_COPPER_GOLEM_STATUE);
    registry.register(&*OXIDIZED_COPPER_GOLEM_STATUE);
    registry.register(&*WAXED_COPPER_GOLEM_STATUE);
    registry.register(&*WAXED_EXPOSED_COPPER_GOLEM_STATUE);
    registry.register(&*WAXED_WEATHERED_COPPER_GOLEM_STATUE);
    registry.register(&*WAXED_OXIDIZED_COPPER_GOLEM_STATUE);
    registry.register(&*LIGHTNING_ROD);
    registry.register(&*EXPOSED_LIGHTNING_ROD);
    registry.register(&*WEATHERED_LIGHTNING_ROD);
    registry.register(&*OXIDIZED_LIGHTNING_ROD);
    registry.register(&*WAXED_LIGHTNING_ROD);
    registry.register(&*WAXED_EXPOSED_LIGHTNING_ROD);
    registry.register(&*WAXED_WEATHERED_LIGHTNING_ROD);
    registry.register(&*WAXED_OXIDIZED_LIGHTNING_ROD);
    registry.register(&*POINTED_DRIPSTONE);
    registry.register(&*DRIPSTONE_BLOCK);
    registry.register(&*CAVE_VINES);
    registry.register(&*CAVE_VINES_PLANT);
    registry.register(&*SPORE_BLOSSOM);
    registry.register(&*AZALEA);
    registry.register(&*FLOWERING_AZALEA);
    registry.register(&*MOSS_CARPET);
    registry.register(&*PINK_PETALS);
    registry.register(&*WILDFLOWERS);
    registry.register(&*LEAF_LITTER);
    registry.register(&*MOSS_BLOCK);
    registry.register(&*BIG_DRIPLEAF);
    registry.register(&*BIG_DRIPLEAF_STEM);
    registry.register(&*SMALL_DRIPLEAF);
    registry.register(&*HANGING_ROOTS);
    registry.register(&*ROOTED_DIRT);
    registry.register(&*MUD);
    registry.register(&*DEEPSLATE);
    registry.register(&*COBBLED_DEEPSLATE);
    registry.register(&*COBBLED_DEEPSLATE_STAIRS);
    registry.register(&*COBBLED_DEEPSLATE_SLAB);
    registry.register(&*COBBLED_DEEPSLATE_WALL);
    registry.register(&*POLISHED_DEEPSLATE);
    registry.register(&*POLISHED_DEEPSLATE_STAIRS);
    registry.register(&*POLISHED_DEEPSLATE_SLAB);
    registry.register(&*POLISHED_DEEPSLATE_WALL);
    registry.register(&*DEEPSLATE_TILES);
    registry.register(&*DEEPSLATE_TILE_STAIRS);
    registry.register(&*DEEPSLATE_TILE_SLAB);
    registry.register(&*DEEPSLATE_TILE_WALL);
    registry.register(&*DEEPSLATE_BRICKS);
    registry.register(&*DEEPSLATE_BRICK_STAIRS);
    registry.register(&*DEEPSLATE_BRICK_SLAB);
    registry.register(&*DEEPSLATE_BRICK_WALL);
    registry.register(&*CHISELED_DEEPSLATE);
    registry.register(&*CRACKED_DEEPSLATE_BRICKS);
    registry.register(&*CRACKED_DEEPSLATE_TILES);
    registry.register(&*INFESTED_DEEPSLATE);
    registry.register(&*SMOOTH_BASALT);
    registry.register(&*RAW_IRON_BLOCK);
    registry.register(&*RAW_COPPER_BLOCK);
    registry.register(&*RAW_GOLD_BLOCK);
    registry.register(&*POTTED_AZALEA_BUSH);
    registry.register(&*POTTED_FLOWERING_AZALEA_BUSH);
    registry.register(&*OCHRE_FROGLIGHT);
    registry.register(&*VERDANT_FROGLIGHT);
    registry.register(&*PEARLESCENT_FROGLIGHT);
    registry.register(&*FROGSPAWN);
    registry.register(&*REINFORCED_DEEPSLATE);
    registry.register(&*DECORATED_POT);
    registry.register(&*CRAFTER);
    registry.register(&*TRIAL_SPAWNER);
    registry.register(&*VAULT);
    registry.register(&*HEAVY_CORE);
    registry.register(&*PALE_MOSS_BLOCK);
    registry.register(&*PALE_MOSS_CARPET);
    registry.register(&*PALE_HANGING_MOSS);
    registry.register(&*OPEN_EYEBLOSSOM);
    registry.register(&*CLOSED_EYEBLOSSOM);
    registry.register(&*POTTED_OPEN_EYEBLOSSOM);
    registry.register(&*POTTED_CLOSED_EYEBLOSSOM);
    registry.register(&*FIREFLY_BUSH);
}
