use spinel::{
    client::{MinecraftClient, events::network::inbound_packet::InboundPacketEvent},
    macros::event_listener,
};

use crate::events::info::packet_filter::packet_is_filtered;

#[event_listener]
fn on_inbound_packet(event: &mut InboundPacketEvent, _client: &mut MinecraftClient) {
    if packet_is_filtered(&event.packet_name) {
        return;
    }

    println!(
        "[Clientbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
