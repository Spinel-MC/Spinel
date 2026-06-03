use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::outbound_packet::OutboundEventPacket},
};

#[event_listener]
fn on_outbound_packet(event: &mut OutboundEventPacket, _server: &mut MinecraftServer) {
    let packet_blacklist = vec!["level_chunk_with_light", "keep_alive", "set_time"];

    if packet_blacklist.contains(&event.packet_name.as_str()) {
        return;
    }

    println!(
        "[Clientbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
