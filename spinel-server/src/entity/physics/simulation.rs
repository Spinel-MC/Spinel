use crate::entity::EntityPosition;
use crate::entity::generic_entity::EntityAerodynamics;
use crate::entity::physics::{EntityPhysicsResult, EntitySweepResult};
use crate::world::{BlockPosition, WorldSnapshot};
use spinel_core::raycast::{RaycastBoundingBox, RaycastIntersection, RaycastNormal};
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::{BlockShapeBox, EntityBoundingBox};

const VELOCITY_EPSILON: f64 = 0.000001;
const COLLISION_RATIO_LIMIT: f64 = 1.0 - VELOCITY_EPSILON;
const COLLISION_BACKOFF: f64 = 0.99999;

pub fn simulate_movement(
    position: EntityPosition,
    velocity_per_tick: Velocity,
    bounding_box: EntityBoundingBox,
    world: &WorldSnapshot,
    aerodynamics: EntityAerodynamics,
    has_no_gravity: bool,
    has_physics: bool,
    is_on_ground: bool,
    is_flying: bool,
    previous_physics_result: Option<EntityPhysicsResult>,
) -> EntityPhysicsResult {
    let collision = if has_physics {
        collide_with_blocks(
            position,
            velocity_per_tick,
            bounding_box,
            world,
            previous_physics_result,
        )
    } else {
        blockless_movement(position, velocity_per_tick)
    };
    let bordered_position = apply_world_border(
        position,
        collision.get_new_position(),
        world.get_world_border(),
    );
    let position_changed = bordered_position.get_x() != position.get_x()
        || bordered_position.get_y() != position.get_y()
        || bordered_position.get_z() != position.get_z();
    let new_velocity_per_tick = update_velocity(
        bordered_position,
        collision.get_new_velocity_per_tick(),
        world,
        aerodynamics,
        position_changed,
        is_flying,
        is_on_ground,
        has_no_gravity,
    );
    let remains_cached = collision.is_cached()
        && new_velocity_per_tick == collision.get_new_velocity_per_tick()
        && bordered_position == collision.get_new_position();
    collision.with_movement(bordered_position, new_velocity_per_tick, remains_cached)
}

pub fn simulate_collision(
    position: EntityPosition,
    velocity_per_tick: Velocity,
    bounding_box: EntityBoundingBox,
    world: &WorldSnapshot,
    previous_physics_result: Option<EntityPhysicsResult>,
) -> EntityPhysicsResult {
    collide_with_blocks(
        position,
        velocity_per_tick,
        bounding_box,
        world,
        previous_physics_result,
    )
}

fn collide_with_blocks(
    position: EntityPosition,
    velocity_per_tick: Velocity,
    bounding_box: EntityBoundingBox,
    world: &WorldSnapshot,
    previous_physics_result: Option<EntityPhysicsResult>,
) -> EntityPhysicsResult {
    let original_delta = velocity_per_tick.0;
    if vector_is_zero(original_delta) {
        return EntityPhysicsResult::without_collision(
            position,
            Velocity(zero_vector()),
            velocity_per_tick,
        );
    }
    if let Some(cached_result) =
        cached_physics(velocity_per_tick, position, world, previous_physics_result)
    {
        return cached_result;
    }
    let mut current_position = position;
    let mut remaining_delta = original_delta;
    let mut collision_x = false;
    let mut collision_y = false;
    let mut collision_z = false;
    let mut collision_points = [None; 3];
    let mut collision_shapes = [None; 3];
    let mut collision_shape_positions = [None; 3];
    let mut final_sweep = EntitySweepResult::no_collision();

    for _ in 0..3 {
        let Some(intersection) =
            earliest_block_intersection(current_position, remaining_delta, bounding_box, world)
        else {
            current_position =
                offset_position(current_position, minestom_swept_delta(remaining_delta));
            break;
        };
        let travelled_ratio = intersection.intersection.ratio.min(COLLISION_RATIO_LIMIT);
        let travelled_delta = minestom_resolved_delta(remaining_delta, travelled_ratio);
        current_position = offset_position(current_position, travelled_delta);
        remaining_delta = subtract_vector(remaining_delta, travelled_delta);
        let collision_axis = match intersection.intersection.normal {
            RaycastNormal::NegativeX | RaycastNormal::PositiveX => {
                collision_x = true;
                remaining_delta.x = 0.0;
                0
            }
            RaycastNormal::NegativeY | RaycastNormal::PositiveY => {
                collision_y = true;
                remaining_delta.y = 0.0;
                1
            }
            RaycastNormal::NegativeZ | RaycastNormal::PositiveZ => {
                collision_z = true;
                remaining_delta.z = 0.0;
                2
            }
        };
        collision_points[collision_axis] = Some(intersection.intersection.position);
        collision_shapes[collision_axis] = Some(intersection.collision_shape);
        collision_shape_positions[collision_axis] = Some(intersection.block_position);
        final_sweep = intersection.sweep();
        if vector_is_zero(remaining_delta) {
            break;
        }
    }

    let resolved_delta = Vector3d {
        x: if collision_x { 0.0 } else { original_delta.x },
        y: if collision_y { 0.0 } else { original_delta.y },
        z: if collision_z { 0.0 } else { original_delta.z },
    };
    EntityPhysicsResult::new(
        current_position,
        Velocity(resolved_delta),
        collision_y && original_delta.y < 0.0,
        collision_x,
        collision_y,
        collision_z,
        velocity_per_tick,
        collision_points,
        collision_shapes,
        collision_shape_positions,
        collision_x || collision_y || collision_z,
        final_sweep,
        false,
    )
}

#[derive(Clone, Copy)]
struct BlockIntersection {
    intersection: RaycastIntersection,
    collision_shape: &'static [BlockShapeBox],
    block_position: BlockPosition,
}

impl BlockIntersection {
    fn sweep(self) -> EntitySweepResult {
        EntitySweepResult::new(
            self.intersection.ratio,
            collision_normal(self.intersection.normal),
            Some(self.intersection.position),
            Some(self.collision_shape),
            Some(self.block_position),
        )
    }
}

fn earliest_block_intersection(
    position: EntityPosition,
    delta: Vector3d,
    bounding_box: EntityBoundingBox,
    world: &WorldSnapshot,
) -> Option<BlockIntersection> {
    let start_minimum = Vector3d {
        x: position.get_x() + bounding_box.minimum_x(),
        y: position.get_y() + bounding_box.minimum_y(),
        z: position.get_z() + bounding_box.minimum_z(),
    };
    let start_maximum = Vector3d {
        x: position.get_x() + bounding_box.maximum_x(),
        y: position.get_y() + bounding_box.maximum_y(),
        z: position.get_z() + bounding_box.maximum_z(),
    };
    let end_minimum = add_vector(start_minimum, delta);
    let end_maximum = add_vector(start_maximum, delta);
    let minimum_block = BlockPosition::new(
        start_minimum.x.min(end_minimum.x).floor() as i32 - 1,
        start_minimum.y.min(end_minimum.y).floor() as i32 - 1,
        start_minimum.z.min(end_minimum.z).floor() as i32 - 1,
    );
    let maximum_block = BlockPosition::new(
        start_maximum.x.max(end_maximum.x).ceil() as i32 + 1,
        start_maximum.y.max(end_maximum.y).ceil() as i32 + 1,
        start_maximum.z.max(end_maximum.z).ceil() as i32 + 1,
    );

    (minimum_block.x..=maximum_block.x)
        .flat_map(|block_x| {
            (minimum_block.y..=maximum_block.y).flat_map(move |block_y| {
                (minimum_block.z..=maximum_block.z)
                    .map(move |block_z| BlockPosition::new(block_x, block_y, block_z))
            })
        })
        .flat_map(|block_position| {
            let collision_shape = world.block_state(block_position).collision_shape();
            collision_shape.iter().filter_map(move |shape| {
                intersect_block_shape(position, delta, bounding_box, block_position, shape).map(
                    |intersection| BlockIntersection {
                        intersection,
                        collision_shape,
                        block_position,
                    },
                )
            })
        })
        .min_by(|left, right| left.intersection.ratio.total_cmp(&right.intersection.ratio))
}

fn intersect_block_shape(
    position: EntityPosition,
    delta: Vector3d,
    bounding_box: EntityBoundingBox,
    block_position: BlockPosition,
    shape: &BlockShapeBox,
) -> Option<RaycastIntersection> {
    let entity_minimum = Vector3d {
        x: position.get_x() + bounding_box.minimum_x(),
        y: position.get_y() + bounding_box.minimum_y(),
        z: position.get_z() + bounding_box.minimum_z(),
    };
    let entity_maximum = Vector3d {
        x: position.get_x() + bounding_box.maximum_x(),
        y: position.get_y() + bounding_box.maximum_y(),
        z: position.get_z() + bounding_box.maximum_z(),
    };
    let shape_minimum = Vector3d {
        x: f64::from(block_position.x) + shape.min_x,
        y: f64::from(block_position.y) + shape.min_y,
        z: f64::from(block_position.z) + shape.min_z,
    };
    let shape_maximum = Vector3d {
        x: f64::from(block_position.x) + shape.max_x,
        y: f64::from(block_position.y) + shape.max_y,
        z: f64::from(block_position.z) + shape.max_z,
    };
    if delta.x == 0.0
        && !intervals_overlap(
            entity_minimum.x,
            entity_maximum.x,
            shape_minimum.x,
            shape_maximum.x,
        )
    {
        return None;
    }
    if delta.y == 0.0
        && !intervals_overlap(
            entity_minimum.y,
            entity_maximum.y,
            shape_minimum.y,
            shape_maximum.y,
        )
    {
        return None;
    }
    if delta.z == 0.0
        && !intervals_overlap(
            entity_minimum.z,
            entity_maximum.z,
            shape_minimum.z,
            shape_maximum.z,
        )
    {
        return None;
    }
    let moving_center = Vector3d {
        x: bounding_box.minimum_x() + bounding_box.get_width() / 2.0,
        y: bounding_box.minimum_y() + bounding_box.get_height() / 2.0,
        z: bounding_box.minimum_z() + bounding_box.depth() / 2.0,
    };
    let ray_center = add_vector(position.as_vector(), moving_center);
    let half_width = bounding_box.get_width() / 2.0;
    let half_height = bounding_box.get_height() / 2.0;
    let half_depth = bounding_box.depth() / 2.0;
    let expanded_minimum = Vector3d {
        x: shape.min_x - ray_center.x + f64::from(block_position.x) - half_width,
        y: shape.min_y - ray_center.y + f64::from(block_position.y) - half_height,
        z: shape.min_z - ray_center.z + f64::from(block_position.z) - half_depth,
    };
    let expanded_maximum = Vector3d {
        x: shape.max_x - ray_center.x + f64::from(block_position.x) + half_width,
        y: shape.max_y - ray_center.y + f64::from(block_position.y) + half_height,
        z: shape.max_z - ray_center.z + f64::from(block_position.z) + half_depth,
    };
    let expanded_shape = RaycastBoundingBox::new(
        minestom_collision_offsets(expanded_minimum, delta),
        minestom_collision_offsets(expanded_maximum, delta),
    );
    RaycastIntersection::between_ray_and_box(zero_vector(), delta, expanded_shape)
        .map(|intersection| {
            let ratio = intersection.ratio * COLLISION_BACKOFF;
            RaycastIntersection {
                ratio,
                position: add_vector(position.as_vector(), multiply_vector(delta, ratio)),
                normal: intersection.normal,
            }
        })
        .filter(|intersection| intersection.ratio <= COLLISION_RATIO_LIMIT)
}

fn intervals_overlap(
    first_minimum: f64,
    first_maximum: f64,
    second_minimum: f64,
    second_maximum: f64,
) -> bool {
    first_minimum < second_maximum && first_maximum > second_minimum
}

fn blockless_movement(
    position: EntityPosition,
    velocity_per_tick: Velocity,
) -> EntityPhysicsResult {
    EntityPhysicsResult::without_collision(
        offset_position(position, velocity_per_tick.0),
        velocity_per_tick,
        velocity_per_tick,
    )
}

fn cached_physics(
    velocity_per_tick: Velocity,
    position: EntityPosition,
    world: &WorldSnapshot,
    previous_physics_result: Option<EntityPhysicsResult>,
) -> Option<EntityPhysicsResult> {
    let previous = previous_physics_result?;
    let collision_shape_position = previous.get_collision_shape_positions()[1]?;
    let collision_shape = previous.get_collision_shapes()[1]?;
    let current_collision_shape = world
        .block_state(collision_shape_position)
        .collision_shape();
    let can_reuse = previous.has_collision_y()
        && velocity_per_tick == previous.get_original_delta()
        && current_collision_shape == collision_shape
        && velocity_per_tick.0.x == 0.0
        && velocity_per_tick.0.z == 0.0
        && position == previous.get_new_position()
        && !collision_shape.is_empty();
    can_reuse.then_some(previous.as_cached())
}

fn collision_normal(normal: RaycastNormal) -> Vector3d {
    match normal {
        RaycastNormal::NegativeX | RaycastNormal::PositiveX => Vector3d {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        RaycastNormal::NegativeY | RaycastNormal::PositiveY => Vector3d {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        RaycastNormal::NegativeZ | RaycastNormal::PositiveZ => Vector3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    }
}

fn apply_world_border(
    current_position: EntityPosition,
    new_position: EntityPosition,
    world_border: crate::world::WorldBorder,
) -> EntityPosition {
    let radius = world_border.diameter() / 2.0;
    let has_x_collision = new_position.get_x() > world_border.center_x() + radius
        || new_position.get_x() < world_border.center_x() - radius;
    let has_z_collision = new_position.get_z() > world_border.center_z() + radius
        || new_position.get_z() < world_border.center_z() - radius;
    if !has_x_collision && !has_z_collision {
        return new_position;
    }
    EntityPosition::new(
        if has_x_collision {
            current_position.get_x()
        } else {
            new_position.get_x()
        },
        new_position.get_y(),
        if has_z_collision {
            current_position.get_z()
        } else {
            new_position.get_z()
        },
        new_position.get_yaw(),
        new_position.get_pitch(),
    )
    .with_head_yaw(new_position.get_head_yaw())
}

fn update_velocity(
    position: EntityPosition,
    current_velocity: Velocity,
    world: &WorldSnapshot,
    aerodynamics: EntityAerodynamics,
    position_changed: bool,
    is_flying: bool,
    is_on_ground: bool,
    has_no_gravity: bool,
) -> Velocity {
    if !position_changed {
        if is_flying {
            return Velocity(zero_vector());
        }
        return Velocity(Vector3d {
            x: 0.0,
            y: if has_no_gravity {
                0.0
            } else {
                -aerodynamics.get_gravity() * aerodynamics.get_vertical_air_resistance()
            },
            z: 0.0,
        });
    }

    let drag = if is_on_ground {
        let block_below = BlockPosition::new(
            position.get_x().floor() as i32,
            (position.get_y() - 0.5000001).floor() as i32,
            position.get_z().floor() as i32,
        );
        f64::from(world.block_state(block_below).block().friction())
            * aerodynamics.get_horizontal_air_resistance()
    } else {
        aerodynamics.get_horizontal_air_resistance()
    };
    let gravity = if is_flying {
        0.0
    } else {
        aerodynamics.get_gravity()
    };
    let vertical_resistance = if is_flying {
        0.6
    } else {
        aerodynamics.get_vertical_air_resistance()
    };
    let velocity = current_velocity.0;
    Velocity(Vector3d {
        x: clamp_velocity(velocity.x * drag),
        y: clamp_velocity(if has_no_gravity {
            velocity.y
        } else {
            (velocity.y - gravity) * vertical_resistance
        }),
        z: clamp_velocity(velocity.z * drag),
    })
}

fn offset_position(position: EntityPosition, delta: Vector3d) -> EntityPosition {
    position.get_offset(delta.x, delta.y, delta.z)
}

fn add_vector(left: Vector3d, right: Vector3d) -> Vector3d {
    Vector3d {
        x: left.x + right.x,
        y: left.y + right.y,
        z: left.z + right.z,
    }
}

fn subtract_vector(left: Vector3d, right: Vector3d) -> Vector3d {
    Vector3d {
        x: left.x - right.x,
        y: left.y - right.y,
        z: left.z - right.z,
    }
}

fn multiply_vector(vector: Vector3d, multiplier: f64) -> Vector3d {
    Vector3d {
        x: vector.x * multiplier,
        y: vector.y * multiplier,
        z: vector.z * multiplier,
    }
}

fn minestom_resolved_delta(delta: Vector3d, ratio: f64) -> Vector3d {
    let resolved_delta = multiply_vector(delta, ratio);
    Vector3d {
        x: clamp_velocity(resolved_delta.x),
        y: clamp_velocity(resolved_delta.y),
        z: clamp_velocity(resolved_delta.z),
    }
}

fn minestom_collision_offsets(offsets: Vector3d, direction: Vector3d) -> Vector3d {
    Vector3d {
        x: minestom_collision_offset(offsets.x, direction.x),
        y: minestom_collision_offset(offsets.y, direction.y),
        z: minestom_collision_offset(offsets.z, direction.z),
    }
}

fn minestom_collision_offset(offset: f64, direction: f64) -> f64 {
    if direction != 0.0 && (offset / direction).abs() < VELOCITY_EPSILON {
        return 0.0;
    }
    offset
}

fn minestom_swept_delta(delta: Vector3d) -> Vector3d {
    Vector3d {
        x: minestom_swept_component(delta.x),
        y: minestom_swept_component(delta.y),
        z: minestom_swept_component(delta.z),
    }
}

fn minestom_swept_component(component: f64) -> f64 {
    let swept_component = component * COLLISION_RATIO_LIMIT;
    if swept_component.abs() < VELOCITY_EPSILON {
        return 0.0;
    }
    swept_component
}

fn zero_vector() -> Vector3d {
    Vector3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

fn vector_is_zero(vector: Vector3d) -> bool {
    vector.x == 0.0 && vector.y == 0.0 && vector.z == 0.0
}

fn clamp_velocity(velocity: f64) -> f64 {
    if velocity.abs() < VELOCITY_EPSILON {
        return 0.0;
    }
    velocity
}
