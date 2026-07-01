use crate::events::startup::StartupEvent;
use crate::server::MinecraftServer;
use spinel_macros::event_listener;

struct StartupCancellationListener;

#[event_listener]
impl StartupCancellationListener {
    #[event_handler]
    pub fn cancel_startup(event: &mut StartupEvent, _server: &mut MinecraftServer) {
        event.cancelled = true;
    }
}

#[test]
fn registered_event_handler_dispatches_from_server_event_registry() {
    let mut server = MinecraftServer::new();

    server.register_event_handler(StartupCancellationListener);

    assert!(server.on_startup("127.0.0.1", 25565));
}

fn cancel_startup_from_global_handler(event: &mut StartupEvent, _server: &mut MinecraftServer) {
    event.cancelled = true;
}

#[test]
fn global_event_handler_listener_dispatches_from_server_event_registry() {
    let mut server = MinecraftServer::new();

    server
        .get_global_event_handler()
        .add_listener::<StartupEvent>(cancel_startup_from_global_handler);

    assert!(server.on_startup("127.0.0.1", 25565));
}
