use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::player_game_mode_request::PlayerGameModeRequestEvent},
};

#[event_listener]
pub fn on_game_mode_change(event: &mut PlayerGameModeRequestEvent, _server: &mut MinecraftServer) {
    let game_mode = event.requested_game_mode();

    event.player().set_game_mode(game_mode);
}
