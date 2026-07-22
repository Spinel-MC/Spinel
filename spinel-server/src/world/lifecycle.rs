impl World {
    pub fn get_handle(&self) -> Option<crate::world::WorldHandle> {
        self.event_dispatcher
            .map(|server| crate::world::WorldHandle::new(server, self.uuid))
    }

    pub fn new(uuid: Uuid, dimension_type: RegistryKey<DimensionType>) -> Self {
        Self::new_with_cached_dimension_type(
            uuid,
            dimension_type.key().clone(),
            dimension_type,
            DimensionType::default(),
        )
    }

    pub fn new_with_dimension_name(
        uuid: Uuid,
        dimension_type: RegistryKey<DimensionType>,
        dimension_name: Identifier,
    ) -> Self {
        Self::new_with_cached_dimension_type(
            uuid,
            dimension_name,
            dimension_type,
            DimensionType::default(),
        )
    }

    pub fn new_with_registries(
        registries: &Registries,
        uuid: Uuid,
        dimension_type: RegistryKey<DimensionType>,
    ) -> Result<Self> {
        Self::new_with_dimension_name_and_registries(
            registries,
            uuid,
            dimension_type.key().clone(),
            dimension_type,
        )
    }

    pub fn new_with_dimension_name_and_registries(
        registries: &Registries,
        uuid: Uuid,
        dimension_name: Identifier,
        dimension_type: RegistryKey<DimensionType>,
    ) -> Result<Self> {
        let cached_dimension_type = registries
            .dimension_type()
            .get(&dimension_type)
            .cloned()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("The dimension {} is not registered.", dimension_type.key()),
                )
            })?;
        Ok(Self::new_with_cached_dimension_type(
            uuid,
            dimension_name,
            dimension_type,
            cached_dimension_type,
        ))
    }
    fn new_base(uuid: Uuid, name: Identifier) -> Self {
        let (completed_chunk_load_sender, completed_chunk_load_receiver) = mpsc::channel();
        Self {
            uuid,
            name: name.clone(),
            entities: Vec::new(),
            entity_tracker: EntityTracker::new(),
            chunks: HashMap::new(),
            cached_snapshot_chunks: RefCell::new(Arc::new(HashMap::new())),
            block_handlers: BlockHandlerRegistry::default(),
            block_placement_rules: BlockPlacementRuleRegistry::default(),
            linked_shared_worlds: Vec::new(),
            source_world: None,
            last_block_change_time: current_time_nanos(),
            currently_changing_blocks: HashMap::new(),
            pending_generation: HashMap::new(),
            loading_chunks: HashSet::new(),
            async_chunk_loads: HashMap::new(),
            completed_chunk_load_sender,
            completed_chunk_load_receiver,
            prepared_chunk_loads: HashMap::new(),
            next_chunk_load_ticket_id: 0,
            player_chunk_load_waiters: HashMap::new(),

            pending_entity_visibility_refreshes: VecDeque::new(),
            pending_entity_visibility_refresh_keys: HashSet::new(),
            generator: None,
            explosion_supplier: None,
            chunk_loader: Arc::new(NoopChunkLoader),
            chunk_supplier: ChunkSupplier::default(),
            registered: false,
            dimension_type: DimensionType::OVERWORLD,
            cached_dimension_type: DimensionType::default(),
            dimension_name: name.clone(),
            auto_chunk_load: true,
            world_age: 0,
            time: 0,
            time_rate: 1,
            time_synchronization_ticks: DEFAULT_TIME_SYNCHRONIZATION_TICKS,
            view_distance: DEFAULT_CHUNK_VIEW_DISTANCE,
            world_border: WorldBorder::DEFAULT,
            boss_bars: Vec::new(),
            scoreboard_teams: HashMap::new(),
            weather: Weather::CLEAR,
            transitioning_weather: Weather::CLEAR,
            remaining_rain_transition_ticks: 0,
            remaining_thunder_transition_ticks: 0,
            tag_handler: TagHandler::new_handler(),
            scheduler: WorldScheduler::default(),
            event_node: WorldEventNode::default(),
            event_dispatcher: None,
        }
    }

    pub(crate) fn new_with_cached_dimension_type(
        uuid: Uuid,
        name: Identifier,
        dimension_type: RegistryKey<DimensionType>,
        cached_dimension_type: DimensionType,
    ) -> Self {
        Self {
            dimension_type,
            cached_dimension_type,
            ..Self::new_base(uuid, name.clone())
        }
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub const fn identity(&self) -> WorldIdentity {
        WorldIdentity::new(self.uuid)
    }

    pub const fn pointers(&self) -> WorldPointers {
        WorldPointers::new(self.uuid)
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub const fn is_registered(&self) -> bool {
        self.registered
    }

    pub(crate) fn set_registered(&mut self, registered: bool) {
        self.registered = registered;
    }

    pub fn get_dimension_type(&self) -> &RegistryKey<DimensionType> {
        &self.dimension_type
    }

    pub const fn cached_dimension_type(&self) -> &DimensionType {
        &self.cached_dimension_type
    }

    pub fn dimension_name(&self) -> &Identifier {
        &self.dimension_name
    }

    pub fn is_in_void(&self, position: EntityPosition) -> bool {
        position.get_y() < f64::from(self.cached_dimension_type.min_y - 64)
    }

    pub const fn view_distance(&self) -> i32 {
        self.view_distance
    }

    pub fn set_view_distance(&mut self, view_distance: i32) {
        self.view_distance = view_distance;
    }

    pub fn shared_worlds(&self) -> &[Uuid] {
        &self.linked_shared_worlds
    }

    pub fn has_shared_worlds(&self) -> bool {
        !self.linked_shared_worlds.is_empty()
    }

    pub(crate) fn add_shared_world(&mut self, world: Uuid) -> bool {
        if self.linked_shared_worlds.contains(&world) {
            return false;
        }
        self.linked_shared_worlds.push(world);
        true
    }

    pub(crate) fn set_source_world(&mut self, world: Uuid) {
        self.source_world = Some(world);
    }

    pub const fn source_world(&self) -> Option<Uuid> {
        self.source_world
    }

    pub fn copy(&self) -> Self {
        let mut copied_world = Self::new_with_cached_dimension_type(
            Uuid::new_v4(),
            self.name.clone(),
            self.dimension_type.clone(),
            self.cached_dimension_type.clone(),
        );
        copied_world.dimension_name = self.dimension_name.clone();
        copied_world.source_world = Some(self.uuid);
        copied_world.last_block_change_time = self.last_block_change_time;
        copied_world.tag_handler = self.tag_handler.copy();
        copied_world.scoreboard_teams = self.scoreboard_teams.clone();
        self.chunks.iter().for_each(|(position, chunk)| {
            let mut copied_chunk = chunk.copy_for_position(*position);
            copied_chunk.set_world(copied_world.uuid);
            copied_world.chunks.insert(*position, copied_chunk);
            copied_world
                .entity_tracker
                .create_chunk_partition(*position);
        });
        copied_world
    }

    pub fn event_node(&mut self) -> &mut WorldEventNode {
        &mut self.event_node
    }

    pub fn update_snapshot(&self) -> WorldSnapshot {
        WorldSnapshot::from_world(self)
    }

    pub(crate) fn snapshot_chunks(&self) -> Arc<HashMap<ChunkPosition, ChunkSnapshot>> {
        let mut cached_snapshot_chunks = self.cached_snapshot_chunks.borrow_mut();
        let snapshot_chunks = Arc::make_mut(&mut cached_snapshot_chunks);
        snapshot_chunks.retain(|position, _| self.is_chunk_loaded(*position));
        self.chunks().for_each(|chunk| {
            let position = ChunkPosition::new(chunk.x(), chunk.z());
            let entity_ids = self
                .chunk_entities(position)
                .into_iter()
                .map(|entity| entity.get_entity_id())
                .collect::<Vec<_>>();
            let cached_snapshot_is_current = snapshot_chunks
                .get(&position)
                .is_some_and(|snapshot| snapshot.matches_chunk(chunk, &entity_ids));
            if cached_snapshot_is_current {
                return;
            }
            snapshot_chunks.insert(
                position,
                ChunkSnapshot::from_chunk_with_entity_ids(chunk, entity_ids),
            );
        });
        Arc::clone(&cached_snapshot_chunks)
    }

    fn refresh_creature_pathfinding_worlds(&mut self) {
        let pathfinding_world = Arc::new(self.update_snapshot());
        self.entities.iter_mut().for_each(|entity| {
            if let Entity::Creature(creature) = entity {
                creature.set_pathfinding_world(pathfinding_world.clone());
            }
        });
    }

    pub(crate) fn set_event_dispatcher(&mut self, server: usize) {
        self.event_dispatcher = Some(server);
    }

    pub(crate) fn dispatch_world_register_event(&mut self) {
        self.dispatch_world_event_node("WorldRegisterEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        WorldRegisterEvent::new(world).dispatch(server);
    }

    pub(crate) fn dispatch_world_unregister_event(&mut self) {
        self.dispatch_world_event_node("WorldUnregisterEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        WorldUnregisterEvent::new(world).dispatch(server);
    }

    fn use_client_event_dispatcher(&mut self, client: &Client) {
        if let Some(server_ptr) = client.server_ptr {
            self.event_dispatcher = Some(server_ptr);
        }
    }

    pub(crate) fn use_server_event_dispatcher(&mut self, server_ptr: usize) {
        self.event_dispatcher = Some(server_ptr);
    }
}
