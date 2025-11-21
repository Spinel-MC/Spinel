use std::thread::sleep;
use std::time::Duration;

use crate as spinel;
use crate::core::network::clientbound::configuration::finish_configuration::FinishConfigurationPacket;
use crate::core::network::clientbound::configuration::update_tags::UpdateTagsPacket;
use crate::core::server::MinecraftServer;
use spinel_macros::packet_listener;
use spinel_network::{Client, ConnectionState};

#[packet_listener(
    id: 0x07,
    state: ConnectionState::Configuration,
    fields:(
        known_packs: Vec<(String, String, String)>,
    ), module: "login")
]
fn on_known_packs(client: &mut Client, _packet: Packet, server: &mut MinecraftServer) -> bool {
    for (_idx, registry_packet) in server.registry_cache.get_packets().iter().enumerate() {
        registry_packet.clone().dispatch(client);
    }

    UpdateTagsPacket::vanilla_tags().dispatch(client);

    sleep(Duration::from_millis(100));

    FinishConfigurationPacket::new().dispatch(client);

    true
}
