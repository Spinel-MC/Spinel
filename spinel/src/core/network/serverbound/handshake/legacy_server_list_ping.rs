use crate::core::events::server_list_ping::{server_list_ping_type::ServerListPingType, ServerListPingEvent};
use crate::core::network::clientbound::handshake::legacy_server_list_ping::dispatch_legacy_server_list_ping_response;
use crate::core::server::MinecraftServer;
use crate as spinel;
use spinel_macros::packet_listener;
use spinel_network::Client;
use std::io::{Read};

#[packet_listener(id: 0xFE, state: "Handshaking", module: "legacy_server_list_ping")]
pub fn handle_legacy_server_list_ping(client: &mut Client, server: &mut MinecraftServer) -> bool {
    client.stream.set_nonblocking(true).unwrap_or_else(|_| ());

    let mut is_versioned = false;
    let mut buf = [0u8; 1];

    if let Ok(1) = client.stream.read(&mut buf) {
        if buf[0] == 0x01 {
            is_versioned = true;
        }
    }

    client.stream.set_nonblocking(false).unwrap_or_else(|_| ());

    let ping_type = if is_versioned {
        ServerListPingType::LegacyVersioned
    } else {
        ServerListPingType::LegacyUnversioned
    };

    let mut event = ServerListPingEvent::new(ping_type);
    event.dispatch(server, client);
    
    if event.cancelled {
        return true;
    }

    dispatch_legacy_server_list_ping_response(event, is_versioned, client);

    client.disconnect();
    true
}