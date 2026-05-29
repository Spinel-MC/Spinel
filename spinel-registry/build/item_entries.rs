use crate::{ASSETS_DIRECTORY, vanilla_path};
use serde::Deserialize;
use std::{collections::BTreeMap, fs, io};

pub(crate) fn item_entries() -> io::Result<Vec<ItemEntry>> {
    let json = fs::read_to_string(format!("{ASSETS_DIRECTORY}/items.json"))?;
    let extraction: ItemExtraction = serde_json::from_str(&json).map_err(io::Error::other)?;
    let mut items = extraction.into_item_entries();
    items.sort_by_key(|item| item.id);
    Ok(items)
}

pub(crate) struct ItemEntry {
    pub(crate) path: String,
    pub(crate) id: i32,
    pub(crate) block_item: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ItemExtraction {
    Spinel { items: Vec<ExtractedItem> },
    Minestom(BTreeMap<String, MinestomItem>),
}

impl ItemExtraction {
    fn into_item_entries(self) -> Vec<ItemEntry> {
        match self {
            Self::Spinel { items } => items
                .into_iter()
                .map(ExtractedItem::into_item_entry)
                .collect(),
            Self::Minestom(items) => items
                .into_iter()
                .map(|(key, item)| item.into_item_entry(&key))
                .collect(),
        }
    }
}

#[derive(Deserialize)]
struct ExtractedItem {
    id: i32,
    name: String,
    #[serde(rename = "blockItem")]
    block_item: Option<String>,
}

#[derive(Deserialize)]
struct MinestomItem {
    id: i32,
    #[serde(rename = "correspondingBlock")]
    corresponding_block: Option<String>,
}

impl ExtractedItem {
    fn into_item_entry(self) -> ItemEntry {
        ItemEntry {
            path: vanilla_path(&self.name).to_string(),
            id: self.id,
            block_item: self
                .block_item
                .as_ref()
                .map(|block_item| vanilla_path(block_item).to_string()),
        }
    }
}

impl MinestomItem {
    fn into_item_entry(self, key: &str) -> ItemEntry {
        ItemEntry {
            path: vanilla_path(key).to_string(),
            id: self.id,
            block_item: self
                .corresponding_block
                .as_ref()
                .map(|block_item| vanilla_path(block_item).to_string()),
        }
    }
}
