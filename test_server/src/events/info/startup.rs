use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::startup::StartupEvent},
    utils::Priority,
};

#[event_listener(priority: Priority::High)]
fn on_startup(event: &mut StartupEvent, _server: &mut MinecraftServer) {
    println!(
        "Starting Spinel Server on {}:{}",
        event.bind_address, event.port
    );
}
