use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::login::PreLoginEvent},
};

#[event_listener(module: "login")]
fn on_pre_login(event: &mut PreLoginEvent, _server: &mut MinecraftServer) {
    event.should_authenticate = false;
    println!(
        "PreLoginEvent: User {} is attempting to log in.",
        event.name
    );
}
