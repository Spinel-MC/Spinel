use crate::entity::game_mode::GameMode;
use spinel_macros::packet;
use spinel_network::types::{Array, CommonPlayerSpawnInfo, Identifier, var_int::VarIntWrapper};

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
        Self::new(
            player_id,
            GameMode::Survival,
            0,
            Identifier::minecraft("overworld"),
        )
    }

    pub fn new(
        player_id: i32,
        game_mode: GameMode,
        dimension_type_id: i32,
        dimension_name: Identifier,
    ) -> Self {
        Self {
            player_id,
            is_hardcore: false,
            levels: Array(vec![dimension_name.clone()]),
            max_players: VarIntWrapper(20),
            chunk_radius: VarIntWrapper(8),
            simulation_distance: VarIntWrapper(8),
            has_reduced_debug_info: false,
            should_show_death_screen: true,
            is_limited_crafting_enabled: false,
            common_player_spawn_info: Self::spawn_info(
                game_mode,
                dimension_type_id,
                dimension_name,
            ),
            is_secure_chat_enforced: false,
        }
    }

    fn spawn_info(
        game_mode: GameMode,
        dimension_type_id: i32,
        dimension_name: Identifier,
    ) -> CommonPlayerSpawnInfo {
        CommonPlayerSpawnInfo {
            dimension_type: dimension_type_id,
            dimension: dimension_name,
            seed: 0,
            game_mode: game_mode.id(),
            previous_game_mode: -1,
            is_debug: false,
            is_flat: true,
            last_death_location: None,
            portal_cooldown: 0,
            sea_level: 63,
        }
    }
}
