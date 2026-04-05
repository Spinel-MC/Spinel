use std::fs;

use spinel::{
    macros::event_listener,
    server::{
        MinecraftServer,
        events::server_list_ping::{
            event::ServerListPingEvent, favicon::Favicon, player_sample::PlayerSample, response_data::ServerListPingEventResponseData
        },
    },
    utils::{
        Priority,
        component::{
            Component,
            color::{NamedTextColor, TextColor},
        },
        constants::{PROTOCOL_VERSION, SERVER_BRAND},
    },
};
use uuid::Uuid;


#[event_listener(priority: Priority::High)]
fn on_event(event: &mut ServerListPingEvent, _server: &mut MinecraftServer) {
    let sample = vec![PlayerSample::new(
        Component::text("A Spinel Server".to_string())
            .color(TextColor::from_named(NamedTextColor::Aqua))
            .into(),
        Uuid::new_v4(),
    )];

    event.response_data = ServerListPingEventResponseData {
        online_players: Some(0),
        max_players: Some(100),
        description: Some(
            Component::text("Minecraft, your way!".to_owned())
                .color(TextColor::from_hex("#ff47d7".to_owned()))
                .into(),
        ),
        brand: Some(SERVER_BRAND.to_owned()),
        protocol: PROTOCOL_VERSION,
        player_sample: Some(sample),
        favicon: Some(Favicon::from_bytes(fs::read("test_server/assets/favicon.png").unwrap())),
        enforce_secure_chat: Some(true),
    };
}
