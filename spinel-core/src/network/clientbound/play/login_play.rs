use crate::entity::game_mode::GameMode;
use ::spinel_macros::packet;
use ::spinel_network::types::{Array, CommonPlayerSpawnInfo, Identifier, var_int::VarIntWrapper};

#[packet(id: "login",
state: ConnectionState::Play,
recipient: Recipient::Client
)]
pub struct LoginPlayPacket {
    pub player_id: i32,
    pub is_hardcore: bool,
    pub levels: Array<Identifier>,
    pub max_players: VarIntWrapper,
    pub chunk_radius: VarIntWrapper,
    pub simulation_distance: VarIntWrapper,
    pub has_reduced_debug_info: bool,
    pub should_show_death_screen: bool,
    pub is_limited_crafting_enabled: bool,
    pub common_player_spawn_info: CommonPlayerSpawnInfo,
    pub is_secure_chat_enforced: bool,
}

impl LoginPlayPacket {
    pub fn new_default(player_id: i32) -> Self {
        Self {
            player_id,
            is_hardcore: false,
            levels: Self::default_levels(),
            max_players: VarIntWrapper(20),
            chunk_radius: VarIntWrapper(8),
            simulation_distance: VarIntWrapper(8),
            has_reduced_debug_info: false,
            should_show_death_screen: true,
            is_limited_crafting_enabled: false,
            common_player_spawn_info: Self::default_spawn_info(),
            is_secure_chat_enforced: false,
        }
    }

    fn default_levels() -> Array<Identifier> {
        Array(vec![
            Identifier::minecraft("overworld"),
            Identifier::minecraft("the_nether"),
            Identifier::minecraft("the_end"),
        ])
    }

    fn default_spawn_info() -> CommonPlayerSpawnInfo {
        CommonPlayerSpawnInfo {
            dimension_type: 1,
            dimension: Identifier::minecraft("overworld"),
            seed: 0,
            game_mode: GameMode::Creative.id(),
            previous_game_mode: -1,
            is_debug: false,
            is_flat: true,
            last_death_location: None,
            portal_cooldown: 0,
            sea_level: 63,
        }
    }
}
