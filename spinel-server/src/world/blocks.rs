impl World {
    pub const fn last_block_change_time(&self) -> u128 {
        self.last_block_change_time
    }

    pub fn refresh_last_block_change_time(&mut self) {
        self.last_block_change_time = current_time_nanos();
    }

    #[cfg(test)]
    pub(crate) fn block_change_guard_contains(
        &self,
        position: BlockPosition,
        block: Block,
    ) -> bool {
        self.currently_changing_blocks
            .get(&position)
            .is_some_and(|changed_block| changed_block.block() == block)
    }

    pub fn block_position_has_placement_collision(&self, position: BlockPosition) -> bool {
        self.block_placement_collision_entity(position).is_some()
    }

    pub fn block_placement_collision_entity(&self, position: BlockPosition) -> Option<EntityId> {
        let block_center = Vector3d {
            x: f64::from(position.x) + 0.5,
            y: f64::from(position.y),
            z: f64::from(position.z) + 0.5,
        };
        let block_box = EntityBoundingBox::new(1.0, 1.0, 1.0);
        self.entities
            .iter()
            .find(|entity| match entity {
                Entity::Creature(entity) => {
                    entity.can_prevent_block_placement()
                        && entity_strictly_intersects_block(
                            entity.get_relative_start(),
                            entity.get_relative_end(),
                            block_center,
                            block_box,
                        )
                }
                Entity::ExperienceOrb(entity) => {
                    entity.can_prevent_block_placement()
                        && entity_strictly_intersects_block(
                            entity.get_relative_start(),
                            entity.get_relative_end(),
                            block_center,
                            block_box,
                        )
                }
                Entity::Generic(entity) => {
                    entity.can_prevent_block_placement()
                        && entity_strictly_intersects_block(
                            entity.get_relative_start(),
                            entity.get_relative_end(),
                            block_center,
                            block_box,
                        )
                }
                Entity::Living(entity) => {
                    entity.can_prevent_block_placement()
                        && entity_strictly_intersects_block(
                            entity.get_relative_start(),
                            entity.get_relative_end(),
                            block_center,
                            block_box,
                        )
                }
                Entity::Item(entity) => {
                    entity.can_prevent_block_placement()
                        && entity_strictly_intersects_block(
                            entity.get_relative_start(),
                            entity.get_relative_end(),
                            block_center,
                            block_box,
                        )
                }
                Entity::Player(player) => {
                    player.can_prevent_block_placement()
                        && player_intersects_block(player.get_position(), block_center, block_box)
                }
                Entity::Projectile(entity) => {
                    entity.can_prevent_block_placement()
                        && entity_strictly_intersects_block(
                            entity.get_relative_start(),
                            entity.get_relative_end(),
                            block_center,
                            block_box,
                        )
                }
            })
            .map(Entity::get_entity_id)
    }

    pub(crate) fn chunk_is_read_only_at(&self, position: BlockPosition) -> bool {
        self.chunk(position.into())
            .is_some_and(|chunk| chunk.is_read_only())
    }

    pub(crate) fn refresh_chunk_for_client(
        &mut self,
        client: &Client,
        position: BlockPosition,
    ) -> bool {
        let player_chunk = ChunkPosition::from(position);
        self.player_by_addr_mut(&client.addr).is_some_and(|player| {
            player.send_loaded_chunk_position(PlayerChunk::new(player_chunk.x, player_chunk.z))
        })
    }

    pub fn register_block_handler(&mut self, block: Block, handler: impl BlockHandler + 'static) {
        self.block_handlers.register(block, handler);
    }

    pub fn register_block_placement_rule(&mut self, rule: impl BlockPlacementRule + 'static) {
        self.block_placement_rules.register(rule);
    }

    pub fn block_is_self_replaceable(&self, replacement: BlockReplacement) -> bool {
        self.block_placement_rules
            .rule(replacement.block())
            .is_some_and(|rule| rule.is_self_replaceable(replacement))
    }

    fn dispatch_world_section_invalidate_event(
        &mut self,
        section_x: i32,
        section_y: i32,
        section_z: i32,
    ) {
        self.dispatch_world_event_node("WorldSectionInvalidateEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        WorldSectionInvalidateEvent::new(world, section_x, section_y, section_z).dispatch(server);
    }

    fn dispatch_world_block_update_event(&mut self, position: BlockPosition, block: Block) {
        self.dispatch_world_event_node("WorldBlockUpdateEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        WorldBlockUpdateEvent::new(world, position, block).dispatch(server);
    }

    fn touch_entity_blocks(&self, entity_id: EntityId, position: EntityPosition) {
        let block_position = BlockPosition::new(
            position.get_x().floor() as i32,
            position.get_y().floor() as i32,
            position.get_z().floor() as i32,
        );
        let Some(block_instance) = self.loaded_block_instance_at(block_position) else {
            return;
        };
        let Some(handler) = block_instance.handler() else {
            return;
        };
        handler.on_touch(BlockHandlerTouch::new(
            block_instance.block(),
            self.uuid,
            block_position,
            entity_id,
        ));
    }

    pub fn block_at(&mut self, position: BlockPosition) -> Result<Block> {
        self.block_state_at(position).map(BlockState::block)
    }

    pub fn block_instance_at(&mut self, position: BlockPosition) -> Result<BlockInstance> {
        self.block_instance_at_with_condition(position, BlockLookupCondition::None)?
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Block instance was not found"))
    }

    pub fn block_state_at(&mut self, position: BlockPosition) -> Result<BlockState> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        Ok(self.load_chunk(chunk_position)?.block_state(position))
    }

    pub fn block_at_with_condition(
        &mut self,
        position: BlockPosition,
        condition: BlockLookupCondition,
    ) -> Result<Option<Block>> {
        self.block_instance_at_with_condition(position, condition)
            .map(|block_instance| block_instance.map(|block_instance| block_instance.block()))
    }

    pub fn block_instance_at_with_condition(
        &mut self,
        position: BlockPosition,
        condition: BlockLookupCondition,
    ) -> Result<Option<BlockInstance>> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        match condition {
            BlockLookupCondition::None => Ok(self
                .load_chunk(chunk_position)?
                .block_instance_with_condition(position, condition)),
            BlockLookupCondition::Cached | BlockLookupCondition::Type => Ok(self
                .chunks
                .get(&chunk_position)
                .filter(|chunk| chunk.is_loaded())
                .and_then(|chunk| chunk.block_instance_with_condition(position, condition))),
        }
    }

    pub fn biome_at(&mut self, position: BlockPosition) -> Result<RegistryKey<Biome>> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        Ok(self.load_chunk(chunk_position)?.biome(position))
    }

    pub fn set_biome(
        &mut self,
        position: BlockPosition,
        biome: RegistryKey<Biome>,
    ) -> Result<bool> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let biome_was_set = self.load_chunk(chunk_position)?.set_biome(position, biome);
        if biome_was_set {
            self.refresh_last_block_change_time();
        }
        Ok(biome_was_set)
    }

    pub fn set_block(&mut self, position: BlockPosition, block: Block) -> Result<bool> {
        self.set_block_instance(position, block.into())
    }

    pub fn set_block_instance(
        &mut self,
        position: BlockPosition,
        block_instance: BlockInstance,
    ) -> Result<bool> {
        self.set_block_instance_with_handler(position, block_instance, None, None)
    }

    pub fn set_block_state(
        &mut self,
        position: BlockPosition,
        block_state: BlockState,
    ) -> Result<bool> {
        self.set_block_instance(position, block_state.into())
    }

    fn set_block_instance_with_handler(
        &mut self,
        position: BlockPosition,
        block_instance: BlockInstance,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
    ) -> Result<bool> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.load_chunk(chunk_position)?;
        self.set_loaded_block_instance_with_handler(
            position,
            block_instance,
            placement,
            destroy,
            true,
            0,
        )
    }

    fn set_loaded_block_with_handler(
        &mut self,
        position: BlockPosition,
        block: Block,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
        do_block_updates: bool,
        update_distance: i32,
    ) -> Result<bool> {
        self.set_loaded_block_instance_with_handler(
            position,
            block.into(),
            placement,
            destroy,
            do_block_updates,
            update_distance,
        )
    }

    fn set_loaded_block_instance_with_handler(
        &mut self,
        position: BlockPosition,
        block_instance: BlockInstance,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
        do_block_updates: bool,
        update_distance: i32,
    ) -> Result<bool> {
        let block_state = self.block_state_after_placement_rule(
            block_instance.block_state(),
            position,
            placement.as_ref(),
            do_block_updates,
        );
        self.set_loaded_block_state_with_handler(
            position,
            block_instance.with_block_state(block_state),
            placement,
            destroy,
            do_block_updates,
            update_distance,
        )
    }

    fn set_loaded_block_state_with_handler(
        &mut self,
        position: BlockPosition,
        block_instance: impl Into<BlockInstance>,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
        do_block_updates: bool,
        update_distance: i32,
    ) -> Result<bool> {
        let block_instance = block_instance.into();
        let block_state = block_instance.block_state();
        let block = block_state.block();
        if self
            .currently_changing_blocks
            .get(&position)
            .is_some_and(|changed_block| *changed_block == block_state)
        {
            return Ok(false);
        }
        self.currently_changing_blocks.insert(position, block_state);
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let Some(mut chunk) = self.chunks.remove(&chunk_position) else {
            return Ok(false);
        };
        let block_was_set = chunk
            .try_set_block_instance_with_handler(
                position,
                block_instance,
                Some(&self.block_handlers),
                placement,
                destroy,
            )
            .block_was_set();
        self.chunks.insert(chunk_position, chunk);
        if !block_was_set {
            return Ok(false);
        }
        self.refresh_last_block_change_time();
        if do_block_updates {
            self.execute_neighbor_block_placement_rules(position, update_distance)?;
        }
        self.refresh_creature_pathfinding_worlds();
        self.broadcast_block_update(position, block_state)?;
        self.broadcast_block_entity_update(position)?;
        self.invalidate_neighbor_chunk_lighting(position);
        self.dispatch_world_block_update_event(position, block);
        Ok(true)
    }

    fn invalidate_neighbor_chunk_lighting(&mut self, position: BlockPosition) {
        let Some(changed_chunk) = self.chunks.get(&ChunkPosition::from(position)) else {
            return;
        };
        if !changed_chunk.is_lighting_chunk() || changed_chunk.is_lighting_invalidation_frozen() {
            return;
        }
        let section_y = position.y.div_euclid(16);
        self.chunks
            .values_mut()
            .filter(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
            .for_each(|chunk| {
                ((section_y - 1)..=(section_y + 1)).for_each(|section_y| {
                    chunk.invalidate_section(section_y);
                });
                chunk.schedule_lighting_update();
            });
    }

    fn invalidate_generated_chunk_lighting(&mut self, position: ChunkPosition) {
        if self
            .chunks
            .get(&position)
            .is_none_or(|chunk| !chunk.is_lighting_chunk())
        {
            return;
        }
        self.chunks
            .values_mut()
            .filter(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
            .for_each(|chunk| {
                (chunk.min_section()..chunk.max_section()).for_each(|section_y| {
                    chunk.invalidate_section(section_y);
                });
                chunk.schedule_generated_lighting_update();
            });
    }

    fn block_state_after_placement_rule(
        &self,
        block_state: BlockState,
        position: BlockPosition,
        placement: Option<&BlockHandlerPlacement>,
        do_block_updates: bool,
    ) -> BlockState {
        if !do_block_updates {
            return block_state;
        }
        let Some(placement) = placement else {
            return block_state;
        };
        let block = block_state.block();
        let Some(rule) = self.block_placement_rules.rule(block) else {
            return block_state;
        };
        let player = placement
            .player()
            .and_then(|player_id| self.entity_by_id(player_id))
            .and_then(|entity| match entity {
                Entity::Player(player) => Some(player),
                Entity::Creature(_) => None,
                Entity::ExperienceOrb(_) => None,
                Entity::Generic(_) => None,
                Entity::Item(_) => None,
                Entity::Living(_) => None,
                Entity::Projectile(_) => None,
            });
        let result_block = rule.block_place(BlockPlacementState::new(
            block,
            placement.block_face(),
            position,
            placement.cursor_position(),
            player.map(Player::get_position),
            placement.player(),
            placement.hand(),
            player.is_some_and(Player::is_sneaking),
        ));
        match result_block {
            Some(result_block) if result_block == block => block_state,
            Some(result_block) => result_block.default_state(),
            None => Block::AIR.default_state(),
        }
    }

    fn execute_neighbor_block_placement_rules(
        &mut self,
        position: BlockPosition,
        update_distance: i32,
    ) -> Result<()> {
        crate::events::player_block_interact::BlockFace::update_faces()
            .into_iter()
            .try_for_each(|update_face| {
                let (normal_x, normal_y, normal_z) = update_face.normal();
                let neighbor_position = BlockPosition::new(
                    position.x + normal_x,
                    position.y + normal_y,
                    position.z + normal_z,
                );
                self.update_neighbor_block_from_rule(
                    neighbor_position,
                    update_face.opposite(),
                    update_distance,
                )
            })
    }

    fn update_neighbor_block_from_rule(
        &mut self,
        neighbor_position: BlockPosition,
        from_face: crate::events::player_block_interact::BlockFace,
        update_distance: i32,
    ) -> Result<()> {
        let Some(neighbor_block) = self.loaded_block_at(neighbor_position) else {
            return Ok(());
        };
        if block_is_air(neighbor_block) {
            return Ok(());
        }
        let Some(rule) = self.block_placement_rules.rule(neighbor_block) else {
            return Ok(());
        };
        if update_distance >= rule.max_update_distance() {
            return Ok(());
        }
        let new_neighbor_block = rule.block_update(BlockUpdateState::new(
            neighbor_position,
            neighbor_block,
            from_face,
        ));
        if neighbor_block == new_neighbor_block {
            return Ok(());
        }
        let chunk_position = ChunkPosition::new(
            neighbor_position.x.div_euclid(16),
            neighbor_position.z.div_euclid(16),
        );
        if !self.is_chunk_loaded(chunk_position) {
            return Ok(());
        }
        self.set_loaded_block_with_handler(
            neighbor_position,
            new_neighbor_block,
            None,
            None,
            true,
            update_distance + 1,
        )?;
        Ok(())
    }

    pub fn loaded_block_at(&self, position: BlockPosition) -> Option<Block> {
        self.loaded_block_state_at(position).map(BlockState::block)
    }

    pub fn loaded_block_instance_at(&self, position: BlockPosition) -> Option<BlockInstance> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .and_then(|chunk| {
                chunk.block_instance_with_condition(position, BlockLookupCondition::None)
            })
    }

    pub fn loaded_block_state_at(&self, position: BlockPosition) -> Option<BlockState> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.block_state(position))
    }

    pub fn client_block_entity_nbt_at(&self, position: BlockPosition) -> Option<NbtCompound> {
        let block_instance = self.loaded_block_instance_at(position)?;
        self.client_block_entity_nbt(position, &block_instance)
    }

    pub fn target_block_position(
        &self,
        entity_id: EntityId,
        max_distance: i32,
    ) -> Option<BlockPosition> {
        self.line_of_sight(entity_id, max_distance)
            .into_iter()
            .next()
    }

    pub fn line_of_sight(&self, entity_id: EntityId, max_distance: i32) -> Vec<BlockPosition> {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return Vec::new();
        };
        let eye_position = entity_eye_position(entity);
        let direction = view_direction(entity.get_position());
        self.ray_positions(eye_position, direction, max_distance as f64)
            .into_iter()
            .filter(|position| {
                self.loaded_block_at(*position)
                    .is_some_and(block_is_sight_block)
            })
            .fold(Vec::new(), |mut positions, position| {
                if positions.last() != Some(&position) {
                    positions.push(position);
                }
                positions
            })
    }

    pub fn has_line_of_sight(&self, source_id: EntityId, target_id: EntityId) -> bool {
        self.has_exact_line_of_sight(source_id, target_id, false)
    }

    pub fn has_exact_line_of_sight(
        &self,
        source_id: EntityId,
        target_id: EntityId,
        exact_view: bool,
    ) -> bool {
        let Some(source) = self.entity_by_id(source_id) else {
            return false;
        };
        let Some(target) = self.entity_by_id(target_id) else {
            return false;
        };
        let source_eye_position = entity_eye_position(source);
        let target_eye_position = entity_eye_position(target);
        let target_direction = normalized_vector_between(source_eye_position, target_eye_position);
        if exact_view
            && !vectors_are_aligned(view_direction(source.get_position()), target_direction)
        {
            return false;
        }
        !self
            .ray_positions(
                source_eye_position,
                target_direction,
                vector_distance(source_eye_position, target_eye_position),
            )
            .into_iter()
            .any(|position| {
                self.loaded_block_at(position)
                    .is_some_and(block_is_sight_block)
            })
    }

    pub fn line_of_sight_entity(
        &self,
        entity_id: EntityId,
        range: f64,
        predicate: impl Fn(&Entity) -> bool,
    ) -> Option<&Entity> {
        let source = self.entity_by_id(entity_id)?;
        let source_eye_position = entity_eye_position(source);
        let direction = view_direction(source.get_position());
        self.entities
            .iter()
            .filter(|entity| entity.get_entity_id() != entity_id)
            .filter(|entity| predicate(entity))
            .filter_map(|entity| {
                let target_eye_position = entity_eye_position(entity);
                let distance = vector_distance(source_eye_position, target_eye_position);
                if distance > range {
                    return None;
                }
                if !ray_reaches_entity(source_eye_position, direction, entity) {
                    return None;
                }
                if !self.has_exact_line_of_sight(entity_id, entity.get_entity_id(), false) {
                    return None;
                }
                Some((distance, entity))
            })
            .min_by(|(first_distance, _), (second_distance, _)| {
                first_distance.total_cmp(second_distance)
            })
            .map(|(_, entity)| entity)
    }

    fn ray_positions(
        &self,
        start: Vector3d,
        direction: Vector3d,
        max_distance: f64,
    ) -> Vec<BlockPosition> {
        let step_count = (max_distance.max(0.0) * 4.0).ceil() as i32;
        (0..=step_count)
            .map(|step| step as f64 * 0.25)
            .map(|distance| {
                BlockPosition::new(
                    (start.x + direction.x * distance).floor() as i32,
                    (start.y + direction.y * distance).floor() as i32,
                    (start.z + direction.z * distance).floor() as i32,
                )
            })
            .collect()
    }

    pub fn block_light(&mut self, position: BlockPosition) -> u8 {
        let chunk_position = ChunkPosition::from(position);
        let requested_chunk_uses_world_lighting = self
            .chunks
            .get(&chunk_position)
            .is_some_and(Chunk::is_lighting_chunk);
        if requested_chunk_uses_world_lighting
            && self.chunks.values().any(Chunk::lighting_is_invalidated)
        {
            WorldLighting::relight(
                &mut self.chunks,
                self.cached_dimension_type.has_skylight,
                None,
            );
        } else if let Some(chunk) = self.chunks.get_mut(&chunk_position) {
            chunk.relight_block_light_at(position.y);
        }
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.block_light(position))
            .unwrap_or_default()
    }

    pub fn sky_light(&mut self, position: BlockPosition) -> u8 {
        let chunk_position = ChunkPosition::from(position);
        let requested_chunk_uses_world_lighting = self
            .chunks
            .get(&chunk_position)
            .is_some_and(Chunk::is_lighting_chunk);
        if requested_chunk_uses_world_lighting
            && self.chunks.values().any(Chunk::lighting_is_invalidated)
        {
            WorldLighting::relight(
                &mut self.chunks,
                self.cached_dimension_type.has_skylight,
                None,
            );
        } else if let Some(chunk) = self.chunks.get_mut(&chunk_position) {
            chunk.relight_sky_light_at(position.y);
        }
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.sky_light(position))
            .unwrap_or_default()
    }

    pub fn relight_chunks(&mut self, positions: &[ChunkPosition]) -> Vec<ChunkPosition> {
        let has_loaded_requested_chunk = positions.iter().any(|position| {
            self.chunks
                .get(position)
                .is_some_and(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
        });
        if !has_loaded_requested_chunk {
            return Vec::new();
        }
        WorldLighting::relight(
            &mut self.chunks,
            self.cached_dimension_type.has_skylight,
            Some(positions),
        )
    }

    pub fn invalidate_section(&mut self, section_x: i32, section_y: i32, section_z: i32) -> bool {
        let position = ChunkPosition::new(section_x, section_z);
        let Some(chunk) = self.chunks.get_mut(&position) else {
            return false;
        };
        if !chunk.invalidate_section(section_y) {
            return false;
        }
        self.dispatch_world_section_invalidate_event(section_x, section_y, section_z);
        true
    }

    pub fn block_position_is_loaded(&self, position: BlockPosition) -> bool {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.is_chunk_loaded(chunk_position)
    }

    pub(crate) fn refresh_block_for_player(
        &mut self,
        client: &mut Client,
        position: BlockPosition,
    ) -> Result<()> {
        let block_state = self.block_state_at(position)?;
        BlockUpdatePacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block_state.state_id(),
        )
        .dispatch(client)
    }

    pub(crate) fn refresh_block_entity_for_player(
        &mut self,
        client: &mut Client,
        position: BlockPosition,
    ) -> Result<()> {
        let Some(block_instance) = self.loaded_block_instance_at(position) else {
            return Ok(());
        };
        let Some(block_entity_type) = block_instance.block().block_entity_type() else {
            return Ok(());
        };
        BlockEntityDataPacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block_entity_type,
            self.client_block_entity_nbt(position, &block_instance),
        )
        .dispatch(client)
    }

    pub fn place_block(&mut self, placement: BlockHandlerPlacement) -> bool {
        self.place_block_with_updates(placement, true)
    }

    pub fn place_block_with_updates(
        &mut self,
        placement: BlockHandlerPlacement,
        do_block_updates: bool,
    ) -> bool {
        let chunk_position = ChunkPosition::new(
            placement.block_position().x.div_euclid(16),
            placement.block_position().z.div_euclid(16),
        );
        if !self.is_chunk_loaded(chunk_position) {
            return false;
        }
        let block_state = self.block_state_after_placement_rule(
            placement.block_state(),
            placement.block_position(),
            Some(&placement),
            do_block_updates,
        );
        self.set_loaded_block_state_with_handler(
            placement.block_position(),
            block_state,
            Some(placement),
            None,
            do_block_updates,
            0,
        )
        .unwrap_or(false)
    }

    pub fn break_block(
        &mut self,
        player_id: EntityId,
        position: BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
    ) -> bool {
        self.break_block_with_updates(player_id, position, block_face, true)
    }

    pub fn break_block_with_updates(
        &mut self,
        player_id: EntityId,
        position: BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
        do_block_updates: bool,
    ) -> bool {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let Some(chunk) = self.chunks.get(&chunk_position) else {
            return false;
        };
        if chunk.is_read_only() || !chunk.is_loaded() {
            return false;
        }
        let Some(block) = self.loaded_block_at(position) else {
            return false;
        };
        if block == Block::AIR {
            self.send_loaded_chunk_to_player(player_id, chunk_position);
            return false;
        }
        let Some(player) = self.player_pointer_for_block_break(player_id) else {
            return false;
        };
        let Some(result_block) =
            self.dispatch_player_block_break_event(player, block, position, block_face)
        else {
            return false;
        };
        let destroy =
            BlockHandlerDestroy::new(block, result_block, self.uuid, position, Some(player_id));
        let block_was_broken = self
            .set_loaded_block_with_handler(
                position,
                result_block,
                None,
                Some(destroy),
                do_block_updates,
                0,
            )
            .unwrap_or(false);
        if !block_was_broken {
            return false;
        }
        if do_block_updates {
            let _ =
                self.dispatch_block_break_effect_except(chunk_position, position, block, player_id);
        }
        true
    }

    pub fn interact_block_handler(
        &self,
        player_id: EntityId,
        hand: crate::entity::PlayerHand,
        block_face: crate::events::player_block_interact::BlockFace,
        position: BlockPosition,
        cursor_position: (f32, f32, f32),
    ) -> bool {
        let Some(block_instance) = self.loaded_block_instance_at(position) else {
            return true;
        };
        let Some(handler) = block_instance.handler() else {
            return true;
        };
        handler.on_interact(BlockHandlerInteraction::new(
            block_instance.block(),
            self.uuid,
            block_face,
            position,
            EntityPosition::new(
                f64::from(cursor_position.0),
                f64::from(cursor_position.1),
                f64::from(cursor_position.2),
                0.0,
                0.0,
            ),
            player_id,
            hand,
        ))
    }

    fn player_pointer_for_block_break(&mut self, player_id: EntityId) -> Option<*mut Player> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Player(player) if player.get_entity_id() == player_id => {
                Some(player as *mut Player)
            }
            _ => None,
        })
    }

    fn dispatch_player_block_break_event(
        &mut self,
        player: *mut Player,
        block: Block,
        position: BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
    ) -> Option<Block> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(Block::AIR);
        };
        let Some(client) = (unsafe { &mut *player })
            .get_client_mut()
            .map(|client| client as *mut Client)
        else {
            return Some(Block::AIR);
        };
        let mut event = PlayerBlockBreakEvent::new(player, block, Block::AIR, position, block_face);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let client = unsafe { &mut *client };
        event.dispatch(server, client);
        if event.is_cancelled() {
            return None;
        }
        Some(event.result_block())
    }

    fn send_loaded_chunk_to_player(&mut self, player_id: EntityId, position: ChunkPosition) {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.get_entity_id() == player_id => Some(player),
                _ => None,
            })
            .for_each(|player| {
                player.send_loaded_chunk_position(PlayerChunk::new(position.x, position.z));
            });
    }

    fn dispatch_block_break_effect_except(
        &mut self,
        chunk_position: ChunkPosition,
        position: BlockPosition,
        block: Block,
        excluded_player: EntityId,
    ) -> Result<()> {
        let packet = WorldEventPacket::new(
            DESTROY_BLOCK_WORLD_EVENT_ID,
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block.state_id(),
            false,
        );
        self.dispatch_packet_to_chunk_viewers_except(chunk_position, packet, excluded_player)
    }

    fn dispatch_packet_to_chunk_viewers_except<P>(
        &mut self,
        position: ChunkPosition,
        packet: P,
        excluded_player: EntityId,
    ) -> Result<()>
    where
        P: DataType + PacketStruct,
    {
        let Some(chunk) = self.chunks.get(&position) else {
            return Ok(());
        };
        let viewer_ids = chunk.viewers().collect::<HashSet<_>>();
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.get_entity_id() != excluded_player
                        && viewer_ids.contains(&player.get_entity_id().get_value()) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| client.send_packet(P::get_id(), &payload))
    }

    fn broadcast_block_update(
        &mut self,
        position: BlockPosition,
        block_state: BlockState,
    ) -> Result<()> {
        let block_position = Position {
            x: position.x,
            y: position.y,
            z: position.z,
        };
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|viewer_client| {
                BlockUpdatePacket::new(block_position, block_state.state_id())
                    .dispatch(viewer_client)
            })
    }

    fn broadcast_block_entity_update(&mut self, position: BlockPosition) -> Result<()> {
        let Some(block_instance) = self.loaded_block_instance_at(position) else {
            return Ok(());
        };
        let Some(block_entity_type) = block_instance.block().block_entity_type() else {
            return Ok(());
        };
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let block_entity_nbt = self.client_block_entity_nbt(position, &block_instance);
        let packet = BlockEntityDataPacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block_entity_type,
            block_entity_nbt,
        );
        self.dispatch_packet_to_chunk_viewers(chunk_position, packet)
    }

    fn client_block_entity_nbt(
        &self,
        _position: BlockPosition,
        block_instance: &BlockInstance,
    ) -> Option<NbtCompound> {
        block_instance.block().block_entity_type()?;
        let block_entity_nbt = block_instance.nbt_or_empty();
        let Some(handler) = block_instance.handler() else {
            return Some(block_entity_nbt);
        };
        let tags = handler.block_entity_tags();
        if tags.is_empty() {
            return Some(NbtCompound::new());
        }
        let mut filtered_nbt = NbtCompound::new();
        tags.into_iter().for_each(|tag| {
            tag.write(&mut filtered_nbt, tag.read(&block_entity_nbt));
        });
        Some(filtered_nbt)
    }
}

fn block_is_air(block: Block) -> bool {
    matches!(block, Block::AIR | Block::CAVE_AIR | Block::VOID_AIR)
}


fn block_is_sight_block(block: Block) -> bool {
    !block_is_air(block)
}

fn entity_eye_position(entity: &Entity) -> Vector3d {
    let position = entity.get_position();
    Vector3d {
        x: position.get_x(),
        y: position.get_y() + entity.get_eye_height(),
        z: position.get_z(),
    }
}

fn view_direction(position: EntityPosition) -> Vector3d {
    let yaw = position.get_yaw().to_radians() as f64;
    let pitch = position.get_pitch().to_radians() as f64;
    let pitch_cosine = pitch.cos();
    Vector3d {
        x: -yaw.sin() * pitch_cosine,
        y: -pitch.sin(),
        z: yaw.cos() * pitch_cosine,
    }
}

fn normalized_vector_between(start: Vector3d, end: Vector3d) -> Vector3d {
    let distance = vector_distance(start, end);
    if distance == 0.0 {
        return Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    Vector3d {
        x: (end.x - start.x) / distance,
        y: (end.y - start.y) / distance,
        z: (end.z - start.z) / distance,
    }
}

fn vector_distance(start: Vector3d, end: Vector3d) -> f64 {
    let delta_x = end.x - start.x;
    let delta_y = end.y - start.y;
    let delta_z = end.z - start.z;
    (delta_x.mul_add(delta_x, delta_y.mul_add(delta_y, delta_z * delta_z))).sqrt()
}

fn vectors_are_aligned(first: Vector3d, second: Vector3d) -> bool {
    let dot_product = first
        .x
        .mul_add(second.x, first.y.mul_add(second.y, first.z * second.z));
    dot_product > 0.995
}

fn ray_reaches_entity(start: Vector3d, direction: Vector3d, entity: &Entity) -> bool {
    let target = entity_eye_position(entity);
    let target_distance = vector_distance(start, target);
    if target_distance == 0.0 {
        return true;
    }
    let bounding_box = entity.get_bounding_box();
    let ray_direction = Vector3d {
        x: direction.x * target_distance,
        y: direction.y * target_distance,
        z: direction.z * target_distance,
    };
    RaycastBoundingBox::from_center_dimensions(
        entity.get_position().as_vector(),
        bounding_box.get_width(),
        bounding_box.get_height(),
        bounding_box.depth(),
    )
    .ray_intersection(start, ray_direction)
    .is_some()
}

fn player_intersects_block(
    player_position: EntityPosition,
    block_center: Vector3d,
    block_box: EntityBoundingBox,
) -> bool {
    let player_box = EntityType::PLAYER.get_bounding_box();
    let player_start = Vector3d {
        x: player_position.get_x() - player_box.get_width() / 2.0,
        y: player_position.get_y(),
        z: player_position.get_z() - player_box.depth() / 2.0,
    };
    let player_end = Vector3d {
        x: player_position.get_x() + player_box.get_width() / 2.0,
        y: player_position.get_y() + player_box.get_height(),
        z: player_position.get_z() + player_box.depth() / 2.0,
    };
    let block_start = Vector3d {
        x: block_center.x - block_box.get_width() / 2.0,
        y: block_center.y,
        z: block_center.z - block_box.depth() / 2.0,
    };
    let block_end = Vector3d {
        x: block_center.x + block_box.get_width() / 2.0,
        y: block_center.y + block_box.get_height(),
        z: block_center.z + block_box.depth() / 2.0,
    };

    boxes_strictly_intersect(player_start, player_end, block_start, block_end)
}

fn entity_strictly_intersects_block(
    entity_start: Vector3d,
    entity_end: Vector3d,
    block_center: Vector3d,
    block_box: EntityBoundingBox,
) -> bool {
    let block_start = Vector3d {
        x: block_center.x - block_box.get_width() / 2.0,
        y: block_center.y,
        z: block_center.z - block_box.depth() / 2.0,
    };
    let block_end = Vector3d {
        x: block_center.x + block_box.get_width() / 2.0,
        y: block_center.y + block_box.get_height(),
        z: block_center.z + block_box.depth() / 2.0,
    };
    boxes_strictly_intersect(entity_start, entity_end, block_start, block_end)
}

fn player_pose_fits_at(world: &World, player_position: EntityPosition, pose: EntityPose) -> bool {
    let Some(player_box) = pose.get_bounding_box(EntityType::PLAYER.get_bounding_box()) else {
        return false;
    };
    let player_start = Vector3d {
        x: player_position.get_x() - player_box.get_width() / 2.0,
        y: player_position.get_y(),
        z: player_position.get_z() - player_box.depth() / 2.0,
    };
    let player_end = Vector3d {
        x: player_position.get_x() + player_box.get_width() / 2.0,
        y: player_position.get_y() + player_box.get_height(),
        z: player_position.get_z() + player_box.depth() / 2.0,
    };
    pose_block_positions(player_start, player_end)
        .into_iter()
        .all(
            |block_position| match world.loaded_block_at(block_position) {
                Some(block) if block != Block::SCAFFOLDING && block.is_solid() => !boxes_intersect(
                    player_start,
                    player_end,
                    block_start(block_position),
                    block_end(block_position),
                ),
                _ => true,
            },
        )
}

fn pose_block_positions(player_start: Vector3d, player_end: Vector3d) -> Vec<BlockPosition> {
    let min_x = player_start.x.floor() as i32;
    let min_y = player_start.y.floor() as i32;
    let min_z = player_start.z.floor() as i32;
    let max_x = player_end.x.floor() as i32;
    let max_y = player_end.y.floor() as i32;
    let max_z = player_end.z.floor() as i32;
    (min_x..=max_x)
        .flat_map(|x| {
            (min_y..=max_y)
                .flat_map(move |y| (min_z..=max_z).map(move |z| BlockPosition::new(x, y, z)))
        })
        .collect()
}

fn block_start(block_position: BlockPosition) -> Vector3d {
    Vector3d {
        x: f64::from(block_position.x),
        y: f64::from(block_position.y),
        z: f64::from(block_position.z),
    }
}

fn block_end(block_position: BlockPosition) -> Vector3d {
    Vector3d {
        x: f64::from(block_position.x) + 1.0,
        y: f64::from(block_position.y) + 1.0,
        z: f64::from(block_position.z) + 1.0,
    }
}

fn boxes_intersect(
    first_start: Vector3d,
    first_end: Vector3d,
    second_start: Vector3d,
    second_end: Vector3d,
) -> bool {
    first_start.x <= second_end.x
        && first_end.x >= second_start.x
        && first_start.y <= second_end.y
        && first_end.y >= second_start.y
        && first_start.z <= second_end.z
        && first_end.z >= second_start.z
}

fn boxes_strictly_intersect(
    first_start: Vector3d,
    first_end: Vector3d,
    second_start: Vector3d,
    second_end: Vector3d,
) -> bool {
    first_start.x < second_end.x
        && first_end.x > second_start.x
        && first_start.y < second_end.y
        && first_end.y > second_start.y
        && first_start.z < second_end.z
        && first_end.z > second_start.z
}
