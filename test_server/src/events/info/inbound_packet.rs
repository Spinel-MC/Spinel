use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::network::inbound_packet::InboundPacketEvent},
};

#[event_listener]
fn on_inbound_packet(event: &mut InboundPacketEvent, _server: &mut MinecraftServer) {
    if event.packet_name == "client_tick_end"
        || event.packet_name == "keep_alive"
        || event.packet_name == "move_player_pos_rot"
        || event.packet_name == "move_player_pos"
        || event.packet_name == "move_player_rot"
    {
        return;
    }

    println!(
        "[Serverbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}
