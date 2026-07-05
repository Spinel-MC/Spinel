impl World {
    pub fn spawn_projectile(
        &mut self,
        shooter_id: Option<EntityId>,
        entity_type: EntityType,
        position: EntityPosition,
    ) -> Result<EntityId> {
        let mut projectile = crate::entity::ProjectileEntity::new(shooter_id, entity_type);
        projectile.set_position(position);
        let projectile_id = projectile.get_entity_id();
        if !self.add_entity(Entity::Projectile(projectile)) {
            return Err(Error::new(ErrorKind::Interrupted, "Entity add cancelled."));
        }
        Ok(projectile_id)
    }

    pub fn shoot_projectile(
        &mut self,
        projectile_id: EntityId,
        target: EntityPosition,
        power: f64,
        spread: f64,
    ) -> bool {
        let Some(shooter_id) = self
            .entity_by_id(projectile_id)
            .and_then(|entity| match entity {
                Entity::Projectile(projectile) => projectile.get_shooter(),
                _ => None,
            })
        else {
            return false;
        };
        let Some((shooter_position, shooter_eye_height)) = self
            .entity_by_id(shooter_id)
            .map(|shooter| (shooter.get_position(), shooter.get_eye_height()))
        else {
            return false;
        };
        let Some(shooter) = self
            .entity_by_id_mut(shooter_id)
            .map(|entity| entity as *mut Entity)
        else {
            return false;
        };
        let Some(projectile_entity) = self
            .entity_by_id_mut(projectile_id)
            .map(|entity| entity as *mut Entity)
        else {
            return false;
        };
        let mut event = EntityShootEvent::new(shooter, projectile_entity, target, power, spread);
        self.dispatch_entity_shoot_event(&mut event);
        let Some(Entity::Projectile(projectile)) = self.entity_by_id_mut(projectile_id) else {
            return false;
        };
        if event.is_cancelled() {
            projectile.remove();
            return false;
        }
        projectile.shoot_from(
            shooter_position.get_offset(0.0, shooter_eye_height, 0.0),
            event.get_target(),
            event.get_power(),
            event.get_spread(),
        );
        true
    }

    fn process_projectile_collision(
        &mut self,
        projectile_id: EntityId,
        position_before_tick: EntityPosition,
        position_after_tick: EntityPosition,
    ) {
        let Some(projectile_state) = self.projectile_collision_state(projectile_id) else {
            return;
        };
        let collision = self.projectile_collision(
            projectile_id,
            position_before_tick,
            position_after_tick,
            projectile_state,
        );
        if self
            .entity_by_id(projectile_id)
            .is_none_or(projectile_entity_is_removed)
        {
            return;
        }
        match collision {
            ProjectileCollision::Stuck(collision_position) => {
                self.stick_projectile(projectile_id, collision_position)
            }
            ProjectileCollision::Free => self.unstick_projectile(projectile_id),
        }
    }

    fn projectile_collision_state(
        &self,
        projectile_id: EntityId,
    ) -> Option<ProjectileCollisionState> {
        let Entity::Projectile(projectile) = self.entity_by_id(projectile_id)? else {
            return None;
        };
        Some(ProjectileCollisionState {
            shooter_id: projectile.get_shooter(),
            alive_ticks: projectile.ticks(),
            bounding_box: projectile.get_bounding_box(),
            is_on_ground: projectile.is_on_ground(),
        })
    }

    fn projectile_collision(
        &mut self,
        projectile_id: EntityId,
        position_before_tick: EntityPosition,
        position_after_tick: EntityPosition,
        projectile_state: ProjectileCollisionState,
    ) -> ProjectileCollision {
        if entity_positions_share_point(position_before_tick, position_after_tick) {
            return self
                .loaded_block_at(block_position_for_entity(position_before_tick))
                .filter(|block| block.is_solid())
                .map_or(ProjectileCollision::Free, |_| {
                    ProjectileCollision::Stuck(position_before_tick)
                });
        }
        projectile_sample_positions(
            position_before_tick,
            position_after_tick,
            projectile_state.bounding_box.get_width(),
        )
        .into_iter()
        .find_map(|collision_position| {
            self.projectile_collision_at(projectile_id, collision_position, projectile_state)
        })
        .unwrap_or(ProjectileCollision::Free)
    }

    fn projectile_collision_at(
        &mut self,
        projectile_id: EntityId,
        collision_position: EntityPosition,
        projectile_state: ProjectileCollisionState,
    ) -> Option<ProjectileCollision> {
        let block = self
            .loaded_block_at(block_position_for_entity(collision_position))
            .unwrap_or(Block::AIR);
        if block.is_solid() {
            let mut event =
                ProjectileCollideWithBlockEvent::new(projectile_id, collision_position, block);
            self.dispatch_projectile_block_collision_event(&mut event);
            if self
                .entity_by_id(projectile_id)
                .is_none_or(projectile_entity_is_removed)
            {
                return Some(ProjectileCollision::Stuck(collision_position));
            }
            if !event.is_cancelled() {
                return Some(ProjectileCollision::Stuck(collision_position));
            }
        }
        let target_id =
            self.projectile_collision_target(projectile_id, collision_position, projectile_state)?;
        let mut event =
            ProjectileCollideWithEntityEvent::new(projectile_id, collision_position, target_id);
        self.dispatch_projectile_entity_collision_event(&mut event);
        (!event.is_cancelled() && projectile_state.is_on_ground)
            .then_some(ProjectileCollision::Stuck(collision_position))
    }

    fn projectile_collision_target(
        &self,
        projectile_id: EntityId,
        collision_position: EntityPosition,
        projectile_state: ProjectileCollisionState,
    ) -> Option<EntityId> {
        self.entity_tracker
            .chunk_entities(
                ChunkPosition::from(collision_position),
                EntityTrackerTarget::Entities,
            )
            .into_iter()
            .filter(|target_id| *target_id != projectile_id)
            .filter(|target_id| {
                projectile_state.alive_ticks >= 3 || projectile_state.shooter_id != Some(*target_id)
            })
            .filter_map(|target_id| {
                self.entity_by_id(target_id)
                    .filter(|target| entity_is_living(target))
                    .filter(|target| {
                        entity_boxes_intersect_at(
                            collision_position,
                            projectile_state.bounding_box,
                            target.get_position(),
                            target.get_bounding_box(),
                        )
                    })
                    .map(|_| target_id)
            })
            .next()
    }

    fn stick_projectile(&mut self, projectile_id: EntityId, collision_position: EntityPosition) {
        let Some(Entity::Projectile(projectile)) = self.entity_by_id_mut(projectile_id) else {
            return;
        };
        if projectile.is_on_ground() {
            return;
        }
        projectile.set_position(collision_position);
        projectile.set_on_ground(true);
        projectile.set_velocity(Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }));
        projectile.set_no_gravity(true);
        projectile.set_was_stuck(true);
        let teleport_packet = projectile.synchronize_position_packet();
        let velocity_packet = projectile.get_velocity_packet();
        self.entity_tracker
            .move_entity(projectile_id, collision_position);
        self.refresh_passenger_positions(projectile_id);
        let _ = self.send_packet_to_player_viewers_and_self(projectile_id, teleport_packet);
        let _ = self.send_packet_to_player_viewers_and_self(projectile_id, velocity_packet);
    }

    fn unstick_projectile(&mut self, projectile_id: EntityId) {
        let Some(Entity::Projectile(projectile)) = self.entity_by_id_mut(projectile_id) else {
            return;
        };
        if !projectile.get_was_stuck() {
            return;
        }
        projectile.set_was_stuck(false);
        let was_on_ground = projectile.is_on_ground();
        projectile.set_no_gravity(was_on_ground);
        projectile.set_on_ground(false);
        self.dispatch_projectile_uncollide_event(ProjectileUncollideEvent::new(projectile_id));
    }

    fn dispatch_entity_shoot_event(&mut self, event: &mut EntityShootEvent) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
    }

    fn dispatch_projectile_block_collision_event(
        &mut self,
        event: &mut ProjectileCollideWithBlockEvent,
    ) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.get_collision_mut().dispatch(server);
    }

    fn dispatch_projectile_entity_collision_event(
        &mut self,
        event: &mut ProjectileCollideWithEntityEvent,
    ) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.get_collision_mut().dispatch(server);
    }

    fn dispatch_projectile_uncollide_event(&mut self, mut event: ProjectileUncollideEvent) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
    }
}

#[derive(Clone, Copy)]
struct ProjectileCollisionState {
    shooter_id: Option<EntityId>,
    alive_ticks: u64,
    bounding_box: spinel_registry::EntityBoundingBox,
    is_on_ground: bool,
}

enum ProjectileCollision {
    Free,
    Stuck(EntityPosition),
}

fn projectile_sample_positions(
    position_before_tick: EntityPosition,
    position_after_tick: EntityPosition,
    projectile_width: f64,
) -> Vec<EntityPosition> {
    let delta_x = position_after_tick.get_x() - position_before_tick.get_x();
    let delta_y = position_after_tick.get_y() - position_before_tick.get_y();
    let delta_z = position_after_tick.get_z() - position_before_tick.get_z();
    let distance = delta_x
        .mul_add(delta_x, delta_y.mul_add(delta_y, delta_z * delta_z))
        .sqrt();
    let sample_distance = projectile_width / 2.0;
    let sample_count = (distance / sample_distance).ceil() as usize;
    if sample_count == 0 {
        return Vec::new();
    }
    let direction_x = delta_x / distance * sample_distance;
    let direction_y = delta_y / distance * sample_distance;
    let direction_z = delta_z / distance * sample_distance;
    (0..sample_count)
        .map(|sample_index| {
            if sample_index == sample_count - 1 {
                return position_after_tick;
            }
            let sample_multiplier = (sample_index + 1) as f64;
            position_before_tick.get_offset(
                direction_x * sample_multiplier,
                direction_y * sample_multiplier,
                direction_z * sample_multiplier,
            )
        })
        .collect()
}

fn block_position_for_entity(position: EntityPosition) -> BlockPosition {
    BlockPosition::new(
        position.get_x().floor() as i32,
        position.get_y().floor() as i32,
        position.get_z().floor() as i32,
    )
}

fn entity_is_living(entity: &Entity) -> bool {
    match entity {
        Entity::Creature(_) | Entity::Player(_) => true,
        Entity::Generic(entity) => entity.get_entity_type().is_living(),
        Entity::ExperienceOrb(_) | Entity::Item(_) | Entity::Projectile(_) => false,
    }
}

fn projectile_entity_is_removed(entity: &Entity) -> bool {
    match entity {
        Entity::Projectile(entity) => entity.is_removed(),
        Entity::Creature(_)
        | Entity::ExperienceOrb(_)
        | Entity::Generic(_)
        | Entity::Item(_)
        | Entity::Player(_) => true,
    }
}

fn entity_positions_share_point(
    first_position: EntityPosition,
    second_position: EntityPosition,
) -> bool {
    first_position.get_x() == second_position.get_x()
        && first_position.get_y() == second_position.get_y()
        && first_position.get_z() == second_position.get_z()
}

fn entity_boxes_intersect_at(
    first_position: EntityPosition,
    first_bounding_box: spinel_registry::EntityBoundingBox,
    second_position: EntityPosition,
    second_bounding_box: spinel_registry::EntityBoundingBox,
) -> bool {
    boxes_intersect(
        entity_box_start(first_position, first_bounding_box),
        entity_box_end(first_position, first_bounding_box),
        entity_box_start(second_position, second_bounding_box),
        entity_box_end(second_position, second_bounding_box),
    )
}

fn entity_box_start(
    position: EntityPosition,
    bounding_box: spinel_registry::EntityBoundingBox,
) -> Vector3d {
    Vector3d {
        x: position.get_x() + bounding_box.minimum_x(),
        y: position.get_y() + bounding_box.minimum_y(),
        z: position.get_z() + bounding_box.minimum_z(),
    }
}

fn entity_box_end(
    position: EntityPosition,
    bounding_box: spinel_registry::EntityBoundingBox,
) -> Vector3d {
    Vector3d {
        x: position.get_x() + bounding_box.maximum_x(),
        y: position.get_y() + bounding_box.maximum_y(),
        z: position.get_z() + bounding_box.maximum_z(),
    }
}

