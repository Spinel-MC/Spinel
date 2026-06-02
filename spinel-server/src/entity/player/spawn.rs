use crate::entity::Player;
use crate::entity::player::QueuedPlayerChunk;
use crate::entity::player::chunks::PlayerChunk;
use crate::entity::player::chunks::PlayerChunkTransition;
use crate::entity::player::position::PlayerPosition;
use crate::events::player_chunk_load::PlayerChunkLoadEvent;
use crate::events::player_chunk_unload::PlayerChunkUnloadEvent;
use crate::network::client::instance::Client;
use crate::world::Chunk;
use spinel_core::network::clientbound::play::chunk_batch_finished::ChunkBatchFinishedPacket;
use spinel_core::network::clientbound::play::chunk_batch_start::ChunkBatchStartPacket;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::commands::CommandsPacket;
use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket;
use spinel_core::network::clientbound::play::login_play::LoginPlayPacket;
use spinel_core::network::clientbound::play::respawn::RespawnPacket;
use spinel_core::network::clientbound::play::set_chunk_cache_center::SetChunkCacheCenterPacket;
use spinel_core::network::clientbound::play::set_default_spawn_position::SetDefaultSpawnPositionPacket;
use spinel_core::network::clientbound::play::set_health::SetHealthPacket;
use spinel_core::network::clientbound::play::set_held_slot::SetHeldSlotPacket;
use spinel_core::network::clientbound::play::set_time::SetTimePacket;
use spinel_core::network::clientbound::play::sync_player_pos::{
    SyncPlayerPositionPacket, SyncPlayerPositionSpec,
};
use spinel_network::types::{GlobalPos, Identifier};
use std::io;

impl Player {
    pub fn refresh_commands(&mut self) -> io::Result<()> {
        let Some(client) = self.client_mut() else {
            return Ok(());
        };
        server_commands_packet(client)
    }

    pub(crate) fn enter_world(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        world_name: Identifier,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
        time_packet: SetTimePacket,
    ) -> io::Result<()> {
        self.position = PlayerPosition::from(self.respawn_point());
        self.loaded_chunk = PlayerChunk::from_position(self.position);
        self.chunks_loaded_by_client = self.loaded_chunk;
        self.reset_chunk_queue_for_world_entry_or_teleport();
        LoginPlayPacket::new(self.entity_id().value(), self.game_mode()).dispatch(client)?;
        client.enter_play();
        self.last_completed_client_tick = 0;
        self.set_on_ground(false);
        self.send_tick_rate(client, ticks_per_second)?;
        self.load_world(client, world_name, chunk_packets, time_packet)?;
        self.mark_entered_world();
        self.refresh_commands()?;
        self.refresh_recipes()
    }

    pub(crate) fn enter_world_with_chunk_positions(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        world_name: Identifier,
        chunks: Vec<PlayerChunk>,
        time_packet: SetTimePacket,
    ) -> io::Result<()> {
        self.position = PlayerPosition::from(self.respawn_point());
        self.loaded_chunk = PlayerChunk::from_position(self.position);
        self.chunks_loaded_by_client = self.loaded_chunk;
        self.reset_chunk_queue_for_world_entry_or_teleport();
        LoginPlayPacket::new(self.entity_id().value(), self.game_mode()).dispatch(client)?;
        client.enter_play();
        self.last_completed_client_tick = 0;
        self.set_on_ground(false);
        self.send_tick_rate(client, ticks_per_second)?;
        self.load_world_chunk_positions(client, world_name, chunks, time_packet)?;
        self.mark_entered_world();
        self.refresh_commands()?;
        self.refresh_recipes()
    }

    pub(crate) fn spawn_after_instance_transition(
        &mut self,
        client: &mut Client,
        world_name: Identifier,
        chunks: Vec<PlayerChunk>,
        time_packet: SetTimePacket,
        world_border_packet: InitializeWorldBorderPacket,
        first_spawn: bool,
        dimension_change: bool,
        update_chunks: bool,
    ) -> io::Result<()> {
        self.prepare_instance_spawn(world_name.clone());
        if dimension_change {
            RespawnPacket::new(self.game_mode(), world_name.clone()).dispatch(client)?;
        }
        if update_chunks {
            SetChunkCacheCenterPacket::new(self.loaded_chunk.x, self.loaded_chunk.z)
                .dispatch(client)?;
            self.queue_chunks(chunks);
        }
        self.sync_position(client)?;
        if dimension_change {
            self.spawn_position(client, world_name)?;
            world_border_packet.dispatch(client)?;
            time_packet.dispatch(client)?;
        }
        if dimension_change || first_spawn {
            self.sync_inventory(client)?;
            SetHeldSlotPacket {
                slot: self.held_slot() as i8,
            }
            .dispatch(client)?;
            GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)?;
        }
        self.mark_entered_world();
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn move_to(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        transition: Option<PlayerChunkTransition>,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
    ) -> io::Result<()> {
        self.position = self.position.at(x, y, z);
        self.set_on_ground(on_ground);
        let Some(transition) = transition else {
            return Ok(());
        };

        self.update_chunks_after_border_crossing(client, transition, chunk_packets)
    }

    pub(crate) fn move_to_loaded_chunks(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        transition: Option<PlayerChunkTransition>,
        chunks: Vec<PlayerChunk>,
        world_view_distance: i32,
    ) -> io::Result<()> {
        self.position = self.position.at(x, y, z);
        self.set_on_ground(on_ground);
        let Some(transition) = transition else {
            return Ok(());
        };

        self.update_chunks_after_border_crossing_with_chunks(
            client,
            transition,
            chunks,
            world_view_distance,
        )
    }

    pub(crate) fn move_to_with_view_loaded_chunks(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        transition: Option<PlayerChunkTransition>,
        chunks: Vec<PlayerChunk>,
        world_view_distance: i32,
    ) -> io::Result<()> {
        self.position = self.position.looking_at(yaw, pitch);
        self.move_to_loaded_chunks(
            client,
            x,
            y,
            z,
            on_ground,
            transition,
            chunks,
            world_view_distance,
        )
    }

    pub(crate) fn set_position(&mut self, position: crate::entity::EntityPosition) {
        self.position = PlayerPosition::new(
            position.x(),
            position.y(),
            position.z(),
            position.yaw(),
            position.pitch(),
        );
    }

    pub(crate) fn set_position_and_view(&mut self, position: crate::entity::EntityPosition) {
        self.set_position(position);
    }

    pub(crate) fn set_instance_position(
        &mut self,
        position: crate::entity::EntityPosition,
        world_view_distance: i32,
        should_reset_chunks: bool,
    ) -> Vec<PlayerChunk> {
        self.position = PlayerPosition::new(
            position.x(),
            position.y(),
            position.z(),
            position.yaw(),
            position.pitch(),
        );
        self.loaded_chunk = PlayerChunk::from_position(self.position);
        self.chunks_loaded_by_client = self.loaded_chunk;
        if should_reset_chunks {
            self.reset_chunk_queue_for_world_entry_or_teleport();
        }
        self.loaded_chunk
            .surrounding(self.effective_chunk_view_distance(world_view_distance))
    }

    pub(crate) fn spawn_chunks(&self, world_view_distance: i32) -> Vec<PlayerChunk> {
        PlayerChunk::from_position(PlayerPosition::from(self.respawn_point()))
            .surrounding(self.effective_chunk_view_distance(world_view_distance))
    }

    pub(crate) fn chunk_transition(
        &self,
        x: f64,
        y: f64,
        z: f64,
        world_view_distance: i32,
    ) -> Option<crate::entity::player::chunks::PlayerChunkTransition> {
        let position = self.position.at(x, y, z);
        self.chunks_loaded_by_client.transition_to(
            PlayerChunk::from_position(position),
            self.effective_chunk_view_distance(world_view_distance),
        )
    }

    pub(crate) fn has_chunk_loaded_by_client(
        &self,
        chunk: PlayerChunk,
        world_view_distance: i32,
    ) -> bool {
        chunk.is_within_view_distance(
            self.chunks_loaded_by_client,
            self.effective_chunk_view_distance(world_view_distance),
        )
    }

    fn load_world(
        &mut self,
        client: &mut Client,
        world_name: Identifier,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
        time_packet: SetTimePacket,
    ) -> io::Result<()> {
        SetChunkCacheCenterPacket::new(self.loaded_chunk.x, self.loaded_chunk.z)
            .dispatch(client)?;
        self.spawn_position(client, world_name)?;
        time_packet.dispatch(client)?;
        SetHealthPacket::new(20.0, 20, 5.0).dispatch(client)?;
        self.sync_main_hand_attributes(client)?;
        self.sync_inventory(client)?;
        self.load_chunks(client, chunk_packets)?;
        self.sync_position(client)?;
        GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)
    }

    fn load_world_chunk_positions(
        &mut self,
        client: &mut Client,
        world_name: Identifier,
        chunks: Vec<PlayerChunk>,
        time_packet: SetTimePacket,
    ) -> io::Result<()> {
        SetChunkCacheCenterPacket::new(self.loaded_chunk.x, self.loaded_chunk.z)
            .dispatch(client)?;
        self.spawn_position(client, world_name)?;
        time_packet.dispatch(client)?;
        SetHealthPacket::new(20.0, 20, 5.0).dispatch(client)?;
        self.sync_main_hand_attributes(client)?;
        self.sync_inventory(client)?;
        self.queue_chunks(chunks);
        GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)
    }

    #[cfg(test)]
    fn update_chunks_after_border_crossing(
        &mut self,
        client: &mut Client,
        transition: PlayerChunkTransition,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
    ) -> io::Result<()> {
        if !self
            .chunk_update_limit_checker
            .add_to_history(transition.next)
        {
            self.loaded_chunk = transition.next;
            return Ok(());
        }
        SetChunkCacheCenterPacket::new(transition.next.x, transition.next.z).dispatch(client)?;
        let effective_view_distance = self.effective_chunk_view_distance(i32::MAX - 1);
        self.forget_chunks(
            client,
            transition.departing,
            transition.next,
            effective_view_distance,
        )?;
        self.load_chunks(client, chunk_packets)?;
        self.chunks_loaded_by_client = transition.next;
        self.loaded_chunk = transition.next;
        Ok(())
    }

    fn update_chunks_after_border_crossing_with_chunks(
        &mut self,
        client: &mut Client,
        transition: PlayerChunkTransition,
        chunks: Vec<PlayerChunk>,
        world_view_distance: i32,
    ) -> io::Result<()> {
        if !self
            .chunk_update_limit_checker
            .add_to_history(transition.next)
        {
            self.loaded_chunk = transition.next;
            return Ok(());
        }
        let effective_view_distance = self.effective_chunk_view_distance(world_view_distance);
        SetChunkCacheCenterPacket::new(transition.next.x, transition.next.z).dispatch(client)?;
        self.forget_chunks(
            client,
            transition.departing,
            transition.next,
            effective_view_distance,
        )?;
        self.queue_chunks(chunks);
        self.prune_queued_chunks_outside_view(transition.next, effective_view_distance);
        self.chunks_loaded_by_client = transition.next;
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
        &mut self,
        client: &mut Client,
        chunk_packets: Vec<ChunkDataAndUpdateLightPacket>,
    ) -> io::Result<()> {
        chunk_packets
            .into_iter()
            .for_each(|chunk_packet| self.send_chunk(chunk_packet));
        self.send_pending_chunks_to(client)
    }

    fn queue_chunks(&mut self, chunks: Vec<PlayerChunk>) {
        chunks.into_iter().for_each(|chunk| {
            self.queue_chunk(chunk);
        });
    }

    pub(crate) fn queue_loaded_chunk_if_current_view(
        &mut self,
        chunk: PlayerChunk,
        world_view_distance: i32,
    ) -> bool {
        if !chunk.is_within_view_distance(
            self.chunks_loaded_by_client,
            self.effective_chunk_view_distance(world_view_distance),
        ) {
            return false;
        }
        self.queue_chunk(chunk)
    }

    pub(crate) fn update_chunks_after_view_distance_change(
        &mut self,
        client: &mut Client,
        arriving: Vec<PlayerChunk>,
        departing: Vec<PlayerChunk>,
        world_view_distance: i32,
    ) -> io::Result<()> {
        let current_center = self.chunks_loaded_by_client;
        let effective_view_distance = self.effective_chunk_view_distance(world_view_distance);
        self.forget_chunks(client, departing, current_center, effective_view_distance)?;
        self.queue_chunks(arriving);
        self.prune_queued_chunks_outside_view(current_center, effective_view_distance);
        Ok(())
    }

    fn queue_chunk(&mut self, chunk: PlayerChunk) -> bool {
        if !self.queued_client_chunks.insert(chunk) {
            return false;
        }
        self.chunk_queue
            .push_back(QueuedPlayerChunk::from_chunk(chunk));
        true
    }

    pub fn send_chunk(&mut self, chunk_packet: ChunkDataAndUpdateLightPacket) {
        let queued_chunk = QueuedPlayerChunk::new(chunk_packet);
        if self.queued_client_chunks.insert(queued_chunk.chunk) {
            self.chunk_queue.push_back(queued_chunk);
        }
    }

    pub fn send_loaded_chunk(&mut self, chunk: &Chunk) -> bool {
        if !chunk.is_loaded() {
            return false;
        }
        self.queue_chunk(PlayerChunk::new(chunk.x(), chunk.z()))
    }

    pub(crate) fn send_loaded_chunk_position(&mut self, chunk: PlayerChunk) -> bool {
        self.queue_chunk(chunk)
    }

    #[cfg(test)]
    pub(crate) fn send_pending_chunks(&mut self) -> io::Result<()> {
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        self.send_pending_chunks_to(client)
    }

    fn send_pending_chunks_to(&mut self, client: &mut Client) -> io::Result<()> {
        self.send_pending_chunks_with(client, |queued_chunk| Ok(queued_chunk.packet))
    }

    pub(crate) fn send_pending_chunks_with(
        &mut self,
        client: &mut Client,
        mut packet_for_chunk: impl FnMut(
            QueuedPlayerChunk,
        ) -> io::Result<Option<ChunkDataAndUpdateLightPacket>>,
    ) -> io::Result<()> {
        if self.chunk_queue.is_empty() || self.chunk_batch_lead >= self.max_chunk_batch_lead {
            return Ok(());
        }
        self.sort_queued_chunks_by_distance();
        self.pending_chunk_count =
            (self.pending_chunk_count + self.target_chunks_per_tick).min(64.0);
        if self.pending_chunk_count < 1.0 {
            return Ok(());
        }
        let mut sent_chunk_count = 0;
        ChunkBatchStartPacket.dispatch(client)?;
        while self.ready_chunk_batch_size() > 0 {
            let Some(queued_chunk) = self.chunk_queue.pop_front() else {
                break;
            };
            self.queued_client_chunks.remove(&queued_chunk.chunk);
            let chunk_x = queued_chunk.chunk.x;
            let chunk_z = queued_chunk.chunk.z;
            let Some(packet) = packet_for_chunk(queued_chunk)? else {
                continue;
            };
            packet.dispatch(client)?;
            self.client_sent_chunks
                .insert(PlayerChunk::new(chunk_x, chunk_z));
            self.dispatch_player_chunk_load_event(client, chunk_x, chunk_z);
            self.pending_chunk_count -= 1.0;
            sent_chunk_count += 1;
        }
        ChunkBatchFinishedPacket::new(sent_chunk_count).dispatch(client)?;
        self.chunk_batch_lead += 1;
        if self.needs_chunk_position_sync {
            self.sync_position(client)?;
            self.needs_chunk_position_sync = false;
        }
        Ok(())
    }

    fn ready_chunk_batch_size(&self) -> i32 {
        let pending_chunks = self.pending_chunk_count.floor() as usize;
        pending_chunks.min(self.chunk_queue.len()) as i32
    }

    fn sort_queued_chunks_by_distance(&mut self) {
        let chunks_loaded_by_client = self.chunks_loaded_by_client;
        self.chunk_queue
            .make_contiguous()
            .sort_by_key(|queued_chunk| queued_chunk.chunk.distance_to(chunks_loaded_by_client));
    }

    pub fn queued_chunk_count(&self) -> usize {
        self.chunk_queue.len()
    }

    pub(crate) fn reset_chunk_queue_for_world_entry_or_teleport(&mut self) {
        self.chunk_queue.clear();
        self.queued_client_chunks.clear();
        self.client_sent_chunks.clear();
        self.needs_chunk_position_sync = true;
        self.target_chunks_per_tick = 9.0;
        self.pending_chunk_count = 0.0;
        self.chunk_batch_lead = 0;
        self.chunk_update_limit_checker.clear_history();
    }

    fn forget_chunks(
        &mut self,
        client: &mut Client,
        chunks: Vec<PlayerChunk>,
        current_center: PlayerChunk,
        effective_view_distance: i32,
    ) -> io::Result<()> {
        chunks.into_iter().try_for_each(|chunk| {
            self.forget_loaded_chunk_if_outside_view(
                client,
                chunk,
                current_center,
                effective_view_distance,
            )
        })
    }

    pub(crate) fn forget_loaded_chunk(
        &mut self,
        client: &mut Client,
        chunk: PlayerChunk,
    ) -> io::Result<()> {
        self.remove_queued_chunk(chunk);
        if !self.client_sent_chunks.remove(&chunk) {
            return Ok(());
        }
        ForgetLevelChunkPacket::new(chunk.packet_position()).dispatch(client)?;
        self.dispatch_player_chunk_unload_event(client, chunk.x, chunk.z);
        Ok(())
    }

    fn forget_loaded_chunk_if_outside_view(
        &mut self,
        client: &mut Client,
        chunk: PlayerChunk,
        current_center: PlayerChunk,
        effective_view_distance: i32,
    ) -> io::Result<()> {
        self.remove_queued_chunk(chunk);
        if chunk.is_within_view_distance(current_center, effective_view_distance) {
            return Ok(());
        }
        if !self.client_sent_chunks.remove(&chunk) {
            return Ok(());
        }
        ForgetLevelChunkPacket::new(chunk.packet_position()).dispatch(client)?;
        self.dispatch_player_chunk_unload_event(client, chunk.x, chunk.z);
        Ok(())
    }

    fn remove_queued_chunk(&mut self, chunk: PlayerChunk) {
        self.chunk_queue.retain(|chunk_packet| {
            chunk_packet.chunk.x != chunk.x || chunk_packet.chunk.z != chunk.z
        });
        self.queued_client_chunks.remove(&chunk);
    }

    fn prune_queued_chunks_outside_view(
        &mut self,
        current_center: PlayerChunk,
        effective_view_distance: i32,
    ) {
        self.chunk_queue.retain(|queued_chunk| {
            queued_chunk
                .chunk
                .is_within_view_distance(current_center, effective_view_distance)
        });
        self.queued_client_chunks
            .retain(|chunk| chunk.is_within_view_distance(current_center, effective_view_distance));
    }
}

impl Player {
    fn dispatch_player_chunk_load_event(
        &mut self,
        client: &mut Client,
        chunk_x: i32,
        chunk_z: i32,
    ) {
        let Some(server_ptr) = client.server_ptr else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let player = self as *mut Player;
        PlayerChunkLoadEvent::new(player, chunk_x, chunk_z).dispatch(server, client);
    }

    fn dispatch_player_chunk_unload_event(
        &mut self,
        client: &mut Client,
        chunk_x: i32,
        chunk_z: i32,
    ) {
        let Some(server_ptr) = client.server_ptr else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let player = self as *mut Player;
        PlayerChunkUnloadEvent::new(player, chunk_x, chunk_z).dispatch(server, client);
    }
}

fn server_commands_packet(client: &mut Client) -> io::Result<()> {
    let Some(server) = client
        .server_ptr
        .map(|server| unsafe { &mut *(server as *mut crate::server::MinecraftServer) })
    else {
        return Ok(());
    };
    let commands_packet: CommandsPacket = server.command_manager.declare_commands_packet();
    commands_packet.dispatch(client)
}
