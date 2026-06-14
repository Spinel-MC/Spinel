use spinel::{
    client::{MinecraftClient, events::network::packet_error::PacketErrorEvent},
    macros::event_listener,
};

#[event_listener]
fn on_packet_error(event: &mut PacketErrorEvent, _client: &mut MinecraftClient) {
    let server_address = event.server().addr;

    println!(
        "[PacketError]: Recipient={:?}, Stage={:?}, State={:?}, PacketId={:?}, PacketName={:?}, Server={}, Error={}",
        event.recipient,
        event.stage,
        event.state,
        event.packet_id,
        event.packet_name,
        server_address,
        event.message
    );
}
