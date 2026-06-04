use spinel::{
    client::{MinecraftClient, events::network::inbound_packet_error::InboundPacketErrorEvent},
    macros::event_listener,
};

#[event_listener]
fn on_inbound_packet_error(event: &mut InboundPacketErrorEvent, _client: &mut MinecraftClient) {
    let server_address = event.server().addr;

    println!(
        "[InboundPacketError]: Stage={:?}, State={:?}, PacketId={:?}, PacketName={:?}, Server={}, Error={}",
        event.stage, event.state, event.packet_id, event.packet_name, server_address, event.message
    );
}
