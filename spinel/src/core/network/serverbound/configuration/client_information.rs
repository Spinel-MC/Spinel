use crate::core::server::MinecraftServer;
use spinel_macros::packet_listener;
use spinel_network::{client::instance::ConnectionState, Client};

use crate as spinel;

#[packet_listener(
    id: 0x00,
    state: ConnectionState::Configuration,
    fields: (
        locale: String(16),
        view_distance: Byte,
        chat_mode: VarInt,
        chat_colors: Bool,
        displayed_skin_parts: UnsignedByte,
        main_hand: VarInt,
        enable_text_filtering: Bool,
        allow_server_listings: Bool,
        particle_status: VarInt
    ),
    module: "login"
)]
fn on_client_information(
    _client: &mut Client,
    packet: Packet,
    _server: &mut MinecraftServer,
) -> bool {
    println!("Received client info (locale: '{}').", packet.locale);

    true
}
