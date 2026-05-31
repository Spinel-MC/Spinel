use crate::{ASSETS_DIRECTORY, vanilla_path};
use heck::ToShoutySnakeCase;
use serde::Deserialize;
use std::{collections::BTreeMap, fs, io};

pub(crate) fn sorted_block_entries() -> io::Result<Vec<BlockEntry>> {
    let mut blocks = extracted_block_entries()?;
    blocks.sort_by_key(|block| block.id);
    Ok(blocks)
}

pub(crate) fn block_entries_by_key() -> io::Result<Vec<BlockEntry>> {
    let mut blocks = extracted_block_entries()?;
    blocks.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(blocks)
}

fn extracted_block_entries() -> io::Result<Vec<BlockEntry>> {
    let json = fs::read_to_string(format!("{ASSETS_DIRECTORY}/blocks.json"))?;
    let extraction: BlockExtraction = serde_json::from_str(&json).map_err(io::Error::other)?;
    Ok(extraction
        .blocks
        .into_iter()
        .map(|(key, block)| BlockEntry {
            variant: vanilla_path(&key).to_shouty_snake_case(),
            path: vanilla_path(&key).to_string(),
            id: block.id,
            state_id: block.default_state_id,
            is_air: block.flags.air,
            is_solid: block.flags.solid,
            is_liquid: block.flags.liquid,
        })
        .collect())
}

pub(crate) struct BlockEntry {
    pub(crate) variant: String,
    pub(crate) path: String,
    pub(crate) id: i32,
    pub(crate) state_id: i32,
    pub(crate) is_air: bool,
    pub(crate) is_solid: bool,
    pub(crate) is_liquid: bool,
}

#[derive(Deserialize)]
struct BlockExtraction {
    blocks: BTreeMap<String, ExtractedBlock>,
}

#[derive(Deserialize)]
struct ExtractedBlock {
    id: i32,
    default_state_id: i32,
    flags: ExtractedBlockFlags,
}

#[derive(Deserialize)]
struct ExtractedBlockFlags {
    air: bool,
    solid: bool,
    liquid: bool,
}
