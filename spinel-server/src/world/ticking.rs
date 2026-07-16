impl World {
    fn dispatch_world_tick_event(&mut self) {
        self.dispatch_world_event_node("WorldTickEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        WorldTickEvent::new(world, self.world_age).dispatch(server);
    }

    fn dispatch_world_tick_end_event(&mut self) {
        self.dispatch_world_event_node("WorldTickEndEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        WorldTickEndEvent::new(world, self.world_age).dispatch(server);
    }

    fn dispatch_world_event_node(&mut self, event_name: &'static str) {
        let mut event_node = std::mem::take(&mut self.event_node);
        event_node.dispatch(event_name, self);
        self.event_node = event_node;
    }

    pub fn tick(&mut self) {
        let registries = Registries::new_vanilla();
        self.tick_with_registries(&registries);
    }

    pub(crate) fn tick_with_registries(&mut self, registries: &Registries) {
        self.process_next_tick_scheduler();
        self.tick_time();
        self.tick_weather();
        self.dispatch_world_tick_event();
        let world_snapshot = self.update_snapshot();
        let mut player_addresses = Vec::new();
        let mut entity_touches = Vec::new();
        let mut moved_entities = Vec::new();
        let mut entity_movements = Vec::new();
        let mut navigation_velocity_packets = Vec::new();
        let mut mergeable_item_entity_ids = Vec::new();
        let mut experience_orb_ids = Vec::new();
        let mut projectile_paths = Vec::new();
        let mut expired_fire_entities = Vec::new();
        let mut expired_effects = Vec::new();
        let mut creature_ai_actions = Vec::new();
        let event_dispatcher = self.event_dispatcher;
        let item_use_completions = self
            .entities
            .iter_mut()
            .filter_map(|entity| {
                let entity_ptr = entity as *mut Entity;
                match entity {
                    Entity::Creature(entity) => {
                        if entity.get_fire_ticks() == 1 {
                            expired_fire_entities.push(entity.get_entity_id());
                        }
                        let previous_position = entity.get_position();
                        entity.tick_before_movement(&world_snapshot, self.world_age as u64);
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        let velocity_before_navigation = entity.get_velocity();
                        entity.tick_after_movement(&world_snapshot, self.world_age as u64);
                        if entity.get_velocity() != velocity_before_navigation {
                            navigation_velocity_packets
                                .push((entity.get_entity_id(), entity.get_velocity_packet()));
                        }
                        if let Some(movement) = entity.position_movement_after_tick() {
                            entity_movements.push(movement);
                        }
                        creature_ai_actions.extend(entity.take_ai_actions());
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        entity.tick_after_event();
                        if entity.get_position() != previous_position {
                            moved_entities.push((entity.get_entity_id(), entity.get_position()));
                        }
                        expired_effects.extend(
                            entity
                                .take_expired_effects()
                                .into_iter()
                                .map(|effect| (entity.get_entity_id(), effect)),
                        );
                        entity_touches.push((entity.get_entity_id(), entity.get_position()));
                        None
                    }
                    Entity::ExperienceOrb(entity) => {
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick();
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        experience_orb_ids.push(entity.get_entity_id());
                        entity_touches.push((entity.get_entity_id(), entity.get_position()));
                        None
                    }
                    Entity::Generic(entity) => {
                        if entity.get_fire_ticks() == 1 {
                            expired_fire_entities.push(entity.get_entity_id());
                        }
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick_before_event();
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        entity.tick_after_event();
                        expired_effects.extend(
                            entity
                                .take_expired_effects()
                                .into_iter()
                                .map(|effect| (entity.get_entity_id(), effect)),
                        );
                        entity_touches.push((entity.get_entity_id(), entity.get_position()));
                        None
                    }
                    Entity::Item(entity) => {
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick();
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        if entity.should_check_merge(self.world_age as u64) {
                            mergeable_item_entity_ids.push(entity.get_entity_id());
                        }
                        entity_touches.push((entity.get_entity_id(), entity.get_position()));
                        None
                    }
                    Entity::Projectile(entity) => {
                        if entity.get_fire_ticks() == 1 {
                            expired_fire_entities.push(entity.get_entity_id());
                        }
                        let position_before_tick = entity.get_position();
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick_before_event();
                        projectile_paths.push((
                            entity.get_entity_id(),
                            position_before_tick,
                            entity.get_position(),
                        ));
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        entity.tick_after_event();
                        expired_effects.extend(
                            entity
                                .take_expired_effects()
                                .into_iter()
                                .map(|effect| (entity.get_entity_id(), effect)),
                        );
                        entity_touches.push((entity.get_entity_id(), entity.get_position()));
                        None
                    }
                    Entity::Player(player) => {
                        player.movement_tick(&world_snapshot);
                        let item_use_completion = player.tick();
                        dispatch_player_tick_event(player);
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        if player.get_fire_ticks() == 1 {
                            expired_fire_entities.push(player.get_entity_id());
                        }
                        player.tick_fire_ticks();
                        expired_effects.extend(
                            player
                                .tick_living_state()
                                .into_iter()
                                .map(|effect| (player.get_entity_id(), effect)),
                        );
                        entity_touches.push((player.get_entity_id(), player.get_position()));
                        if player.has_entered_world() && player.is_online() {
                            player_addresses.push(player.addr);
                        }
                        dispatch_player_tick_end_event(player);
                        item_use_completion
                    }
                }
            })
            .collect::<Vec<_>>();
        creature_ai_actions
            .into_iter()
            .for_each(|action| self.apply_creature_ai_action(action));
        experience_orb_ids
            .into_iter()
            .for_each(|experience_orb_id| self.tick_experience_orb(experience_orb_id));
        mergeable_item_entity_ids
            .into_iter()
            .for_each(|item_entity_id| self.merge_item_entity(item_entity_id));
        let synchronization_packets = self
            .entities
            .iter_mut()
            .filter_map(|entity| {
                entity
                    .get_scheduled_position_sync_packet()
                    .map(|position_packet| {
                        (
                            entity.get_entity_id(),
                            position_packet,
                            entity.get_velocity_packet(),
                        )
                    })
            })
            .collect::<Vec<_>>();
        let metadata_packets = self
            .entities
            .iter_mut()
            .filter_map(|entity| {
                entity
                    .get_dirty_metadata_packet()
                    .map(|packet| (entity.get_entity_id(), packet))
            })
            .collect::<Vec<_>>();
        moved_entities
            .into_iter()
            .for_each(|(entity_id, position)| {
                self.entity_tracker.move_entity(entity_id, position);
                self.refresh_passenger_positions(entity_id);
            });
        entity_movements
            .into_iter()
            .for_each(|movement| self.apply_entity_movement(movement));
        navigation_velocity_packets
            .into_iter()
            .for_each(|(entity_id, velocity_packet)| {
                let _ = self.send_packet_to_player_viewers_and_self(entity_id, velocity_packet);
            });
        metadata_packets
            .into_iter()
            .for_each(|(entity_id, packet)| {
                let _ = self.send_packet_to_player_viewers_and_self(entity_id, packet);
            });
        synchronization_packets.into_iter().for_each(
            |(entity_id, position_packet, velocity_packet)| {
                let _ = self.send_packet_to_entity_viewers(entity_id, position_packet);
                let _ = self.send_packet_to_entity_viewers(entity_id, velocity_packet);
            },
        );
        projectile_paths.into_iter().for_each(
            |(projectile_id, position_before_tick, position_after_tick)| {
                self.process_projectile_collision(
                    projectile_id,
                    position_before_tick,
                    position_after_tick,
                );
            },
        );
        self.dispatch_expired_fire_events(expired_fire_entities);
        expired_effects.into_iter().for_each(|(entity_id, effect)| {
            let _ = self.dispatch_entity_effect_removal(entity_id, effect);
        });
        self.process_living_item_pickups();
        self.process_player_experience_pickups();
        entity_touches
            .into_iter()
            .for_each(|(entity_id, position)| self.touch_entity_blocks(entity_id, position));
        self.dispatch_chunk_loader_failures();
        let _ = self.process_completed_chunk_loads();

        let _ = self.process_pending_entity_visibility_refreshes();
        self.tick_chunks(self.world_age as u64);
        player_addresses.into_iter().for_each(|address| {
            let _ = self.send_pending_chunks_for_player_address(address, registries);
        });
        item_use_completions.into_iter().for_each(|completion| {
            let _ = self.finish_player_item_use(completion);
        });
        self.process_tick_end_scheduler();
        self.dispatch_world_tick_end_event();
        self.currently_changing_blocks.clear();
    }
}

fn dispatch_player_tick_event(player: &mut Player) {
    let player_ptr = player as *mut Player;
    let Some(client) = player.get_client_mut() else {
        return;
    };
    let Some(server_ptr) = client.server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    PlayerTickEvent::new(player_ptr).dispatch(server, client);
}

fn dispatch_entity_tick_event(entity: *mut Entity, server_ptr: Option<usize>) {
    let Some(server_ptr) = server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    EntityTickEvent::new(entity).dispatch(server);
}

fn dispatch_player_tick_end_event(player: &mut Player) {
    let player_ptr = player as *mut Player;
    let Some(client) = player.get_client_mut() else {
        return;
    };
    let Some(server_ptr) = client.server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    PlayerTickEndEvent::new(player_ptr).dispatch(server, client);
}
