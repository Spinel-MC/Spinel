use spinel::client::MinecraftClient;
use spinel::core::network::clientbound::configuration::finish_configuration::FinishConfigurationPacket as ServerFinishConfigurationPacket;
use spinel::core::network::clientbound::configuration::known_packs::KnownPacksPacket as ServerKnownPacksPacket;
use spinel::core::network::clientbound::configuration::registry_data::RegistryDataPacket;
use spinel::core::network::clientbound::configuration::update_tags::UpdateTagsPacket;
use spinel::core::network::serverbound::configuration::finish_configuration::FinishConfigurationPacket as ClientFinishConfigurationPacket;
use spinel::core::network::serverbound::configuration::known_packs::KnownPacksPacket as ClientKnownPacksPacket;
use spinel::macros::packet_listener;
use spinel::nbt::NbtCompound;
use spinel::network::types::Identifier;
use spinel::network::{ConnectionState, Server};

use crate::dispatch::report_dispatch_result;

#[packet_listener(state: ConnectionState::Configuration)]
fn on_select_known_packs(
    server: &mut Server,
    _packet: ServerKnownPacksPacket,
    _client: &mut MinecraftClient,
) -> bool {
    report_dispatch_result(
        ClientKnownPacksPacket {
            known_packs: vec![],
        }
        .dispatch(server),
        "known packs packet",
    );
    true
}

#[packet_listener(state: ConnectionState::Configuration)]
fn on_registry_data(
    _server: &mut Server,
    packet: RegistryDataPacket,
    client: &mut MinecraftClient,
) -> bool {
    let Ok(mut registries) = client.registries.lock() else {
        return false;
    };

    let registry_key = packet.registry_id.to_string();
    let registry_entries: Vec<(String, Option<NbtCompound>)> = packet
        .entries
        .iter()
        .map(
            |(identifier, registry_entry): &(Identifier, Option<NbtCompound>)| {
                (identifier.to_string(), registry_entry.clone())
            },
        )
        .collect();
    registries
        .dynamic_registries
        .put(registry_key, registry_entries);
    true
}

#[packet_listener(state: ConnectionState::Configuration)]
fn on_update_tags(
    _server: &mut Server,
    _packet: UpdateTagsPacket,
    _client: &mut MinecraftClient,
) -> bool {
    true
}

#[packet_listener(state: ConnectionState::Configuration)]
fn on_finish_config(
    server: &mut Server,
    _packet: ServerFinishConfigurationPacket,
    client: &mut MinecraftClient,
) -> bool {
    if !RegistryValidation::new(client).validate() {
        println!("Protocol error: registry loading failed.");
        return false;
    }

    report_dispatch_result(
        ClientFinishConfigurationPacket {}.dispatch(server),
        "finish configuration packet",
    );
    server.state = ConnectionState::Play;
    client.state = ConnectionState::Play;
    println!("Client entered Play state.");
    true
}

struct RegistryValidation<'a> {
    client: &'a MinecraftClient,
}

impl<'a> RegistryValidation<'a> {
    fn new(client: &'a MinecraftClient) -> Self {
        Self { client }
    }

    fn validate(&self) -> bool {
        let Ok(registries) = self.client.registries.lock() else {
            return false;
        };
        let mut all_registries_are_valid = true;

        for (registry_id, requires_entries) in Self::expected_registries() {
            match registries.dynamic_registries.get(registry_id) {
                Some(entries) if requires_entries && entries.is_empty() => {
                    println!("ERROR: Registry must be non-empty: {}", registry_id);
                    all_registries_are_valid = false;
                }
                Some(_) => {}
                None => {
                    if Self::missing_registry_is_allowed_for_repro(registry_id) {
                        continue;
                    }
                    println!("ERROR: Missing dynamic registry: {}", registry_id);
                    all_registries_are_valid = false;
                }
            }
        }

        if all_registries_are_valid {
            println!("Registry validation passed.");
        }

        all_registries_are_valid
    }

    fn expected_registries() -> [(&'static str, bool); 23] {
        [
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
        ]
    }

    const fn missing_registry_is_allowed_for_repro(registry_id: &str) -> bool {
        matches!(
            registry_id.as_bytes(),
            b"minecraft:test_environment" | b"minecraft:test_instance"
        )
    }
}
