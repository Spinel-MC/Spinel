use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::startup::StartupEvent},
    utils::Priority,
};

#[event_listener(priority: Priority::High)]
fn on_startup(_event: &mut StartupEvent, _server: &mut MinecraftServer) {
    println!("Server is booting up...");
}
