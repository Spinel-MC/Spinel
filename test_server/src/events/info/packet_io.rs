use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::packet_io::PacketIoEvent},
};

#[event_listener]
fn on_packet_io(event: &mut PacketIoEvent, _server: &mut MinecraftServer) {
    println!(
        "[{}]: State={:?}, ID={:#04X}, resource=\"{}\", PayloadSize={}",
        event.direction.as_label(),
        event.state,
        event.id,
        event.packet_name,
        event.payload_size
    );
}
