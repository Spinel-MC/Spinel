const MAX_COMPLETED_CHUNK_LOADS_PER_TICK: usize = 32;

impl World {
    pub fn generator(&self) -> Option<&(dyn Generator + Send + Sync)> {
        self.generator.as_deref()
    }

    pub fn set_generator(
        &mut self,
        generator: impl Fn(&mut GenerationUnit) + Send + Sync + 'static,
    ) {
        self.generator = Some(Arc::new(generator));
    }

    pub fn set_fallible_generator(
        &mut self,
        generator: impl Fn(&mut GenerationUnit) -> std::result::Result<(), GenerateChunkError>
        + Send
        + Sync
        + 'static,
    ) {
        self.generator = Some(Arc::new(FallibleGenerator::new(generator)));
    }

    pub fn clear_generator(&mut self) {
        self.generator = None;
    }

    pub fn set_chunk_loader(&mut self, chunk_loader: impl ChunkLoader + 'static) {
        self.chunk_loader = Arc::new(chunk_loader);
    }

    pub fn chunk_loader(&self) -> &dyn ChunkLoader {
        self.chunk_loader.as_ref()
    }

    pub fn set_chunk_supplier(
        &mut self,
        create_chunk: impl Fn(ChunkPosition) -> Chunk + Send + Sync + 'static,
    ) {
        self.chunk_supplier = ChunkSupplier::new(create_chunk);
    }

    pub fn chunk_supplier(&self) -> &ChunkSupplier {
        &self.chunk_supplier
    }

    pub fn load_world(&mut self) -> Result<()> {
        let chunk_loader = self.chunk_loader.clone();
        chunk_loader.load_world(self)
    }

    pub fn save_world(&self) -> Result<()> {
        self.chunk_loader.save_world(self)
    }

    pub fn save_chunk(&self, position: ChunkPosition) -> Result<bool> {
        let Some(chunk) = self.chunks.get(&position) else {
            return Ok(false);
        };
        self.chunk_loader.save_chunk(chunk)?;
        Ok(true)
    }

    pub fn save_chunks(&self) -> Result<()> {
        let chunks = self.chunks.values().collect::<Vec<_>>();
        self.chunk_loader.save_chunks(&chunks)
    }

    pub fn chunk(&self, position: ChunkPosition) -> Option<&Chunk> {
        self.chunks.get(&position)
    }

    pub fn chunks(&self) -> impl Iterator<Item = &Chunk> {
        self.chunks.values()
    }

    pub fn chunk_at(&self, x: f64, z: f64) -> Option<&Chunk> {
        self.chunk(ChunkPosition::new(
            (x.floor() as i32).div_euclid(16),
            (z.floor() as i32).div_euclid(16),
        ))
    }

    pub fn chunk_at_position(&self, position: impl Into<ChunkPosition>) -> Option<&Chunk> {
        self.chunk(position.into())
    }

    pub fn is_chunk_loaded(&self, position: ChunkPosition) -> bool {
        self.chunks
            .get(&position)
            .is_some_and(|chunk| chunk.is_loaded())
    }

    pub fn is_chunk_loaded_at(&self, position: impl Into<ChunkPosition>) -> bool {
        self.is_chunk_loaded(position.into())
    }

    pub fn enable_auto_chunk_load(&mut self, enable: bool) {
        self.auto_chunk_load = enable;
    }

    pub const fn has_enabled_auto_chunk_load(&self) -> bool {
        self.auto_chunk_load
    }

    pub fn load_chunk(&mut self, position: ChunkPosition) -> Result<&mut Chunk> {
        self.load_chunk_with_event_flag(position, true)
    }

    pub fn load_chunk_at(&mut self, position: impl Into<ChunkPosition>) -> Result<&mut Chunk> {
        self.load_chunk(position.into())
    }

    pub fn load_chunk_result(&mut self, position: ChunkPosition) -> Result<&mut Chunk> {
        self.load_chunk(position)
    }

    pub fn load_optional_chunk(&mut self, position: ChunkPosition) -> Option<&mut Chunk> {
        if self.chunks.contains_key(&position) {
            return self.chunks.get_mut(&position);
        }
        if !self.auto_chunk_load {
            return None;
        }
        self.load_chunk(position).ok()
    }

    pub fn load_optional_chunk_at(
        &mut self,
        position: impl Into<ChunkPosition>,
    ) -> Option<&mut Chunk> {
        self.load_optional_chunk(position.into())
    }

    pub fn load_optional_chunk_result(
        &mut self,
        position: ChunkPosition,
    ) -> Result<Option<&mut Chunk>> {
        if self.chunks.contains_key(&position) {
            return Ok(self.chunks.get_mut(&position));
        }
        if !self.auto_chunk_load {
            return Ok(None);
        }
        self.load_chunk(position).map(Some)
    }

    pub fn load_optional_chunks(
        &mut self,
        positions: &[ChunkPosition],
    ) -> Result<Vec<ChunkPosition>> {
        positions
            .iter()
            .copied()
            .map(|position| {
                self.load_optional_chunk_result(position)
                    .map(|chunk| chunk.map(|_| position))
            })
            .collect::<Result<Vec<_>>>()
            .map(|positions| positions.into_iter().flatten().collect())
    }

    pub fn retrieve_chunk<'world>(
        &'world self,
        current_chunk: Option<&'world Chunk>,
        position: impl Into<ChunkPosition>,
    ) -> Option<&'world Chunk> {
        let position = position.into();
        let current_chunk_matches = current_chunk.is_some_and(|chunk| {
            chunk.is_loaded() && chunk.x() == position.x && chunk.z() == position.z
        });
        if current_chunk_matches {
            return current_chunk;
        }
        self.chunk(position)
    }

    pub fn load_chunk_future(&mut self, position: ChunkPosition) -> Result<ChunkLoadTicket> {
        self.load_chunk_future_with_optional_flag(position, true)
            .and_then(|ticket| {
                ticket.ok_or_else(|| Error::new(ErrorKind::NotFound, "Chunk was not loaded"))
            })
    }

    pub fn load_optional_chunk_future(
        &mut self,
        position: ChunkPosition,
    ) -> Result<Option<ChunkLoadTicket>> {
        self.load_chunk_future_with_optional_flag(position, self.auto_chunk_load)
    }

    pub fn complete_chunk_load(&mut self, ticket: &ChunkLoadTicket) -> Result<bool> {
        if ticket.is_completed() {
            return Ok(true);
        }
        if self.chunks.contains_key(&ticket.position) {
            ticket.complete();
            self.async_chunk_loads.remove(&ticket.position);
            self.queue_waiting_players_for_loaded_chunk(ticket.position);
            return Ok(true);
        }
        self.receive_completed_chunk_loads();
        let Some((_, prepared_chunk_load)) = self.prepared_chunk_loads.remove(&ticket.id) else {
            return Ok(false);
        };
        self.async_chunk_loads.remove(&ticket.position);
        let PreparedChunkLoad {
            mut chunk,
            generation_forks,
            requires_generation_completion,
        } = match prepared_chunk_load {
            Ok(prepared_chunk_load) => prepared_chunk_load,
            Err(failure) => {
                self.player_chunk_load_waiters.remove(&ticket.position);
                self.dispatch_chunk_loader_error_event(ChunkLoaderFailure::new(
                    failure.operation,
                    Some(ticket.position),
                    failure.error.to_string(),
                ));
                return Err(failure.error);
            }
        };
        chunk.set_world(self.uuid);
        generation_forks
            .into_iter()
            .for_each(|fork| self.store_generation_fork(fork));
        self.chunks.insert(ticket.position, chunk);
        self.entity_tracker.create_chunk_partition(ticket.position);
        if requires_generation_completion {
            let Some(mut chunk) = self.chunks.remove(&ticket.position) else {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Prepared chunk disappeared before generation completion",
                ));
            };
            self.apply_pending_generation(&mut chunk);
            chunk.on_generate();
            self.chunks.insert(ticket.position, chunk);
            self.invalidate_generated_chunk_lighting(ticket.position);
        }
        if let Some(chunk) = self.chunks.get_mut(&ticket.position) {
            chunk.on_load();
        }
        self.dispatch_world_chunk_load_event(ticket.position);
        ticket.complete();
        self.queue_waiting_players_for_loaded_chunk(ticket.position);
        Ok(true)
    }

    pub fn chunk_load_in_progress(&self, position: ChunkPosition) -> bool {
        self.async_chunk_loads.contains_key(&position)
    }

    pub fn save_world_future(&self) -> WorldIoTask {
        self.optional_io_task(self.chunk_loader.supports_parallel_saving(), {
            let chunk_loader = self.chunk_loader.clone();
            let world_tags = crate::world::WorldPersistentTags::from_world(self);
            move || chunk_loader.save_world_tags(world_tags)
        })
    }

    pub fn save_chunk_future(&self, position: ChunkPosition) -> WorldIoTask {
        let Some(chunk) = self
            .chunks
            .get(&position)
            .map(|chunk| chunk.copy_for_position(position))
        else {
            return WorldIoTask::completed(Ok(()));
        };
        self.optional_io_task(self.chunk_loader.supports_parallel_saving(), {
            let chunk_loader = self.chunk_loader.clone();
            move || chunk_loader.save_chunk(&chunk)
        })
    }

    pub fn save_chunks_future(&self) -> WorldIoTask {
        let chunks = self
            .chunks
            .values()
            .map(|chunk| chunk.copy_for_position(ChunkPosition::new(chunk.x(), chunk.z())))
            .collect::<Vec<_>>();
        self.optional_io_task(self.chunk_loader.supports_parallel_saving(), {
            let chunk_loader = self.chunk_loader.clone();
            move || {
                let chunk_refs = chunks.iter().collect::<Vec<_>>();
                chunk_loader.save_chunks(&chunk_refs)
            }
        })
    }

    pub fn unload_chunk(&mut self, chunk: impl Into<ChunkPosition>) -> Result<bool> {
        let position = chunk.into();
        if !self.chunks.contains_key(&position) {
            return Ok(false);
        }
        self.send_chunk_unload_to_players(position)?;
        self.dispatch_world_chunk_unload_event(position);
        self.remove_entities_in_chunk(position);
        self.entity_tracker.delete_chunk_partition(position);
        let Some(mut chunk) = self.chunks.remove(&position) else {
            return Ok(false);
        };
        chunk.unload();
        self.chunk_loader.unload_chunk(&mut chunk)?;
        Ok(true)
    }

    pub fn tick_chunks(&mut self, time: u64) -> usize {
        if self.chunks.values().any(Chunk::lighting_update_is_due) {
            let has_skylight = self.cached_dimension_type.has_skylight;
            self.chunks
                .values_mut()
                .for_each(|chunk| {
                    chunk.relight_invalidated_sections(has_skylight);
                });
        }
        let mut lighting_packets = Vec::new();
        let ticked_block_count = self
            .chunks
            .iter_mut()
            .filter(|(_, chunk)| chunk.is_loaded())
            .map(|(position, chunk)| {
                let light_data = chunk.tick_lighting();
                if let Some(light_data) = light_data {
                    lighting_packets.push((
                        *position,
                        LightUpdatePacket::new(position.x, position.z, light_data),
                    ));
                }
                chunk.tick(self.uuid, &self.block_handlers, time)
            })
            .sum();
        lighting_packets.into_iter().for_each(|(position, packet)| {
            let _ = self.dispatch_packet_to_chunk_viewers(position, packet);
        });
        ticked_block_count
    }

    fn dispatch_chunk_loader_failures(&mut self) {
        if self.event_dispatcher.is_none() {
            return;
        }
        self.chunk_loader
            .drain_failures()
            .into_iter()
            .for_each(|failure| self.dispatch_chunk_loader_error_event(failure));
    }

    fn dispatch_chunk_loader_error_event(&mut self, failure: ChunkLoaderFailure) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        ChunkLoaderErrorEvent::new(
            world,
            failure.operation,
            failure.chunk_position,
            failure.message,
        )
        .dispatch(server);
    }

    fn dispatch_world_chunk_load_event(&mut self, position: ChunkPosition) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        WorldChunkLoadEvent::new(world, position).dispatch(server);
    }

    fn dispatch_world_chunk_unload_event(&mut self, position: ChunkPosition) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        WorldChunkUnloadEvent::new(world, position).dispatch(server);
    }

    fn send_chunk_unload_to_players(&mut self, position: ChunkPosition) -> Result<()> {
        let player_chunk = PlayerChunk {
            x: position.x,
            z: position.z,
        };
        let world_view_distance = self.view_distance;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.has_entered_world()
                        && player.has_chunk_loaded_by_client(player_chunk, world_view_distance) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .try_for_each(|player| {
                let Some(client) = player.get_client_mut().map(|client| client as *mut Client)
                else {
                    return Ok(());
                };
                let client = unsafe { &mut *client };
                player.forget_loaded_chunk(client, player_chunk)
            })
    }

    fn remove_entities_in_chunk(&mut self, position: ChunkPosition) {
        let removed_entity_ids = self
            .entities
            .iter()
            .filter(|entity| chunk_position_for_entity_position(entity.get_position()) == position)
            .map(Entity::get_entity_id)
            .collect::<Vec<_>>();
        removed_entity_ids.into_iter().for_each(|entity_id| {
            self.remove_entity(entity_id);
        });
    }

    pub fn regenerate_chunk(&mut self, position: ChunkPosition) {
        if let Some(chunk) = self.chunks.get_mut(&position) {
            chunk.clear_invalidated();
        }
        self.generate_chunk(position);
    }

    pub fn generate_chunk(&mut self, position: ChunkPosition) {
        let _ = self.generate_chunk_result(position);
    }

    pub fn generate_chunk_result(&mut self, position: ChunkPosition) -> Result<bool> {
        self.load_chunk(position)?;
        let Some(generator) = self.generator.take() else {
            return Ok(self.chunks.contains_key(&position));
        };
        let generation_result =
            self.generate_loaded_chunk_with_result(position, generator.as_ref());
        self.generator = Some(generator);
        generation_result
    }

    pub fn generate_chunk_with_result(
        &mut self,
        position: ChunkPosition,
        generator: &(dyn Generator + Send + Sync),
    ) -> Result<bool> {
        self.load_chunk(position)?;
        self.generate_loaded_chunk_with_result(position, generator)
    }

    fn generate_loaded_chunk_with_result(
        &mut self,
        position: ChunkPosition,
        generator: &(dyn Generator + Send + Sync),
    ) -> Result<bool> {
        let Some(mut chunk) = self.chunks.remove(&position) else {
            return Ok(false);
        };
        let generation_result = self.apply_generation(&mut chunk, generator);
        self.chunks.insert(position, chunk);
        generation_result.map(|_| {
            self.invalidate_generated_chunk_lighting(position);
            self.queue_chunk_for_viewers(position);
            true
        })
    }

    fn unload_chunks_without_online_viewers(&mut self) -> Result<usize> {
        if !self.has_online_players() {
            return Ok(0);
        }
        let unload_positions = self
            .chunks
            .keys()
            .copied()
            .filter(|position| !self.chunk_has_online_viewer(*position))
            .collect::<Vec<_>>();
        let mut unloaded_chunk_count = 0;
        for position in unload_positions {
            if self.unload_chunk(position)? {
                unloaded_chunk_count += 1;
            }
        }
        Ok(unloaded_chunk_count)
    }

    fn has_online_players(&self) -> bool {
        self.entities.iter().any(|entity| match entity {
            Entity::Player(player) => player.has_entered_world() && player.is_online(),
            _ => false,
        })
    }

    fn chunk_has_online_viewer(&self, position: ChunkPosition) -> bool {
        let player_chunk = PlayerChunk::new(position.x, position.z);
        self.entities.iter().any(|entity| match entity {
            Entity::Player(player) => {
                player.has_entered_world()
                    && player.is_online()
                    && player.has_chunk_loaded_by_client(player_chunk, self.view_distance)
            }
            _ => false,
        })
    }

    pub fn send_chunk_to_viewers(
        &mut self,
        position: ChunkPosition,
        _registries: &Registries,
    ) -> Result<()> {
        self.queue_chunk_for_viewers(position);
        Ok(())
    }

    fn queue_chunk_for_viewers(&mut self, position: ChunkPosition) {
        let Some(chunk) = self.chunks.get(&position).filter(|chunk| chunk.is_loaded()) else {
            return;
        };
        let viewer_ids = chunk.viewers().collect::<HashSet<_>>();
        if viewer_ids.is_empty() {
            return;
        }
        let player_chunk = PlayerChunk::new(position.x, position.z);
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if viewer_ids.contains(&player.get_entity_id().get_value()) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .for_each(|player| {
                player.send_loaded_chunk_position(player_chunk);
            });
    }

    fn apply_generator(&mut self, chunk: &mut Chunk) -> Result<()> {
        if !chunk.requires_generation_completion() {
            return Ok(());
        }
        let generation_result = if chunk.requires_generation() {
            let Some(generator) = self.generator.take() else {
                self.apply_pending_generation(chunk);
                chunk.on_generate();
                return Ok(());
            };
            let generation_result = self.apply_generation(chunk, generator.as_ref());
            self.generator = Some(generator);
            generation_result
        } else {
            self.apply_pending_generation(chunk);
            Ok(())
        };
        chunk.on_generate();
        generation_result
    }

    fn apply_generation(
        &mut self,
        chunk: &mut Chunk,
        generator: &(dyn Generator + Send + Sync),
    ) -> Result<()> {
        generate_chunk(chunk, generator)?
            .into_iter()
            .for_each(|fork| self.store_generation_fork(fork));
        self.apply_pending_generation(chunk);
        Ok(())
    }

    fn finish_new_chunk_generation(&mut self, position: ChunkPosition) -> Result<bool> {
        let Some(mut chunk) = self.chunks.remove(&position) else {
            return Ok(false);
        };
        let chunk_will_generate = chunk.requires_generation_completion();
        let generation_result = self.apply_generator(&mut chunk);
        self.chunks.insert(position, chunk);
        if chunk_will_generate {
            self.invalidate_generated_chunk_lighting(position);
        }
        generation_result.map(|_| true)
    }

    fn load_initial_player_chunks(
        &mut self,
        player_address: SocketAddr,
        chunks: &[PlayerChunk],
    ) -> Result<()> {
        let chunk_load_tickets = chunks
            .iter()
            .copied()
            .map(|chunk| self.initial_player_chunk_load_ticket(player_address, chunk))
            .collect::<Result<Vec<_>>>()?;
        for chunk_load_ticket in chunk_load_tickets.into_iter().flatten() {
            while !self.complete_chunk_load(&chunk_load_ticket)? {
                std::thread::yield_now();
            }
        }
        Ok(())
    }

    fn initial_player_chunk_load_ticket(
        &mut self,
        player_address: SocketAddr,
        chunk: PlayerChunk,
    ) -> Result<Option<ChunkLoadTicket>> {
        let position = ChunkPosition::from(chunk);
        if self.is_chunk_loaded(position) {
            self.queue_loaded_chunk_for_player(player_address, chunk);
            return Ok(None);
        }
        let Some(chunk_load_ticket) = self.load_optional_chunk_future(position)? else {
            return Ok(None);
        };
        self.player_chunk_load_waiters
            .entry(position)
            .or_default()
            .push(player_address);
        Ok(Some(chunk_load_ticket))
    }

    pub(super) fn schedule_player_chunk_loads(
        &mut self,
        player_address: SocketAddr,
        chunks: &[PlayerChunk],
    ) -> Result<()> {
        for chunk in chunks {
            let position = ChunkPosition::from(*chunk);
            if self.is_chunk_loaded(position) {
                self.queue_loaded_chunk_for_player(player_address, *chunk);
                continue;
            }
            if self.load_optional_chunk_future(position)?.is_none() {
                continue;
            }
            self.player_chunk_load_waiters
                .entry(position)
                .or_default()
                .push(player_address);
        }
        Ok(())
    }

    pub(crate) fn process_completed_chunk_loads(&mut self) -> Result<()> {
        self.receive_completed_chunk_loads();
        let completed_tickets = self
            .prepared_chunk_loads
            .values()
            .take(MAX_COMPLETED_CHUNK_LOADS_PER_TICK)
            .map(|(ticket, _)| ticket.clone())
            .collect::<Vec<_>>();
        for ticket in completed_tickets {
            self.complete_chunk_load(&ticket)?;
        }
        Ok(())
    }

    fn receive_completed_chunk_loads(&mut self) {
        while let Ok(completed_chunk_load) = self.completed_chunk_load_receiver.try_recv() {
            self.prepared_chunk_loads.insert(
                completed_chunk_load.ticket.id,
                (
                    completed_chunk_load.ticket,
                    completed_chunk_load.prepared_chunk_load,
                ),
            );
        }
    }

    fn queue_waiting_players_for_loaded_chunk(&mut self, position: ChunkPosition) {
        let Some(player_addresses) = self.player_chunk_load_waiters.remove(&position) else {
            return;
        };
        let chunk = PlayerChunk::new(position.x, position.z);
        player_addresses.into_iter().for_each(|player_address| {
            self.queue_loaded_chunk_for_player(player_address, chunk);
        });
    }

    fn queue_loaded_chunk_for_player(&mut self, player_address: SocketAddr, chunk: PlayerChunk) {
        let Some(player_id) = self.player_by_addr_mut(&player_address).map(|player| {
            player.queue_loaded_chunk(chunk);
            player.get_entity_id()
        }) else {
            return;
        };
        let position = ChunkPosition::from(chunk);
        if let Some(world_chunk) = self.chunks.get_mut(&position) {
            world_chunk.add_viewer(player_id);
        }
    }

    fn loaded_chunk_packet(
        chunks: &mut HashMap<ChunkPosition, Chunk>,
        has_skylight: bool,
        position: ChunkPosition,
        registries: &Registries,
    ) -> Result<Option<ChunkDataAndUpdateLightPacket>> {
        if let Some(chunk) = chunks
            .get_mut(&position)
            .filter(|chunk| chunk.lighting_is_invalidated())
        {
            chunk.relight_invalidated_sections(has_skylight);
        }
        let Some(chunk) = chunks.get(&position) else {
            return Ok(None);
        };
        if !chunk.is_loaded() {
            return Ok(None);
        }
        Ok(Some(ChunkDataAndUpdateLightPacket::with_light_data(
            chunk.x(),
            chunk.z(),
            chunk
                .data(registries)
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Chunk has unregistered biome"))?,
            chunk.light_data(),
        )))
    }

    pub(crate) fn send_pending_chunks_for_client(
        &mut self,
        client: &mut Client,
        registries: &Registries,
    ) -> Result<()> {
        let Some(player) = self.player_pointer_by_addr(&client.addr) else {
            return Ok(());
        };
        let chunks = &mut self.chunks as *mut HashMap<ChunkPosition, Chunk>;
        let has_skylight = self.cached_dimension_type.has_skylight;
        unsafe { &mut *player }.send_pending_chunks_with(client, |queued_chunk| {
            let position = ChunkPosition::from(queued_chunk.chunk);
            Self::loaded_chunk_packet(unsafe { &mut *chunks }, has_skylight, position, registries)
        })
    }

    fn send_pending_chunks_for_player_address(
        &mut self,
        address: SocketAddr,
        registries: &Registries,
    ) -> Result<()> {
        let Some(player) = self.player_pointer_by_addr(&address) else {
            return Ok(());
        };
        let Some(client) = (unsafe { &mut *player }).get_client_mut() else {
            return Ok(());
        };
        let client = client as *mut Client;
        let chunks = &mut self.chunks as *mut HashMap<ChunkPosition, Chunk>;
        let has_skylight = self.cached_dimension_type.has_skylight;
        unsafe { &mut *player }.send_pending_chunks_with(unsafe { &mut *client }, |queued_chunk| {
            let position = ChunkPosition::from(queued_chunk.chunk);
            Self::loaded_chunk_packet(unsafe { &mut *chunks }, has_skylight, position, registries)
        })
    }

    fn movement_enters_unloaded_chunk(&self, transition: Option<&PlayerChunkTransition>) -> bool {
        let Some(transition) = transition else {
            return false;
        };
        let target_position = ChunkPosition::from(transition.next);
        !self.auto_chunk_load && !self.is_chunk_loaded(target_position)
    }

    fn store_generation_fork(&mut self, fork: GenerationFork) {
        fork.target_positions().into_iter().for_each(|position| {
            if let Some(chunk) = self.chunks.get_mut(&position) {
                fork.apply_to(chunk);
                return;
            }
            self.pending_generation
                .entry(position)
                .or_default()
                .push(fork.clone());
        });
    }

    fn apply_pending_generation(&mut self, chunk: &mut Chunk) {
        let position = ChunkPosition::new(chunk.x(), chunk.z());
        if let Some(forks) = self.pending_generation.remove(&position) {
            forks.iter().for_each(|fork| fork.apply_to(chunk));
        }
    }

    fn load_chunk_future_with_optional_flag(
        &mut self,
        position: ChunkPosition,
        should_load_missing_chunk: bool,
    ) -> Result<Option<ChunkLoadTicket>> {
        if self.is_chunk_loaded(position) {
            let ticket = self.next_completed_chunk_load_ticket(position);
            return Ok(Some(ticket));
        }
        if !should_load_missing_chunk {
            return Ok(None);
        }
        if let Some(ticket) = self.async_chunk_loads.get(&position).cloned() {
            return Ok(Some(ticket));
        }
        let ticket = self.next_chunk_load_ticket(position);
        let supports_parallel_loading = self.chunk_loader.supports_parallel_loading();
        let chunk_loader = self.chunk_loader.clone();
        let chunk_supplier = self.chunk_supplier.clone();
        let generator = self.generator.clone();
        let synchronously_loaded_chunk = if supports_parallel_loading {
            None
        } else {
            Some(chunk_loader.load_chunk(position)?)
        };
        if !supports_parallel_loading {
            let prepared_chunk_load = catch_unwind(AssertUnwindSafe(|| {
                prepare_chunk_load(
                    position,
                    chunk_loader,
                    chunk_supplier,
                    generator,
                    synchronously_loaded_chunk,
                )
            }))
            .unwrap_or_else(|panic_payload| {
                Err(PreparedChunkLoadFailure {
                    operation: ChunkLoaderOperation::LoadChunk,
                    error: Error::other(chunk_loading_panic_message(panic_payload)),
                })
            });
            self.prepared_chunk_loads
                .insert(ticket.id, (ticket.clone(), prepared_chunk_load));
            self.async_chunk_loads.insert(position, ticket.clone());
            return Ok(Some(ticket));
        }        let completed_chunk_load_sender = self.completed_chunk_load_sender.clone();
        let executor_ticket = ticket.clone();
        ChunkLoadingExecutor::global().execute(move || {
            let prepared_chunk_load = catch_unwind(AssertUnwindSafe(|| {
                prepare_chunk_load(
                    position,
                    chunk_loader,
                    chunk_supplier,
                    generator,
                    synchronously_loaded_chunk,
                )
            }))
            .unwrap_or_else(|panic_payload| {
                Err(PreparedChunkLoadFailure {
                    operation: ChunkLoaderOperation::LoadChunk,
                    error: Error::other(chunk_loading_panic_message(panic_payload)),
                })
            });
            let _ = completed_chunk_load_sender.send(CompletedChunkLoad {
                ticket: executor_ticket,
                prepared_chunk_load,
            });
        });
        self.async_chunk_loads.insert(position, ticket.clone());
        Ok(Some(ticket))
    }

    fn next_completed_chunk_load_ticket(&mut self, position: ChunkPosition) -> ChunkLoadTicket {
        let ticket = self.next_chunk_load_ticket(position);
        ticket.complete();
        ticket
    }

    fn next_chunk_load_ticket(&mut self, position: ChunkPosition) -> ChunkLoadTicket {
        self.next_chunk_load_ticket_id += 1;
        ChunkLoadTicket {
            id: self.next_chunk_load_ticket_id,
            position,
            is_completed: Arc::new(AtomicBool::new(false)),
        }
    }

    fn optional_io_task(
        &self,
        should_run_parallel: bool,
        task: impl FnOnce() -> Result<()> + Send + 'static,
    ) -> WorldIoTask {
        if !should_run_parallel {
            return WorldIoTask::completed(task());
        }
        WorldIoTask::running(std::thread::spawn(task))
    }

    fn load_chunk_with_event_flag(
        &mut self,
        position: ChunkPosition,
        should_dispatch_load_event: bool,
    ) -> Result<&mut Chunk> {
        if self.loading_chunks.contains(&position) {
            return self.chunks.get_mut(&position).ok_or_else(|| {
                Error::new(
                    ErrorKind::WouldBlock,
                    "Chunk load is already in progress for this position",
                )
            });
        }
        let chunk_was_missing = !self.chunks.contains_key(&position);
        if chunk_was_missing {
            self.loading_chunks.insert(position);
            let load_result = self.load_or_create_chunk(position);
            self.loading_chunks.remove(&position);
            load_result?;
        }
        self.finish_new_chunk_generation(position)?;
        if chunk_was_missing {
            let chunk = self.chunks.get_mut(&position).ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    "Loaded chunk disappeared before on-load callback",
                )
            })?;
            chunk.on_load();
        }
        if chunk_was_missing && should_dispatch_load_event {
            self.dispatch_world_chunk_load_event(position);
        }
        self.chunks
            .get_mut(&position)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Chunk was not loaded"))
    }

    fn load_or_create_chunk(&mut self, position: ChunkPosition) -> Result<()> {
        let mut chunk = match self.chunk_loader.load_chunk(position)? {
            Some(mut chunk) => {
                chunk.mark_loaded_from_storage();
                chunk
            }
            None => self.chunk_supplier.create_chunk(position),
        };
        chunk.set_world(self.uuid);
        self.chunks.insert(position, chunk);
        self.entity_tracker.create_chunk_partition(position);
        Ok(())
    }
}

#[derive(Clone)]
pub struct ChunkSupplier {
    create_chunk: Arc<dyn Fn(ChunkPosition) -> Chunk + Send + Sync>,
}

#[derive(Clone, Debug)]
pub struct ChunkLoadTicket {
    id: u64,
    position: ChunkPosition,
    is_completed: Arc<AtomicBool>,
}

pub struct EntityTeleportTicket {
    entity_id: EntityId,
    teleport: EntityTeleport,
    chunk_load_tickets: Vec<ChunkLoadTicket>,
    should_confirm: bool,
    completed: bool,
}

pub struct WorldIoTask {
    handle: Option<JoinHandle<Result<()>>>,
    completed: Option<Result<()>>,
}

struct CompletedChunkLoad {
    ticket: ChunkLoadTicket,
    prepared_chunk_load: std::result::Result<PreparedChunkLoad, PreparedChunkLoadFailure>,
}

struct PreparedChunkLoad {
    chunk: Chunk,
    generation_forks: Vec<GenerationFork>,
    requires_generation_completion: bool,
}

struct PreparedChunkLoadFailure {
    operation: ChunkLoaderOperation,
    error: Error,
}

impl ChunkSupplier {
    pub fn new(create_chunk: impl Fn(ChunkPosition) -> Chunk + Send + Sync + 'static) -> Self {
        Self {
            create_chunk: Arc::new(create_chunk),
        }
    }

    pub fn create_chunk(&self, position: ChunkPosition) -> Chunk {
        (self.create_chunk)(position)
    }
}

impl ChunkLoadTicket {
    pub const fn id(&self) -> u64 {
        self.id
    }

    pub const fn position(&self) -> ChunkPosition {
        self.position
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed.load(Ordering::SeqCst)
    }

    fn complete(&self) {
        self.is_completed.store(true, Ordering::SeqCst);
    }
}

impl PartialEq for ChunkLoadTicket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.position == other.position
    }
}

impl Eq for ChunkLoadTicket {}

impl EntityTeleportTicket {
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn teleport(&self) -> &EntityTeleport {
        &self.teleport
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

impl WorldIoTask {
    fn completed(result: Result<()>) -> Self {
        Self {
            handle: None,
            completed: Some(result),
        }
    }

    fn running(handle: JoinHandle<Result<()>>) -> Self {
        Self {
            handle: Some(handle),
            completed: None,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.completed.is_some() || self.handle.as_ref().is_some_and(JoinHandle::is_finished)
    }

    pub fn join(mut self) -> Result<()> {
        if let Some(result) = self.completed.take() {
            return result;
        }
        let Some(handle) = self.handle.take() else {
            return Ok(());
        };
        handle
            .join()
            .map_err(|_| Error::new(ErrorKind::Other, "World IO task panicked"))?
    }
}

impl Default for ChunkSupplier {
    fn default() -> Self {
        Self::new(Chunk::new)
    }
}

fn prepare_chunk_load(
    position: ChunkPosition,
    chunk_loader: Arc<dyn ChunkLoader>,
    chunk_supplier: ChunkSupplier,
    generator: Option<Arc<dyn Generator + Send + Sync>>,
    synchronously_loaded_chunk: Option<Option<Chunk>>,
) -> std::result::Result<PreparedChunkLoad, PreparedChunkLoadFailure> {
    let loaded_chunk = match synchronously_loaded_chunk {
        Some(loaded_chunk) => loaded_chunk,
        None => chunk_loader
            .load_chunk(position)
            .map_err(|error| PreparedChunkLoadFailure {
                operation: ChunkLoaderOperation::LoadChunk,
                error,
            })?,
    };
    let mut chunk = match loaded_chunk {
        Some(mut chunk) => {
            chunk.mark_loaded_from_storage();
            chunk
        }
        None => chunk_supplier.create_chunk(position),
    };
    let requires_generation_completion = chunk.requires_generation_completion();
    let generation_forks = match (chunk.requires_generation(), generator) {
        (true, Some(generator)) => {
            generate_chunk(&mut chunk, generator.as_ref()).map_err(|error| {
                PreparedChunkLoadFailure {
                    operation: ChunkLoaderOperation::GenerateChunk,
                    error,
                }
            })?
        }
        _ => Vec::new(),
    };
    Ok(PreparedChunkLoad {
        chunk,
        generation_forks,
        requires_generation_completion,
    })
}

fn chunk_loading_panic_message(panic_payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = panic_payload.downcast_ref::<String>() {
        return message.clone();
    }
    if let Some(message) = panic_payload.downcast_ref::<&'static str>() {
        return (*message).to_string();
    }
    "Chunk loading worker panicked without a string message.".to_string()
}

fn generate_chunk(
    chunk: &mut Chunk,
    generator: &(dyn Generator + Send + Sync),
) -> Result<Vec<GenerationFork>> {
    let size = BlockSize::new(16, (chunk.sections().len() as i32) << 4, 16);
    let start = BlockPosition::new(chunk.x() << 4, -64, chunk.z() << 4);
    let mut unit = GenerationUnit::new(size, start, chunk.sections().to_vec());
    generator.generate(&mut unit).map_err(Error::other)?;
    let (sections, generation_forks) = unit.into_generation();
    chunk.replace_sections(sections);
    Ok(generation_forks)
}




