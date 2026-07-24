impl World {
    pub fn spawn_item_entity(
        &mut self,
        item_stack: spinel_registry::ItemStack,
        position: EntityPosition,
    ) -> Result<EntityId> {
        let mut item_entity = ItemEntity::new(item_stack);
        item_entity.set_position(position);
        item_entity.spawn();
        let entity_id = item_entity.get_entity_id();
        if !self.add_entity(Entity::Item(item_entity)) {
            return Err(Error::new(ErrorKind::Interrupted, "Entity add cancelled."));
        }
        Ok(entity_id)
    }

    pub fn spawn_experience_orb(
        &mut self,
        experience_count: i16,
        position: EntityPosition,
    ) -> Result<EntityId> {
        let mut experience_orb = ExperienceOrb::new(experience_count);
        experience_orb.set_position(position);
        let entity_id = experience_orb.get_entity_id();
        if !self.add_entity(Entity::ExperienceOrb(experience_orb)) {
            return Err(Error::new(ErrorKind::Interrupted, "Entity add cancelled."));
        }
        Ok(entity_id)
    }

    pub fn set_experience_orb_count(
        &mut self,
        entity_id: EntityId,
        experience_count: i16,
    ) -> Result<bool> {
        let viewer_ids = match self.entity_by_id(entity_id) {
            Some(Entity::ExperienceOrb(experience_orb)) => experience_orb.get_viewers(),
            _ => return Ok(false),
        };
        viewer_ids
            .iter()
            .try_for_each(|viewer_id| self.send_entity_remove_to_player(entity_id, *viewer_id))?;
        let Some(Entity::ExperienceOrb(experience_orb)) = self.entity_by_id_mut(entity_id) else {
            return Ok(false);
        };
        experience_orb.set_experience_count(experience_count);
        viewer_ids.into_iter().try_for_each(|viewer_id| {
            self.send_single_entity_spawn_to_player(entity_id, viewer_id)
        })?;
        Ok(true)
    }

    pub fn set_entity_equipment(
        &mut self,
        entity_id: EntityId,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) -> Result<bool> {
        let Some(item_stack) =
            self.dispatch_entity_equip_event(entity_id, equipment_slot, item_stack)
        else {
            return Ok(false);
        };
        let equipment_packet = SetEquipmentPacket::new(
            entity_id.get_value(),
            vec![EntityEquipmentEntry {
                slot: equipment_slot.get_entity_equipment_slot(),
                item: Slot::from_item_stack(&item_stack),
            }],
        );
        let Some((attributes_packet, should_send_to_self)) =
            self.apply_entity_equipment(entity_id, equipment_slot, item_stack)
        else {
            return Ok(false);
        };
        self.send_packet_to_entity_viewers(entity_id, equipment_packet)?;
        if should_send_to_self {
            self.send_packet_to_player_viewers_and_self(entity_id, attributes_packet)?;
        } else {
            self.send_packet_to_entity_viewers(entity_id, attributes_packet)?;
        }
        Ok(true)
    }

    pub fn add_entity_effect(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> Result<bool> {
        if !self.dispatch_entity_potion_add_event(entity_id, effect.clone()) {
            return Ok(false);
        }
        if self.entity_has_effect(entity_id, effect.get_effect_key()) {
            self.remove_entity_effect(entity_id, effect.get_effect_key())?;
        }
        let Some(packet) = self.apply_entity_effect(entity_id, effect) else {
            return Ok(false);
        };
        self.send_packet_to_player_viewers_and_self(entity_id, packet)?;
        Ok(true)
    }

    pub fn remove_entity_effect(
        &mut self,
        entity_id: EntityId,
        effect_key: &RegistryKey<MobEffect>,
    ) -> Result<bool> {
        let Some(effect) = self.entity_effect(entity_id, effect_key).cloned() else {
            return Ok(false);
        };
        self.dispatch_entity_effect_removal(entity_id, effect)?;
        Ok(true)
    }

    pub fn clear_entity_effects(&mut self, entity_id: EntityId) -> Result<usize> {
        let effect_keys = self
            .entity_effects(entity_id)
            .into_iter()
            .map(TimedPotionEffect::get_effect_key)
            .cloned()
            .collect::<Vec<_>>();
        effect_keys.iter().try_for_each(|effect_key| {
            self.remove_entity_effect(entity_id, effect_key).map(|_| ())
        })?;
        Ok(effect_keys.len())
    }

    pub fn set_entity_fire_ticks(&mut self, entity_id: EntityId, fire_ticks: i32) -> bool {
        let Some(current_fire_ticks) =
            self.entity_by_id(entity_id)
                .and_then(|entity| match entity {
                    Entity::Creature(entity) => Some(entity.get_fire_ticks()),
                    Entity::ExperienceOrb(_) => None,
                    Entity::Generic(_) => None,
                    Entity::Living(entity) => Some(entity.get_fire_ticks()),
                    Entity::Item(_) => None,
                    Entity::Player(player) => Some(player.get_fire_ticks()),
                    Entity::Projectile(_) => None,
                })
        else {
            return false;
        };
        let requested_fire_ticks = fire_ticks.max(0);
        if requested_fire_ticks > 0 {
            let Some(approved_fire_ticks) =
                self.dispatch_entity_set_fire_event(entity_id, requested_fire_ticks)
            else {
                return false;
            };
            return self.apply_entity_fire_ticks(entity_id, approved_fire_ticks);
        }
        if current_fire_ticks != 0 && self.dispatch_entity_fire_extinguish_event(entity_id, false) {
            return self.apply_entity_cancelled_fire_extinguish(entity_id, 0);
        }
        self.apply_entity_fire_ticks(entity_id, requested_fire_ticks)
    }

    pub fn damage_entity(
        &mut self,
        registries: &Registries,
        entity_id: EntityId,
        mut damage: Damage,
    ) -> Result<bool> {
        if !damage.resolve_type(registries) {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    "damage type {} is not registered",
                    damage.damage_type().key()
                ),
            ));
        }
        if self.entity_rejects_damage(entity_id, &damage) {
            return Ok(false);
        }
        if damage.get_sound().is_none() {
            damage.set_sound(Some(
                damage.get_default_sound(self.entity_is_player(entity_id)),
            ));
        }
        let Some(damage) = self.dispatch_entity_damage_event(entity_id, damage) else {
            return Ok(false);
        };
        if damage.should_animate() {
            let damage_type_id = damage.damage_type_id(registries).ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "damage type {} is not registered",
                        damage.damage_type().key()
                    ),
                )
            })?;
            self.send_packet_to_player_viewers_and_self(
                entity_id,
                DamageEventPacket {
                    target_entity_id: entity_id.get_value(),
                    damage_type_id,
                    source_entity_id: damage
                        .get_attacker()
                        .map(|attacker| attacker.get_value() + 1)
                        .unwrap_or(0),
                    source_direct_id: damage
                        .get_source()
                        .map(|source| source.get_value() + 1)
                        .unwrap_or(0),
                    source_position: damage.get_source_position(),
                },
            )?;
        }
        if let Some(sound) = damage.get_sound() {
            let sound_source_id = damage_sound_source_id(entity_id, self);
            self.play_entity_sound_except(None, sound, sound_source_id, entity_id, 1.0, 1.0, 0)?;
        }
        self.apply_entity_damage(entity_id, damage)?;
        Ok(true)
    }

    pub fn kill_entity(&mut self, entity_id: EntityId) -> Result<bool> {
        if self.entity_should_be_removed_when_killed(entity_id) {
            return Ok(self.remove_entity(entity_id).is_some());
        }
        if self.entity_is_dead(entity_id) {
            return Ok(false);
        }
        if self
            .entity_by_id(entity_id)
            .is_some_and(|entity| matches!(entity, Entity::Player(_)))
        {
            self.apply_player_death_before_living_death(entity_id)?;
        }
        self.send_packet_to_player_viewers_and_self(
            entity_id,
            EntityStatusPacket {
                entity_id: entity_id.get_value(),
                status: 3,
            },
        )?;
        self.apply_living_death_state(entity_id)?;
        self.dispatch_entity_death_event(entity_id);
        Ok(true)
    }

    fn dispatch_expired_fire_events(&mut self, entity_ids: Vec<EntityId>) {
        entity_ids.into_iter().for_each(|entity_id| {
            if self.dispatch_entity_fire_extinguish_event(entity_id, true) {
                self.apply_entity_cancelled_fire_extinguish(entity_id, 0);
            }
        });
    }

    fn process_living_item_pickups(&mut self) {
        let living_entity_ids = self
            .entities
            .iter()
            .filter_map(|entity| match entity {
                Entity::Living(entity) if entity.can_pickup_item() => Some(entity.get_entity_id()),
                Entity::Creature(entity) if entity.can_pickup_item() => Some(entity.get_entity_id()),
                Entity::Player(player) if player.can_pickup_item() => Some(player.get_entity_id()),
                _ => None,
            })
            .collect::<Vec<_>>();
        living_entity_ids
            .into_iter()
            .for_each(|entity_id| self.process_living_entity_item_pickups(entity_id));
    }

    fn tick_experience_orb(&mut self, experience_orb_id: EntityId) {
        let current_tick = self.world_age;
        let Some(Entity::ExperienceOrb(experience_orb)) = self.entity_by_id_mut(experience_orb_id)
        else {
            return;
        };
        experience_orb.apply_gravity();
        let experience_orb_position = experience_orb.get_position();
        let current_target = experience_orb.get_target();
        let target_refresh_tick =
            current_tick - 20 + i64::from(experience_orb.get_entity_id().get_value() % 100);
        let should_refresh_target =
            experience_orb.get_last_target_update_tick() < target_refresh_tick;

        let target_is_missing_or_distant = current_target.is_none_or(|target_id| {
            self.entity_by_id(target_id).is_none_or(|target| {
                target
                    .get_position()
                    .get_distance_squared(experience_orb_position)
                    > 64.0
            })
        });
        let refreshed_target = (should_refresh_target && target_is_missing_or_distant)
            .then(|| self.closest_player_to(experience_orb_position, 8.0))
            .flatten();

        if should_refresh_target {
            let Some(Entity::ExperienceOrb(experience_orb)) =
                self.entity_by_id_mut(experience_orb_id)
            else {
                return;
            };
            if target_is_missing_or_distant {
                experience_orb.set_target(refreshed_target);
            }
            experience_orb.set_last_target_update_tick(current_tick);
        }

        let target = self
            .entity_by_id(experience_orb_id)
            .and_then(|entity| match entity {
                Entity::ExperienceOrb(experience_orb) => experience_orb.get_target(),
                _ => None,
            })
            .and_then(|target_id| match self.entity_by_id(target_id) {
                Some(Entity::Player(player)) if player.get_game_mode() != GameMode::Spectator => {
                    Some((target_id, player.get_position(), player.get_eye_height()))
                }
                _ => None,
            });
        let Some(Entity::ExperienceOrb(experience_orb)) = self.entity_by_id_mut(experience_orb_id)
        else {
            return;
        };
        match target {
            Some((_, target_position, eye_height)) => {
                experience_orb.apply_attraction(target_position, eye_height);
            }
            None => experience_orb.set_target(None),
        }
        experience_orb.apply_drag();
    }

    fn closest_player_to(
        &self,
        position: EntityPosition,
        maximum_distance: f64,
    ) -> Option<EntityId> {
        let maximum_distance_squared = maximum_distance * maximum_distance;
        self.players()
            .map(|player| {
                (
                    player.get_entity_id(),
                    player.get_position().get_distance_squared(position),
                )
            })
            .filter(|(_, distance_squared)| *distance_squared <= maximum_distance_squared)
            .min_by(|(_, first_distance), (_, second_distance)| {
                first_distance.total_cmp(second_distance)
            })
            .map(|(player_id, _)| player_id)
    }

    fn process_player_experience_pickups(&mut self) {
        let current_tick = self.world_age;
        let player_ids = self
            .entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.get_experience_pickup_is_ready(current_tick) => {
                    player.refresh_experience_pickup_cooldown(current_tick);
                    Some(player.get_entity_id())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        player_ids.into_iter().for_each(|player_id| {
            self.process_player_experience_pickup(player_id);
        });
    }

    fn process_player_experience_pickup(&mut self, player_id: EntityId) {
        let Some((player_position, expanded_bounding_box)) =
            self.entity_by_id(player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => {
                        Some((player.get_position(), player.get_expanded_bounding_box()))
                    }
                    _ => None,
                })
        else {
            return;
        };
        let experience_orb_ids = self.entity_tracker.nearby_entities(
            player_position,
            expanded_bounding_box.get_width(),
            EntityTrackerTarget::ExperienceOrbs,
        );
        experience_orb_ids
            .into_iter()
            .for_each(|experience_orb_id| {
                let Some(experience_count) =
                    self.entity_by_id(experience_orb_id)
                        .and_then(|entity| match entity {
                            Entity::ExperienceOrb(experience_orb)
                                if experience_orb.get_intersects_box_at(
                                    player_position.as_vector(),
                                    expanded_bounding_box,
                                ) =>
                            {
                                Some(experience_orb.get_experience_count())
                            }
                            _ => None,
                        })
                else {
                    return;
                };
                if self.dispatch_pickup_experience_event(
                    player_id,
                    experience_orb_id,
                    experience_count,
                ) {
                    return;
                }
                self.remove_entity(experience_orb_id);
            });
    }

    fn dispatch_pickup_experience_event(
        &mut self,
        player_id: EntityId,
        experience_orb_id: EntityId,
        experience_count: i16,
    ) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return false;
        };
        let mut event = PickupExperienceEvent::new(player_id, experience_orb_id, experience_count);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.is_cancelled()
    }

    fn process_living_entity_item_pickups(&mut self, living_entity_id: EntityId) {
        if !self.refresh_living_item_pickup_cooldown(living_entity_id) {
            return;
        }
        let Some((position, expanded_bounding_box)) = self
            .entity_by_id(living_entity_id)
            .and_then(living_item_pickup_scan)
        else {
            return;
        };
        let item_entity_ids = self.entity_tracker.nearby_entities(
            position,
            expanded_bounding_box.get_width(),
            EntityTrackerTarget::Items,
        );
        item_entity_ids.into_iter().for_each(|item_entity_id| {
            if !self.item_entity_can_be_picked_up_by(
                living_entity_id,
                item_entity_id,
                position,
                expanded_bounding_box,
            ) {
                return;
            }
            if self.dispatch_pickup_item_event(living_entity_id, item_entity_id) {
                return;
            }
            let Some(pickup_item_count) = self
                .entity_by_id(item_entity_id)
                .and_then(item_entity)
                .map(|item_entity| item_entity.get_item_stack().amount())
            else {
                return;
            };
            let _ = self.send_packet_to_player_viewers_and_self(
                living_entity_id,
                TakeItemEntityPacket {
                    collected_entity_id: item_entity_id.get_value(),
                    collector_entity_id: living_entity_id.get_value(),
                    pickup_item_count,
                },
            );
            self.remove_entity(item_entity_id);
        });
    }

    fn refresh_living_item_pickup_cooldown(&mut self, living_entity_id: EntityId) -> bool {
        let Some(entity) = self.entity_by_id_mut(living_entity_id) else {
            return false;
        };
        match entity {
            Entity::Living(entity) if entity.get_item_pickup_cooldown() == 0 => {
                entity.set_item_pickup_cooldown(5);
                true
            }
            Entity::Player(player) if player.get_item_pickup_cooldown() == 0 => {
                player.set_item_pickup_cooldown(5);
                true
            }
            _ => false,
        }
    }

    fn item_entity_can_be_picked_up_by(
        &self,
        living_entity_id: EntityId,
        item_entity_id: EntityId,
        living_position: EntityPosition,
        expanded_bounding_box: EntityBoundingBox,
    ) -> bool {
        let Some(item_entity) = self.entity_by_id(item_entity_id).and_then(item_entity) else {
            return false;
        };
        if !item_entity.is_pickable() {
            return false;
        }
        if self
            .entity_by_id(living_entity_id)
            .is_some_and(|entity| matches!(entity, Entity::Player(_)))
            && !item_entity.is_viewer(living_entity_id)
        {
            return false;
        }
        item_entity.get_intersects_box_at(living_position.as_vector(), expanded_bounding_box)
    }

    fn dispatch_pickup_item_event(
        &mut self,
        living_entity_id: EntityId,
        item_entity_id: EntityId,
    ) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return false;
        };
        let mut event = PickupItemEvent::new(living_entity_id, item_entity_id);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.is_cancelled()
    }

    fn merge_item_entity(&mut self, source_item_entity_id: EntityId) {
        let Some((source_position, merge_range)) = self
            .entity_by_id(source_item_entity_id)
            .and_then(item_entity)
            .map(|item_entity| (item_entity.get_position(), item_entity.get_merge_range()))
        else {
            return;
        };
        let nearby_item_entity_ids = self.entity_tracker.nearby_entities(
            source_position,
            f64::from(merge_range),
            EntityTrackerTarget::Items,
        );
        nearby_item_entity_ids
            .into_iter()
            .filter(|merged_item_entity_id| *merged_item_entity_id != source_item_entity_id)
            .for_each(|merged_item_entity_id| {
                self.merge_item_entity_pair(source_item_entity_id, merged_item_entity_id);
            });
    }

    fn merge_item_entity_pair(
        &mut self,
        source_item_entity_id: EntityId,
        merged_item_entity_id: EntityId,
    ) {
        let Some(source_item_stack) = self
            .entity_by_id(source_item_entity_id)
            .and_then(item_entity)
            .filter(|item_entity| item_entity.is_pickable() && item_entity.is_mergeable())
            .map(|item_entity| item_entity.get_item_stack().clone())
        else {
            return;
        };
        let Some(merged_item_stack) = self
            .entity_by_id(merged_item_entity_id)
            .and_then(item_entity)
            .filter(|item_entity| item_entity.is_pickable() && item_entity.is_mergeable())
            .map(|item_entity| item_entity.get_item_stack().clone())
        else {
            return;
        };
        if !source_item_stack.is_similar(&merged_item_stack) {
            return;
        }
        let total_amount = source_item_stack.amount() + merged_item_stack.amount();
        if total_amount < 0 || total_amount > source_item_stack.max_stack_size() {
            return;
        }
        let result = source_item_stack.with_amount(total_amount);
        let Some(result) = self.dispatch_entity_item_merge_event(
            source_item_entity_id,
            merged_item_entity_id,
            result,
        ) else {
            return;
        };
        let Some(source_item_entity) =
            self.entity_by_id_mut(source_item_entity_id)
                .and_then(|entity| match entity {
                    Entity::Item(item_entity) => Some(item_entity),
                    _ => None,
                })
        else {
            return;
        };
        source_item_entity.set_item_metadata(result);
        self.remove_entity(merged_item_entity_id);
    }

    fn dispatch_entity_item_merge_event(
        &mut self,
        source_item_entity_id: EntityId,
        merged_item_entity_id: EntityId,
        result: ItemStack,
    ) -> Option<ItemStack> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(result);
        };
        let Some(source_item_entity) = self
            .entity_by_id_mut(source_item_entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(result);
        };
        let Some(merged_item_entity) = self
            .entity_by_id_mut(merged_item_entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(result);
        };
        let mut event = EntityItemMergeEvent::new(source_item_entity, merged_item_entity, result);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        (!event.is_cancelled()).then(|| event.get_result().clone())
    }

    fn dispatch_entity_equip_event(
        &mut self,
        entity_id: EntityId,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) -> Option<ItemStack> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(item_stack);
        };
        let mut event = EntityEquipEvent::new(entity_id, item_stack, equipment_slot);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        Some(event.get_equipped_item().clone())
    }

    fn apply_entity_equipment(
        &mut self,
        entity_id: EntityId,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) -> Option<(
        spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket,
        bool,
    )> {
        let entity = self.entity_by_id_mut(entity_id)?;
        match entity {
            Entity::Living(entity) => {
                entity.set_equipment_state(equipment_slot, item_stack);
                Some((entity.update_attributes_packet(), false))
            }
            Entity::Player(player) => {
                if !player.set_equipment(equipment_slot, item_stack) {
                    return None;
                }
                Some((player.update_attributes_packet(), true))
            }
            _ => None,
        }
    }

    fn dispatch_entity_potion_add_event(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return true;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return true;
        };
        let mut event = EntityPotionAddEvent::new(entity, effect);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        !event.is_cancelled()
    }

    fn dispatch_entity_potion_remove_event(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
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
        EntityPotionRemoveEvent::new(entity, effect).dispatch(server);
    }

    fn entity_effect(
        &self,
        entity_id: EntityId,
        effect_key: &RegistryKey<MobEffect>,
    ) -> Option<&TimedPotionEffect> {
        match self.entity_by_id(entity_id)? {
            Entity::Creature(entity) => entity.get_effect(effect_key),
            Entity::Living(entity) => entity.get_effect(effect_key),
            Entity::Player(player) => player.get_effect(effect_key),
            _ => None,
        }
    }

    fn entity_effects(&self, entity_id: EntityId) -> Vec<&TimedPotionEffect> {
        match self.entity_by_id(entity_id) {
            Some(Entity::Creature(entity)) => entity.get_active_effects(),
            Some(Entity::Living(entity)) => entity.get_active_effects(),
            Some(Entity::Player(player)) => player.get_active_effects(),
            _ => Vec::new(),
        }
    }

    fn entity_has_effect(&self, entity_id: EntityId, effect_key: &RegistryKey<MobEffect>) -> bool {
        self.entity_effect(entity_id, effect_key).is_some()
    }

    fn apply_entity_effect(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> Option<EntityEffectPacket> {
        match self.entity_by_id_mut(entity_id)? {
            Entity::Creature(entity) => Some(entity.add_effect(effect)),
            Entity::Living(entity) => Some(entity.add_effect(effect)),
            Entity::Player(player) => Some(player.add_effect(effect)),
            _ => None,
        }
    }

    fn apply_entity_effect_removal(
        &mut self,
        entity_id: EntityId,
        effect_key: &RegistryKey<MobEffect>,
    ) -> Option<RemoveEntityEffectPacket> {
        match self.entity_by_id_mut(entity_id)? {
            Entity::Creature(entity) => entity.remove_effect(effect_key),
            Entity::Living(entity) => entity.remove_effect(effect_key),
            Entity::Player(player) => player.remove_effect(effect_key),
            _ => None,
        }
    }

    fn dispatch_entity_effect_removal(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> Result<()> {
        let packet = self
            .apply_entity_effect_removal(entity_id, effect.get_effect_key())
            .unwrap_or_else(|| effect.remove_packet(entity_id));
        self.send_packet_to_player_viewers_and_self(entity_id, packet)?;
        self.dispatch_entity_potion_remove_event(entity_id, effect);
        Ok(())
    }

    fn entity_is_dead(&self, entity_id: EntityId) -> bool {
        self.entity_by_id(entity_id)
            .is_none_or(|entity| match entity {
                Entity::Creature(entity) => entity.is_dead(),
                Entity::ExperienceOrb(_) => false,
                Entity::Generic(_) => false,
                Entity::Living(entity) => entity.is_dead(),
                Entity::Item(_) => false,
                Entity::Player(player) => player.is_dead(),
                Entity::Projectile(_) => false,
            })
    }

    fn entity_should_be_removed_when_killed(&self, entity_id: EntityId) -> bool {
        self.entity_by_id(entity_id).is_some_and(|entity| {
            matches!(
                entity,
                Entity::ExperienceOrb(_) | Entity::Generic(_) | Entity::Item(_) | Entity::Projectile(_)
            )
        })
    }

    fn entity_is_player(&self, entity_id: EntityId) -> bool {
        self.entity_by_id(entity_id)
            .is_some_and(|entity| matches!(entity, Entity::Player(_)))
    }

    fn apply_player_death_before_living_death(&mut self, entity_id: EntityId) -> Result<()> {
        let Some(Entity::Player(player)) = self.entity_by_id_mut(entity_id) else {
            return Ok(());
        };
        player.kill()
    }

    fn apply_living_death_state(&mut self, entity_id: EntityId) -> Result<()> {
        let passenger_ids = self
            .entity_by_id(entity_id)
            .map(|entity| entity.get_passengers().iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return Ok(());
        };
        match entity {
            Entity::Creature(entity) => {
                entity.kill();
                entity.set_velocity(Velocity(Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }));
            }
            Entity::ExperienceOrb(_) | Entity::Generic(_) => {}
            Entity::Living(entity) => {
                entity.kill();
                entity.set_velocity(Velocity(Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }));
            }
            Entity::Item(_) => {}
            Entity::Player(player) => {
                player.set_pose(EntityPose::Dying);
            }
            Entity::Projectile(_) => {}
        }
        passenger_ids.into_iter().try_for_each(|passenger_id| {
            self
                .remove_passenger(entity_id, passenger_id)
                .map(|_| ())
                .map_err(|passenger_error| std::io::Error::other(passenger_error.to_string()))
        })?;
        Ok(())
    }

    fn dispatch_entity_death_event(&mut self, entity_id: EntityId) {
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
        EntityDeathEvent::new(entity).dispatch(server);
    }

    fn dispatch_entity_attack_event(&mut self, entity_id: EntityId, target_id: EntityId) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        EntityAttackEvent::new(entity_id, target_id).dispatch(server);
    }

    fn entity_rejects_damage(&self, entity_id: EntityId, damage: &Damage) -> bool {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return true;
        };
        match entity {
            Entity::Creature(entity) => {
                entity.is_dead()
                    || (damage.damage_type() != &DamageType::OUT_OF_WORLD
                        && entity.is_immune_to_damage(&damage.damage_type().key().to_string()))
            }
            Entity::ExperienceOrb(_) | Entity::Generic(_) => true,
            Entity::Living(entity) => {
                entity.is_dead()
                    || (damage.damage_type() != &DamageType::OUT_OF_WORLD
                        && entity.is_immune_to_damage(&damage.damage_type().key().to_string()))
            }
            Entity::Item(_) => true,
            Entity::Player(player) => {
                player.is_dead()
                    || (damage.damage_type() != &DamageType::OUT_OF_WORLD
                        && player.is_invulnerable())
            }
            Entity::Projectile(_) => true,
        }
    }

    fn dispatch_entity_damage_event(
        &mut self,
        entity_id: EntityId,
        damage: Damage,
    ) -> Option<Damage> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(damage);
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(damage);
        };
        let mut event = EntityDamageEvent::new(entity, damage);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        if event.is_cancelled() {
            return None;
        }
        Some(event.damage().clone())
    }

    fn apply_entity_damage(&mut self, entity_id: EntityId, damage: Damage) -> Result<bool> {
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return Ok(false);
        };
        let should_kill = match entity {
            Entity::Creature(entity) => {
                entity.apply_damage(damage);
                entity.get_health() <= 0.0
            }
            Entity::ExperienceOrb(_) | Entity::Generic(_) => return Ok(false),
            Entity::Living(entity) => {
                entity.apply_damage(damage);
                entity.get_health() <= 0.0
            }
            Entity::Item(_) => return Ok(false),
            Entity::Player(player) => {
                player.apply_damage(damage)?;
                player.get_health() <= 0.0
            }
            Entity::Projectile(_) => return Ok(false),
        };
        if should_kill {
            self.kill_entity(entity_id)?;
        }
        Ok(true)
    }

    fn dispatch_entity_set_fire_event(
        &mut self,
        entity_id: EntityId,
        fire_ticks: i32,
    ) -> Option<i32> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(fire_ticks);
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(fire_ticks);
        };
        let mut event = EntitySetFireEvent::new(entity, fire_ticks);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        if event.is_cancelled() {
            return None;
        }
        Some(event.get_fire_ticks())
    }

    fn dispatch_entity_fire_extinguish_event(
        &mut self,
        entity_id: EntityId,
        natural: bool,
    ) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return false;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return false;
        };
        let mut event = EntityFireExtinguishEvent::new(entity, natural);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.is_cancelled()
    }

    fn apply_entity_fire_ticks(&mut self, entity_id: EntityId, fire_ticks: i32) -> bool {
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return false;
        };
        match entity {
            Entity::Creature(entity) => entity.set_fire_ticks(fire_ticks),
            Entity::Living(entity) => entity.set_fire_ticks(fire_ticks),
            Entity::Player(player) => player.set_fire_ticks(fire_ticks),
            _ => return false,
        }
        true
    }

    fn apply_entity_cancelled_fire_extinguish(
        &mut self,
        entity_id: EntityId,
        fire_ticks: i32,
    ) -> bool {
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return false;
        };
        match entity {
            Entity::Creature(entity) => {
                entity.set_fire_ticks_after_cancelled_extinguish(fire_ticks);
                entity.set_on_fire(true);
            }
            Entity::ExperienceOrb(_) | Entity::Generic(_) => return false,
            Entity::Living(entity) => {
                entity.set_fire_ticks_after_cancelled_extinguish(fire_ticks);
                entity.set_on_fire(true);
            }
            Entity::Item(_) => return false,
            Entity::Player(player) => {
                player.set_fire_ticks_after_cancelled_extinguish(fire_ticks);
                player.set_on_fire(true);
            }
            Entity::Projectile(_) => return false,
        }
        true
    }
}

fn living_item_pickup_scan(entity: &Entity) -> Option<(EntityPosition, EntityBoundingBox)> {
    match entity {
        Entity::Creature(entity) => {
            Some((entity.get_position(), entity.get_expanded_bounding_box()))
        }
        Entity::Living(entity) => {
            Some((entity.get_position(), entity.get_expanded_bounding_box()))
        }
        Entity::Player(player) => Some((player.get_position(), player.get_expanded_bounding_box())),
        _ => None,
    }
}

fn item_entity(entity: &Entity) -> Option<&ItemEntity> {
    match entity {
        Entity::Item(item_entity) => Some(item_entity),
        _ => None,
    }
}

fn damage_sound_source_id(entity_id: EntityId, world: &World) -> i32 {
    match world.entity_by_id(entity_id) {
        Some(Entity::Player(_)) => 1,
        _ => 5,
    }
}
