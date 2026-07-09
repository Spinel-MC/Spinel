use crate::events::server_list_ping::response_data::ServerListPingEventResponseData;
use serde_json::Value;
use spinel_utils::component::Component;

#[test]
fn manually_constructed_response_data_omits_enforces_secure_chat_when_none() {
    let response_data = ServerListPingEventResponseData {
        online_players: Some(0),
        max_players: Some(100),
        description: Some(Component::text("A Spinel Server").into()),
        brand: None,
        protocol: 10,
        player_sample: None,
        favicon: None,
        enforce_secure_chat: None,
    };

    let status_response = parse_status_response(response_data);

    assert!(status_response.get("enforcesSecureChat").is_none());
}

#[test]
fn builder_constructed_response_data_omits_enforces_secure_chat_when_setter_is_not_called() {
    let response_data = ServerListPingEventResponseData::new()
        .set_online_players(0)
        .set_max_players(100)
        .set_description(Component::text("A Spinel Server"))
        .set_protocol(10);

    let status_response = parse_status_response(response_data);

    assert!(status_response.get("enforcesSecureChat").is_none());
}

#[test]
fn builder_constructed_response_data_omits_enforces_secure_chat_when_false_is_set() {
    let response_data = ServerListPingEventResponseData::new()
        .set_online_players(0)
        .set_max_players(100)
        .set_description(Component::text("A Spinel Server"))
        .set_protocol(10)
        .set_enforce_secure_chat(false);

    let status_response = parse_status_response(response_data);

    assert!(status_response.get("enforcesSecureChat").is_none());
}

#[test]
fn builder_constructed_response_data_includes_enforces_secure_chat_when_true_is_set() {
    let response_data = ServerListPingEventResponseData::new()
        .set_online_players(0)
        .set_max_players(100)
        .set_description(Component::text("A Spinel Server"))
        .set_protocol(10)
        .set_enforce_secure_chat(true);

    let status_response = parse_status_response(response_data);

    assert_eq!(
        status_response.get("enforcesSecureChat"),
        Some(&Value::Bool(true))
    );
}

fn parse_status_response(response_data: ServerListPingEventResponseData) -> Value {
    serde_json::from_str(&response_data.to_status_response_json(false)).unwrap()
}
