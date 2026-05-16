use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::outbound_packet_error::OutboundPacketErrorEvent},
};

#[event_listener]
fn on_outbound_packet_error(event: &mut OutboundPacketErrorEvent, _server: &mut MinecraftServer) {
    let client_address = event.client().addr;
    println!(
        "[OutboundPacketError]: State={:?}, PacketId={:?}, PacketName={:?}, Client={}, Error={}",
        event.state, event.packet_id, event.packet_name, client_address, event.message
    );
}
