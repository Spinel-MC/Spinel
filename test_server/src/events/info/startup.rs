use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::startup::StartupEvent},
    utils::Priority,
};

pub struct StartupInfoListener;

#[event_listener]
impl StartupInfoListener {
    #[event_handler(priority: Priority::High)]
    pub fn on_startup(event: &mut StartupEvent, _server: &mut MinecraftServer) {
        println!(
            "Starting Spinel Server on {}:{}",
            event.bind_address, event.port
        );
    }
}
