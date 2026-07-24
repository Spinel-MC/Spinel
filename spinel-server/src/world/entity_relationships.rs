impl World {
    pub fn add_passenger(
        &mut self,
        vehicle_id: EntityId,
        passenger_id: EntityId,
    ) -> std::result::Result<bool, crate::entity::Error> {
        if vehicle_id == passenger_id {
            return Ok(false);
        }
        let Some(vehicle) = self.entity_by_id(vehicle_id) else {
            return Ok(false);
        };
        if vehicle.get_vehicle() == Some(passenger_id) {
            return Ok(false);
        }
        let Some(passenger) = self.entity_by_id(passenger_id) else {
            return Ok(false);
        };
        if let Some(previous_vehicle_id) = passenger.get_vehicle() {
            self.remove_passenger(previous_vehicle_id, passenger_id)?;
        }
        let Some(vehicle_index) = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == vehicle_id)
        else {
            return Ok(false);
        };
        let passenger_index = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == passenger_id);
        let Some(passenger_index) = passenger_index else {
            let vehicle = &mut self.entities[vehicle_index];
            if !vehicle.detach_passenger(passenger_id) {
                return Ok(false);
            }
            let passenger_packet = vehicle.get_passenger_packet();
            self.send_packet_to_player_viewers_and_self(vehicle_id, passenger_packet)?;
            return Ok(true);
        };
        let passenger_packet = {
            let (vehicle, passenger) =
                distinct_entities_mut(&mut self.entities, vehicle_index, passenger_index);
            let passenger_was_added = vehicle
                .add_passenger(passenger)
                ?;
            if !passenger_was_added {
                return Ok(false);
            }
            vehicle.get_passenger_packet()
        };
        let passenger_position = self.entities[passenger_index].get_position();
        self.entity_tracker
            .move_entity(passenger_id, passenger_position);
        self.schedule_entity_visibility_refresh(passenger_id);
        self.refresh_passenger_positions(passenger_id);
        self.send_packet_to_player_viewers_and_self(vehicle_id, passenger_packet)?;
        if let Some(position_sync_packet) = self
            .entity_by_id_mut(passenger_id)
            .map(Entity::synchronize_position_packet)
        {
            self.send_packet_to_player_viewers_and_self(passenger_id, position_sync_packet)?;
        }
        Ok(true)
    }

    pub fn remove_passenger(
        &mut self,
        vehicle_id: EntityId,
        passenger_id: EntityId,
    ) -> std::result::Result<bool, crate::entity::Error> {
        if vehicle_id == passenger_id {
            return Ok(false);
        }
        let Some(vehicle_index) = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == vehicle_id)
        else {
            return Ok(false);
        };
        let passenger_index = self
            .entities
            .iter()
            .position(|entity| entity.get_entity_id() == passenger_id);
        let Some(passenger_index) = passenger_index else {
            let vehicle = &mut self.entities[vehicle_index];
            if !vehicle.detach_passenger(passenger_id) {
                return Ok(false);
            }
            let passenger_packet = vehicle.get_passenger_packet();
            self.send_packet_to_player_viewers_and_self(vehicle_id, passenger_packet)?;
            return Ok(true);
        };
        let passenger_packet = {
            let (vehicle, passenger) =
                distinct_entities_mut(&mut self.entities, vehicle_index, passenger_index);
            let passenger_was_removed = vehicle
                .remove_passenger(passenger)
                ?;
            if !passenger_was_removed {
                return Ok(false);
            }
            vehicle.get_passenger_packet()
        };
        self.send_packet_to_player_viewers_and_self(vehicle_id, passenger_packet)?;
        if let Some(position_sync_packet) = self
            .entity_by_id_mut(passenger_id)
            .map(Entity::synchronize_position_packet)
        {
            self.send_packet_to_player_viewers_and_self(passenger_id, position_sync_packet)?;
        }
        Ok(true)
    }

    pub fn set_leash_holder(
        &mut self,
        entity_id: EntityId,
        leash_holder_id: Option<EntityId>,
    ) -> Result<bool> {
        let Some(previous_leash_holder_id) =
            self.entity_by_id(entity_id).map(Entity::get_leash_holder)
        else {
            return Ok(false);
        };
        if leash_holder_id.is_some_and(|holder_id| self.entity_by_id(holder_id).is_none()) {
            return Ok(false);
        }
        if let Some(previous_leash_holder_id) = previous_leash_holder_id {
            if let Some(previous_leash_holder) = self.entity_by_id_mut(previous_leash_holder_id) {
                previous_leash_holder.remove_leashed_entity(entity_id);
            }
        }
        if let Some(leash_holder_id) = leash_holder_id {
            if let Some(leash_holder) = self.entity_by_id_mut(leash_holder_id) {
                leash_holder.add_leashed_entity(entity_id);
            }
        }
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return Ok(false);
        };
        entity.set_leash_holder(leash_holder_id);
        let attach_entity_packet = entity.get_attach_entity_packet();
        self.send_packet_to_player_viewers_and_self(entity_id, attach_entity_packet)?;
        Ok(true)
    }

    fn detach_entity_passenger_relations(&mut self, entity_id: EntityId) {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return;
        };
        let passenger_ids = entity.get_passengers().iter().copied().collect::<Vec<_>>();
        let vehicle_id = entity.get_vehicle();
        passenger_ids.into_iter().for_each(|passenger_id| {
            let _ = self.remove_passenger(entity_id, passenger_id);
        });
        if let Some(vehicle_id) = vehicle_id {
            let _ = self.remove_passenger(vehicle_id, entity_id);
        }
    }

    fn detach_leashed_entities(&mut self, entity_id: EntityId) {
        let leashed_entity_ids = self
            .entity_by_id(entity_id)
            .map(|entity| {
                entity
                    .get_leashed_entities()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        leashed_entity_ids
            .into_iter()
            .for_each(|leashed_entity_id| {
                let _ = self.set_leash_holder(leashed_entity_id, None);
            });
    }

    fn refresh_passenger_positions(&mut self, vehicle_id: EntityId) {
        let mut pending_vehicle_ids = vec![vehicle_id];
        let mut refreshed_vehicle_ids = std::collections::BTreeSet::new();
        while let Some(vehicle_id) = pending_vehicle_ids.pop() {
            if !refreshed_vehicle_ids.insert(vehicle_id) {
                continue;
            }
            let passenger_positions = self
                .entity_by_id(vehicle_id)
                .map(|vehicle| {
                    vehicle
                        .get_passengers()
                        .iter()
                        .filter_map(|passenger_id| {
                            self.entity_by_id(*passenger_id).map(|passenger| {
                                (*passenger_id, vehicle.get_passenger_position(passenger))
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            passenger_positions
                .into_iter()
                .for_each(|(passenger_id, passenger_position)| {
                    if let Some(passenger) = self.entity_by_id_mut(passenger_id) {
                        passenger.set_position(passenger_position);
                    }
                    self.entity_tracker
                        .move_entity(passenger_id, passenger_position);
                    self.schedule_entity_visibility_refresh(passenger_id);
                    pending_vehicle_ids.push(passenger_id);
                });
        }
    }
}

fn distinct_entities_mut(
    entities: &mut [Entity],
    first_index: usize,
    second_index: usize,
) -> (&mut Entity, &mut Entity) {
    if first_index < second_index {
        let (before_second, from_second) = entities.split_at_mut(second_index);
        return (&mut before_second[first_index], &mut from_second[0]);
    }
    let (before_first, from_first) = entities.split_at_mut(first_index);
    (&mut from_first[0], &mut before_first[second_index])
}
