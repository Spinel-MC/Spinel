use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::outbound_packet::OutboundEventPacket},
};

#[event_listener]
fn on_outbound_packet(event: &mut OutboundEventPacket, _server: &mut MinecraftServer) {
    if event.packet_name == "level_chunk_with_light" || event.packet_name == "keep_alive" {
        return;
    }

    println!(
        "[Clientbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
