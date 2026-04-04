use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::status::status_response::StatusResponsePacket;
use spinel_macros::packet_listener;

#[packet_listener()]
fn on_status_response(
    _client: &mut Server,
    packet: StatusResponsePacket,
    _mc_client: &mut MinecraftClient,
) -> bool {
    println!("Received Status Response:");
    println!("{}", packet.json_response);

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&packet.json_response) {
        if let Some(version) = json.get("version") {
            println!(
                "  Server Version: {}",
                version
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
            );
        }
    }

    true
}
