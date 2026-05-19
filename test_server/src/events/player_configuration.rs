use spinel::{
    core::entity::game_mode::GameMode,
    macros::event_listener,
    server::{
        MinecraftServer, entity::PlayerSpawnPoint,
        events::player_configuration::AsyncPlayerConfigurationEvent,
    },
};

#[event_listener(module: "login")]
fn on_player_configuration(
    event: &mut AsyncPlayerConfigurationEvent,
    server: &mut MinecraftServer,
) {
    let Some(world) = server.world_manager.worlds().first() else {
        return;
    };

    event.set_spawning_world(world.uuid);
    event.player().set_game_mode(GameMode::Creative);
    event
        .player()
        .set_respawn_point(PlayerSpawnPoint::new(0.0, 4.0, 0.0, 0.0, 0.0));
}
