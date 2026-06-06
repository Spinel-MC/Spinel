use spinel::{
    client::{MinecraftClient, events::network::outbound_packet::OutboundPacketEvent},
    macros::event_listener,
};

use crate::events::info::packet_filter::packet_is_filtered;

#[event_listener]
fn on_outbound_packet(event: &mut OutboundPacketEvent, _client: &mut MinecraftClient) {
    if packet_is_filtered(&event.packet_name) {
        return;
    }

    println!(
        "[Serverbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
