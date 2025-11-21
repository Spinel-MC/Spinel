use crate as spinel;

use serde_json::{Map, Value, json};
use spinel_macros::event_dispatcher;
use spinel_utils::component::text::TextComponent;

use crate::core::events::server_list_ping::{
    player_info::PlayerSample, server_list_ping_type::ServerListPingType,
};
use crate::util::constants::{PROTOCOL_VERSION, SERVER_BRAND};

pub mod player_info;
pub mod server_list_ping_type;

#[event_dispatcher(with_client: true)]
pub struct ServerListPingEvent {
    pub response_data: ServerListPingEventResponseData,
    pub server_list_ping_type: ServerListPingType,
    pub hide_players: bool,
    pub cancelled: bool,
}

impl ServerListPingEvent {
    pub fn new(server_list_ping_type: ServerListPingType) -> Self {
        Self {
            response_data: ServerListPingEventResponseData::new(),
            server_list_ping_type,
            cancelled: false,
            hide_players: false,
            client_ptr: None,
        }
    }

    pub fn hide_players(&mut self) {
        self.hide_players = true;
    }
}

#[derive(Default)]
pub struct ServerListPingEventResponseData {
    pub online_players: Option<i32>,
    pub max_players: Option<i32>,
    pub description: Option<TextComponent>,
    pub brand: Option<String>,
    pub protocol: u16,
    pub player_sample: Option<Vec<PlayerSample>>,
    pub favicon: Option<String>,
    pub enforce_secure_chat: Option<bool>,
}

impl ServerListPingEventResponseData {
    pub fn new() -> Self {
        Self {
            online_players: None,
            max_players: None,
            description: None,
            brand: Some(SERVER_BRAND.to_owned()),
            protocol: PROTOCOL_VERSION,
            player_sample: Some(vec![]),
            favicon: None,
            enforce_secure_chat: None,
        }
    }

    pub fn to_status_response_json(&self, hide_players: bool) -> String {
        let mut root_json_map = Map::new();

        let mut version_json_map = Map::new();
        version_json_map.insert("protocol".to_string(), json!(self.protocol));
        insert_if_some(
            &mut version_json_map,
            "name",
            self.brand.as_ref().map(|s| Value::from(s.as_str())),
        );
        root_json_map.insert("version".to_string(), Value::Object(version_json_map));

        if !hide_players {
            let players_json_value = {
                let mut players_json_map = Map::new();
                insert_if_some(
                    &mut players_json_map,
                    "max",
                    self.max_players.map(Value::from),
                );
                insert_if_some(
                    &mut players_json_map,
                    "online",
                    self.online_players.map(Value::from),
                );

                let player_sample_json_list = self.player_sample.as_ref().and_then(|player_sample_vec_ref| {
                                        if player_sample_vec_ref.is_empty() {
                                        None
                                        } else {
                                        let formatted_sample: Vec<Value> = player_sample_vec_ref
                                                .iter() 
                                                .map(|player| json!({"name": player.name.to_plain_string(), "id": player.uuid.to_string()}))
                                                .collect();
                                        Some(Value::Array(formatted_sample))
                                        }
                                });
                insert_if_some(&mut players_json_map, "sample", player_sample_json_list);

                if players_json_map.is_empty() {
                    None
                } else {
                    Some(Value::Object(players_json_map))
                }
            };
            insert_if_some(&mut root_json_map, "players", players_json_value);
        }

        let description_json_value = self.description.as_ref().map(|description_component_ref| {
            serde_json::to_value(description_component_ref).unwrap_or_else(|e| {
                eprintln!("Error serializing description TextComponent: {}", e);
                Value::Null
            })
        });
        insert_if_some(&mut root_json_map, "description", description_json_value);

        insert_if_some(
            &mut root_json_map,
            "favicon",
            self.favicon.as_ref().map(|s| Value::from(s.as_str())),
        );

        insert_if_some(
            &mut root_json_map,
            "enforcesSecureChat",
            self.enforce_secure_chat.map(Value::from),
        );

        Value::Object(root_json_map).to_string()
    }
}

fn insert_if_some(map: &mut Map<String, Value>, key: &str, value: Option<Value>) {
    if let Some(actual_value) = value {
        map.insert(key.to_string(), actual_value);
    }
}
