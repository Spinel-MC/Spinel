use crate::{ASSETS_DIRECTORY, vanilla_path};
use serde::Deserialize;
use std::{collections::BTreeMap, fs, io};

pub(crate) fn entity_entries() -> io::Result<Vec<EntityEntry>> {
    let json = fs::read_to_string(format!("{ASSETS_DIRECTORY}/entity_types.json"))?;
    let extraction: EntityExtraction = serde_json::from_str(&json).map_err(io::Error::other)?;
    let mut entity_types = extraction
        .entity_types
        .into_iter()
        .map(ExtractedEntityType::into_entity_entry)
        .collect::<Vec<_>>();
    entity_types.sort_by_key(|entity_type| entity_type.id);
    Ok(entity_types)
}

pub(crate) struct EntityEntry {
    pub(crate) path: String,
    pub(crate) id: i32,
    pub(crate) translation_key: String,
    pub(crate) packet_type: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) eye_height: f64,
    pub(crate) client_tracking_range: i32,
    pub(crate) fire_immune: bool,
    pub(crate) default_attributes: BTreeMap<String, f64>,
    pub(crate) attachments: BTreeMap<String, Vec<f64>>,
}

#[derive(Deserialize)]
struct EntityExtraction {
    #[serde(rename = "entityTypes")]
    entity_types: Vec<ExtractedEntityType>,
}

#[derive(Deserialize)]
struct ExtractedEntityType {
    id: i32,
    name: String,
    #[serde(rename = "translationKey")]
    translation_key: String,
    #[serde(rename = "packetType")]
    packet_type: String,
    width: f64,
    height: f64,
    #[serde(rename = "eyeHeight")]
    eye_height: f64,
    #[serde(rename = "clientTrackingRange")]
    client_tracking_range: i32,
    #[serde(rename = "fireImmune")]
    fire_immune: bool,
    #[serde(rename = "defaultAttributes")]
    default_attributes: BTreeMap<String, f64>,
    attachments: BTreeMap<String, Vec<f64>>,
}

impl ExtractedEntityType {
    fn into_entity_entry(self) -> EntityEntry {
        EntityEntry {
            path: vanilla_path(&self.name).to_string(),
            id: self.id,
            translation_key: self.translation_key,
            packet_type: self.packet_type,
            width: self.width,
            height: self.height,
            eye_height: self.eye_height,
            client_tracking_range: self.client_tracking_range,
            fire_immune: self.fire_immune,
            default_attributes: self.default_attributes,
            attachments: self.attachments,
        }
    }
}
