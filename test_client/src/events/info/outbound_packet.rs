use spinel::{
    client::{MinecraftClient, events::network::outbound_packet::OutboundPacketEvent},
    macros::event_listener,
};

#[event_listener]
fn on_outbound_packet(event: &mut OutboundPacketEvent, _client: &mut MinecraftClient) {
    let packet_blacklist = vec![
        "client_tick_end",
        "keep_alive",
        "move_player_pos_rot",
        "move_player_pos",
        "move_player_rot",
        "move_entity_pos_rot",
        "rotate_head",
        "block_changed_ack",
        "swing",
    ];

    if packet_blacklist.contains(&event.packet_name.as_str()) {
        return;
    }

    println!(
        "[Serverbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
