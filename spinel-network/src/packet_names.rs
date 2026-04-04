use crate::ConnectionState;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
struct PacketDef {
    id: String,
}

#[derive(Deserialize, Debug)]
struct PacketMap {
    #[serde(flatten)]
    packets: HashMap<String, PacketDef>,
}

#[derive(Deserialize, Debug)]
struct StateMap {
    handshake: Option<PacketMap>,
    status: Option<PacketMap>,
    login: Option<PacketMap>,
    config: Option<PacketMap>,
    play: Option<PacketMap>,
}

#[derive(Deserialize, Debug)]
struct PacketsJson {
    serverbound: StateMap,
    clientbound: StateMap,
}

static PACKET_NAMES: OnceLock<HashMap<(bool, ConnectionState, i32), String>> = OnceLock::new();

fn load_packet_names() -> HashMap<(bool, ConnectionState, i32), String> {
    let json_str = include_str!("../../spinel-registry/build_assets/packets.json");
    let data: PacketsJson = serde_json::from_str(json_str).expect("Failed to parse packets.json");

    let mut map = HashMap::new();

    let mut add_state =
        |is_clientbound: bool, state: ConnectionState, packet_map: Option<PacketMap>| {
            if let Some(pm) = packet_map {
                for (name, def) in pm.packets {
                    let id_str = def.id.trim_start_matches("0x");
                    if let Ok(id) = i32::from_str_radix(id_str, 16) {
                        map.insert((is_clientbound, state, id), name);
                    }
                }
            }
        };

    add_state(
        true,
        ConnectionState::Handshaking,
        data.clientbound.handshake,
    );
    add_state(true, ConnectionState::Status, data.clientbound.status);
    add_state(true, ConnectionState::Login, data.clientbound.login);
    add_state(
        true,
        ConnectionState::Configuration,
        data.clientbound.config,
    );
    add_state(true, ConnectionState::Play, data.clientbound.play);

    add_state(
        false,
        ConnectionState::Handshaking,
        data.serverbound.handshake,
    );
    add_state(false, ConnectionState::Status, data.serverbound.status);
    add_state(false, ConnectionState::Login, data.serverbound.login);
    add_state(
        false,
        ConnectionState::Configuration,
        data.serverbound.config,
    );
    add_state(false, ConnectionState::Play, data.serverbound.play);

    map
}

pub fn get_packet_name(is_clientbound: bool, state: ConnectionState, id: i32) -> String {
    let map = PACKET_NAMES.get_or_init(load_packet_names);
    map.get(&(is_clientbound, state, id))
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_clientbound_packet_name(state: ConnectionState, id: i32) -> String {
    get_packet_name(true, state, id)
}

pub fn get_serverbound_packet_name(state: ConnectionState, id: i32) -> String {
    get_packet_name(false, state, id)
}
