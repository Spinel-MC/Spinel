use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::player_spawn::PlayerSpawnEvent},
};

pub struct PlayerJoinListener;

#[event_listener]
impl PlayerJoinListener {
    #[event_handler]
    pub fn on_player_spawn(event: &mut PlayerSpawnEvent, _server: &mut MinecraftServer) {
        if !event.first_spawn() {
            return;
        }

        println!("{} joined the server.", event.player().username);
    }
}
