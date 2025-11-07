use std::thread::sleep;
use std::time::Duration;

use crate as spinel;
use crate::core::network::clientbound::configuration::finish_configuration::FinishConfigurationPacket;
use crate::core::network::clientbound::configuration::registry_data::RegistryDataPacket;
use crate::core::network::clientbound::configuration::update_tags::UpdateTagsPacket;
use crate::core::server::MinecraftServer;
use spinel_macros::packet_listener;
use spinel_network::{Client, ConnectionState};

#[packet_listener(
    id: 0x07,
    state: ConnectionState::Configuration,
    fields:(
        known_packs: Vec<(String, String, String)>,
    )
)]
fn on_known_packs(client: &mut Client, _packet: Packet, _server: &mut MinecraftServer) -> bool {
    for registry_packet in RegistryDataPacket::vanilla_registries() {
        registry_packet.dispatch(client);
    }

    UpdateTagsPacket::vanilla_tags().dispatch(client);

    sleep(Duration::from_secs(2));
    FinishConfigurationPacket::new().dispatch(client);

    println!("Server configuration sent. Waiting for client to acknowledge finish.");

    true
}
