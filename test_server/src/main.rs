use spinel::{
    core::{
        events::{
            login::PreLoginEvent,
            server_list_ping::{
                ServerListPingEvent, ServerListPingEventResponseData, player_info::PlayerSample,
            },
        },
        server::MinecraftServer,
    },
    spinel_macros::{event_listener, import_module},
    utils::{
        Priority,
        component::{
            Component,
            color::{NamedTextColor, TextColor},
        },
    },
};
use uuid::Uuid;

import_module!("minecraft:server_list_ping");
import_module!("minecraft:login");

//END GOAL: Recreate Bedwars.

#[tokio::main]
async fn main() {
    println!("server starting on port 25565!");

    let server = MinecraftServer::new();

    server.start("127.0.0.1", 25565).await;
}

#[event_listener(event: "login", module: "login")]
fn on_pre_login(event: &mut PreLoginEvent, _server: &mut MinecraftServer) {
    event.should_authenticate = false; // false for now, until auth is implemented
    println!(
        "PreLoginEvent: User {} is attempting to log in.",
        event.name
    );
}

#[event_listener(event: "server_list_ping", priority: Priority::High)]
fn on_event(event: &mut ServerListPingEvent, _server: &mut MinecraftServer) {
    println!("{}", event.client().addr);

    let sample = vec![PlayerSample::new(
        Component::text("lol".to_string())
            .color(TextColor::from_named(NamedTextColor::Aqua))
            .into(),
        Uuid::new_v4(),
    )];

    event.response_data = ServerListPingEventResponseData {
        online_players: Some(0),
        max_players: Some(-1),
        description: Some(
            Component::text("Minecraft, your way!".to_owned())
                .color(TextColor {
                    value: "#ff47d7".to_owned(),
                })
                .into(),
        ),
        brand: Some("Spinel".to_owned()),
        protocol: 772,
        player_sample: Some(sample),
        favicon: None,
        enforce_secure_chat: Some(true),
    };
}
