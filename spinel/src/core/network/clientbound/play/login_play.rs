use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_network::types::{position::Position, var_int::VarInt};

#[packet_dispatcher(id: 0x2B)]
pub struct LoginPlayPacket {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<String>,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: String,
    pub dimension_name: String,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<(String, Position)>,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
    pub enforces_secure_chat: bool,
}

impl LoginPlayPacket {
    pub fn new_default(entity_id: i32) -> Self {
        Self {
            entity_id,
            is_hardcore: false,
            dimension_names: vec!["minecraft:overworld".to_string()],
            max_players: VarInt(20),
            view_distance: VarInt(10),
            simulation_distance: VarInt(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: "minecraft:overworld".to_string(),
            dimension_name: "minecraft:overworld".to_string(),
            hashed_seed: 0,
            game_mode: 1,
            previous_game_mode: -1,
            is_debug: false,
            is_flat: true,
            death_location: None,
            portal_cooldown: VarInt(0),
            sea_level: VarInt(63),
            enforces_secure_chat: false,
        }
    }
}
