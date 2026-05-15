use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::inbound_packet_error::InboundPacketErrorEvent},
};

#[event_listener]
fn on_inbound_packet_error(event: &mut InboundPacketErrorEvent, _server: &mut MinecraftServer) {
    let client_address = event.client().addr;
    println!(
        "[InboundPacketError]: Stage={:?}, State={:?}, PacketId={:?}, PacketName={:?}, Client={}, Error={}",
        event.stage, event.state, event.packet_id, event.packet_name, client_address, event.message
    );
}
