use crate::ConnectionState;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Deserialize, Debug, Clone)]
struct PacketDef {
    id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct PacketMap {
    #[serde(flatten)]
    packets: HashMap<String, PacketDef>,
}

#[derive(Deserialize, Debug, Clone)]
struct StateMap {
    handshake: Option<PacketMap>,
    status: Option<PacketMap>,
    login: Option<PacketMap>,
    config: Option<PacketMap>,
    configuration: Option<PacketMap>,
    play: Option<PacketMap>,
}

#[derive(Deserialize, Debug)]
struct PacketsJson {
    packets: Option<Directions>,
    serverbound: Option<StateMap>,
    clientbound: Option<StateMap>,
}

#[derive(Deserialize, Debug)]
struct Directions {
    serverbound: StateMap,
    clientbound: StateMap,
}

impl PacketsJson {
    fn directions(&self) -> Option<(&StateMap, &StateMap)> {
        if let Some(packets) = &self.packets {
            return Some((&packets.serverbound, &packets.clientbound));
        }

        Some((self.serverbound.as_ref()?, self.clientbound.as_ref()?))
    }
}

static PACKET_NAMES: OnceLock<HashMap<(bool, ConnectionState, i32), String>> = OnceLock::new();

impl StateMap {
    fn config(self) -> Option<PacketMap> {
        self.configuration.or(self.config)
    }
}

pub struct PacketNameRegistry;

impl PacketNameRegistry {
    pub fn get_clientbound_packet_name(state: ConnectionState, id: i32) -> String {
        Self::get_packet_name(true, state, id)
    }

    pub fn get_serverbound_packet_name(state: ConnectionState, id: i32) -> String {
        Self::get_packet_name(false, state, id)
    }

    fn get_packet_name(is_clientbound: bool, state: ConnectionState, id: i32) -> String {
        let packet_name_map = PACKET_NAMES.get_or_init(Self::load_packet_names);
        packet_name_map
            .get(&(is_clientbound, state, id))
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string())
    }

    fn load_packet_names() -> HashMap<(bool, ConnectionState, i32), String> {
        let json_str = include_str!("../../spinel-registry/build_assets/packets.json");
        let packet_data = serde_json::from_str::<PacketsJson>(json_str).unwrap_or(PacketsJson {
            packets: None,
            serverbound: None,
            clientbound: None,
        });

        let Some((serverbound_packets, clientbound_packets)) = packet_data.directions() else {
            return HashMap::new();
        };

        let mut packet_name_map = HashMap::new();
        Self::add_direction_packets(&mut packet_name_map, true, clientbound_packets);
        Self::add_direction_packets(&mut packet_name_map, false, serverbound_packets);
        packet_name_map
    }

    fn add_direction_packets(
        packet_name_map: &mut HashMap<(bool, ConnectionState, i32), String>,
        is_clientbound: bool,
        state_map: &StateMap,
    ) {
        Self::add_state_packets(
            packet_name_map,
            is_clientbound,
            ConnectionState::Handshaking,
            state_map.handshake.clone(),
        );
        Self::add_state_packets(
            packet_name_map,
            is_clientbound,
            ConnectionState::Status,
            state_map.status.clone(),
        );
        Self::add_state_packets(
            packet_name_map,
            is_clientbound,
            ConnectionState::Login,
            state_map.login.clone(),
        );
        Self::add_state_packets(
            packet_name_map,
            is_clientbound,
            ConnectionState::Configuration,
            state_map.clone().config(),
        );
        Self::add_state_packets(
            packet_name_map,
            is_clientbound,
            ConnectionState::Play,
            state_map.play.clone(),
        );
    }

    fn add_state_packets(
        packet_name_map: &mut HashMap<(bool, ConnectionState, i32), String>,
        is_clientbound: bool,
        connection_state: ConnectionState,
        packet_map: Option<PacketMap>,
    ) {
        let Some(packet_map) = packet_map else {
            return;
        };

        for (packet_name, packet_definition) in packet_map.packets {
            if let Ok(packet_id) =
                i32::from_str_radix(packet_definition.id.trim_start_matches("0x"), 16)
            {
                packet_name_map.insert((is_clientbound, connection_state, packet_id), packet_name);
            }
        }
    }
}
