use spinel::network::Recipient;
use spinel::{
    client::{MinecraftClient, events::network::packet::PacketEvent},
    macros::event_listener,
};

use crate::events::info::packet_filter::packet_is_filtered;

#[event_listener]
fn on_outbound_packet(event: &mut PacketEvent, _client: &mut MinecraftClient) {
    if event.recipient != Recipient::Server || packet_is_filtered(&event.packet_name) {
        return;
    }

    println!(
        "[Serverbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
