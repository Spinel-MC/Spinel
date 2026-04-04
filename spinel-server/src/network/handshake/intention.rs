use crate::events::intention::IntentionEvent;
use crate::instance::MinecraftServer;
use crate::network::client::instance::Client;
use spinel_core::network::serverbound::handshake::intention::IntentionPacket;
use spinel_macros::packet_listener;
use spinel_network::ConnectionState;

#[packet_listener(id: "intention", state: ConnectionState::Handshaking, module: "intention")]
fn on_intention(
    client: &mut Client,
    packet: IntentionPacket,
    server: &mut MinecraftServer,
) -> bool {
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
        3 => client.disconnect(),
        _ => {
            return false;
        }
    }

    true
}
