use super::super::packet_filter::packet_is_filtered;

#[test]
fn filters_server_inbound_and_outbound_packet_names() {
    assert!(packet_is_filtered("move_entity_pos_rot"));
    assert!(packet_is_filtered("move_entity_pos"));
    assert!(packet_is_filtered("entity_position_sync"));
    assert!(packet_is_filtered("level_chunk_with_light"));
    assert!(packet_is_filtered("chunk_batch_received"));
    assert!(!packet_is_filtered("disconnect"));
}
