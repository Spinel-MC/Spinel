impl World {
    pub fn update_entity_viewable_rule(
        &mut self,
        entity_id: EntityId,
        predicate: impl Fn(EntityId) -> bool + Send + Sync + 'static,
    ) -> Result<bool> {
        let Some(entity) = self.get_entity_mut(entity_id) else {
            return Ok(false);
        };
        entity.get_view_mut().update_viewable_rule(predicate);
        self.refresh_visibility_for_entity(entity_id)?;
        Ok(true)
    }

    pub fn clear_entity_viewable_rule(&mut self, entity_id: EntityId) -> Result<bool> {
        let Some(entity) = self.get_entity_mut(entity_id) else {
            return Ok(false);
        };
        entity.get_view_mut().clear_viewable_rule();
        self.refresh_visibility_for_entity(entity_id)?;
        Ok(true)
    }

    pub fn update_entity_viewer_rule(
        &mut self,
        entity_id: EntityId,
        predicate: impl Fn(EntityId) -> bool + Send + Sync + 'static,
    ) -> Result<bool> {
        let Some(entity) = self.get_entity_mut(entity_id) else {
            return Ok(false);
        };
        entity.get_view_mut().update_viewer_rule(predicate);
        self.refresh_visibility_for_entity(entity_id)?;
        Ok(true)
    }

    pub fn clear_entity_viewer_rule(&mut self, entity_id: EntityId) -> Result<bool> {
        let Some(entity) = self.get_entity_mut(entity_id) else {
            return Ok(false);
        };
        entity.get_view_mut().clear_viewer_rule();
        self.refresh_visibility_for_entity(entity_id)?;
        Ok(true)
    }

    pub fn add_entity_viewer(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<bool> {
        if viewed_entity_id == viewer_player_id {
            return Ok(false);
        }
        let Some(viewed_entity) = self.entity_by_id(viewed_entity_id) else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Entity must be in this world before adding viewers",
            ));
        };
        if viewed_entity.get_world() != Some(self.uuid) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Entity must be active in this world before adding viewers",
            ));
        }
        if !matches!(self.entity_by_id(viewer_player_id), Some(Entity::Player(_))) {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Viewer player was not found in this world",
            ));
        }
        let Some(viewed_entity) = self.entity_by_id_mut(viewed_entity_id) else {
            return Ok(false);
        };
        if !viewed_entity.get_view_mut().manual_add(viewer_player_id) {
            return Ok(false);
        }
        self.send_single_entity_spawn_to_player(viewed_entity_id, viewer_player_id)?;
        Ok(true)
    }

    pub fn remove_entity_viewer(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<bool> {
        if viewed_entity_id == viewer_player_id {
            return Ok(false);
        }
        let Some(viewed_entity) = self.entity_by_id(viewed_entity_id) else {
            return Ok(false);
        };
        if !viewed_entity
            .get_view()
            .get_manual_viewers()
            .contains(&viewer_player_id)
        {
            return Ok(false);
        }
        if !matches!(self.entity_by_id(viewer_player_id), Some(Entity::Player(_))) {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Viewer player was not found in this world",
            ));
        }
        self.hide_single_visibility_pair(viewed_entity_id, viewer_player_id)?;
        Ok(true)
    }

    pub fn viewable_chunk_players(&self, position: ChunkPosition) -> Vec<&Player> {
        self.entity_tracker
            .viewable(position, self.view_distance)
            .into_iter()
            .filter_map(|entity_id| match self.entity_by_id(entity_id) {
                Some(Entity::Player(player)) => Some(player),
                _ => None,
            })
            .collect()
    }

    fn broadcast_player_metadata(
        &mut self,
        changed_player_id: EntityId,
        metadata_entity_id: i32,
        metadata_entries: Vec<MetadataEntry>,
    ) -> Result<()> {
        let viewer_ids = self
            .entity_by_id(changed_player_id)
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
                SetEntityDataPacket::new(metadata_entity_id, metadata_entries.clone())
                    .dispatch(viewer_client)
            })
    }

    pub fn send_packet_to_player_viewers_and_self<P>(
        &mut self,
        player_id: EntityId,
        packet: P,
    ) -> Result<()>
    where
        P: PacketStruct + DataType,
    {
        let Some(mut viewer_ids) = self.entity_by_id(player_id).map(Entity::get_viewers) else {
            return Ok(());
        };
        viewer_ids.insert(player_id);
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.has_entered_world()
                        && viewer_ids.contains(&player.get_entity_id()) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| client.send_packet(P::get_id(), &payload))
    }

    fn send_packet_to_entity_viewers<P>(&mut self, entity_id: EntityId, packet: P) -> Result<()>
    where
        P: PacketStruct + DataType,
    {
        let viewer_ids = self
            .entity_by_id(entity_id)
            .map(Entity::get_viewers)
            .unwrap_or_default();
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if viewer_ids.contains(&player.get_entity_id()) => {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| client.send_packet(P::get_id(), &payload))
    }

    fn refresh_visibility_for_entity(&mut self, entity_id: EntityId) -> Result<()> {
        let Some(position) = self.entity_by_id(entity_id).map(Entity::get_position) else {
            return Ok(());
        };
        let mut player_ids = self.entity_tracker.nearby_entities_by_chunk_range(
            position,
            ENTITY_VIEW_DISTANCE,
            EntityTrackerTarget::Players,
        );
        player_ids.extend(
            self.entity_by_id(entity_id)
                .map(Entity::get_viewers)
                .unwrap_or_default(),
        );
        player_ids.sort_unstable();
        player_ids.dedup();
        player_ids
            .into_iter()
            .try_for_each(|player_id| self.refresh_visibility_pair(entity_id, player_id))?;
        if !matches!(self.entity_by_id(entity_id), Some(Entity::Player(_))) {
            return Ok(());
        }
        let mut viewed_entity_ids = self.entity_tracker.nearby_entities_by_chunk_range(
            position,
            ENTITY_VIEW_DISTANCE,
            EntityTrackerTarget::Entities,
        );
        viewed_entity_ids.extend(
            self.entity_by_id(entity_id)
                .map(|entity| entity.get_view().get_viewed_entities().iter().copied())
                .into_iter()
                .flatten(),
        );
        viewed_entity_ids.sort_unstable();
        viewed_entity_ids.dedup();
        viewed_entity_ids
            .into_iter()
            .try_for_each(|viewed_entity_id| {
                self.refresh_visibility_pair(viewed_entity_id, entity_id)
            })
    }

    fn refresh_visibility_pair(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        if viewed_entity_id == viewer_player_id {
            return Ok(());
        }
        let Some(viewed_entity_index) = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == viewed_entity_id)
        else {
            return Ok(());
        };
        let Some(viewer_player_index) = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == viewer_player_id)
        else {
            return Ok(());
        };
        let (viewed_entity, viewer_player) =
            distinct_entities_mut(&mut self.entities, viewed_entity_index, viewer_player_index);
        let Entity::Player(viewer_player) = viewer_player else {
            return Ok(());
        };
        let viewed_player_is_vanished =
            matches!(viewed_entity, Entity::Player(player) if player.is_vanished());
        let should_be_visible = automatic_visibility_pair_is_allowed(
            viewed_entity,
            viewer_player,
            viewed_player_is_vanished,
        );
        let is_automatically_visible = viewed_entity
            .get_view()
            .get_automatic_viewers()
            .contains(&viewer_player_id);
        if should_be_visible && !viewed_entity.get_view().is_viewer(viewer_player_id) {
            viewed_entity.get_view_mut().automatic_add(viewer_player_id);
            viewer_player
                .get_view_mut()
                .register_viewed_entity(viewed_entity_id);
            return self.send_entity_chain_spawn_to_player(viewed_entity_id, viewer_player_id);
        }
        if !should_be_visible && is_automatically_visible {
            return self.hide_visibility_pair(viewed_entity_id, viewer_player_id);
        }
        Ok(())
    }

    fn hide_entity_from_all_viewers(&mut self, entity_id: EntityId) -> Result<()> {
        let viewer_ids = self
            .entity_by_id(entity_id)
            .map(Entity::get_viewers)
            .unwrap_or_default();
        viewer_ids
            .into_iter()
            .try_for_each(|viewer_id| self.hide_visibility_pair(entity_id, viewer_id))?;
        let viewed_entity_ids = self
            .entity_by_id(entity_id)
            .map(|entity| entity.get_view().get_viewed_entities().clone())
            .unwrap_or_default();
        viewed_entity_ids
            .into_iter()
            .try_for_each(|viewed_entity_id| self.hide_visibility_pair(viewed_entity_id, entity_id))
    }

    fn hide_visibility_pair(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let mut pending_entity_ids = vec![viewed_entity_id];
        let mut hidden_entity_ids = HashSet::new();
        while let Some(entity_id) = pending_entity_ids.pop() {
            if !hidden_entity_ids.insert(entity_id) {
                continue;
            }
            let passenger_ids = self
                .entity_by_id(entity_id)
                .map(|entity| {
                    entity
                        .get_passengers()
                        .iter()
                        .rev()
                        .copied()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            self.hide_single_visibility_pair(entity_id, viewer_player_id)?;
            pending_entity_ids.extend(
                passenger_ids
                    .into_iter()
                    .filter(|passenger_id| *passenger_id != viewer_player_id),
            );
        }
        Ok(())
    }

    fn hide_single_visibility_pair(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        if self
            .entity_by_id(viewed_entity_id)
            .is_none_or(|entity| !entity.get_view().is_viewer(viewer_player_id))
        {
            return Ok(());
        }
        let leash_detach_packets = self
            .entity_by_id(viewed_entity_id)
            .map(|entity| {
                entity
                    .get_leashed_entities()
                    .iter()
                    .map(|leashed_entity_id| {
                        spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket {
                            attached_entity_id: leashed_entity_id.get_value(),
                            holding_entity_id: -1,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let Some(viewed_entity_index) = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == viewed_entity_id)
        else {
            return Ok(());
        };
        let Some(viewer_player_index) = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == viewer_player_id)
        else {
            return Ok(());
        };
        let (viewed_entity, viewer_player) =
            distinct_entities_mut(&mut self.entities, viewed_entity_index, viewer_player_index);
        let Entity::Player(viewer_player) = viewer_player else {
            return Ok(());
        };
        viewed_entity
            .get_view_mut()
            .automatic_remove(viewer_player_id);
        viewed_entity.get_view_mut().manual_remove(viewer_player_id);
        viewer_player
            .get_view_mut()
            .unregister_viewed_entity(viewed_entity_id);
        if let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.get_client_mut(),
                    _ => None,
                })
        {
            leash_detach_packets
                .into_iter()
                .try_for_each(|packet| packet.dispatch(client))?;
        }
        self.send_entity_remove_to_player(viewed_entity_id, viewer_player_id)
    }

    fn send_entity_chain_spawn_to_player(
        &mut self,
        root_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let visible_chain = self.collect_visible_passenger_chain(root_entity_id, viewer_player_id);
        let visible_entity_ids = visible_chain.iter().copied().collect::<HashSet<_>>();
        let viewer_dispatches = visible_chain
            .iter()
            .filter_map(|entity_id| {
                self.entity_viewer_snapshot(*entity_id).map(|snapshot| {
                    (
                        snapshot,
                        self.leash_packets_for_new_viewer(*entity_id, viewer_player_id),
                    )
                })
            })
            .collect::<Vec<_>>();
        let passenger_packets = visible_chain
            .iter()
            .rev()
            .filter_map(|entity_id| self.entity_by_id(*entity_id))
            .filter(|entity| {
                entity
                    .get_passengers()
                    .iter()
                    .any(|passenger_id| visible_entity_ids.contains(passenger_id))
            })
            .map(Entity::get_passenger_packet)
            .collect::<Vec<_>>();
        let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.get_client_mut(),
                    _ => None,
                })
        else {
            return Ok(());
        };
        viewer_dispatches
            .into_iter()
            .try_for_each(|(snapshot, leash_packets)| {
                snapshot.dispatch_with_leashes(client, leash_packets)
            })?;
        passenger_packets
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))
    }

    fn send_single_entity_spawn_to_player(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let Some(snapshot) = self.entity_viewer_snapshot(viewed_entity_id) else {
            return Ok(());
        };
        let leash_packets = self.leash_packets_for_new_viewer(viewed_entity_id, viewer_player_id);
        let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.get_client_mut(),
                    _ => None,
                })
        else {
            return Ok(());
        };
        snapshot.dispatch_with_leashes(client, leash_packets)
    }

    fn collect_visible_passenger_chain(
        &mut self,
        root_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Vec<EntityId> {
        let mut visible_chain = vec![root_entity_id];
        let mut collected_entity_ids = HashSet::from([root_entity_id]);
        let mut pending_entity_ids = self
            .entity_by_id(root_entity_id)
            .map(|entity| {
                entity
                    .get_passengers()
                    .iter()
                    .rev()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        while let Some(entity_id) = pending_entity_ids.pop() {
            if collected_entity_ids.contains(&entity_id)
                || !self.register_automatic_visibility_pair(entity_id, viewer_player_id)
            {
                continue;
            }
            collected_entity_ids.insert(entity_id);
            visible_chain.push(entity_id);
            if let Some(entity) = self.entity_by_id(entity_id) {
                pending_entity_ids.extend(entity.get_passengers().iter().rev().copied());
            }
        }
        visible_chain
    }

    fn register_automatic_visibility_pair(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> bool {
        if viewed_entity_id == viewer_player_id {
            return false;
        }
        let Some(viewed_entity_index) = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == viewed_entity_id)
        else {
            return false;
        };
        let Some(viewer_player_index) = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == viewer_player_id)
        else {
            return false;
        };
        let (viewed_entity, viewer_player) =
            distinct_entities_mut(&mut self.entities, viewed_entity_index, viewer_player_index);
        let Entity::Player(viewer_player) = viewer_player else {
            return false;
        };
        let viewed_player_is_vanished =
            matches!(viewed_entity, Entity::Player(player) if player.is_vanished());
        if viewed_entity.get_view().is_viewer(viewer_player_id)
            || !automatic_visibility_pair_is_allowed(
                viewed_entity,
                viewer_player,
                viewed_player_is_vanished,
            )
        {
            return false;
        }
        viewed_entity.get_view_mut().automatic_add(viewer_player_id);
        viewer_player
            .get_view_mut()
            .register_viewed_entity(viewed_entity_id);
        true
    }

    fn leash_packets_for_new_viewer(
        &self,
        entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Vec<spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket> {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return Vec::new();
        };
        let mut packets = Vec::new();
        if entity.get_leash_holder().is_some_and(|leash_holder_id| {
            leash_holder_id == viewer_player_id
                || self
                    .entity_by_id(leash_holder_id)
                    .is_some_and(|holder| holder.get_view().is_viewer(viewer_player_id))
        }) {
            packets.push(entity.get_attach_entity_packet());
        }
        packets.extend(
            entity
                .get_leashed_entities()
                .iter()
                .filter_map(|leashed_entity_id| self.entity_by_id(*leashed_entity_id))
                .filter(|leashed_entity| leashed_entity.get_view().is_viewer(viewer_player_id))
                .map(Entity::get_attach_entity_packet),
        );
        packets
    }

    fn entity_viewer_snapshot(&self, viewed_entity_id: EntityId) -> Option<EntityViewerSnapshot> {
        self.entity_by_id(viewed_entity_id)
            .map(EntityViewerSnapshot::from_entity)
    }

    fn send_entity_remove_to_player(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let Some((viewed_entity_type, viewed_entity_uuid)) = self
            .entity_by_id(viewed_entity_id)
            .map(|entity| (entity.get_entity_type(), entity.get_uuid()))
        else {
            return Ok(());
        };
        let Some(client) = self
            .entity_by_id_mut(viewer_player_id)
            .and_then(|entity| match entity {
                Entity::Player(player) => player.get_client_mut(),
                _ => None,
            })
        else {
            return Ok(());
        };
        if viewed_entity_type == EntityType::PLAYER {
            PlayerInfoRemovePacket::new(viewed_entity_uuid).dispatch(client)?;
        }
        RemoveEntitiesPacket::new(vec![viewed_entity_id.get_value()]).dispatch(client)
    }

    fn send_entity_switch_remove_to_player(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.get_client_mut(),
                    _ => None,
                })
        else {
            return Ok(());
        };
        RemoveEntitiesPacket::new(vec![viewed_entity_id.get_value()]).dispatch(client)
    }

    fn broadcast_player_equipment(
        &mut self,
        changed_player_id: EntityId,
        equipment_entity_id: i32,
        equipment_entries: Vec<
            spinel_core::network::clientbound::play::set_equipment::EntityEquipmentEntry,
        >,
    ) -> Result<()> {
        let viewer_ids = self
            .entity_by_id(changed_player_id)
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
                SetEquipmentPacket::new(equipment_entity_id, equipment_entries.clone())
                    .dispatch(viewer_client)
            })
    }

    pub(crate) fn synchronize_player_visibility(&mut self, client: &mut Client) -> Result<()> {
        let Some(joining_player_id) = self.player_by_addr(&client.addr).map(Player::get_entity_id)
        else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        self.refresh_visibility_for_entity(joining_player_id)
    }

    pub(crate) fn send_player_remove_to_viewers(
        &mut self,
        player_id: EntityId,
        player_uuid: Uuid,
    ) -> Result<()> {
        let viewer_ids = self
            .entity_by_id(player_id)
            .map(Entity::get_viewers)
            .unwrap_or_default();
        viewer_ids.into_iter().try_for_each(|viewer_id| {
            let Some(client) = self
                .entity_by_id_mut(viewer_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.get_client_mut(),
                    _ => None,
                })
            else {
                return Ok(());
            };
            PlayerInfoRemovePacket::new(player_uuid).dispatch(client)
        })
    }

    pub(crate) fn dispatch_player_info_update_to_online_players(
        &mut self,
        packet: PlayerInfoUpdatePacket,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() && player.is_online() => {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| packet.clone().dispatch(client))
    }

    pub(crate) fn dispatch_player_info_updates_to_players(
        &mut self,
        player_uuids: &[Uuid],
        packets: &[PlayerInfoUpdatePacket],
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.has_entered_world()
                        && player.is_online()
                        && player_uuids.contains(&player.get_uuid()) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| {
                packets
                    .iter()
                    .cloned()
                    .try_for_each(|packet| packet.dispatch(client))
            })
    }

    pub(crate) fn dispatch_player_info_remove_to_online_players(
        &mut self,
        player_uuid: Uuid,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() && player.is_online() => {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|client| PlayerInfoRemovePacket::new(player_uuid).dispatch(client))
    }

    fn schedule_entity_visibility_refresh(&mut self, entity_id: EntityId) {
        if self
            .pending_entity_visibility_refresh_keys
            .insert(entity_id)
        {
            self.pending_entity_visibility_refreshes
                .push_back(entity_id);
        }
    }

    pub(crate) fn process_pending_entity_visibility_refreshes(&mut self) -> Result<()> {
        let mut pending_entity_ids = VecDeque::new();
        std::mem::swap(
            &mut pending_entity_ids,
            &mut self.pending_entity_visibility_refreshes,
        );
        self.pending_entity_visibility_refresh_keys.clear();
        while let Some(entity_id) = pending_entity_ids.pop_front() {
            self.refresh_visibility_for_entity(entity_id)?;
        }
        Ok(())
    }
}

struct GenericEntityViewerSnapshot {
    player_info_packet: Option<PlayerInfoUpdatePacket>,
    spawn_packet: SpawnEntityPacket,
    velocity_packet: Option<EntityVelocityPacket>,
    metadata_packet: SetEntityDataPacket,
    equipment_packet: SetEquipmentPacket,
    head_look_packet: EntityHeadLookPacket,
    attributes_packet: Option<UpdateAttributesPacket>,
    effect_packets: Vec<EntityEffectPacket>,
}

impl GenericEntityViewerSnapshot {
    fn from_entity(entity: &GenericEntity) -> Self {
        Self {
            player_info_packet: (entity.get_entity_type() == EntityType::PLAYER).then(|| {
                PlayerInfoUpdatePacket::add_listed_player(
                    entity.get_uuid(),
                    format!("test_player_{}", entity.get_entity_id().get_value()),
                )
            }),
            spawn_packet: entity.spawn_packet(),
            velocity_packet: entity.has_velocity().then(|| entity.get_velocity_packet()),
            metadata_packet: entity.get_metadata_packet(),
            equipment_packet: entity.get_equipment_packet(),
            head_look_packet: entity.get_head_look_packet(),
            attributes_packet: entity
                .has_attributes()
                .then(|| entity.update_attributes_packet()),
            effect_packets: entity.get_effect_packets(),
        }
    }

    fn from_experience_orb(experience_orb: &ExperienceOrb) -> Self {
        let mut snapshot = Self::from_entity(experience_orb);
        snapshot.spawn_packet = experience_orb.spawn_packet();
        snapshot
    }

    fn from_item_entity(item_entity: &ItemEntity) -> Self {
        let mut snapshot = Self::from_entity(item_entity);
        snapshot.spawn_packet = item_entity.spawn_packet();
        snapshot
    }

    fn from_projectile(projectile: &crate::entity::ProjectileEntity) -> Self {
        let mut snapshot = Self::from_entity(projectile);
        snapshot.spawn_packet = projectile.spawn_packet();
        snapshot
    }

    fn dispatch_with_leashes(
        self,
        client: &mut Client,
        leash_packets: Vec<
            spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket,
        >,
    ) -> Result<()> {
        if let Some(player_info_packet) = self.player_info_packet {
            player_info_packet.dispatch(client)?;
        }
        self.spawn_packet.dispatch(client)?;
        if let Some(velocity_packet) = self.velocity_packet {
            velocity_packet.dispatch(client)?;
        }
        self.metadata_packet.dispatch(client)?;
        leash_packets
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))?;
        self.head_look_packet.dispatch(client)?;
        self.equipment_packet.dispatch(client)?;
        if let Some(attributes_packet) = self.attributes_packet {
            attributes_packet.dispatch(client)?;
        }
        self.effect_packets
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))
    }
}

enum EntityViewerSnapshot {
    Generic(GenericEntityViewerSnapshot),
    Player(PlayerViewerSnapshot),
}

impl EntityViewerSnapshot {
    fn from_entity(entity: &Entity) -> Self {
        match entity {
            Entity::Creature(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_entity(entity))
            }
            Entity::ExperienceOrb(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_experience_orb(entity))
            }
            Entity::Generic(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_entity(entity))
            }
            Entity::Item(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_item_entity(entity))
            }
            Entity::Player(player) => Self::Player(PlayerViewerSnapshot::from_player(player)),
            Entity::Projectile(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_projectile(entity))
            }
        }
    }

    fn dispatch_with_leashes(
        self,
        client: &mut Client,
        leash_packets: Vec<
            spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket,
        >,
    ) -> Result<()> {
        match self {
            Self::Generic(snapshot) => snapshot.dispatch_with_leashes(client, leash_packets),
            Self::Player(snapshot) => snapshot.dispatch_with_leashes(client, leash_packets),
        }
    }
}

fn chunk_position_for_entity_position(position: EntityPosition) -> ChunkPosition {
    ChunkPosition::new(
        (position.get_x().floor() as i32).div_euclid(16),
        (position.get_z().floor() as i32).div_euclid(16),
    )
}

fn entity_positions_are_within_view_distance(
    viewed_entity_position: EntityPosition,
    viewer_position: EntityPosition,
) -> bool {
    let viewed_entity_chunk = chunk_position_for_entity_position(viewed_entity_position);
    let viewer_chunk = chunk_position_for_entity_position(viewer_position);
    (viewed_entity_chunk.x - viewer_chunk.x).abs() <= ENTITY_VIEW_DISTANCE
        && (viewed_entity_chunk.z - viewer_chunk.z).abs() <= ENTITY_VIEW_DISTANCE
}

fn automatic_visibility_pair_is_allowed(
    viewed_entity: &Entity,
    viewer_player: &Player,
    viewed_player_is_vanished: bool,
) -> bool {
    viewer_player.has_entered_world()
        && !viewed_player_is_vanished
        && viewer_player.get_vehicle() != Some(viewed_entity.get_entity_id())
        && entity_positions_are_within_view_distance(
            viewed_entity.get_position(),
            viewer_player.get_position(),
        )
        && viewed_entity.get_view().is_auto_viewable()
        && viewer_player.get_view().is_auto_viewer()
        && viewed_entity
            .get_view()
            .viewable_rule_allows(viewer_player.get_entity_id())
        && viewer_player
            .get_view()
            .viewer_rule_allows(viewed_entity.get_entity_id())
}

