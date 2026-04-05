use crate::MinecraftServer;
use crate::events::server_list_ping::event::ServerListPingEvent;
use crate::events::server_list_ping::ping_type::ServerListPingType;
use crate::network::client::instance::Client;
use spinel_core::network::serverbound::handshake::legacy_server_list_ping::LegacyServerListPingPacket;
use spinel_macros::packet_listener;
use std::io::Read;

#[packet_listener(module: "legacy_server_list_ping")]
pub fn handle_legacy_server_list_ping(
    client: &mut Client,
    _packet: LegacyServerListPingPacket,
    server: &mut MinecraftServer,
) -> bool {
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

pub fn dispatch_legacy_server_list_ping_response(
    event: ServerListPingEvent,
    supports_versions: bool,
    client: &mut Client,
) {
    let data = event.response_data;

    let payload_str = if supports_versions {
        format!(
            "§1\u{0000}{}\u{0000}{}\u{0000}{}\u{0000}{}\u{0000}{}",
            127,
            data.brand.unwrap_or_default(),
            data.description.unwrap_or_default().to_legacy_string(),
            data.online_players.unwrap_or_default(),
            data.max_players.unwrap_or_default()
        )
    } else {
        format!(
            "{}§{}§{}",
            data.description.unwrap_or_default().to_legacy_string(),
            data.online_players.unwrap_or_default(),
            data.max_players.unwrap_or_default()
        )
    };

    let mut packet_bytes = Vec::new();
    packet_bytes.push(0xFF);

    let utf16_chars: Vec<u16> = payload_str.encode_utf16().collect();
    packet_bytes.extend_from_slice(&(utf16_chars.len() as u16).to_be_bytes());
    for &char_code in &utf16_chars {
        packet_bytes.extend_from_slice(&char_code.to_be_bytes());
    }

    let _ = client.send_raw_bytes(&packet_bytes);
}
