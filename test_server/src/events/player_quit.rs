use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::player_disconnect::PlayerDisconnectEvent},
};

pub struct PlayerQuitListener;

#[event_listener]
impl PlayerQuitListener {
    #[event_handler]
    pub fn on_player_disconnect(event: &mut PlayerDisconnectEvent, _server: &mut MinecraftServer) {
        println!("{} left the server.", event.player().username);
    }
}
