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
            bounding_box: projectile.get_bounding_box(),
            is_on_ground: projectile.is_on_ground(),
            alive_ticks: projectile.get_alive_ticks(),
            velocity_per_tick: Vector3d {
                x: projectile.get_velocity().0.x / 20.0,
                y: projectile.get_velocity().0.y / 20.0,
                z: projectile.get_velocity().0.z / 20.0,
            },
            has_left_shooter_collision_range: projectile.has_left_shooter_collision_range(),
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
        let projectile_state = self
            .update_projectile_left_shooter_collision_range(
                projectile_id,
                position_after_tick,
                projectile_state,
            );
        self.projectile_collision_candidates(
            projectile_id,
            position_before_tick,
            position_after_tick,
            projectile_state,
        )
        .into_iter()
        .find_map(|collision_candidate| {
            self.apply_projectile_collision_candidate(
                projectile_id,
                collision_candidate,
                projectile_state,
            )
        })
        .unwrap_or(ProjectileCollision::Free)
    }

    fn projectile_collision_candidates(
        &self,
        projectile_id: EntityId,
        position_before_tick: EntityPosition,
        position_after_tick: EntityPosition,
        projectile_state: ProjectileCollisionState,
    ) -> Vec<ProjectileCollisionCandidate> {
        let movement = movement_between_positions(position_before_tick, position_after_tick);
        let start = position_before_tick.as_vector();
        let mut candidates = self.projectile_block_collision_candidates(start, movement);
        candidates.extend(self.projectile_entity_collision_candidates(
            projectile_id,
            start,
            movement,
            projectile_state,
        ));
        candidates.sort_by(|first_candidate, second_candidate| {
            first_candidate
                .get_ratio()
                .total_cmp(&second_candidate.get_ratio())
        });
        candidates
    }

    fn projectile_block_collision_candidates(
        &self,
        start: Vector3d,
        movement: Vector3d,
    ) -> Vec<ProjectileCollisionCandidate> {
        scanned_block_positions(start, movement)
            .into_iter()
            .filter_map(|block_position| {
                let block_state = self.loaded_block_state_at(block_position)?;
                let block = block_state.block();
                if block == Block::AIR {
                    return None;
                }
                block_state
                    .collision_shape()
                    .iter()
                    .filter_map(|shape_box| {
                        block_shape_raycast_box(block_position, *shape_box)
                            .ray_intersection(start, movement)
                    })
                    .min_by(|first_hit, second_hit| first_hit.ratio.total_cmp(&second_hit.ratio))
                    .map(|hit| ProjectileCollisionCandidate::Block {
                        ratio: hit.ratio,
                        position: entity_position_at(hit.position),
                        block,
                    })
            })
            .collect()
    }

    fn projectile_entity_collision_candidates(
        &self,
        projectile_id: EntityId,
        start: Vector3d,
        movement: Vector3d,
        projectile_state: ProjectileCollisionState,
    ) -> Vec<ProjectileCollisionCandidate> {
        let hit_margin = projectile_hit_margin(projectile_state.alive_ticks);
        self.entities
            .iter()
            .filter(|target| target.get_entity_id() != projectile_id)
            .filter(|target| projectile_can_hit_entity(projectile_state, target.get_entity_id()))
            .filter(|target| entity_is_living(target))
            .filter_map(|target| {
                inflated_entity_raycast_box(target, hit_margin)
                    .ray_intersection(start, movement)
                    .map(|hit| ProjectileCollisionCandidate::Entity {
                        ratio: hit.ratio,
                        position: entity_position_at(hit.position),
                        target_id: target.get_entity_id(),
                    })
            })
            .collect()
    }

    fn apply_projectile_collision_candidate(
        &mut self,
        projectile_id: EntityId,
        collision_candidate: ProjectileCollisionCandidate,
        projectile_state: ProjectileCollisionState,
    ) -> Option<ProjectileCollision> {
        match collision_candidate {
            ProjectileCollisionCandidate::Block {
                position, block, ..
            } => self.apply_projectile_block_collision_candidate(projectile_id, position, block),
            ProjectileCollisionCandidate::Entity {
                position,
                target_id,
                ..
            } => self.apply_projectile_entity_collision_candidate(
                projectile_id,
                position,
                target_id,
                projectile_state,
            ),
        }
    }

    fn apply_projectile_block_collision_candidate(
        &mut self,
        projectile_id: EntityId,
        collision_position: EntityPosition,
        block: Block,
    ) -> Option<ProjectileCollision> {
        let mut event = ProjectileCollideWithBlockEvent::new(projectile_id, collision_position, block);
        self.dispatch_projectile_block_collision_event(&mut event);
        if self
            .entity_by_id(projectile_id)
            .is_none_or(projectile_entity_is_removed)
        {
            return Some(ProjectileCollision::Stuck(collision_position));
        }
        (!event.is_cancelled()).then_some(ProjectileCollision::Stuck(collision_position))
    }

    fn apply_projectile_entity_collision_candidate(
        &mut self,
        projectile_id: EntityId,
        collision_position: EntityPosition,
        target_id: EntityId,
        projectile_state: ProjectileCollisionState,
    ) -> Option<ProjectileCollision> {
        let mut event =
            ProjectileCollideWithEntityEvent::new(projectile_id, collision_position, target_id);
        self.dispatch_projectile_entity_collision_event(&mut event);
        if self
            .entity_by_id(projectile_id)
            .is_none_or(projectile_entity_is_removed)
        {
            return Some(ProjectileCollision::Stuck(collision_position));
        }
        if event.is_cancelled() {
            return None;
        }
        Some(if projectile_state.is_on_ground {
            ProjectileCollision::Stuck(collision_position)
        } else {
            ProjectileCollision::Free
        })
    }

    fn update_projectile_left_shooter_collision_range(
        &mut self,
        projectile_id: EntityId,
        projectile_position: EntityPosition,
        mut projectile_state: ProjectileCollisionState,
    ) -> ProjectileCollisionState {
        if projectile_state.has_left_shooter_collision_range {
            return projectile_state;
        }
        if projectile_state.shooter_id.is_none() {
            projectile_state.has_left_shooter_collision_range = true;
            return projectile_state;
        }
        if self.projectile_intersects_shooter_collision_range(projectile_position, projectile_state)
        {
            return projectile_state;
        }
        let Some(Entity::Projectile(projectile)) = self.entity_by_id_mut(projectile_id) else {
            return projectile_state;
        };
        projectile.set_has_left_shooter_collision_range(true);
        projectile_state.has_left_shooter_collision_range = true;
        projectile_state
    }

    fn projectile_intersects_shooter_collision_range(
        &self,
        projectile_position: EntityPosition,
        projectile_state: ProjectileCollisionState,
    ) -> bool {
        let Some(shooter_id) = projectile_state.shooter_id else {
            return false;
        };
        let Some(shooter) = self.entity_by_id(shooter_id) else {
            return false;
        };
        expanded_projectile_box_intersects_entity(
            projectile_position,
            projectile_state.bounding_box,
            projectile_state.velocity_per_tick,
            shooter,
        )
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
    bounding_box: spinel_registry::EntityBoundingBox,
    is_on_ground: bool,
    alive_ticks: u64,
    velocity_per_tick: Vector3d,
    has_left_shooter_collision_range: bool,
}

enum ProjectileCollision {
    Free,
    Stuck(EntityPosition),
}

enum ProjectileCollisionCandidate {
    Block {
        ratio: f64,
        position: EntityPosition,
        block: Block,
    },
    Entity {
        ratio: f64,
        position: EntityPosition,
        target_id: EntityId,
    },
}

impl ProjectileCollisionCandidate {
    const fn get_ratio(&self) -> f64 {
        match self {
            Self::Block { ratio, .. } | Self::Entity { ratio, .. } => *ratio,
        }
    }
}

fn movement_between_positions(
    position_before_tick: EntityPosition,
    position_after_tick: EntityPosition,
) -> Vector3d {
    Vector3d {
        x: position_after_tick.get_x() - position_before_tick.get_x(),
        y: position_after_tick.get_y() - position_before_tick.get_y(),
        z: position_after_tick.get_z() - position_before_tick.get_z(),
    }
}

fn scanned_block_positions(start: Vector3d, movement: Vector3d) -> Vec<BlockPosition> {
    let end = Vector3d {
        x: start.x + movement.x,
        y: start.y + movement.y,
        z: start.z + movement.z,
    };
    let minimum_x = start.x.min(end.x).floor() as i32 - 1;
    let minimum_y = start.y.min(end.y).floor() as i32 - 1;
    let minimum_z = start.z.min(end.z).floor() as i32 - 1;
    let maximum_x = start.x.max(end.x).floor() as i32 + 1;
    let maximum_y = start.y.max(end.y).floor() as i32 + 1;
    let maximum_z = start.z.max(end.z).floor() as i32 + 1;
    (minimum_x..=maximum_x)
        .flat_map(|block_x| {
            (minimum_y..=maximum_y).flat_map(move |block_y| {
                (minimum_z..=maximum_z)
                    .map(move |block_z| BlockPosition::new(block_x, block_y, block_z))
            })
        })
        .collect()
}

fn block_shape_raycast_box(
    block_position: BlockPosition,
    shape_box: spinel_registry::BlockShapeBox,
) -> RaycastBoundingBox {
    RaycastBoundingBox::new(
        Vector3d {
            x: f64::from(block_position.x) + shape_box.min_x,
            y: f64::from(block_position.y) + shape_box.min_y,
            z: f64::from(block_position.z) + shape_box.min_z,
        },
        Vector3d {
            x: f64::from(block_position.x) + shape_box.max_x,
            y: f64::from(block_position.y) + shape_box.max_y,
            z: f64::from(block_position.z) + shape_box.max_z,
        },
    )
}

fn inflated_entity_raycast_box(entity: &Entity, inflation: f64) -> RaycastBoundingBox {
    RaycastBoundingBox::new(
        inflated_box_start(
            entity_box_start(entity.get_position(), entity.get_bounding_box()),
            inflation,
        ),
        inflated_box_end(
            entity_box_end(entity.get_position(), entity.get_bounding_box()),
            inflation,
        ),
    )
}

fn inflated_box_start(box_start: Vector3d, inflation: f64) -> Vector3d {
    Vector3d {
        x: box_start.x - inflation,
        y: box_start.y - inflation,
        z: box_start.z - inflation,
    }
}

fn inflated_box_end(box_end: Vector3d, inflation: f64) -> Vector3d {
    Vector3d {
        x: box_end.x + inflation,
        y: box_end.y + inflation,
        z: box_end.z + inflation,
    }
}

fn projectile_hit_margin(alive_ticks: u64) -> f64 {
    ((alive_ticks as f64 - 2.0) / 20.0).clamp(0.0, 0.3)
}

fn projectile_can_hit_entity(
    projectile_state: ProjectileCollisionState,
    target_id: EntityId,
) -> bool {
    projectile_state.shooter_id != Some(target_id)
        || projectile_state.has_left_shooter_collision_range
}

fn entity_position_at(position: Vector3d) -> EntityPosition {
    EntityPosition::new(position.x, position.y, position.z, 0.0, 0.0)
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
        Entity::Creature(_) | Entity::Living(_) | Entity::Player(_) => true,
        Entity::Generic(_) => false,
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
        | Entity::Living(_)
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

fn expanded_projectile_box_intersects_entity(
    projectile_position: EntityPosition,
    projectile_bounding_box: spinel_registry::EntityBoundingBox,
    projectile_velocity_per_tick: Vector3d,
    entity: &Entity,
) -> bool {
    let projectile_box_start = entity_box_start(projectile_position, projectile_bounding_box);
    let projectile_box_end = entity_box_end(projectile_position, projectile_bounding_box);
    boxes_intersect(
        expanded_box_start(projectile_box_start, projectile_velocity_per_tick, 1.0),
        expanded_box_end(projectile_box_end, projectile_velocity_per_tick, 1.0),
        entity_box_start(entity.get_position(), entity.get_bounding_box()),
        entity_box_end(entity.get_position(), entity.get_bounding_box()),
    )
}

fn expanded_box_start(box_start: Vector3d, expansion: Vector3d, inflation: f64) -> Vector3d {
    Vector3d {
        x: box_start.x + expansion.x.min(0.0) - inflation,
        y: box_start.y + expansion.y.min(0.0) - inflation,
        z: box_start.z + expansion.z.min(0.0) - inflation,
    }
}

fn expanded_box_end(box_end: Vector3d, expansion: Vector3d, inflation: f64) -> Vector3d {
    Vector3d {
        x: box_end.x + expansion.x.max(0.0) + inflation,
        y: box_end.y + expansion.y.max(0.0) + inflation,
        z: box_end.z + expansion.z.max(0.0) + inflation,
    }
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

