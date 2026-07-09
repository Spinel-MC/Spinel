use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::packet_error::PacketErrorEvent},
};

pub struct PacketErrorInfoListener;

#[event_listener]
impl PacketErrorInfoListener {
    #[event_handler]
    pub fn on_packet_error(event: &mut PacketErrorEvent, _server: &mut MinecraftServer) {
        let client_address = event.client().addr;
        println!(
            "[PacketError]: Recipient={:?}, Stage={:?}, State={:?}, PacketId={:?}, PacketName={:?}, Client={}, Error={} ",
            event.recipient,
            event.stage,
            event.state,
            event.packet_id,
            event.packet_name,
            client_address,
            event.message
        );
    }
}
