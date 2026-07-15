#[cfg(test)]
mod tests {
    use spinel_server::MinecraftServer;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    spinel_macros::import_module!("test:startup_module");

    #[spinel_macros::fn_event_listener(module: "test:startup_module")]
    fn on_startup(
        _event: &mut spinel_server::events::startup::StartupEvent,
        _server: &mut MinecraftServer,
    ) {
        TEST_LISTENER_CALLED.store(true, Ordering::SeqCst);
    }

    static TEST_LISTENER_CALLED: AtomicBool = AtomicBool::new(false);

    #[test]
    fn test_module_registration() {
        let count = inventory::iter::<&'static spinel_events::RegisteredServerModule>()
            .filter(|module| module.name == "test:startup_module")
            .count();
        assert!(count > 0, "Module was not registered");
    }

    #[tokio::test]
    async fn test_server_module_startup() {
        TEST_LISTENER_CALLED.store(false, Ordering::SeqCst);
        let server = MinecraftServer::new();
        let server_arc = Arc::new(Mutex::new(server));
        {
            let mut server_guard = server_arc.lock().unwrap();
            server_guard.on_startup("127.0.0.1", 25560);
        }
        assert!(
            TEST_LISTENER_CALLED.load(Ordering::SeqCst),
            "Server module startup listener was not called"
        );
    }
}
