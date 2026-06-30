#[cfg(test)]
mod tests {
    use spinel_server::MinecraftServer;
    use std::sync::{Arc, Mutex};
    spinel_macros::import_module!("test:startup_module");
    #[spinel_macros::event_listener(module: "test:startup_module")]
    fn on_startup(
        _event: &mut spinel_server::events::startup::StartupEvent,
        _server: &mut MinecraftServer,
    ) {
        unsafe {
            TEST_LISTENER_CALLED = true;
        }
    }

    static mut TEST_LISTENER_CALLED: bool = false;

    #[test]
    fn test_module_registration() {
        let count = inventory::iter::<&'static spinel_events::RegisteredServerModule>()
            .filter(|m| m.name == "test:startup_module")
            .count();
        assert!(count > 0, "Module was not registered");
    }

    #[tokio::test]
    async fn test_server_module_startup() {
        unsafe {
            TEST_LISTENER_CALLED = false;
        }
        let server = MinecraftServer::new();
        let server_arc = Arc::new(Mutex::new(server));
        {
            let mut server_guard = server_arc.lock().unwrap();
            server_guard.on_startup("127.0.0.1", 25560);
        }
        unsafe {
            assert!(
                TEST_LISTENER_CALLED,
                "Server module startup listener was not called"
            );
        }
    }
}
