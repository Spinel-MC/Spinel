impl World {
    pub(crate) fn dispatch_add_entity_to_instance_event(&mut self, entity: &mut Entity) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return false;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        let entity = entity as *mut Entity;
        let mut event = AddEntityToInstanceEvent::new(world, entity);
        event.dispatch(server);
        event.is_cancelled()
    }

    fn dispatch_entity_spawn_event(&mut self, entity_id: EntityId) {
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
        let world = self as *mut World;
        EntitySpawnEvent::new(entity, world).dispatch(server);
    }

    fn dispatch_entity_despawn_event(&mut self, entity_id: EntityId) {
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
        EntityDespawnEvent::new(entity).dispatch(server);
    }

    fn dispatch_remove_entity_from_instance_event(&mut self, entity_id: EntityId) {
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
        let world = self as *mut World;
        RemoveEntityFromInstanceEvent::new(world, entity).dispatch(server);
    }

    pub(crate) fn add_entity(&mut self, mut entity: Entity) -> bool {
        if self.dispatch_add_entity_to_instance_event(&mut entity) {
            return false;
        }
        self.add_entity_after_instance_event(entity);
        true
    }

    pub(crate) fn add_entity_after_instance_event(&mut self, mut entity: Entity) {
        entity.assign_world(self.uuid);
        if let Entity::Creature(creature) = &mut entity {
            creature.set_event_dispatcher(self.event_dispatcher);
            creature.set_pathfinding_world(Arc::new(self.update_snapshot()));
        }
        self.entity_tracker.register(&entity);
        let entity_id = entity.get_entity_id();
        self.entities.push(entity);
        self.schedule_entity_visibility_refresh(entity_id);
        self.dispatch_entity_spawn_event(entity_id);
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) -> Option<Entity> {
        self.dispatch_entity_despawn_event(entity_id);
        self.remove_entity_from_instance(entity_id)
    }

    pub(crate) fn remove_entity_from_instance(&mut self, entity_id: EntityId) -> Option<Entity> {
        self.dispatch_remove_entity_from_instance_event(entity_id);
        self.detach_entity_passenger_relations(entity_id);
        self.detach_leashed_entities(entity_id);
        let _ = self.hide_entity_from_all_viewers(entity_id);
        let entity_index = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == entity_id)?;
        self.entity_tracker.unregister(entity_id);
        Some(self.entities.remove(entity_index))
    }

    pub(crate) fn remove_player_by_uuid(&mut self, player_uuid: Uuid) -> Option<Player> {
        let player_id = self.entities.iter().find_map(|entity| match entity {
            Entity::Player(player) if player.get_uuid() == player_uuid => {
                Some(entity.get_entity_id())
            }
            _ => None,
        })?;
        let Entity::Player(player) = self.remove_entity_from_instance(player_id)? else {
            return None;
        };
        Some(player)
    }

    pub fn entity_tracker(&self) -> &EntityTracker {
        &self.entity_tracker
    }

    #[cfg(test)]
    pub(crate) fn entity_tracker_mut(&mut self) -> &mut EntityTracker {
        &mut self.entity_tracker
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    pub fn get_entity(&self, entity_id: EntityId) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.get_entity_id() == entity_id)
    }

    pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<&mut Entity> {
        self.entities
            .iter_mut()
            .find(|entity| entity.get_entity_id() == entity_id)
    }

    pub(crate) fn entity_by_id(&self, entity_id: EntityId) -> Option<&Entity> {
        self.get_entity(entity_id)
    }

    pub(crate) fn entity_by_id_mut(&mut self, entity_id: EntityId) -> Option<&mut Entity> {
        self.get_entity_mut(entity_id)
    }

    pub fn entity_by_uuid(&self, entity_uuid: Uuid) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.get_uuid() == entity_uuid)
    }

    pub(crate) fn entity_by_uuid_mut(&mut self, entity_uuid: Uuid) -> Option<&mut Entity> {
        self.entities
            .iter_mut()
            .find(|entity| entity.get_uuid() == entity_uuid)
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.entities.iter().filter_map(|entity| match entity {
            Entity::Player(player) => Some(player),
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Living(_) => None,
            Entity::Projectile(_) => None,
        })
    }

    pub fn chunk_entities(&self, position: ChunkPosition) -> Vec<&Entity> {
        self.entity_tracker
            .chunk_entities(position, EntityTrackerTarget::Entities)
            .into_iter()
            .filter_map(|entity_id| self.entity_by_id(entity_id))
            .collect()
    }

    pub fn nearby_entities(&self, position: EntityPosition, range: f64) -> Vec<&Entity> {
        self.entity_tracker
            .nearby_entities(position, range, EntityTrackerTarget::Entities)
            .into_iter()
            .filter_map(|entity_id| self.entity_by_id(entity_id))
            .collect()
    }

    pub fn creatures(&self) -> Vec<&crate::entity::LivingEntity> {
        self.entities
            .iter()
            .filter_map(|entity| match entity {
                Entity::Living(entity) => Some(entity),
                _ => None,
            })
            .collect()
    }

    pub fn experience_orbs(&self) -> Vec<&ExperienceOrb> {
        self.entity_tracker
            .entities(EntityTrackerTarget::ExperienceOrbs)
            .into_iter()
            .filter_map(|entity_id| match self.entity_by_id(entity_id) {
                Some(Entity::ExperienceOrb(entity)) => Some(entity),
                _ => None,
            })
            .collect()
    }

    pub fn player_by_uuid(&self, player_uuid: Uuid) -> Option<&Player> {
        self.players()
            .find(|player| player.get_uuid() == player_uuid)
    }

    pub fn set_player_pose(&mut self, player_uuid: Uuid, pose: EntityPose) -> bool {
        let Some(player) = self.player_by_uuid(player_uuid) else {
            return false;
        };
        if !player_pose_fits_at(self, player.get_position(), pose) {
            return false;
        }
        let Some(player) = self.player_by_uuid_mut(player_uuid) else {
            return false;
        };
        player.set_pose(pose);
        true
    }

    pub fn player_by_uuid_mut(&mut self, player_uuid: Uuid) -> Option<&mut Player> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Living(_) => None,
            Entity::Player(player) if player.get_uuid() == player_uuid => Some(player),
            Entity::Player(_) => None,
            Entity::Projectile(_) => None,
        })
    }

    pub fn spawn_entity(
        &mut self,
        entity_type: EntityType,
        position: EntityPosition,
        nbt: Option<&NbtCompound>,
    ) -> Result<EntityId> {
        let mut entity = Entity::new(entity_type);
        entity.set_position(position);
        if let Some(nbt) = nbt {
            match &mut entity {
                Entity::Generic(generic_entity) => generic_entity.apply_summon_nbt(nbt),
                Entity::Living(living_entity) => living_entity.apply_summon_nbt(nbt),
                _ => {}
            }
        }
        let entity_id = entity.get_entity_id();
        if !self.add_entity(entity) {
            return Err(Error::new(ErrorKind::Interrupted, "Entity add cancelled."));
        }
        Ok(entity_id)
    }

    pub fn switch_entity_type(
        &mut self,
        entity_id: EntityId,
        entity_type: EntityType,
    ) -> Result<bool> {
        let Some(viewer_ids) = self.entity_by_id(entity_id).map(Entity::get_viewers) else {
            return Ok(false);
        };
        viewer_ids.iter().try_for_each(|viewer_id| {
            self.send_entity_switch_remove_to_player(entity_id, *viewer_id)
        })?;
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return Ok(false);
        };
        if !entity.switch_entity_type(entity_type) {
            return Ok(false);
        }
        viewer_ids.into_iter().try_for_each(|viewer_id| {
            self.send_single_entity_spawn_to_player(entity_id, viewer_id)
        })?;
        Ok(true)
    }

    fn broadcast_entity_status(&mut self, entity_id: i32, status: i8) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::get_client_mut)
            .try_for_each(|viewer_client| {
                EntityStatusPacket { entity_id, status }.dispatch(viewer_client)
            })
    }
}
