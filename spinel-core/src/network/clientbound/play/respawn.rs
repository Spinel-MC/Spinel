use crate::entity::game_mode::GameMode;
use spinel_macros::packet;
use spinel_network::types::{CommonPlayerSpawnInfo, Identifier};

#[packet(id: "respawn", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct RespawnPacket {
    pub common_player_spawn_info: CommonPlayerSpawnInfo,
    pub data_to_keep: i8,
}

impl RespawnPacket {
    pub const COPY_ATTRIBUTES: i8 = 0x1;
    pub const COPY_METADATA: i8 = 0x2;
    pub const COPY_ALL: i8 = Self::COPY_ATTRIBUTES | Self::COPY_METADATA;

    pub fn new(game_mode: GameMode, dimension: Identifier) -> Self {
        Self {
            common_player_spawn_info: CommonPlayerSpawnInfo {
                dimension_type: 1,
                dimension,
                seed: 0,
                game_mode: game_mode.id(),
                previous_game_mode: game_mode.id() as i8,
                is_debug: false,
                is_flat: true,
                last_death_location: None,
                portal_cooldown: 0,
                sea_level: 63,
            },
            data_to_keep: Self::COPY_ALL,
        }
    }
}
