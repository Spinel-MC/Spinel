use super::super::player_info_update::{
    PlayerInfoActions, PlayerInfoProperty, PlayerInfoUpdatePacket,
};
use crate::entity::game_mode::GameMode;
use spinel_network::DataType;
use spinel_network::wrappers::JsonTextComponent;
use spinel_utils::component::Component;
use uuid::Uuid;

#[test]
fn player_info_update_add_listed_player_matches_minestom_action_shape() {
    let packet = PlayerInfoUpdatePacket::add_listed_player(Uuid::nil(), "Player");
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerInfoUpdatePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(PlayerInfoUpdatePacket::get_id(), 0x44);
    assert_eq!(payload[0], 9);
    assert_eq!(decoded_packet.actions, PlayerInfoActions(9));
    assert_eq!(decoded_packet.entries.0[0].uuid, Uuid::nil());
    assert_eq!(decoded_packet.entries.0[0].username, "Player");
    assert!(decoded_packet.entries.0[0].listed);
}

#[test]
fn player_info_update_game_mode_matches_minestom_action_shape() {
    let packet = PlayerInfoUpdatePacket::update_game_mode(Uuid::nil(), GameMode::Creative);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(PlayerInfoUpdatePacket::get_id(), 0x44);
    assert_eq!(payload[0], PlayerInfoActions::UPDATE_GAME_MODE);
    assert_eq!(payload[1], 1);
    assert_eq!(&payload[2..18], Uuid::nil().as_bytes());
    assert_eq!(payload[18], GameMode::Creative.id());
}

#[test]
fn player_info_update_listed_matches_minestom_action_shape() {
    let packet = PlayerInfoUpdatePacket::update_listed(Uuid::nil(), true);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(payload[0], PlayerInfoActions::UPDATE_LISTED);
    assert_eq!(payload[1], 1);
    assert_eq!(&payload[2..18], Uuid::nil().as_bytes());
    assert_eq!(payload[18], 1);
}

#[test]
fn player_info_update_latency_matches_minestom_action_shape() {
    let packet = PlayerInfoUpdatePacket::update_latency(Uuid::nil(), 42);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(payload[0], PlayerInfoActions::UPDATE_LATENCY);
    assert_eq!(payload[1], 1);
    assert_eq!(&payload[2..18], Uuid::nil().as_bytes());
    assert_eq!(payload[18], 42);
}

#[test]
fn player_info_update_add_player_encodes_profile_properties() {
    let packet = PlayerInfoUpdatePacket::add_player_with_properties(
        Uuid::nil(),
        "Player",
        true,
        vec![PlayerInfoProperty {
            name: "textures".to_string(),
            value: "texture-data".to_string(),
            signature: Some("signature-data".to_string()),
        }],
    );
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerInfoUpdatePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.entries.0[0].properties.len(), 1);
    assert_eq!(decoded_packet.entries.0[0].properties[0].name, "textures");
    assert_eq!(
        decoded_packet.entries.0[0].properties[0].value,
        "texture-data"
    );
    assert_eq!(
        decoded_packet.entries.0[0].properties[0]
            .signature
            .as_deref(),
        Some("signature-data")
    );
}

#[test]
fn player_info_update_display_name_matches_minestom_action_shape() {
    let display_name = Component::text("Display").build();
    let packet = PlayerInfoUpdatePacket::update_display_name(Uuid::nil(), Some(display_name));
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerInfoUpdatePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(payload[0], PlayerInfoActions::UPDATE_DISPLAY_NAME);
    assert_eq!(decoded_packet.entries.0[0].uuid, Uuid::nil());
    assert!(decoded_packet.entries.0[0].display_name.is_some());
}

#[test]
fn player_info_update_display_name_none_encodes_optional_absent() {
    let packet = PlayerInfoUpdatePacket::update_display_name(Uuid::nil(), None);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(payload[0], PlayerInfoActions::UPDATE_DISPLAY_NAME);
    assert_eq!(payload[18], 0);
}

#[test]
fn player_info_update_list_order_matches_minestom_action_shape() {
    let packet = PlayerInfoUpdatePacket::update_list_order(Uuid::nil(), 12);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerInfoUpdatePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(payload[0], PlayerInfoActions::UPDATE_LIST_ORDER);
    assert_eq!(decoded_packet.entries.0[0].list_order, 12);
}

#[test]
fn player_info_update_hat_matches_minestom_action_shape() {
    let packet = PlayerInfoUpdatePacket::update_hat(Uuid::nil(), false);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerInfoUpdatePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(payload[0], PlayerInfoActions::UPDATE_HAT);
    assert!(!decoded_packet.entries.0[0].display_hat);
}

#[test]
fn player_info_update_combined_actions_encode_in_enum_order() {
    let packet = PlayerInfoUpdatePacket {
        actions: PlayerInfoActions(
            PlayerInfoActions::UPDATE_GAME_MODE
                | PlayerInfoActions::UPDATE_LISTED
                | PlayerInfoActions::UPDATE_DISPLAY_NAME,
        ),
        entries: spinel_network::types::Array(vec![
            super::super::player_info_update::PlayerInfoEntry {
                uuid: Uuid::nil(),
                username: String::new(),
                properties: Vec::new(),
                listed: true,
                latency: 0,
                game_mode: GameMode::Adventure,
                display_name: Some(Component::text("Display").build()),
                list_order: 0,
                display_hat: true,
            },
        ]),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerInfoUpdatePacket::decode(&mut payload.as_slice()).unwrap();
    let display_name = JsonTextComponent(decoded_packet.entries.0[0].display_name.clone().unwrap());
    let mut display_name_payload = Vec::new();
    display_name.encode(&mut display_name_payload).unwrap();

    assert_eq!(payload[0], 44);
    assert_eq!(payload[18], GameMode::Adventure.id());
    assert_eq!(payload[19], 1);
    assert!(payload[20] != 0);
    assert_eq!(decoded_packet.entries.0[0].game_mode, GameMode::Adventure);
    assert!(decoded_packet.entries.0[0].listed);
}
