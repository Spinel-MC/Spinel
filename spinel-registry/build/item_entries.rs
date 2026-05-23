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
            block_item: item
                .block_item
                .as_ref()
                .map(|block_item| vanilla_path(block_item).to_string()),
            max_stack_size: item.max_stack_size(),
        })
        .collect::<Vec<_>>();
    items.sort_by_key(|item| item.id);
    Ok(items)
}

pub(crate) struct ItemEntry {
    pub(crate) path: String,
    pub(crate) id: i32,
    pub(crate) block_item: Option<String>,
    pub(crate) max_stack_size: i32,
}

#[derive(Deserialize)]
struct ItemExtraction {
    items: Vec<ExtractedItem>,
}

#[derive(Deserialize)]
struct ExtractedItem {
    id: i32,
    name: String,
    #[serde(rename = "blockItem")]
    block_item: Option<String>,
    #[serde(rename = "maxStackSize")]
    max_stack_size: Option<i32>,
    components: Option<serde_json::Value>,
}

impl ExtractedItem {
    fn max_stack_size(&self) -> i32 {
        if let Some(max_stack_size) = self.max_stack_size {
            return max_stack_size;
        }
        self.components
            .as_ref()
            .and_then(|components| components.get("minecraft:max_stack_size"))
            .and_then(serde_json::Value::as_i64)
            .and_then(|value| i32::try_from(value).ok())
            .unwrap_or_else(|| if self.name == "air" { 0 } else { 64 })
    }
}
