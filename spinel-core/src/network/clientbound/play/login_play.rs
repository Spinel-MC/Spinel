use ::spinel_macros::packet;
use ::spinel_network::types::{Array, CommonPlayerSpawnInfo, Identifier};

#[packet(id: "login",
state: ConnectionState::Play,
recipient: Recipient::Client,
generate_fields: true
)]
pub struct LoginPlayPacket;

impl LoginPlayPacket {
    pub fn new_default(player_id: i32) -> Self {
        Self {
            player_id,
            hardcore: false,
            levels: Array(vec![
                Identifier::minecraft("overworld"),
                Identifier::minecraft("the_nether"),
                Identifier::minecraft("the_end"),
            ]),
            max_players: 20,
            chunk_radius: 10,
            simulation_distance: 10,
            reduced_debug_info: false,
            show_death_screen: true,
            do_limited_crafting: false,
            common_player_spawn_info: CommonPlayerSpawnInfo {
                dimension_type: 1,
                dimension: Identifier::minecraft("overworld"),
                seed: 0,
                game_mode: 1,
                previous_game_mode: -1,
                is_debug: false,
                is_flat: true,
                last_death_location: None,
                portal_cooldown: 0,
                sea_level: 63,
            },
            enforces_secure_chat: false,
        }
    }
}
