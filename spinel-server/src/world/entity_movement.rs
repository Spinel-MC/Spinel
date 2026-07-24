impl World {
    pub fn teleport_entity(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
    ) -> Result<Option<EntityTeleport>> {
        self.teleport_entity_with_velocity(
            entity_id,
            position,
            Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            chunks,
            flags.with(TeleportFlags::DELTA_COORD),
        )
    }

    pub fn teleport_entity_with_velocity(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
    ) -> Result<Option<EntityTeleport>> {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return Ok(None);
        };
        if let Entity::Player(player) = entity {
            return self.teleport_player_with_velocity(
                player.uuid,
                position,
                velocity,
                chunks,
                flags,
                true,
            );
        }
        let previous_position = entity.get_position();
        let teleport = EntityTeleport::resolve(
            previous_position,
            entity.get_velocity(),
            position,
            velocity,
            chunks,
            flags,
        );
        self.dispatch_entity_teleport_event(entity_id, position, teleport.get_position(), flags);
        self.load_teleport_chunks(
            previous_position,
            teleport.get_position(),
            teleport.get_chunks(),
        )?;
        let entity = self
            .entity_by_id_mut(entity_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Entity was removed before teleport"))?;
        entity.set_position(teleport.get_position());
        entity.set_velocity(teleport.get_velocity());
        self.entity_tracker
            .move_entity(entity_id, teleport.get_position());
        self.refresh_passenger_positions(entity_id);
        self.schedule_entity_visibility_refresh(entity_id);
        let synchronization_packet = self
            .entity_by_id_mut(entity_id)
            .map(Entity::synchronize_position_packet)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Entity was removed after teleport"))?;
        self.send_packet_to_entity_viewers(entity_id, synchronization_packet)?;
        Ok(Some(teleport))
    }

    pub fn teleport_entity_future(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> Result<Option<EntityTeleportTicket>> {
        self.teleport_entity_future_with_velocity(
            entity_id,
            position,
            Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            chunks,
            flags.with(TeleportFlags::DELTA_COORD),
            should_confirm,
        )
    }

    pub fn teleport_entity_future_with_velocity(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> Result<Option<EntityTeleportTicket>> {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return Ok(None);
        };
        let previous_position = entity.get_position();
        let teleport = EntityTeleport::resolve(
            previous_position,
            entity.get_velocity(),
            position,
            velocity,
            chunks,
            flags,
        );
        self.dispatch_entity_teleport_event(entity_id, position, teleport.get_position(), flags);
        let chunk_load_tickets = self.begin_teleport_chunk_loads(
            previous_position,
            teleport.get_position(),
            teleport.get_chunks(),
        )?;
        Ok(Some(EntityTeleportTicket {
            entity_id,
            teleport,
            chunk_load_tickets,
            should_confirm,
            completed: false,
        }))
    }

    pub fn set_entity_velocity(&mut self, entity_id: EntityId, velocity: Velocity) -> Result<bool> {
        if self.entity_by_id(entity_id).is_none() {
            return Ok(false);
        }
        let Some(velocity) = self.dispatch_entity_velocity_event(entity_id, velocity) else {
            return Ok(false);
        };
        let entity = self.entity_by_id_mut(entity_id).ok_or_else(|| {
            Error::other(format!(
                "entity {entity_id:?} disappeared during velocity event dispatch"
            ))
        })?;
        entity.set_velocity(velocity);
        let velocity_packet = entity.get_velocity_packet();
        self.send_packet_to_player_viewers_and_self(entity_id, velocity_packet)?;
        Ok(true)
    }

    pub fn complete_entity_teleport(
        &mut self,
        ticket: &mut EntityTeleportTicket,
    ) -> Result<Option<EntityTeleport>> {
        if ticket.completed {
            return Ok(Some(ticket.teleport.clone()));
        }
        for chunk_load_ticket in &ticket.chunk_load_tickets {
            if !self.complete_chunk_load(chunk_load_ticket)? {
                return Ok(None);
            }
        }
        let entity_id = ticket.entity_id;
        let teleport = ticket.teleport.clone();
        let player_chunk_transition = match self.entity_by_id(entity_id) {
            Some(Entity::Player(player)) => player.get_chunk_transition(
                teleport.get_position().get_x(),
                teleport.get_position().get_y(),
                teleport.get_position().get_z(),
                self.view_distance,
            ),
            Some(_) => None,
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Entity was removed before teleport completion",
                ));
            }
        };
        if let Some(player_chunk_transition) = player_chunk_transition.as_ref() {
            let player_address = self
                .entity_by_id(entity_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => Some(player.get_address()),
                    _ => None,
                })
                .ok_or_else(|| Error::new(ErrorKind::NotFound, "Player was removed"))?;
            self.schedule_player_chunk_loads(player_address, &player_chunk_transition.arriving)?;
        }
        let world_view_distance = self.view_distance;
        let entity = self.entity_by_id_mut(entity_id).ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                "Entity was removed before teleport completion",
            )
        })?;
        match entity {
            Entity::Player(player) => {
                player
                    .refresh_chunks_after_teleport(player_chunk_transition, world_view_distance)?;
                player.apply_teleport(&teleport, ticket.should_confirm)?;
            }
            _ => {
                entity.set_position(teleport.get_position());
                entity.set_velocity(teleport.get_velocity());
            }
        }
        self.entity_tracker
            .move_entity(entity_id, teleport.get_position());
        self.refresh_passenger_positions(entity_id);
        self.schedule_entity_visibility_refresh(entity_id);
        let synchronization_packet = self
            .entity_by_id_mut(entity_id)
            .map(Entity::synchronize_position_packet)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Entity was removed after teleport"))?;
        self.send_packet_to_entity_viewers(entity_id, synchronization_packet)?;
        ticket.completed = true;
        Ok(Some(teleport))
    }

    pub fn set_entity_position(&mut self, entity_id: EntityId, position: EntityPosition) -> bool {
        let Some(previous_position) = self.entity_by_id(entity_id).map(Entity::get_position) else {
            return false;
        };
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return false;
        };
        entity.set_position(position);
        let current_position = entity.get_position();
        self.entity_tracker.move_entity(entity_id, current_position);
        self.refresh_passenger_positions(entity_id);
        if chunk_position_for_entity_position(previous_position)
            == chunk_position_for_entity_position(current_position)
        {
            return true;
        }
        self.schedule_entity_visibility_refresh(entity_id);
        true
    }

    pub fn steer_boat(
        &mut self,
        vehicle_id: EntityId,
        left_paddle_turning: bool,
        right_paddle_turning: bool,
    ) -> bool {
        let Some(Entity::Generic(vehicle)) = self.entity_by_id_mut(vehicle_id) else {
            return false;
        };
        if !vehicle.get_entity_type().path().contains("boat") {
            return false;
        }
        if vehicle.is_left_paddle_turning() != left_paddle_turning {
            vehicle.set_left_paddle_turning(left_paddle_turning);
        }
        if vehicle.is_right_paddle_turning() != right_paddle_turning {
            vehicle.set_right_paddle_turning(right_paddle_turning);
        }
        true
    }

    pub fn move_living_entity(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        on_ground: bool,
    ) -> Result<bool> {
        let Some((previous_position, current_position, movement_packet, head_look_packet)) =
            self.move_living_entity_state(entity_id, position, on_ground)
        else {
            return Ok(false);
        };
        self.entity_tracker.move_entity(entity_id, current_position);
        self.refresh_passenger_positions(entity_id);
        if chunk_position_for_entity_position(previous_position)
            != chunk_position_for_entity_position(current_position)
        {
            self.schedule_entity_visibility_refresh(entity_id);
        }
        self.send_packet_to_entity_viewers(entity_id, movement_packet)?;
        self.send_packet_to_entity_viewers(entity_id, head_look_packet)?;
        Ok(true)
    }

    pub fn look_living_entity_at_position(
        &mut self,
        entity_id: EntityId,
        target: EntityPosition,
        on_ground: bool,
    ) -> Result<bool> {
        let Some((rotation_packet, head_look_packet)) =
            self.look_generic_entity_state_at_position(entity_id, target, on_ground)
        else {
            return Ok(false);
        };
        self.send_packet_to_entity_viewers(entity_id, rotation_packet)?;
        self.send_packet_to_entity_viewers(entity_id, head_look_packet)?;
        Ok(true)
    }

    pub fn swing_living_entity_main_hand(&mut self, entity_id: EntityId) -> Result<bool> {
        let Some(animation_packet) = self.living_entity_main_hand_animation(entity_id) else {
            return Ok(false);
        };
        self.send_packet_to_entity_viewers(entity_id, animation_packet)?;
        Ok(true)
    }

    pub fn swing_creature_main_hand(&mut self, entity_id: EntityId) -> Result<bool> {
        let Some(animation_packet) = self.creature_main_hand_animation(entity_id) else {
            return Ok(false);
        };
        self.send_packet_to_entity_viewers(entity_id, animation_packet)?;
        Ok(true)
    }

    pub fn creature_attack_entity(
        &mut self,
        creature_id: EntityId,
        target_id: EntityId,
        should_swing_main_hand: bool,
    ) -> Result<bool> {
        if !self.creature_can_attack_entity(creature_id, target_id) {
            return Ok(false);
        }
        if should_swing_main_hand {
            let Some(animation_packet) = self.creature_main_hand_animation(creature_id) else {
                return Ok(false);
            };
            self.send_packet_to_entity_viewers(creature_id, animation_packet)?;
        }
        self.dispatch_entity_attack_event(creature_id, target_id);
        Ok(true)
    }

    fn apply_creature_ai_action(&mut self, action: CreatureAiAction) {
        match action {
            CreatureAiAction::Attack {
                source,
                target,
                should_swing_main_hand,
            } => {
                let _ = self.creature_attack_entity(source, target, should_swing_main_hand);
            }
            CreatureAiAction::Shoot {
                shooter,
                mut projectile,
                target,
                power,
                spread,
            } => {
                let Some((shooter_position, shooter_eye_height)) = self
                    .entity_by_id(shooter)
                    .map(|entity| (entity.get_position(), entity.get_eye_height()))
                else {
                    return;
                };
                projectile.set_shooter(Some(shooter));
                projectile.set_position(shooter_position.get_offset(0.0, shooter_eye_height, 0.0));
                let projectile_id = projectile.get_entity_id();
                if !self.add_entity(Entity::Projectile(projectile)) {
                    return;
                }
                self.shoot_projectile(projectile_id, target, power, spread);
            }
        }
    }

    fn move_living_entity_state(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        on_ground: bool,
    ) -> Option<(
        EntityPosition,
        EntityPosition,
        EntityPositionAndRotationPacket,
        EntityHeadLookPacket,
    )> {
        let Some(Entity::Living(entity)) = self.entity_by_id_mut(entity_id) else {
            return None;
        };
        let previous_position = entity.get_position();
        entity.set_position(position);
        Some((
            previous_position,
            entity.get_position(),
            entity.get_position_and_rotation_delta_packet(previous_position, on_ground),
            entity.get_head_look_packet(),
        ))
    }

    fn look_generic_entity_state_at_position(
        &mut self,
        entity_id: EntityId,
        target: EntityPosition,
        on_ground: bool,
    ) -> Option<(EntityRotationPacket, EntityHeadLookPacket)> {
        let Some(Entity::Living(entity)) = self.entity_by_id_mut(entity_id) else {
            return None;
        };
        entity.look_at_position(target);
        Some((
            entity.get_rotation_packet(on_ground),
            entity.get_head_look_packet(),
        ))
    }

    fn living_entity_main_hand_animation(
        &self,
        entity_id: EntityId,
    ) -> Option<spinel_core::network::clientbound::play::entity_animation::EntityAnimationPacket>
    {
        let Some(Entity::Living(entity)) = self.entity_by_id(entity_id) else {
            return None;
        };
        Some(entity.swing_main_hand())
    }

    fn creature_main_hand_animation(
        &self,
        entity_id: EntityId,
    ) -> Option<spinel_core::network::clientbound::play::entity_animation::EntityAnimationPacket>
    {
        let Some(Entity::Creature(entity)) = self.entity_by_id(entity_id) else {
            return None;
        };
        Some(entity.swing_main_hand())
    }

    fn creature_can_attack_entity(&self, creature_id: EntityId, target_id: EntityId) -> bool {
        if creature_id == target_id {
            return false;
        }
        let source_is_creature =
            matches!(self.entity_by_id(creature_id), Some(Entity::Creature(_)));
        source_is_creature && self.entity_by_id(target_id).is_some()
    }

    pub(crate) fn move_generic_entities_for_player(
        &mut self,
        client: &mut Client,
    ) -> Result<usize> {
        let moved_entities = self
            .entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Generic(entity) if !entity.is_removed() => Some(entity),
                _ => None,
            })
            .map(|entity| {
                let previous_position = entity.get_position();
                entity.set_position(previous_position.get_offset(0.0, 1.0, 0.0));
                entity.set_velocity(Velocity(Vector3d {
                    x: 0.0,
                    y: 0.25,
                    z: 0.0,
                }));
                entity.teleport_packet().dispatch(client)?;
                entity
                    .get_velocity_packet()
                    .dispatch(client)
                    .map(|_| (entity.get_entity_id(), entity.get_position()))
            })
            .collect::<Result<Vec<_>>>()?;
        moved_entities.iter().for_each(|(entity_id, position)| {
            self.entity_tracker.move_entity(*entity_id, *position);
        });
        moved_entities
            .iter()
            .try_for_each(|(entity_id, _)| self.refresh_visibility_for_entity(*entity_id))?;
        Ok(moved_entities.len())
    }

    pub(crate) fn remove_generic_entities_for_player(
        &mut self,
        client: &mut Client,
    ) -> Result<usize> {
        let removed_entities = self
            .entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Generic(entity) if !entity.is_removed() => {
                    entity.remove();
                    Some((
                        entity.get_entity_id(),
                        entity.get_entity_type(),
                        entity.get_uuid(),
                    ))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let removed_entity_count = removed_entities.len();
        removed_entities
            .iter()
            .try_for_each(|(entity_id, entity_type, uuid)| {
                RemoveEntitiesPacket::new(vec![entity_id.get_value()]).dispatch(client)?;
                if *entity_type == EntityType::PLAYER {
                    PlayerInfoRemovePacket::new(*uuid).dispatch(client)?;
                }
                Ok::<(), std::io::Error>(())
            })?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.addr != client.addr && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|viewer_client| {
                removed_entities
                    .iter()
                    .try_for_each(|(entity_id, entity_type, uuid)| {
                        RemoveEntitiesPacket::new(vec![entity_id.get_value()])
                            .dispatch(viewer_client)?;
                        if *entity_type == EntityType::PLAYER {
                            PlayerInfoRemovePacket::new(*uuid).dispatch(viewer_client)?;
                        }
                        Ok::<(), std::io::Error>(())
                    })
            })?;
        removed_entities.iter().for_each(|(entity_id, _, _)| {
            self.entity_tracker.unregister(*entity_id);
        });
        self.entities.retain(|entity| match entity {
            Entity::Creature(entity) => !entity.is_removed(),
            Entity::ExperienceOrb(entity) => !entity.is_removed(),
            Entity::Generic(entity) => !entity.is_removed(),
            Entity::Item(entity) => !entity.is_removed(),
            Entity::Living(entity) => !entity.is_removed(),
            Entity::Player(_) => true,
            Entity::Projectile(entity) => !entity.is_removed(),
        });
        Ok(removed_entity_count)
    }

    fn apply_entity_movement(&mut self, movement: EntityMovement) {
        let entity_id = movement.get_entity_id();
        self.entity_tracker
            .move_entity(entity_id, movement.get_position());
        self.refresh_passenger_positions(entity_id);
        let (movement_packet, head_look_packet) = movement.into_packets();
        let Some(packet) = movement_packet else {
            return;
        };
        match packet {
            EntityMovementPacket::Position(packet) => {
                let _ = self.send_packet_to_entity_viewers(entity_id, packet);
            }
            EntityMovementPacket::Teleport(packet) => {
                let _ = self.send_packet_to_entity_viewers(entity_id, packet);
            }
        }
        if let Some(packet) = head_look_packet {
            let _ = self.send_packet_to_entity_viewers(entity_id, packet);
        }
    }

    fn dispatch_entity_teleport_event(
        &mut self,
        entity_id: EntityId,
        teleport_position: EntityPosition,
        new_position: EntityPosition,
        relative_flags: TeleportFlags,
    ) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        EntityTeleportEvent::new(entity, teleport_position, new_position, relative_flags)
            .dispatch(server);
    }

    fn dispatch_entity_velocity_event(
        &mut self,
        entity_id: EntityId,
        velocity: Velocity,
    ) -> Option<Velocity> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(velocity);
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(velocity);
        };
        let mut event = EntityVelocityEvent::new(entity, velocity);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        if event.is_cancelled() {
            return None;
        }
        Some(event.get_velocity())
    }

    fn load_teleport_chunks(
        &mut self,
        previous_position: EntityPosition,
        destination: EntityPosition,
        chunks: Option<&[i64]>,
    ) -> Result<()> {
        let explicit_chunk_positions = chunks
            .unwrap_or_default()
            .iter()
            .copied()
            .map(ChunkPosition::from_index)
            .collect::<Vec<_>>();
        if explicit_chunk_positions.is_empty() {
            let previous_chunk = ChunkPosition::from(previous_position);
            let destination_chunk = ChunkPosition::from(destination);
            if previous_chunk == destination_chunk {
                return Ok(());
            }
            self.load_optional_chunk_result(destination_chunk)?;
            return Ok(());
        }
        self.load_optional_chunks(&explicit_chunk_positions)?;
        Ok(())
    }

    fn begin_teleport_chunk_loads(
        &mut self,
        previous_position: EntityPosition,
        destination: EntityPosition,
        chunks: Option<&[i64]>,
    ) -> Result<Vec<ChunkLoadTicket>> {
        let explicit_chunk_positions = chunks
            .unwrap_or_default()
            .iter()
            .copied()
            .map(ChunkPosition::from_index)
            .collect::<Vec<_>>();
        if !explicit_chunk_positions.is_empty() {
            return explicit_chunk_positions
                .into_iter()
                .map(|position| self.load_optional_chunk_future(position))
                .collect::<Result<Vec<_>>>()
                .map(|tickets| tickets.into_iter().flatten().collect());
        }
        let previous_chunk = ChunkPosition::from(previous_position);
        let destination_chunk = ChunkPosition::from(destination);
        if previous_chunk == destination_chunk {
            return Ok(Vec::new());
        }
        self.load_optional_chunk_future(destination_chunk)
            .map(|ticket| ticket.into_iter().collect())
    }
}



