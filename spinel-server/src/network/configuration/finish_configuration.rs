use crate::entity::Entity;
use crate::entity::Player;
use crate::events::player_configuration::AsyncPlayerConfigurationEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use ::spinel_core::network::serverbound::configuration::finish_configuration::FinishConfigurationPacket;
use ::spinel_macros::packet_listener;
use ::spinel_network::ConnectionState;
use ::spinel_utils::component::Component;
use std::io;
use uuid::Uuid;

#[packet_listener(id: "finish_configuration", state: ConnectionState::Configuration)]
fn on_finish_configuration(
    client: &mut Client,
    _packet: FinishConfigurationPacket,
    server: &mut MinecraftServer,
) -> bool {
    println!("Client acknowledged finish configuration. Transitioning to Play state.");
    let player_was_configured = configure_player(client, server);
    if player_was_configured {
        client.state = ConnectionState::Play;
        return dispatch_play_packets(client, server).is_ok();
    }

    false
}

fn configure_player(client: &mut Client, server: &mut MinecraftServer) -> bool {
    let Some(player) = create_player(client) else {
        let _ = server.disconnect(client, Component::text("Invalid login sequence."));
        return false;
    };

    let event = dispatch_player_configuration_event(client, server, player);
    let Some(spawning_world) = require_spawning_world(client, server, &event) else {
        return false;
    };

    place_player(client, server, spawning_world, event.into_player())
}

fn dispatch_player_configuration_event(
    client: &mut Client,
    server: &mut MinecraftServer,
    player: Player,
) -> AsyncPlayerConfigurationEvent {
    let mut event = AsyncPlayerConfigurationEvent::new(player, true);
    tokio::runtime::Handle::current().block_on(event.dispatch(server, client));
    event
}

fn require_spawning_world(
    client: &mut Client,
    server: &mut MinecraftServer,
    event: &AsyncPlayerConfigurationEvent,
) -> Option<Uuid> {
    let Some(spawning_world) = event.spawning_world() else {
        let _ = server.disconnect(
            client,
            Component::text(
                "You need to specify a spawning world in the AsyncPlayerConfigurationEvent.",
            ),
        );
        return None;
    };

    Some(spawning_world)
}

fn place_player(
    client: &mut Client,
    server: &mut MinecraftServer,
    spawning_world: Uuid,
    player: Player,
) -> bool {
    if add_configured_player(server, spawning_world, player) {
        return true;
    }

    let _ = server.disconnect(client, Component::text("Spawning world is not registered."));
    false
}

fn create_player(client: &Client) -> Option<Player> {
    let login_metadata = client.login_metadata.as_ref()?;
    Some(Player::new(
        login_metadata.uuid?,
        login_metadata.username.clone()?,
        login_metadata.protocol_version,
        client.addr,
    ))
}

fn add_configured_player(
    server: &mut MinecraftServer,
    spawning_world: Uuid,
    player: Player,
) -> bool {
    if server.world_manager.world(spawning_world).is_none() {
        return false;
    }

    server
        .world_manager
        .add_entity(spawning_world, Entity::Player(player))
}

fn dispatch_play_packets(client: &mut Client, server: &mut MinecraftServer) -> io::Result<()> {
    server
        .world_manager
        .enter_player(client, server.ticks_per_second)
}
