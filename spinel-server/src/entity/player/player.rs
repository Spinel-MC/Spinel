use crate::entity::PlayerSpawnPoint;
use crate::entity::player::chunks::PlayerChunk;
use crate::entity::player::position::PlayerPosition;
use crate::network::client::instance::Client;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::ticking_state::TickingStatePacket;
use spinel_core::network::clientbound::play::ticking_step::TickingStepPacket;
use std::io;
use std::net::SocketAddr;
use uuid::Uuid;

pub struct Player {
    pub uuid: Uuid,
    pub username: String,
    pub protocol_version: i32,
    pub addr: SocketAddr,
    pub(crate) loaded_chunk: PlayerChunk,
    pub(crate) position: PlayerPosition,
    game_mode: GameMode,
    respawn_point: PlayerSpawnPoint,
    pub(super) last_completed_client_tick: u64,
}

impl Player {
    pub fn new(uuid: Uuid, username: String, protocol_version: i32, addr: SocketAddr) -> Self {
        let respawn_point = PlayerSpawnPoint::default();
        let position = PlayerPosition::from(respawn_point);
        Self {
            uuid,
            username,
            protocol_version,
            addr,
            loaded_chunk: PlayerChunk::from_position(position),
            position,
            game_mode: GameMode::Survival,
            respawn_point,
            last_completed_client_tick: 0,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) -> bool {
        self.game_mode = game_mode;
        true
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub fn set_respawn_point(&mut self, respawn_point: PlayerSpawnPoint) {
        self.respawn_point = respawn_point;
    }

    pub fn respawn_point(&self) -> PlayerSpawnPoint {
        self.respawn_point
    }

    pub(crate) fn tick(&mut self) {}

    pub(super) fn send_tick_rate(
        &self,
        client: &mut Client,
        ticks_per_second: u32,
    ) -> io::Result<()> {
        TickingStatePacket {
            tick_rate: ticks_per_second as f32,
            is_frozen: false,
        }
        .dispatch(client)?;
        TickingStepPacket::new(0).dispatch(client)
    }

    pub(crate) fn finish_client_tick(&mut self, server_tick: u64) {
        self.last_completed_client_tick = server_tick;
    }

    pub(crate) fn look(&mut self, yaw: f32, pitch: f32) {
        self.position = self.position.looking_at(yaw, pitch);
    }
}

#[cfg(test)]
mod tests {
    use super::Player;
    use spinel_core::entity::game_mode::GameMode;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use uuid::Uuid;

    #[test]
    fn player_game_mode_defaults_to_survival_and_can_be_set_during_configuration() {
        let mut player = Player::new(
            Uuid::nil(),
            "Player".to_string(),
            0,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
        );

        assert_eq!(player.game_mode(), GameMode::Survival);
        assert!(player.set_game_mode(GameMode::Creative));
        assert_eq!(player.game_mode(), GameMode::Creative);
    }
}
