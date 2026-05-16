use crate::entity::Player;
use crate::entity::player::chunks::PlayerChunk;
use crate::entity::player::chunks::PlayerChunkTransition;
use crate::entity::player::position::PlayerPosition;
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::chunk_batch_finished::ChunkBatchFinishedPacket;
use spinel_core::network::clientbound::play::chunk_batch_start::ChunkBatchStartPacket;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::login_play::LoginPlayPacket;
use spinel_core::network::clientbound::play::set_chunk_cache_center::SetChunkCacheCenterPacket;
use spinel_core::network::clientbound::play::set_default_spawn_position::SetDefaultSpawnPositionPacket;
use spinel_core::network::clientbound::play::set_health::SetHealthPacket;
use spinel_core::network::clientbound::play::sync_player_pos::{
    SyncPlayerPositionPacket, SyncPlayerPositionSpec,
};
use spinel_network::types::{GlobalPos, Identifier};
use std::io;

impl Player {
    pub(crate) fn enter_world(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        world_name: Identifier,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
    ) -> io::Result<()> {
        self.position = PlayerPosition::from(self.respawn_point());
        self.loaded_chunk = PlayerChunk::from_position(self.position);
        LoginPlayPacket::new_default(41).dispatch(client)?;
        client.enter_play();
        self.last_completed_client_tick = 0;
        self.send_tick_rate(client, ticks_per_second)?;
        self.load_world(client, world_name, chunk_packets)
    }

    pub(crate) fn move_to(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        transition: Option<PlayerChunkTransition>,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
    ) -> io::Result<()> {
        self.position = self.position.at(x, y, z);
        let Some(transition) = transition else {
            return Ok(());
        };

        self.sync_chunks(client, transition, chunk_packets)
    }

    pub(crate) fn spawn_chunks(&self) -> Vec<PlayerChunk> {
        PlayerChunk::from_position(PlayerPosition::from(self.respawn_point())).surrounding()
    }

    pub(crate) fn chunk_transition(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Option<crate::entity::player::chunks::PlayerChunkTransition> {
        let position = self.position.at(x, y, z);
        self.loaded_chunk
            .transition_to(PlayerChunk::from_position(position))
    }

    fn load_world(
        &self,
        client: &mut Client,
        world_name: Identifier,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
    ) -> io::Result<()> {
        SetChunkCacheCenterPacket::new(self.loaded_chunk.x, self.loaded_chunk.z)
            .dispatch(client)?;
        self.spawn_position(client, world_name)?;
        SetHealthPacket::new(20.0, 20, 5.0).dispatch(client)?;
        self.load_chunks(client, chunk_packets)?;
        self.sync_position(client)?;
        GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)
    }

    fn sync_chunks(
        &mut self,
        client: &mut Client,
        transition: PlayerChunkTransition,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
    ) -> io::Result<()> {
        SetChunkCacheCenterPacket::new(transition.next.x, transition.next.z).dispatch(client)?;
        self.forget_chunks(client, transition.departing)?;
        self.load_chunks(client, chunk_packets)?;
        self.loaded_chunk = transition.next;
        Ok(())
    }

    fn spawn_position(&self, client: &mut Client, world_name: Identifier) -> io::Result<()> {
        SetDefaultSpawnPositionPacket::new(
            GlobalPos {
                dimension: world_name,
                position: self.respawn_point().block_position(),
            },
            self.position.yaw,
            self.position.pitch,
        )
        .dispatch(client)
    }

    fn sync_position(&self, client: &mut Client) -> io::Result<()> {
        SyncPlayerPositionPacket::new(SyncPlayerPositionSpec {
            teleport_id: 0,
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
            yaw: self.position.yaw,
            pitch: self.position.pitch,
        })
        .dispatch(client)
    }

    fn load_chunks(
        &self,
        client: &mut Client,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
    ) -> io::Result<()> {
        let chunk_batch_size = chunk_packets.len() as i32;
        ChunkBatchStartPacket.dispatch(client)?;
        chunk_packets
            .into_iter()
            .try_for_each(|chunk_packet| chunk_packet.dispatch(client))?;
        ChunkBatchFinishedPacket::new(chunk_batch_size).dispatch(client)
    }

    fn forget_chunks(&self, client: &mut Client, chunks: Vec<PlayerChunk>) -> io::Result<()> {
        chunks.into_iter().try_for_each(|chunk| {
            ForgetLevelChunkPacket::new(chunk.packet_position()).dispatch(client)
        })
    }
}
