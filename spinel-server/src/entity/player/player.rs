use crate::entity::PlayerSpawnPoint;
use crate::entity::player::chunks::PlayerChunk;
use crate::entity::player::position::PlayerPosition;
use crate::network::client::instance::Client;
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
            respawn_point,
            last_completed_client_tick: 0,
        }
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
