const FILTERED_PACKET_NAMES: [&str; 14] = [
    "client_tick_end",
    "keep_alive",
    "move_player_pos_rot",
    "move_player_pos",
    "move_player_rot",
    "move_entity_pos_rot",
    "move_entity_pos",
    "entity_position_sync",
    "rotate_head",
    "block_changed_ack",
    "swing",
    "level_chunk_with_light",
    "chunk_batch_received",
    "set_time",
];

pub fn packet_is_filtered(packet_name: &str) -> bool {
    if std::env::var_os("SPINEL_TRACE_ALL_PACKETS").is_some() {
        return false;
    }
    FILTERED_PACKET_NAMES.contains(&packet_name)
}
