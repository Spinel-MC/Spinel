use crate::{ASSETS_DIRECTORY, vanilla_path};
use serde::Deserialize;
use std::{fs, io};

pub(crate) fn item_entries() -> io::Result<Vec<ItemEntry>> {
    let json = fs::read_to_string(format!("{ASSETS_DIRECTORY}/items.json"))?;
    let extraction: ItemExtraction = serde_json::from_str(&json).map_err(io::Error::other)?;
    let mut items = extraction
        .items
        .into_iter()
        .map(|item| ItemEntry {
            path: vanilla_path(&item.name).to_string(),
            id: item.id,
        })
        .collect::<Vec<_>>();
    items.sort_by_key(|item| item.id);
    Ok(items)
}

pub(crate) struct ItemEntry {
    pub(crate) path: String,
    pub(crate) id: i32,
}

#[derive(Deserialize)]
struct ItemExtraction {
    items: Vec<ExtractedItem>,
}

#[derive(Deserialize)]
struct ExtractedItem {
    id: i32,
    name: String,
}
