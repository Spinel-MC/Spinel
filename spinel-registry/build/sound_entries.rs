use serde::Deserialize;
use std::{fs, io};

const SOUND_EVENTS_FILE: &str = "assets/sound_events.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoundEventsFile {
    sound_events: Vec<SoundEventEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SoundEventEntry {
    pub id: i32,
    pub name: String,
}

pub fn sound_entries() -> io::Result<Vec<SoundEventEntry>> {
    let json = fs::read_to_string(SOUND_EVENTS_FILE)?;
    let sound_events: SoundEventsFile = serde_json::from_str(&json).map_err(io::Error::other)?;
    Ok(sound_events.sound_events)
}
