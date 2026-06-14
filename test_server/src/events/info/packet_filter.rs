pub fn packet_is_filtered(packet_name: &str, filtered_packet_names: &[&str]) -> bool {
    if std::env::var_os("SPINEL_TRACE_ALL_PACKETS").is_some() {
        return false;
    }
    filtered_packet_names.contains(&packet_name)
}
