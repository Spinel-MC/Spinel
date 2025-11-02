use crate::{
    self as spinel,
    core::{
        network::clientbound::configuration::{
            feature_flags::FeatureFlagsPacket, finish_configuration::FinishConfigurationPacket, known_packs::KnownPacksPacket, registry_data::RegistryDataPacket, update_tags::UpdateTagsPacket
        },
        server::MinecraftServer,
    },
};
use spinel_macros::packet_listener;
use spinel_network::{client::instance::ConnectionState, types::alias::Array, Client};

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
fn on_client_information(client: &mut Client, packet: Packet, _server: &mut MinecraftServer) -> bool {
    println!("Received client info (locale: '{}'). Sending server configuration...", packet.locale);
    
    FeatureFlagsPacket { feature_flags: Array(vec!["minecraft:vanilla".to_string()]) }.dispatch(client);


    //TODO: Known Packs packet

    KnownPacksPacket::new(vec![(
        "".to_owned(),
        "".to_owned(),
        "".to_owned()
    )]).dispatch(client);
    
    true
}