use serde::Deserialize;
use std::{collections::BTreeMap, fs, io};

const STATIC_PROTOCOL_REGISTRIES_FILE: &str = "assets/static_protocol_registries.json";

#[derive(Debug, Deserialize)]
pub struct StaticProtocolEntry {
    pub id: i32,
    pub name: String,
}

pub fn static_protocol_entries(registry_name: &str) -> io::Result<Vec<StaticProtocolEntry>> {
    let json = fs::read_to_string(STATIC_PROTOCOL_REGISTRIES_FILE)?;
    let mut registries: BTreeMap<String, Vec<StaticProtocolEntry>> =
        serde_json::from_str(&json).map_err(io::Error::other)?;
    registries.remove(registry_name).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("missing static protocol registry {registry_name}"),
        )
    })
}
