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
    FILTERED_PACKET_NAMES.contains(&packet_name)
}

#[cfg(test)]
mod tests {
    use super::packet_is_filtered;

    #[test]
    fn filters_server_inbound_and_outbound_packet_names() {
        assert!(packet_is_filtered("move_entity_pos_rot"));
        assert!(packet_is_filtered("move_entity_pos"));
        assert!(packet_is_filtered("entity_position_sync"));
        assert!(packet_is_filtered("level_chunk_with_light"));
        assert!(packet_is_filtered("chunk_batch_received"));
        assert!(!packet_is_filtered("disconnect"));
    }
}
