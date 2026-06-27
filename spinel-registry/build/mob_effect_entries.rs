use serde::Deserialize;
use std::{fs, io};

const MOB_EFFECTS_FILE_PATH: &str = "assets/mob_effects.json";

#[derive(Deserialize)]
pub(crate) struct MobEffectEntry {
    pub(crate) id: i32,
    pub(crate) name: String,
    #[serde(rename = "translationKey")]
    pub(crate) translation_key: String,
    pub(crate) color: i32,
    pub(crate) instantaneous: bool,
}

#[derive(Deserialize)]
struct MobEffectExtraction {
    mob_effects: Vec<MobEffectEntry>,
}

pub(crate) fn mob_effect_entries() -> io::Result<Vec<MobEffectEntry>> {
    let json = fs::read_to_string(MOB_EFFECTS_FILE_PATH)?;
    let mut extraction: MobEffectExtraction =
        serde_json::from_str(&json).map_err(io::Error::other)?;
    extraction.mob_effects.sort_by_key(|entry| entry.id);
    Ok(extraction.mob_effects)
}
