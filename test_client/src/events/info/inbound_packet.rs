use spinel::{
    client::{MinecraftClient, events::network::inbound_packet::InboundPacketEvent},
    macros::event_listener,
};

#[event_listener]
fn on_inbound_packet(event: &mut InboundPacketEvent, _client: &mut MinecraftClient) {
    let packet_blacklist = vec!["level_chunk_with_light", "keep_alive", "set_time"];

    if packet_blacklist.contains(&event.packet_name.as_str()) {
        return;
    }

    println!(
        "[Clientbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
