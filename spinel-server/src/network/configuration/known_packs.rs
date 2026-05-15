use ::std::thread::sleep;
use ::std::time::Duration;

use crate::server::MinecraftServer;
use crate::network::client::instance::Client;
use ::spinel_core::network::clientbound::configuration::finish_configuration::FinishConfigurationPacket;
use ::spinel_core::network::clientbound::configuration::update_tags::UpdateTagsPacket;
use ::spinel_core::network::serverbound::configuration::known_packs::KnownPacksPacket;
use ::spinel_macros::packet_listener;
use std::io;

struct ConfigurationCompletion<'a> {
    client: &'a mut Client,
    server: &'a MinecraftServer,
}

impl<'a> ConfigurationCompletion<'a> {
    fn dispatch(mut self) -> io::Result<()> {
        self.dispatch_registry_packets()?;
        UpdateTagsPacket::vanilla_tags().dispatch(self.client)?;
        sleep(Duration::from_millis(100));
        FinishConfigurationPacket::new().dispatch(self.client)
    }

    fn dispatch_registry_packets(&mut self) -> io::Result<()> {
        self.server
            .registry_cache
            .get_packets()
            .iter()
            .cloned()
            .try_for_each(|packet| packet.dispatch(self.client))
    }
}

#[packet_listener(module: "login")]
fn on_known_packs(
    client: &mut Client,
    _packet: KnownPacksPacket,
    server: &mut MinecraftServer,
) -> bool {
    ConfigurationCompletion { client, server }
        .dispatch()
        .is_ok()
}
