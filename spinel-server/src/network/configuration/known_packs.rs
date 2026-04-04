use ::std::thread::sleep;
use ::std::time::Duration;

use crate::instance::MinecraftServer;
use crate::network::client::instance::Client;
use ::spinel_core::network::clientbound::configuration::finish_configuration::FinishConfigurationPacket;
use ::spinel_core::network::clientbound::configuration::update_tags::UpdateTagsPacket;
use ::spinel_core::network::serverbound::configuration::known_packs::KnownPacksPacket;
use ::spinel_macros::packet_listener;

#[packet_listener(module: "login")]
fn on_known_packs(
    client: &mut Client,
    _packet: KnownPacksPacket,
    server: &mut MinecraftServer,
) -> bool {
    for registry_packet in server.registry_cache.get_packets().iter() {
        registry_packet.clone().dispatch(client);
    }

    UpdateTagsPacket::vanilla_tags().dispatch(client);

    sleep(Duration::from_millis(100));

    FinishConfigurationPacket::new().dispatch(client);

    true
}
