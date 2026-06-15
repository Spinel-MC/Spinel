#[cfg(test)]
mod tests {
    // Test module that will be registered - using import_module!
    spinel_macros::import_module!("test:client_module");

    #[test]
    fn test_module_registration() {
        // Check if the module was registered
        let count = inventory::iter::<&'static spinel_events::RegisteredClientModule>()
            .filter(|m| m.name == "test:client_module")
            .count();
        println!(
            "Found {} RegisteredClientModule entries for 'test:client_module'",
            count
        );
        assert!(count > 0, "Module was not registered");
    }
}
