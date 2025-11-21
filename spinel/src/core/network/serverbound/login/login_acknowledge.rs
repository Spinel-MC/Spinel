use crate::{
    self as spinel,
    core::server::MinecraftServer,
    util::constants::{MINECRAFT_VERSION, SERVER_BRAND},
};
use spinel_macros::packet_listener;
use spinel_network::{client::instance::ConnectionState, Client};

#[packet_listener(id: 0x03, state: ConnectionState::Login, module: "login")]
fn on_login_acknowledge(client: &mut Client, server: &mut MinecraftServer) -> bool {
    client.state = ConnectionState::Configuration;

    send_brand_packet(client);

    send_known_packs_packet(client);

    true
}

fn send_brand_packet(client: &mut Client) {
    // Plugin Message packet structure:
    // - Channel: String (VarInt length + UTF-8)
    // - Data: byte array (remaining bytes)

    let channel = "minecraft:brand";
    let brand_data = SERVER_BRAND.as_bytes();

    let mut buffer = spinel::internal::encoder::NetworkBuffer::new();
    buffer.write_string(channel);
    // Write brand as String (VarInt length + UTF-8)
    buffer.write_string(&String::from_utf8_lossy(brand_data));

    let packet_id = 0x01; // Plugin Message (clientbound, configuration)
    client.send_packet(packet_id, &buffer.into_buffer());
}

fn send_known_packs_packet(client: &mut Client) {
    // Select Known Packs packet structure:
    // - Known Pack Count: VarInt
    // - For each pack:
    //   - Namespace: String
    //   - ID: String
    //   - Version: String

    let mut buffer = spinel::internal::encoder::NetworkBuffer::new();

    // Send one known pack: minecraft:core with version 1.21.10
    buffer.write_varint(1); // Count
    buffer.write_string("minecraft"); // Namespace
    buffer.write_string("core"); // ID
    buffer.write_string(MINECRAFT_VERSION); // Version

    let packet_id = 0x0E; // Select Known Packs (clientbound, configuration)
    client.send_packet(packet_id, &buffer.into_buffer());
}
