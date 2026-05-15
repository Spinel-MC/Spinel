pub fn report_dispatch_result(dispatch_result: std::io::Result<()>, packet_name: &str) {
    if let Err(error) = dispatch_result {
        eprintln!("Failed to dispatch {}: {}", packet_name, error);
    }
}
