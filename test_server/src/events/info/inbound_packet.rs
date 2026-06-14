use spinel::network::Recipient;
use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::packet::PacketEvent},
};

use crate::events::info::packet_filter::packet_is_filtered;

#[event_listener]
fn on_inbound_packet(event: &mut PacketEvent, _server: &mut MinecraftServer) {
    let filtered_packet_names = [
        "client_tick_end",
        "keep_alive",
        "move_player_pos_rot",
        "move_player_pos",
        "move_player_rot",
        "move_entity_pos_rot",
        "rotate_head",
        "block_changed_ack",
        "swing",
        "chunk_batch_received",
    ];

    if event.recipient != Recipient::Server
        || packet_is_filtered(&event.packet_name, &filtered_packet_names)
    {
        return;
    }

    println!(
        "[Serverbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
