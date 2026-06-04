pub fn report_dispatch_result(dispatch_result: std::io::Result<()>, packet_name: &str) -> bool {
    match dispatch_result {
        Ok(()) => true,
        Err(error) => {
            eprintln!("Failed to dispatch {}: {}", packet_name, error);
            false
        }
    }
}
