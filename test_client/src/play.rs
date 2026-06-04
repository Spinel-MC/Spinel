use spinel::client::MinecraftClient;
use spinel::core::network::clientbound::play::set_health::SetHealthPacket;
use spinel::macros::packet_listener;
use spinel::network::Server;

#[packet_listener(state: spinel::network::ConnectionState::Play)]
fn on_set_health(
    _server: &mut Server,
    packet: SetHealthPacket,
    _client: &mut MinecraftClient,
) -> bool {
    if packet.health <= 0.0 {
        println!("Player is dead. Type `respawn` to respawn.");
    }

    true
}
