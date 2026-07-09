use serde_json::{Map, Value, json};
use spinel_utils::{
    component::text::TextComponent,
    constants::{PROTOCOL_VERSION, SERVER_BRAND},
};

use crate::events::server_list_ping::{favicon::Favicon, player_sample::PlayerSample};

pub struct ServerListPingEventResponseData {
    pub online_players: Option<i32>,
    pub max_players: Option<i32>,
    pub description: Option<TextComponent>,
    pub brand: Option<String>,
    pub protocol: u16,
    pub player_sample: Option<Vec<PlayerSample>>,
    pub favicon: Option<Favicon>,
    pub enforce_secure_chat: Option<bool>,
}

impl Default for ServerListPingEventResponseData {
    fn default() -> Self {
        Self {
            online_players: None,
            max_players: None,
            description: None,
            brand: Some(SERVER_BRAND.to_owned()),
            protocol: PROTOCOL_VERSION,
            player_sample: None,
            favicon: None,
            enforce_secure_chat: None,
        }
    }
}

impl ServerListPingEventResponseData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_online_players(mut self, online_players: i32) -> Self {
        self.online_players = Some(online_players);
        self
    }

    pub fn set_max_players(mut self, max_players: i32) -> Self {
        self.max_players = Some(max_players);
        self
    }

    pub fn set_description(mut self, description: impl Into<TextComponent>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn set_brand(mut self, brand: impl Into<String>) -> Self {
        self.brand = Some(brand.into());
        self
    }

    pub fn set_protocol(mut self, protocol: u16) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn set_player_sample(mut self, player_sample: Vec<PlayerSample>) -> Self {
        self.player_sample = Some(player_sample);
        self
    }

    pub fn set_favicon(mut self, favicon: Favicon) -> Self {
        self.favicon = Some(favicon);
        self
    }

    pub fn set_enforce_secure_chat(mut self, enforce_secure_chat: bool) -> Self {
        self.enforce_secure_chat = if enforce_secure_chat {
            Some(true)
        } else {
            None
        };
        self
    }

    pub fn to_status_response_json(&self, hide_players: bool) -> String {
        let mut root_json_map = Map::new();
        root_json_map.insert("version".to_string(), self.version_json_value());

        if !hide_players {
            insert_if_some(&mut root_json_map, "players", self.players_json_value());
        }

        for (json_key, json_value) in [
            ("description", self.description_json_value()),
            ("favicon", self.favicon_json_value()),
            (
                "enforcesSecureChat",
                self.enforce_secure_chat.map(Value::from),
            ),
        ] {
            insert_if_some(&mut root_json_map, json_key, json_value);
        }

        Value::Object(root_json_map).to_string()
    }

    fn version_json_value(&self) -> Value {
        let mut version_json_map = Map::new();
        version_json_map.insert("protocol".to_string(), json!(self.protocol));
        insert_if_some(
            &mut version_json_map,
            "name",
            self.brand.as_ref().map(|brand| Value::from(brand.as_str())),
        );
        Value::Object(version_json_map)
    }

    fn players_json_value(&self) -> Option<Value> {
        let mut players_json_map = Map::new();

        for (json_key, json_value) in [
            ("max", self.max_players.map(Value::from)),
            ("online", self.online_players.map(Value::from)),
            ("sample", self.player_sample_json_value()),
        ] {
            insert_if_some(&mut players_json_map, json_key, json_value);
        }

        object_json_value_if_not_empty(players_json_map)
    }

    fn player_sample_json_value(&self) -> Option<Value> {
        let player_sample = self.player_sample.as_ref()?;

        if player_sample.is_empty() {
            return None;
        }

        Some(Value::Array(
            player_sample
                .iter()
                .map(|player_sample_entry| {
                    json!({
                        "name": player_sample_entry.name.to_plain_string(),
                        "id": player_sample_entry.uuid.to_string(),
                    })
                })
                .collect(),
        ))
    }

    fn description_json_value(&self) -> Option<Value> {
        let description = self.description.as_ref()?;
        serde_json::to_value(description).ok()
    }

    fn favicon_json_value(&self) -> Option<Value> {
        self.favicon
            .as_ref()
            .map(|favicon| Value::from(favicon.base64.as_str()))
    }
}

fn insert_if_some(map: &mut Map<String, Value>, key: &str, value: Option<Value>) {
    if let Some(actual_value) = value {
        map.insert(key.to_string(), actual_value);
    }
}

fn object_json_value_if_not_empty(map: Map<String, Value>) -> Option<Value> {
    if map.is_empty() {
        return None;
    }

    Some(Value::Object(map))
}
