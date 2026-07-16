impl World {
    pub fn teleport_player(
        &mut self,
        player_uuid: Uuid,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> Result<Option<crate::entity::EntityTeleport>> {
        self.teleport_player_with_velocity(
            player_uuid,
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

    pub(crate) fn respawn_player(&mut self, client: &mut Client) -> Result<bool> {
        self.use_client_event_dispatcher(client);
        let world_view_distance = self.view_distance;
        let (player_uuid, player_id, respawn_position, respawn_chunks) = {
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            let Some(respawn_position) = player.respawn()? else {
                return Ok(false);
            };
            let respawn_chunks = player.reset_chunks_after_respawn(respawn_position, world_view_distance);
            (
                player.get_uuid(),
                player.get_entity_id(),
                respawn_position,
                respawn_chunks,
            )
        };
        self.remove_player_from_delivered_chunk_viewers(player_id);
        self.schedule_player_chunk_loads(client.addr, &respawn_chunks)?;
        self.teleport_player(
            player_uuid,
            respawn_position,
            None,
            TeleportFlags::absolute(),
            true,
        )?;
        Ok(true)
    }

    pub fn teleport_player_with_velocity(
        &mut self,
        player_uuid: Uuid,
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> Result<Option<crate::entity::EntityTeleport>> {
        let Some(player) = self.player_by_uuid(player_uuid) else {
            return Ok(None);
        };
        let entity_id = player.get_entity_id();
        let previous_position = player.get_position();
        let destination = player.teleport_destination(position, flags);
        let chunk_transition = player.get_chunk_transition(
            destination.get_x(),
            destination.get_y(),
            destination.get_z(),
            self.view_distance,
        );
        self.dispatch_entity_teleport_event(entity_id, position, destination, flags);
        self.load_teleport_chunks(previous_position, destination, chunks.as_deref())?;
        let world_view_distance = self.view_distance;
        let teleport = {
            let player = self.player_by_uuid_mut(player_uuid).ok_or_else(|| {
                Error::new(ErrorKind::NotFound, "Player was removed before teleport")
            })?;
            player.refresh_chunks_after_teleport(chunk_transition, world_view_distance)?;
            player.teleport_with_velocity_chunks_and_flags(
                position,
                velocity,
                chunks,
                flags,
                should_confirm,
            )?
        };
        let entity_id = self
            .player_by_uuid(player_uuid)
            .map(Player::get_entity_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Player was removed after teleport"))?;
        self.entity_tracker.move_entity(entity_id, destination);
        self.refresh_passenger_positions(entity_id);
        self.schedule_entity_visibility_refresh(entity_id);
        let synchronization_packet = self
            .entity_by_id_mut(entity_id)
            .map(Entity::synchronize_position_packet)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Player was removed after teleport"))?;
        self.send_packet_to_entity_viewers(entity_id, synchronization_packet)?;
        Ok(Some(teleport))
    }

    pub(crate) fn enter_player(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        registries: &Registries,
    ) -> Result<()> {
        self.use_client_event_dispatcher(client);
        let chunks = match self.player_by_addr(&client.addr) {
            Some(player) => player.spawn_chunks(self.view_distance),
            None => Vec::new(),
        };
        self.load_initial_player_chunks(client.addr, &chunks)?;
        self.finish_player_entry(client, ticks_per_second, registries, chunks)
    }

    fn finish_player_entry(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        registries: &Registries,
        chunks: Vec<PlayerChunk>,
    ) -> Result<()> {
        let dimension_type_id = registries
            .dynamic_registry_id(
                &spinel_registry::DIMENSION_TYPE_REGISTRY,
                self.dimension_type.key(),
            )
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Dimension type {} is not registered",
                        self.dimension_type.key()
                    ),
                )
            })?;
        let time_packet = self.time_packet();
        let weather = self.weather;
        let world_name = self.name.clone();
        let world_uuid = self.uuid;
        let world_view_distance = self.view_distance;
        let world_border_packet = self
            .world_border
            .initialize_packet(self.world_border.diameter(), 0);
        let boss_bar_packets = self
            .boss_bars
            .iter()
            .map(BossBar::add_packet)
            .collect::<Vec<_>>();
        let (player, first_spawn, player_id, player_position) = {
            let dimension_type = self.dimension_type.clone();
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            player.assign_world(world_uuid);
            player.set_dimension_type(dimension_type);
            let first_spawn = !player.has_entered_world();
            player.unsafe_init_with_chunk_positions(
                client,
                ticks_per_second,
                dimension_type_id,
                world_name.clone(),
                world_view_distance,
                chunks,
                world_border_packet,
                time_packet,
                weather,
            )?;
            boss_bar_packets
                .into_iter()
                .try_for_each(|packet| packet.dispatch(client))?;
            (
                player as *mut Player,
                first_spawn,
                player.get_entity_id(),
                player.get_position(),
            )
        };
        self.entity_tracker.move_entity(player_id, player_position);
        self.refresh_visibility_for_entity(player_id)?;
        self.send_pending_chunks_for_client(client, registries)?;
        dispatch_player_spawn_event(player, self as *mut World, first_spawn, client);
        self.synchronize_player_visibility(client)
    }

    pub(crate) fn move_player(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        _registries: &Registries,
    ) -> Result<()> {
        self.use_client_event_dispatcher(client);
        let world_view_distance = self.view_distance;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        if !player.has_entered_world() {
            return Ok(());
        }
        let previous_position = player.get_position();
        if player_coordinate_is_too_large(x)
            || player_coordinate_is_too_large(y)
            || player_coordinate_is_too_large(z)
        {
            return player.kick(Component::text("You moved too far away!"));
        }
        if previous_position.get_x() == x
            && previous_position.get_y() == y
            && previous_position.get_z() == z
        {
            return Ok(());
        }
        if player.has_pending_teleport_confirmation() {
            return Ok(());
        }
        let packet_position = EntityPosition::new(
            x,
            y,
            z,
            previous_position.get_yaw(),
            previous_position.get_pitch(),
        );
        let Some(event_position) =
            self.process_player_move_event(client, previous_position, packet_position, on_ground)?
        else {
            return Ok(());
        };
        let pending_transition = self.player_by_addr(&client.addr).and_then(|player| {
            player.get_chunk_transition(
                event_position.get_x(),
                event_position.get_y(),
                event_position.get_z(),
                self.view_distance,
            )
        });
        if self.movement_enters_unloaded_chunk(pending_transition.as_ref()) {
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            return player.synchronize_position_after_teleport(
                previous_position,
                Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                TeleportFlags::absolute(),
                true,
            );
        }
        let transition = self
            .player_by_addr_mut(&client.addr)
            .and_then(|player| player.accept_chunk_transition(pending_transition));
        let chunks = match transition.as_ref() {
            Some(transition) => transition.arriving.clone(),
            None => Vec::new(),
        };
        self.schedule_player_chunk_loads(client.addr, &chunks)?;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let moving_player_id = player.get_entity_id();
        player.move_to_loaded_chunks(
            client,
            event_position.get_x(),
            event_position.get_y(),
            event_position.get_z(),
            on_ground,
            transition,
            chunks,
            world_view_distance,
        )?;
        let current_position = player.get_position();
        self.entity_tracker
            .move_entity(moving_player_id, current_position);
        self.refresh_visibility_for_entity(moving_player_id)?;
        self.broadcast_player_movement(
            moving_player_id,
            previous_position,
            current_position,
            on_ground,
        )
    }

    pub(crate) fn move_player_with_view(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        _registries: &Registries,
    ) -> Result<()> {
        self.use_client_event_dispatcher(client);
        let world_view_distance = self.view_distance;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        if !player.has_entered_world() {
            return Ok(());
        }
        let previous_position = player.get_position();
        if player_coordinate_is_too_large(x)
            || player_coordinate_is_too_large(y)
            || player_coordinate_is_too_large(z)
        {
            return player.kick(Component::text("You moved too far away!"));
        }
        let packet_position = EntityPosition::new(x, y, z, yaw, pitch);
        if previous_position == packet_position {
            return Ok(());
        }
        if player.has_pending_teleport_confirmation() {
            return Ok(());
        }
        let Some(event_position) =
            self.process_player_move_event(client, previous_position, packet_position, on_ground)?
        else {
            return Ok(());
        };
        let pending_transition = self.player_by_addr(&client.addr).and_then(|player| {
            player.get_chunk_transition(
                event_position.get_x(),
                event_position.get_y(),
                event_position.get_z(),
                self.view_distance,
            )
        });
        if self.movement_enters_unloaded_chunk(pending_transition.as_ref()) {
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            return player.synchronize_position_after_teleport(
                previous_position,
                Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                TeleportFlags::absolute(),
                true,
            );
        }
        let transition = self
            .player_by_addr_mut(&client.addr)
            .and_then(|player| player.accept_chunk_transition(pending_transition));
        let chunks = match transition.as_ref() {
            Some(transition) => transition.arriving.clone(),
            None => Vec::new(),
        };
        self.schedule_player_chunk_loads(client.addr, &chunks)?;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let moving_player_id = player.get_entity_id();
        player.move_to_with_view_loaded_chunks(
            client,
            event_position.get_x(),
            event_position.get_y(),
            event_position.get_z(),
            event_position.get_yaw(),
            event_position.get_pitch(),
            on_ground,
            transition,
            chunks,
            world_view_distance,
        )?;
        let current_position = player.get_position();
        self.entity_tracker
            .move_entity(moving_player_id, current_position);
        self.refresh_visibility_for_entity(moving_player_id)?;
        self.broadcast_player_movement(
            moving_player_id,
            previous_position,
            current_position,
            on_ground,
        )
    }

    fn process_player_move_event(
        &mut self,
        client: &mut Client,
        current_position: EntityPosition,
        packet_position: EntityPosition,
        on_ground: bool,
    ) -> Result<Option<EntityPosition>> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_ptr = player as *mut Player;
        let Some(server_ptr) = client.server_ptr else {
            return Ok(Some(packet_position));
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let mut event = PlayerMoveEvent::new(player_ptr, packet_position, on_ground);
        event.dispatch(server, client);
        let player = unsafe { &mut *player_ptr };
        if player.get_position() != current_position {
            return Ok(None);
        }
        if event.is_cancelled() {
            player.synchronize_position_after_teleport(
                current_position,
                Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                TeleportFlags::absolute(),
                true,
            )?;
            return Ok(None);
        }
        let event_position = event.new_position();
        if packet_position == event_position {
            return Ok(Some(event_position));
        }
        if packet_position.get_x() == event_position.get_x()
            && packet_position.get_y() == event_position.get_y()
            && packet_position.get_z() == event_position.get_z()
        {
            player.set_position_and_view(event_position);
            player.set_on_ground(on_ground);
            return Ok(None);
        }
        player.synchronize_position_after_teleport(
            event_position,
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            TeleportFlags::absolute(),
            true,
        )?;
        player.set_position(event_position);
        Ok(None)
    }

    pub(crate) fn animate_player_hand(
        &mut self,
        client: &Client,
        hand: crate::entity::PlayerHand,
    ) -> Result<()> {
        let Some(player) = self.player_by_addr(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let animating_player_id = player.get_entity_id();
        let animation_packet = player.get_animation_packet(hand);
        let animation_entity_id = animation_packet.entity_id;
        let animation = animation_packet.animation;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.get_entity_id() != animating_player_id
                        && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|viewer_client| {
                spinel_core::network::clientbound::play::entity_animation::EntityAnimationPacket {
                    entity_id: animation_entity_id,
                    animation,
                }
                .dispatch(viewer_client)
            })
    }

    pub(crate) fn refresh_player_input(
        &mut self,
        client: &Client,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        jump: bool,
        shift: bool,
        sprint: bool,
    ) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.get_entity_id();
        if !player.refresh_input(forward, backward, left, right, jump, shift, sprint) {
            return Ok(());
        }
        let Some(metadata_packet) = player.get_dirty_metadata_packet() else {
            return Ok(());
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)
    }

    pub(crate) fn set_player_sprinting(&mut self, client: &Client, sprinting: bool) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.get_entity_id();
        if !player.set_sprinting(sprinting) {
            return Ok(());
        }
        let Some(metadata_packet) = player.get_dirty_metadata_packet() else {
            return Ok(());
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)
    }

    pub(crate) fn refresh_player_settings(
        &mut self,
        client: &mut Client,
        settings: ClientInformation,
    ) -> Result<()> {
        let world_view_distance = self.view_distance;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let current_center = player.chunks_loaded_by_client;
        let previous_view_distance = player.effective_view_distance(world_view_distance);
        player.refresh_settings(settings);
        let next_view_distance = player.effective_view_distance(world_view_distance);
        let view_distance_changed = previous_view_distance != next_view_distance;
        let player_entity_id = player.get_entity_id();
        let metadata_packet = player.get_dirty_metadata_packet();
        if let Some(metadata_packet) = metadata_packet {
            let metadata_entity_id = metadata_packet.entity_id;
            let metadata_entries = metadata_packet.entries.0;
            SetEntityDataPacket::new(metadata_entity_id, metadata_entries.clone()).dispatch(client)?;
            self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)?;
        }
        if !view_distance_changed {
            return Ok(());
        }
        let previous_chunks = current_center
            .surrounding(previous_view_distance)
            .into_iter()
            .collect::<HashSet<_>>();
        let next_chunks = current_center
            .surrounding(next_view_distance)
            .into_iter()
            .collect::<HashSet<_>>();
        let arriving = next_chunks
            .difference(&previous_chunks)
            .copied()
            .collect::<Vec<_>>();
        let departing = previous_chunks
            .difference(&next_chunks)
            .copied()
            .collect::<Vec<_>>();
        self.cancel_player_chunk_loads(client.addr, &departing);
        self.schedule_player_chunk_loads(client.addr, &arriving)?;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        player.update_chunks_after_view_distance_change(client, arriving, departing)
    }

    pub(crate) fn start_player_flying_with_elytra(&mut self, client: &Client) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.get_entity_id();
        if !player.set_flying_with_elytra(true) {
            return Ok(());
        }
        let Some(metadata_packet) = player.get_dirty_metadata_packet() else {
            return Ok(());
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)
    }

    pub(crate) fn set_player_held_slot(
        &mut self,
        client: &mut Client,
        held_slot: i32,
        server: *mut crate::server::MinecraftServer,
    ) -> Result<bool> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.get_entity_id();
        let server = unsafe { &mut *server };
        if !player.change_held_slot(held_slot, server, client) {
            return Ok(false);
        }
        let equipment_packet = player.get_visible_equipment_packet();
        let equipment_entity_id = equipment_packet.entity_id;
        let equipment_entries = equipment_packet.equipment.0;
        let metadata_packet = player.get_dirty_metadata_packet();
        self.broadcast_player_equipment(player_entity_id, equipment_entity_id, equipment_entries)?;
        if let Some(metadata_packet) = metadata_packet {
            self.broadcast_player_metadata(
                player_entity_id,
                metadata_packet.entity_id,
                metadata_packet.entries.0,
            )?;
        }
        Ok(true)
    }

    pub(crate) fn refresh_player_metadata(&mut self, client: &Client) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.get_entity_id();
        let Some(metadata_packet) = player.get_dirty_metadata_packet() else {
            return Ok(());
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)
    }

    pub(crate) fn refresh_player_visible_equipment(&mut self, client: &Client) -> Result<()> {
        let Some(player) = self.player_by_addr(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.get_entity_id();
        let equipment_packet = player.get_visible_equipment_packet();
        let equipment_entity_id = equipment_packet.entity_id;
        let equipment_entries = equipment_packet.equipment.0;
        self.broadcast_player_equipment(player_entity_id, equipment_entity_id, equipment_entries)
    }

    pub(crate) fn look_player(
        &mut self,
        client: &Client,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let previous_position = player.get_position();
        player.look(yaw, pitch);
        player.set_on_ground(on_ground);
        let current_position = player.get_position();
        let looking_player_id = player.get_entity_id();
        self.broadcast_player_movement(
            looking_player_id,
            previous_position,
            current_position,
            on_ground,
        )
    }

    pub(crate) fn refresh_player_status(
        &mut self,
        client: &mut Client,
        on_ground: bool,
    ) -> Result<()> {
        let (player, player_entity_id, metadata_packet, stopped_flying_with_elytra) = {
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            let stopped_flying_with_elytra = player.refresh_on_ground(on_ground);
            (
                player as *mut Player,
                player.get_entity_id(),
                player.get_dirty_metadata_packet(),
                stopped_flying_with_elytra,
            )
        };

        if stopped_flying_with_elytra {
            self.dispatch_player_stop_flying_with_elytra_event(client, player);
        }

        let Some(metadata_packet) = metadata_packet else {
            return Ok(());
        };

        self.broadcast_player_metadata(
            player_entity_id,
            metadata_packet.entity_id,
            metadata_packet.entries.0,
        )?;

        Ok(())
    }

    fn dispatch_player_stop_flying_with_elytra_event(
        &mut self,
        client: &mut Client,
        player: *mut Player,
    ) {
        let Some(server_ptr) = client.server_ptr else {
            return;
        };
        let mut event = PlayerStopFlyingWithElytraEvent::new(player);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server, client);
    }

    fn broadcast_player_movement(
        &mut self,
        moving_player_id: EntityId,
        previous_position: EntityPosition,
        current_position: EntityPosition,
        on_ground: bool,
    ) -> Result<()> {
        let moved_entity_id = moving_player_id.get_value();
        let moved_position = current_position.as_vector();
        let moved_yaw = current_position.get_yaw();
        let moved_pitch = current_position.get_pitch();
        let moved_delta_x =
            EntityPositionPacket::delta(current_position.get_x(), previous_position.get_x());
        let moved_delta_y =
            EntityPositionPacket::delta(current_position.get_y(), previous_position.get_y());
        let moved_delta_z =
            EntityPositionPacket::delta(current_position.get_z(), previous_position.get_z());
        let position_changed = current_position.get_x() != previous_position.get_x()
            || current_position.get_y() != previous_position.get_y()
            || current_position.get_z() != previous_position.get_z();
        if position_changed {
            self.refresh_passenger_positions(moving_player_id);
        }
        let view_changed = current_position.get_yaw() != previous_position.get_yaw()
            || current_position.get_pitch() != previous_position.get_pitch();
        let movement_requires_teleport =
            (current_position.get_x() - previous_position.get_x()).abs() > 8.0
                || (current_position.get_y() - previous_position.get_y()).abs() > 8.0
                || (current_position.get_z() - previous_position.get_z()).abs() > 8.0;
        let viewer_ids = self
            .entity_by_id(moving_player_id)
            .map(Entity::get_viewers)
            .unwrap_or_default();
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if viewer_ids.contains(&player.get_entity_id())
                        && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|viewer_client| {
                if movement_requires_teleport {
                    EntityTeleportPacket {
                        entity_id: moved_entity_id,
                        position: moved_position,
                        delta: Vector3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        yaw: moved_yaw,
                        pitch: moved_pitch,
                        flags: 0,
                        on_ground,
                    }
                    .dispatch(viewer_client)?;
                    return Ok(());
                }
                if position_changed && view_changed {
                    EntityPositionAndRotationPacket {
                        entity_id: moved_entity_id,
                        delta_x: moved_delta_x,
                        delta_y: moved_delta_y,
                        delta_z: moved_delta_z,
                        yaw: EntityAngle(moved_yaw),
                        pitch: EntityAngle(moved_pitch),
                        on_ground,
                    }
                    .dispatch(viewer_client)?;
                    return EntityHeadLookPacket {
                        entity_id: moved_entity_id,
                        head_yaw: EntityAngle(moved_yaw),
                    }
                    .dispatch(viewer_client);
                }
                if position_changed {
                    return EntityPositionAndRotationPacket {
                        entity_id: moved_entity_id,
                        delta_x: moved_delta_x,
                        delta_y: moved_delta_y,
                        delta_z: moved_delta_z,
                        yaw: EntityAngle(moved_yaw),
                        pitch: EntityAngle(moved_pitch),
                        on_ground,
                    }
                    .dispatch(viewer_client);
                }
                if !view_changed {
                    return Ok(());
                }
                EntityHeadLookPacket {
                    entity_id: moved_entity_id,
                    head_yaw: EntityAngle(moved_yaw),
                }
                .dispatch(viewer_client)?;
                EntityPositionAndRotationPacket {
                    entity_id: moved_entity_id,
                    delta_x: moved_delta_x,
                    delta_y: moved_delta_y,
                    delta_z: moved_delta_z,
                    yaw: EntityAngle(moved_yaw),
                    pitch: EntityAngle(moved_pitch),
                    on_ground,
                }
                .dispatch(viewer_client)
            })
    }

    pub fn set_player_skin(
        &mut self,
        player_id: EntityId,
        skin: Option<PlayerSkin>,
    ) -> Result<bool> {
        let Some(Entity::Player(player)) = self.entity_by_id_mut(player_id) else {
            return Ok(false);
        };
        player.apply_skin(skin);
        self.refresh_player_skin(player_id)?;
        Ok(true)
    }

    pub fn set_player_vanished(&mut self, player_id: EntityId, vanished: bool) -> Result<bool> {
        let Some(Entity::Player(player)) = self.entity_by_id_mut(player_id) else {
            return Ok(false);
        };
        if player.is_vanished() == vanished {
            return Ok(true);
        }
        player.set_vanished(vanished);
        self.refresh_visibility_for_entity(player_id)?;
        Ok(true)
    }

    pub fn refresh_player_skin(&mut self, player_id: EntityId) -> Result<()> {
        let Some(Entity::Player(player)) = self.entity_by_id(player_id) else {
            return Ok(());
        };
        let player_uuid = player.get_uuid();
        let viewer_ids = player.get_viewers();
        let snapshot = PlayerViewerSnapshot::from_player(player);
        viewer_ids.into_iter().try_for_each(|viewer_id| {
            self.send_player_skin_refresh_to_viewer(player_id, player_uuid, &snapshot, viewer_id)
        })
    }

    fn send_player_skin_refresh_to_viewer(
        &mut self,
        player_id: EntityId,
        player_uuid: Uuid,
        snapshot: &PlayerViewerSnapshot,
        viewer_id: EntityId,
    ) -> Result<()> {
        let Some(client) = self
            .entity_by_id_mut(viewer_id)
            .and_then(|entity| match entity {
                Entity::Player(player) => player.get_client_mut(),
                _ => None,
            })
        else {
            return Ok(());
        };
        PlayerInfoRemovePacket::new(player_uuid).dispatch(client)?;
        RemoveEntitiesPacket::new(vec![player_id.get_value()]).dispatch(client)?;
        snapshot.get_dispatch(client)
    }

    fn finish_player_item_use(
        &mut self,
        completion: crate::entity::player::PlayerItemUseCompletion,
    ) -> Result<()> {
        let _ = self.broadcast_entity_status(completion.entity_id, completion.status);
        let player = unsafe { &mut *completion.player };
        let Some(client) = player.get_client_mut().map(|client| client as *mut Client) else {
            return Ok(());
        };
        let client = unsafe { &mut *client };
        let Some(server_ptr) = client.server_ptr else {
            return self.refresh_player_metadata_by_entity_id(completion.entity_id);
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        player.finish_item_use(
            completion.hand,
            completion.item_stack,
            completion.duration,
            server,
            client,
        )?;
        self.refresh_player_metadata_by_entity_id(completion.entity_id)
    }

    fn refresh_player_metadata_by_entity_id(&mut self, entity_id: i32) -> Result<()> {
        let Some(player) = self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Player(player) if player.get_entity_id().get_value() == entity_id => {
                Some(player)
            }
            _ => None,
        }) else {
            return Ok(());
        };
        let changed_player_id = player.get_entity_id();
        let Some(metadata_packet) = player.get_dirty_metadata_packet() else {
            return Ok(());
        };
        self.broadcast_player_metadata(
            changed_player_id,
            metadata_packet.entity_id,
            metadata_packet.entries.0,
        )
    }

    pub(crate) fn remove_player_by_connection_address(&mut self, addr: &SocketAddr) -> Result<()> {
        let removed_player_ids = self
            .entities
            .iter()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.addr == *addr => Some(player.get_entity_id()),
                _ => None,
            })
            .collect::<Vec<_>>();
        removed_player_ids.into_iter().for_each(|player_id| {
            let _ = self.remove_entity_from_instance(player_id);
        });
        Ok(())
    }

    pub(crate) fn player_by_addr_mut(&mut self, addr: &SocketAddr) -> Option<&mut Player> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Player(player) if player.addr == *addr => Some(player),
            Entity::Player(_) => None,
            Entity::Projectile(_) => None,
        })
    }

    pub(crate) fn player_by_addr(&self, addr: &SocketAddr) -> Option<&Player> {
        self.entities.iter().find_map(|entity| match entity {
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Player(player) if player.addr == *addr => Some(player),
            Entity::Player(_) => None,
            Entity::Projectile(_) => None,
        })
    }

    pub(crate) fn player_pointer_by_addr(&mut self, addr: &SocketAddr) -> Option<*mut Player> {
        self.player_by_addr_mut(addr)
            .map(|player| player as *mut Player)
    }

    pub(crate) fn dispatch_player_spawn(
        &mut self,
        player_uuid: Uuid,
        first_spawn: bool,
        client: &mut Client,
    ) {
        let Some(player) = self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Player(player) if player.get_uuid() == player_uuid => Some(player),
            _ => None,
        }) else {
            return;
        };
        dispatch_player_spawn_event(
            player as *mut Player,
            self as *mut World,
            first_spawn,
            client,
        );
    }
}

fn player_coordinate_is_too_large(coordinate: f64) -> bool {
    coordinate.abs() > MAX_PLAYER_COORDINATE
}

fn dispatch_player_spawn_event(
    player: *mut Player,
    world: *mut World,
    first_spawn: bool,
    client: &mut Client,
) {
    let Some(server_ptr) = client.server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    PlayerSpawnEvent::new(player, world, first_spawn).dispatch(server, client);
}

