use crate::{
    self as spinel,
    core::{events::intention::IntentionEvent, server::MinecraftServer},
};
use spinel_macros::packet_listener;
use spinel_network::{Client, client::instance::ConnectionState};

#[packet_listener(id: 0x00, state: "Handshaking", module: "intention", fields:(
        protocol_version: VarInt,
        server_address: String,
        server_port: UnsignedShort,
        intention: VarInt,
))]
fn on_intention(client: &mut Client, packet: Packet, server: &mut MinecraftServer) -> bool {
    let mut event = IntentionEvent::new(
        packet.protocol_version,
        packet.server_address,
        packet.server_port,
        packet.intention,
    );

    event.dispatch(server, client);

    if event.cancelled {
        return true;
    }

    match packet.intention {
        1 => client.state = ConnectionState::Status,
        2 => client.state = ConnectionState::Login,
        3 => client.disconnect(), //TODO: Implement transfer intention.
        _ => {
            return false;
        }
    }

    true
}
