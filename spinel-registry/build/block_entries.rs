use crate::{ASSETS_DIRECTORY, vanilla_path};
use heck::ToShoutySnakeCase;
use serde::Deserialize;
use std::{collections::BTreeMap, fs, io};

pub(crate) fn sorted_block_entries() -> io::Result<Vec<BlockEntry>> {
    let mut blocks = extracted_entries()?.0;
    blocks.sort_by_key(|block| block.id);
    Ok(blocks)
}

pub(crate) fn block_entries_by_key() -> io::Result<Vec<BlockEntry>> {
    let mut blocks = extracted_entries()?.0;
    blocks.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(blocks)
}

pub(crate) fn sorted_block_state_entries() -> io::Result<Vec<BlockStateEntry>> {
    let mut states = extracted_entries()?.1;
    states.sort_by_key(|state| state.id);
    Ok(states)
}

pub(crate) fn block_shapes() -> io::Result<Vec<Vec<BlockShapeBoxEntry>>> {
    Ok(extraction()?.block_shapes)
}

fn extracted_entries() -> io::Result<(Vec<BlockEntry>, Vec<BlockStateEntry>)> {
    let extraction = extraction()?;
    let mut block_entries = Vec::with_capacity(extraction.blocks.len());
    let mut block_state_entries = Vec::new();
    extraction.blocks.into_iter().for_each(|(key, block)| {
        let variant = vanilla_path(&key).to_shouty_snake_case();
        block_entries.push(BlockEntry {
            variant: variant.clone(),
            path: vanilla_path(&key).to_string(),
            id: block.id,
            state_id: block.default_state_id,
            is_air: block.flags.air,
            is_solid: block.flags.solid,
            is_liquid: block.flags.liquid,
            is_replaceable: block.flags.replaceable,
            hardness: block.hardness,
            friction: block.friction,
            requires_tool: block.requires_tool,
            block_entity_type: block.block_entity_type,
        });
        block.states.into_values().for_each(|state| {
            block_state_entries.push(BlockStateEntry {
                id: state.id,
                block_variant: variant.clone(),
                properties: state.properties.into_iter().collect(),
                light_emission: state.light_emission,
                light_block: state.light_block,
                propagates_skylight_down: state.propagates_skylight_down,
                uses_shape_for_light_occlusion: state.uses_shape_for_light_occlusion,
                collision_shape: state.collision_shape,
                occlusion_shape: state.occlusion_shape,
                face_occlusion_shapes: state.face_occlusion_shapes,
            });
        });
    });
    Ok((block_entries, block_state_entries))
}

fn extraction() -> io::Result<BlockExtraction> {
    let json = fs::read_to_string(format!("{ASSETS_DIRECTORY}/blocks.json"))?;
    serde_json::from_str(&json).map_err(io::Error::other)
}

pub(crate) struct BlockEntry {
    pub(crate) variant: String,
    pub(crate) path: String,
    pub(crate) id: i32,
    pub(crate) state_id: i32,
    pub(crate) is_air: bool,
    pub(crate) is_solid: bool,
    pub(crate) is_liquid: bool,
    pub(crate) is_replaceable: bool,
    pub(crate) hardness: f32,
    pub(crate) friction: f32,
    pub(crate) requires_tool: bool,
    pub(crate) block_entity_type: Option<String>,
}

pub(crate) struct BlockStateEntry {
    pub(crate) id: i32,
    pub(crate) block_variant: String,
    pub(crate) properties: Vec<(String, String)>,
    pub(crate) light_emission: u8,
    pub(crate) light_block: u8,
    pub(crate) propagates_skylight_down: bool,
    pub(crate) uses_shape_for_light_occlusion: bool,
    pub(crate) collision_shape: u16,
    pub(crate) occlusion_shape: u16,
    pub(crate) face_occlusion_shapes: BTreeMap<String, u16>,
}

#[derive(Deserialize)]
struct BlockExtraction {
    blocks: BTreeMap<String, ExtractedBlock>,
    block_shapes: Vec<Vec<BlockShapeBoxEntry>>,
}

#[derive(Deserialize)]
struct ExtractedBlock {
    id: i32,
    default_state_id: i32,
    block_entity_type: Option<String>,
    states: BTreeMap<String, ExtractedBlockState>,
    hardness: f32,
    friction: f32,
    requires_tool: bool,
    flags: ExtractedBlockFlags,
}

#[derive(Deserialize)]
struct ExtractedBlockState {
    id: i32,
    properties: BTreeMap<String, String>,
    light_emission: u8,
    light_block: u8,
    propagates_skylight_down: bool,
    uses_shape_for_light_occlusion: bool,
    collision_shape: u16,
    occlusion_shape: u16,
    face_occlusion_shapes: BTreeMap<String, u16>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BlockShapeBoxEntry {
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) min_z: f64,
    pub(crate) max_x: f64,
    pub(crate) max_y: f64,
    pub(crate) max_z: f64,
}

#[derive(Deserialize)]
struct ExtractedBlockFlags {
    air: bool,
    solid: bool,
    liquid: bool,
    #[serde(default)]
    replaceable: bool,
}
