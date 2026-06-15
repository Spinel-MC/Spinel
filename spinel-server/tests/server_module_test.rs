#[cfg(test)]
mod tests {
    use spinel_server::MinecraftServer;
    use std::sync::{Arc, Mutex};

    // Test module that will be registered - now using import_module!
    spinel_macros::import_module!("test:startup_module");

    // Event listener for startup
    #[spinel_macros::event_listener(module: "test:startup_module")]
    fn on_startup(
        _event: &mut spinel_server::events::startup::StartupEvent,
        _server: &mut MinecraftServer,
    ) {
        // Set a flag to indicate the listener was called
        unsafe {
            TEST_LISTENER_CALLED = true;
        }
        println!("TEST: Startup listener was called!");
    }

    static mut TEST_LISTENER_CALLED: bool = false;

    #[test]
    fn test_module_registration() {
        // Check if the module was registered
        let count = inventory::iter::<&'static spinel_events::RegisteredServerModule>()
            .filter(|m| m.name == "test:startup_module")
            .count();
        println!(
            "Found {} RegisteredServerModule entries for 'test:startup_module'",
            count
        );
        assert!(count > 0, "Module was not registered");
    }

    #[tokio::test]
    async fn test_server_module_startup() {
        // Reset the flag
        unsafe {
            TEST_LISTENER_CALLED = false;
        }

        // Create and start server
        let server = MinecraftServer::new();
        let server_arc = Arc::new(Mutex::new(server));

        // Trigger startup event
        {
            let mut server_guard = server_arc.lock().unwrap();
            server_guard.on_startup();
        }

        // Check if the listener was called
        unsafe {
            assert!(
                TEST_LISTENER_CALLED,
                "Server module startup listener was not called"
            );
        }
    }
}
