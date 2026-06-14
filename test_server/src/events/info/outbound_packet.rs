use crate::events::info::packet_filter::packet_is_filtered;
use spinel::network::Recipient;
use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::packet::PacketEvent},
};

#[event_listener]
fn on_outbound_packet(event: &mut PacketEvent, _server: &mut MinecraftServer) {
    let filtered_packet_names = [
        "level_chunk_with_light",
        "chunk_batch_start",
        "chunk_batch_finished",
        "forget_level_chunk",
        "set_chunk_cache_center",
        "keep_alive",
        "set_time",
        "move_entity_pos_rot",
        "move_entity_pos",
        "entity_position_sync",
    ];

    if event.recipient != Recipient::Client
        || packet_is_filtered(&event.packet_name, &filtered_packet_names)
    {
        return;
    }

    println!(
        "[Clientbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
