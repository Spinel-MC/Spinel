use spinel::client::MinecraftClient;
use spinel::core::network::clientbound::configuration::finish_configuration::FinishConfigurationPacket as S2CFinishConfig;
use spinel::core::network::clientbound::configuration::known_packs::KnownPacksPacket as S2CKnownPacks;
use spinel::core::network::clientbound::configuration::registry_data::RegistryDataPacket;
use spinel::core::network::clientbound::configuration::update_tags::UpdateTagsPacket;
use spinel::core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel::core::network::clientbound::login::encryption_request::EncryptionRequestPacket;
use spinel::core::network::clientbound::login::login_success::LoginSuccessPacket;
use spinel::core::network::clientbound::login::set_compression::SetCompressionPacket;
use spinel::core::network::clientbound::status::pong_response::PongResponsePacket;
use spinel::core::network::clientbound::status::status_response::StatusResponsePacket;
use spinel::core::network::serverbound::configuration::client_information::ClientInformationPacket;
use spinel::core::network::serverbound::configuration::finish_configuration::FinishConfigurationPacket as ConfigFinishPacket;
use spinel::core::network::serverbound::configuration::known_packs::KnownPacksPacket as ConfigKnownPacksPacket;
use spinel::core::network::serverbound::handshake::intention::IntentionPacket;
use spinel::core::network::serverbound::login::login_acknowledge::LoginAcknowledgedPacket;
use spinel::core::network::serverbound::login::login_start::LoginStartPacket;
use spinel::core::network::serverbound::status::ping_request::PingRequestPacket;
use spinel::core::network::serverbound::status::status_request::StatusRequestPacket;
use spinel::macros::packet_listener;
use spinel::network::{ConnectionState, Server};

use spinel::utils::constants::PROTOCOL_VERSION;
use spinel::uuid::Uuid;

pub use spinel::client::ClientPacketListener;

#[tokio::main]
async fn main() {
    println!("=== Spinel Test Client ===\n");

    let mut client = MinecraftClient::new();

    println!("--- STATUS PING START ---");
    client.connect("127.0.0.1", 25565).await;

    IntentionPacket {
        protocol_version: PROTOCOL_VERSION as i32,
        server_address: "127.0.0.1".to_string(),
        server_port: 25565,
        intention: 1,
    }
    .dispatch(&mut client);

    client.state = ConnectionState::Status;

    println!("Sending Status Request...");
    StatusRequestPacket {}.dispatch(&mut client);

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    println!("Sending Ping Request...");
    PingRequestPacket {
        timestamp: 123456789,
    }
    .dispatch(&mut client);

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    client.disconnect().await;
    println!("--- STATUS PING END ---\n");

    println!("--- JOIN START ---");
    client.connect("127.0.0.1", 25565).await;

    client.state = ConnectionState::Handshaking;
    IntentionPacket {
        protocol_version: PROTOCOL_VERSION as i32,
        server_address: "127.0.0.1".to_string(),
        server_port: 25565,
        intention: 2,
    }
    .dispatch(&mut client);

    client.state = ConnectionState::Login;

    println!("Sending Login Start...");
    LoginStartPacket {
        name: "Spinel".to_string(),
        uuid: Uuid::new_v4(),
    }
    .dispatch(&mut client);

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    client.disconnect().await;
    println!("--- JOIN END ---");
}

#[packet_listener(state: ConnectionState::Status)]
fn on_status_response(
    _server: &mut Server,
    _packet: StatusResponsePacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Received Status Response!");
    true
}

#[packet_listener(state: ConnectionState::Status)]
fn on_pong(
    _server: &mut Server,
    _packet: PongResponsePacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Received Pong Response!");
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_login_disconnect(
    _server: &mut Server,
    _packet: LoginDisconnectPacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Received Login Disconnect!");
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_encryption_request(
    _server: &mut Server,
    _packet: EncryptionRequestPacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Received Encryption Request (Hello)!");
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_set_compression(
    server: &mut Server,
    packet: SetCompressionPacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Setting compression threshold to: {}", packet.threshold);
    server.set_compression(packet.threshold);
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_login_success(
    server: &mut Server,
    _packet: LoginSuccessPacket,
    client: &mut MinecraftClient,
) -> bool {
    println!("Login Success!");

    server.state = ConnectionState::Login;
    client.state = ConnectionState::Login;
    LoginAcknowledgedPacket {}.dispatch(server);

    server.state = ConnectionState::Configuration;
    client.state = ConnectionState::Configuration;
    println!("Sent Login Acknowledged. Transitioning to Configuration.");

    println!("Sending Client Information...");
    ClientInformationPacket::default().dispatch(server);

    true
}

#[packet_listener(state: ConnectionState::Configuration)]
fn on_select_known_packs(
    server: &mut Server,
    _packet: S2CKnownPacks,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Server requested known packs (S2C 0x0E)");
    ConfigKnownPacksPacket {
        known_packs: vec![],
    }
    .dispatch(server);
    true
}

#[packet_listener(state: ConnectionState::Configuration)]
fn on_registry_data(
    _server: &mut Server,
    packet: RegistryDataPacket,
    client: &mut MinecraftClient,
) -> bool {
    println!(
        "Received Registry: {} with {} entries (S2C 0x07)",
        packet.registry_id,
        packet.entries.len()
    );

    if let Ok(mut registries) = client.registries.lock() {
        let registry_key = packet.registry_id.to_string();
        let entries: Vec<(String, Option<spinel::nbt::NbtCompound>)> = packet
            .entries
            .iter()
            .map(
                |(id, data): &(
                    spinel::network::types::Identifier,
                    Option<spinel::nbt::NbtCompound>,
                )| { (id.to_string(), data.clone()) },
            )
            .collect();

        registries.dynamic_registries.put(registry_key, entries);
    }

    true
}

#[packet_listener(state: ConnectionState::Configuration)]
fn on_update_tags(
    _server: &mut Server,
    _packet: UpdateTagsPacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Received tags update (S2C 0x0D)");
    true
}

#[packet_listener(state: ConnectionState::Configuration)]
fn on_finish_config(
    server: &mut Server,
    _packet: S2CFinishConfig,
    client: &mut MinecraftClient,
) -> bool {
    println!("Finish Configuration received! (S2C 0x03)");

    println!("--- VALIDATING REGISTRIES ---");
    let registries = client.registries.lock().unwrap();

    let expected_registries = vec![
        ("minecraft:worldgen/biome", false),
        ("minecraft:chat_type", false),
        ("minecraft:trim_pattern", false),
        ("minecraft:trim_material", false),
        ("minecraft:wolf_variant", true),
        ("minecraft:wolf_sound_variant", true),
        ("minecraft:pig_variant", true),
        ("minecraft:frog_variant", true),
        ("minecraft:cat_variant", true),
        ("minecraft:cow_variant", true),
        ("minecraft:chicken_variant", true),
        ("minecraft:zombie_nautilus_variant", true),
        ("minecraft:painting_variant", true),
        ("minecraft:dimension_type", false),
        ("minecraft:damage_type", false),
        ("minecraft:banner_pattern", false),
        ("minecraft:enchantment", false),
        ("minecraft:jukebox_song", false),
        ("minecraft:instrument", false),
        ("minecraft:test_environment", false),
        ("minecraft:test_instance", false),
        ("minecraft:dialog", false),
        ("minecraft:timeline", false),
    ];

    let mut failed = false;
    for (id, required_non_empty) in expected_registries {
        match registries.dynamic_registries.get(id) {
            Some(entries) => {
                if required_non_empty && entries.is_empty() {
                    println!("ERROR: Registry must be non-empty: {}", id);
                    failed = true;
                } else {
                    println!("OK: {} ({} entries)", id, entries.len());
                }
            }
            None => {
                println!("ERROR: Missing dynamic registry: {}", id);
                failed = true;
            }
        }
    }

    if failed {
        println!("\n!!! PROTOCOL ERROR: REGISTRY LOADING FAILED !!!");
        println!("This mimics the vanilla client's behavior when registries are missing or empty.");
        panic!("Registry loading failed. See logs above for details.");
    }

    println!("--- REGISTRY VALIDATION PASSED ---");

    ConfigFinishPacket {}.dispatch(server);

    server.state = ConnectionState::Play;
    client.state = ConnectionState::Play;
    println!("SUCCESS: Transitioning to PLAY state!");
    true
}
