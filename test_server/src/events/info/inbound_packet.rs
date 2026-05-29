use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::inbound_packet::InboundPacketEvent},
};

#[event_listener]
fn on_inbound_packet(event: &mut InboundPacketEvent, _server: &mut MinecraftServer) {
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
