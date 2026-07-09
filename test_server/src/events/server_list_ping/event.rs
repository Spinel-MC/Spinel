use std::fs;

use spinel::{
    macros::event_listener,
    server::{
        MinecraftServer,
        events::server_list_ping::{
            event::ServerListPingEvent, favicon::Favicon, player_sample::PlayerSample,
            response_data::ServerListPingEventResponseData,
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

pub struct ServerListPingListener;

#[event_listener]
impl ServerListPingListener {
    #[event_handler(priority: Priority::High)]
    pub fn on_event(event: &mut ServerListPingEvent, _server: &mut MinecraftServer) {
        let sample = vec![PlayerSample::new(
            Component::text("A Spinel Server")
                .color(TextColor::from_named(NamedTextColor::Aqua))
                .into(),
            Uuid::new_v4(),
        )];

        event.response_data = ServerListPingEventResponseData::new()
            .set_online_players(0)
            .set_max_players(100)
            .set_description(
                Component::text("Minecraft, your way!")
                    .color(TextColor::from_hex("#ff47d7".to_owned())),
            )
            .set_brand(SERVER_BRAND)
            .set_protocol(PROTOCOL_VERSION)
            .set_player_sample(sample)
            .set_favicon(Favicon::from_bytes(
                fs::read("test_server/assets/favicon.png").unwrap(),
            ))
            .set_enforce_secure_chat(true);
    }
}
