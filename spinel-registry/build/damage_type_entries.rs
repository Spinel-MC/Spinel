use serde::Deserialize;
use std::{fs, io};

const DAMAGE_TYPES_FILE_PATH: &str = "assets/damage_types.json";

#[derive(Deserialize)]
struct ExtractedDamageTypes {
    damage_types: std::collections::HashMap<String, ExtractedDamageType>,
}

#[derive(Deserialize)]
struct ExtractedDamageType {
    message_id: String,
    scaling: String,
    exhaustion: f32,
    effects: String,
    death_message_type: String,
}

#[derive(Deserialize)]
pub(crate) struct DamageTypeEntry {
    #[serde(default)]
    pub(crate) path: String,
    pub(crate) message_id: String,
    pub(crate) scaling: String,
    pub(crate) exhaustion: f32,
    #[serde(default = "default_effects")]
    pub(crate) effects: String,
    #[serde(default = "default_death_message_type")]
    pub(crate) death_message_type: String,
}

pub(crate) fn damage_type_entries() -> io::Result<Vec<DamageTypeEntry>> {
    let json = fs::read_to_string(DAMAGE_TYPES_FILE_PATH)?;
    let extracted_damage_types: ExtractedDamageTypes =
        serde_json::from_str(&json).map_err(io::Error::other)?;
    let mut entries = extracted_damage_types
        .damage_types
        .into_iter()
        .map(|(identifier, damage_type)| DamageTypeEntry {
            path: identifier
                .strip_prefix("minecraft:")
                .unwrap_or(&identifier)
                .to_owned(),
            message_id: damage_type.message_id,
            scaling: damage_type.scaling,
            exhaustion: damage_type.exhaustion,
            effects: damage_type.effects,
            death_message_type: damage_type.death_message_type,
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(entries)
}

fn default_effects() -> String {
    "hurt".to_owned()
}

fn default_death_message_type() -> String {
    "default".to_owned()
}
