use spinel::{
    core::entity::game_mode::GameMode,
    macros::event_listener,
    registry::{ItemStack, Material},
    server::{
        MinecraftServer, entity::PlayerSpawnPoint,
        events::player_configuration::AsyncPlayerConfigurationEvent,
    },
};

#[event_listener()]
fn on_player_configuration(
    event: &mut AsyncPlayerConfigurationEvent,
    server: &mut MinecraftServer,
) {
    let Some(world) = server.world_manager.worlds().first() else {
        return;
    };

    event.set_spawning_world(world.uuid);
    event
        .player()
        .set_respawn_point(PlayerSpawnPoint::new(0.0, 4.0, 0.0, 0.0, 0.0));

    event.player().set_game_mode(GameMode::Survival);
    let _ = event.player().set_permission_level(4);

    event
        .player()
        .inventory()
        .add_item_stack(ItemStack::of(Material::DIAMOND_PICKAXE));

    event
        .player()
        .inventory()
        .add_item_stack(ItemStack::of(Material::DIAMOND_HELMET));

    event
        .player()
        .inventory()
        .add_item_stack(ItemStack::of(Material::DIRT));
}
