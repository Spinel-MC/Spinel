use serde::Deserialize;
use std::{fs, io};

const POTIONS_FILE_PATH: &str = "assets/potions.json";

#[derive(Deserialize)]
pub(crate) struct PotionEntry {
    pub(crate) id: i32,
    pub(crate) key: String,
    pub(crate) effects: Vec<PotionEffectEntry>,
}

#[derive(Deserialize)]
pub(crate) struct PotionEffectEntry {
    pub(crate) effect: String,
    pub(crate) amplifier: i32,
    pub(crate) duration: i32,
    pub(crate) ambient: bool,
    pub(crate) visible: bool,
    #[serde(rename = "showIcon")]
    pub(crate) show_icon: bool,
}

#[derive(Deserialize)]
struct PotionExtraction {
    potions: Vec<PotionEntry>,
}

pub(crate) fn potion_entries() -> io::Result<Vec<PotionEntry>> {
    let json = fs::read_to_string(POTIONS_FILE_PATH)?;
    let mut extraction: PotionExtraction = serde_json::from_str(&json).map_err(io::Error::other)?;
    extraction.potions.sort_by_key(|entry| entry.id);
    Ok(extraction.potions)
}