use crate::core::events::server_list_ping::ServerListPingEvent;
use spinel_network::Client;

pub fn dispatch_legacy_server_list_ping_response(event: ServerListPingEvent, supports_versions: bool, client: &mut Client){
    let data = event.response_data;

    let payload_str = if supports_versions {
        format!("§1\u{0000}{}\u{0000}{}\u{0000}{}\u{0000}{}\u{0000}{}",
            127,
            data.brand.unwrap_or_default(),
            data.description.unwrap_or_default().to_legacy_string(),
            data.online_players.unwrap_or_default(),
            data.max_players.unwrap_or_default()
        )
    } else {
        format!("{}§{}§{}",
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
    
    client.send_raw_bytes(&packet_bytes);
}