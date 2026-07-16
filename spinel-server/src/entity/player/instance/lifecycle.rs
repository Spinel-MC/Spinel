use crate::entity::player::chunks::PlayerChunk;
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::start_configuration::StartConfigurationPacket;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::RegistryKey;
use spinel_registry::dimension_type::DimensionType;
use std::io;
use uuid::Uuid;

use super::state::Player;

impl Player {
    pub fn set_pending_options(&mut self, spawning_world: Uuid, hardcore: bool) {
        self.pending_spawning_world = Some(spawning_world);
        self.hardcore = hardcore;
    }

    pub const fn get_pending_spawning_world(&self) -> Option<Uuid> {
        self.pending_spawning_world
    }

    pub const fn get_current_world(&self) -> Option<Uuid> {
        self.current_world
    }

    pub(crate) fn assign_world(&mut self, world: Uuid) {
        self.current_world = Some(world);
    }

    pub(crate) fn set_dimension_type(&mut self, dimension_type: RegistryKey<DimensionType>) {
        self.dimension_type = dimension_type;
    }

    pub fn get_dimension_type(&self) -> &RegistryKey<DimensionType> {
        &self.dimension_type
    }

    pub const fn is_hardcore(&self) -> bool {
        self.hardcore
    }

    pub fn unsafe_init(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        dimension_type_id: i32,
        world_name: Identifier,
        chunk_radius: i32,
        chunk_packets: Vec<
            spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket,
        >,
        world_border_packet: spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket,
        time_packet: spinel_core::network::clientbound::play::set_time::SetTimePacket,
        weather: crate::world::Weather,
    ) -> io::Result<()> {
        self.pending_spawning_world = None;
        self.living.revive();
        self.world_name = Some(world_name.clone());
        self.enter_world(
            client,
            ticks_per_second,
            dimension_type_id,
            world_name,
            chunk_radius,
            chunk_packets,
            world_border_packet,
            time_packet,
            weather,
        )
    }

    pub(crate) fn unsafe_init_with_chunk_positions(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        dimension_type_id: i32,
        world_name: Identifier,
        chunk_radius: i32,
        chunks: Vec<PlayerChunk>,
        world_border_packet: spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket,
        time_packet: spinel_core::network::clientbound::play::set_time::SetTimePacket,
        weather: crate::world::Weather,
    ) -> io::Result<()> {
        self.pending_spawning_world = None;
        self.living.revive();
        self.world_name = Some(world_name.clone());
        self.enter_world_with_chunk_positions(
            client,
            ticks_per_second,
            dimension_type_id,
            world_name,
            chunk_radius,
            chunks,
            world_border_packet,
            time_packet,
            weather,
        )
    }

    pub(in crate::entity::player) fn prepare_world_spawn(&mut self, world_name: Identifier) {
        self.pending_spawning_world = None;
        self.living.revive();
        self.world_name = Some(world_name);
    }

    pub fn start_configuration_phase(&mut self) -> io::Result<()> {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "player must be in play state for reconfiguration",
            ));
        }
        StartConfigurationPacket.dispatch(client)?;
        client.state = ConnectionState::Configuration;
        self.has_entered_world = false;
        Ok(())
    }

    pub(crate) const fn has_entered_world(&self) -> bool {
        self.has_entered_world
    }

    pub(crate) fn mark_entered_world(&mut self) {
        self.has_entered_world = true;
    }

    pub fn schedule_remove_after_ticks(&mut self, delay_ticks: u64) {
        self.delayed_remove_ticks = Some(self.alive_ticks.saturating_add(delay_ticks));
    }

    pub fn schedule_remove_after_duration(&mut self, duration: std::time::Duration) {
        let duration_millis = duration.as_millis();
        let delay_ticks = u64::try_from(duration_millis.div_ceil(50)).unwrap_or(u64::MAX);
        self.schedule_remove_after_ticks(delay_ticks);
    }
}
