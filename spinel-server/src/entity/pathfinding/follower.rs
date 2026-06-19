use crate::entity::physics::simulate_collision;
use crate::entity::{EntityPosition, GenericEntity};
use crate::world::{BlockPosition, WorldSnapshot};
use spinel_network::types::math::Vector3d;
use spinel_network::types::velocity::Velocity;
use spinel_registry::Identifier;
use spinel_registry::registry_values::attribute::Attribute;
use std::cell::Cell;
use std::sync::LazyLock;

const DEFAULT_JUMP_HEIGHT: f32 = 4.0;
const JUMP_HEIGHT_VELOCITY_MULTIPLIER: f64 = 2.5;
const VANILLA_REACHED_DESTINATION_DISTANCE_SQUARED: f64 = 2.5000003E-7;
const VANILLA_MAXIMUM_YAW_CHANGE: f32 = 90.0;
const VANILLA_GROUND_ACCELERATION: f64 = 0.21600002;
const VANILLA_AIR_ACCELERATION: f64 = 0.02;
const SERVER_TICKS_PER_SECOND: f64 = 20.0;
static VANILLA_DOORS_TAG: LazyLock<Identifier> = LazyLock::new(|| Identifier::minecraft("doors"));
static VANILLA_FENCES_TAG: LazyLock<Identifier> = LazyLock::new(|| Identifier::minecraft("fences"));

pub trait NodeFollower: Send {
    fn move_towards(
        &self,
        entity: &mut GenericEntity,
        world: &WorldSnapshot,
        target: EntityPosition,
        speed: f64,
        look_at: EntityPosition,
    );

    fn jump(
        &self,
        entity: &mut GenericEntity,
        point: Option<EntityPosition>,
        target: Option<EntityPosition>,
    );

    fn is_at_point(&self, entity: &GenericEntity, point: EntityPosition) -> bool {
        same_block(entity.position(), point)
    }

    fn movement_speed(&self, entity: &GenericEntity) -> f64 {
        if !entity.entity_type().is_living() {
            return 0.1;
        }
        entity.attribute_value(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        )
    }

    fn physics_timing(&self) -> NodeFollowerPhysicsTiming {
        NodeFollowerPhysicsTiming::AfterPhysics
    }

    fn stop_following_path(&self, _entity: &mut GenericEntity) {}

    fn should_advance_reached_node_before_moving(&self) -> bool {
        false
    }

    fn should_execute_path_node_jump(&self) -> bool {
        true
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NodeFollowerPhysicsTiming {
    BeforePhysics,
    AfterPhysics,
}

#[derive(Default)]
pub struct GroundNodeFollower;

#[derive(Default)]
pub struct FlyingNodeFollower;

#[derive(Default)]
pub struct WaterNodeFollower;

#[derive(Default)]
pub struct NoPhysicsNodeFollower;

pub struct VanillaGroundNodeFollower {
    move_control_state: Cell<VanillaMoveControlState>,
}

impl Default for VanillaGroundNodeFollower {
    fn default() -> Self {
        Self {
            move_control_state: Cell::new(VanillaMoveControlState::Wait),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VanillaMoveControlState {
    Wait,
    Jumping,
}

impl GroundNodeFollower {
    pub fn jump_with_height(&self, entity: &mut GenericEntity, height: f32) {
        set_jump_velocity(entity, height);
    }
}

impl NoPhysicsNodeFollower {
    pub fn jump_with_height(&self, entity: &mut GenericEntity, height: f32) {
        set_jump_velocity(entity, height);
    }
}

impl NodeFollower for GroundNodeFollower {
    fn move_towards(
        &self,
        entity: &mut GenericEntity,
        world: &WorldSnapshot,
        target: EntityPosition,
        speed: f64,
        look_at: EntityPosition,
    ) {
        move_with_physics(
            entity,
            world,
            target,
            speed,
            look_at,
            VerticalMovement::None,
        );
    }

    fn jump(
        &self,
        entity: &mut GenericEntity,
        _point: Option<EntityPosition>,
        _target: Option<EntityPosition>,
    ) {
        if !entity.is_on_ground() {
            return;
        }
        self.jump_with_height(entity, DEFAULT_JUMP_HEIGHT);
    }
}

impl NodeFollower for FlyingNodeFollower {
    fn move_towards(
        &self,
        entity: &mut GenericEntity,
        world: &WorldSnapshot,
        target: EntityPosition,
        speed: f64,
        look_at: EntityPosition,
    ) {
        move_with_physics(
            entity,
            world,
            target,
            speed,
            look_at,
            VerticalMovement::TowardsTarget,
        );
    }

    fn jump(
        &self,
        _entity: &mut GenericEntity,
        _point: Option<EntityPosition>,
        _target: Option<EntityPosition>,
    ) {
    }
}

impl NodeFollower for WaterNodeFollower {
    fn move_towards(
        &self,
        entity: &mut GenericEntity,
        world: &WorldSnapshot,
        target: EntityPosition,
        speed: f64,
        look_at: EntityPosition,
    ) {
        let position = entity.position();
        let current_block = world.block(BlockPosition::new(
            position.x().floor() as i32,
            position.y().floor() as i32,
            position.z().floor() as i32,
        ));
        let speed = if current_block.is_liquid() {
            speed * 0.5
        } else {
            speed
        };
        move_with_physics(
            entity,
            world,
            target,
            speed,
            look_at,
            VerticalMovement::TowardsTarget,
        );
    }

    fn jump(
        &self,
        _entity: &mut GenericEntity,
        _point: Option<EntityPosition>,
        _target: Option<EntityPosition>,
    ) {
    }
}

impl NodeFollower for NoPhysicsNodeFollower {
    fn move_towards(
        &self,
        entity: &mut GenericEntity,
        _world: &WorldSnapshot,
        target: EntityPosition,
        speed: f64,
        look_at: EntityPosition,
    ) {
        let current = entity.position();
        if target.y() > current.y() && entity.is_on_ground() {
            self.jump_with_height(entity, DEFAULT_JUMP_HEIGHT);
        }
        let movement = movement_towards(current, target, speed, look_at, VerticalMovement::None);
        entity.set_position(
            current
                .offset(movement.velocity.0.x, 0.0, movement.velocity.0.z)
                .with_view(movement.yaw, movement.pitch),
        );
    }

    fn jump(
        &self,
        entity: &mut GenericEntity,
        _point: Option<EntityPosition>,
        _target: Option<EntityPosition>,
    ) {
        if !entity.is_on_ground() {
            return;
        }
        self.jump_with_height(entity, DEFAULT_JUMP_HEIGHT);
    }
}

impl NodeFollower for VanillaGroundNodeFollower {
    fn move_towards(
        &self,
        entity: &mut GenericEntity,
        world: &WorldSnapshot,
        target: EntityPosition,
        speed: f64,
        _look_at: EntityPosition,
    ) {
        let current = entity.position();
        if self.move_control_state.get() == VanillaMoveControlState::Jumping {
            apply_vanilla_acceleration(entity, world, speed, current.yaw());
            if entity.is_on_ground() {
                self.move_control_state.set(VanillaMoveControlState::Wait);
            }
            return;
        }
        let delta_x = target.x() - current.x();
        let delta_y = target.y() - current.y();
        let delta_z = target.z() - current.z();
        let distance_squared = delta_x * delta_x + delta_y * delta_y + delta_z * delta_z;
        if distance_squared < VANILLA_REACHED_DESTINATION_DISTANCE_SQUARED {
            return;
        }

        let target_yaw = (delta_z.atan2(delta_x).to_degrees() - 90.0) as f32;
        let yaw = vanilla_wrapped_yaw(current.yaw(), target_yaw, VANILLA_MAXIMUM_YAW_CHANGE);
        entity.set_position(
            EntityPosition::new(current.x(), current.y(), current.z(), yaw, current.pitch())
                .with_head_yaw(current.head_yaw()),
        );

        apply_vanilla_acceleration(entity, world, speed, yaw);

        let horizontal_distance_squared = delta_x * delta_x + delta_z * delta_z;
        let step_height = entity.attribute_value(
            Attribute::STEP_HEIGHT.protocol_id(),
            Attribute::STEP_HEIGHT.default_value(),
        );
        let is_close_enough_to_jump =
            horizontal_distance_squared < entity.bounding_box().width().max(1.0);
        let should_jump_to_target = delta_y > step_height && is_close_enough_to_jump;
        let should_jump_out_of_collision_shape =
            vanilla_collision_shape_requires_jump(entity, world);
        if should_jump_to_target || should_jump_out_of_collision_shape {
            vanilla_jump(entity);
            self.move_control_state
                .set(VanillaMoveControlState::Jumping);
        }
    }

    fn jump(
        &self,
        entity: &mut GenericEntity,
        _point: Option<EntityPosition>,
        _target: Option<EntityPosition>,
    ) {
        vanilla_jump(entity);
        self.move_control_state
            .set(VanillaMoveControlState::Jumping);
    }

    fn movement_speed(&self, _entity: &GenericEntity) -> f64 {
        1.0
    }

    fn physics_timing(&self) -> NodeFollowerPhysicsTiming {
        NodeFollowerPhysicsTiming::BeforePhysics
    }

    fn stop_following_path(&self, _entity: &mut GenericEntity) {
        self.move_control_state.set(VanillaMoveControlState::Wait);
    }

    fn should_advance_reached_node_before_moving(&self) -> bool {
        true
    }

    fn should_execute_path_node_jump(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy)]
enum VerticalMovement {
    None,
    TowardsTarget,
}

struct FollowerMovement {
    velocity: Velocity,
    yaw: f32,
    pitch: f32,
}

fn move_with_physics(
    entity: &mut GenericEntity,
    world: &WorldSnapshot,
    target: EntityPosition,
    speed: f64,
    look_at: EntityPosition,
    vertical_movement: VerticalMovement,
) {
    let current = entity.position();
    let movement = movement_towards(current, target, speed, look_at, vertical_movement);
    let collision = simulate_collision(
        current,
        movement.velocity,
        entity.bounding_box(),
        world,
        None,
    );
    entity.set_position(
        collision
            .new_position()
            .with_view(movement.yaw, movement.pitch),
    );
}

fn movement_towards(
    current: EntityPosition,
    target: EntityPosition,
    speed: f64,
    look_at: EntityPosition,
    vertical_movement: VerticalMovement,
) -> FollowerMovement {
    let delta_x = target.x() - current.x();
    let delta_y = target.y() - current.y();
    let delta_z = target.z() - current.z();
    let distance_squared = delta_x * delta_x + delta_y * delta_y + delta_z * delta_z;
    let speed = speed.min(distance_squared);
    let radians = delta_z.atan2(delta_x);
    let yaw = look_yaw(look_at.x() - current.x(), look_at.z() - current.z());
    let pitch = look_pitch(
        look_at.x() - current.x(),
        look_at.y() - current.y(),
        look_at.z() - current.z(),
    );
    let vertical_speed = match vertical_movement {
        VerticalMovement::None => 0.0,
        VerticalMovement::TowardsTarget => {
            let requested_vertical_speed = delta_y.signum() * 0.5 * speed;
            if requested_vertical_speed.abs().min(delta_y.abs()) == delta_y.abs() {
                delta_y
            } else {
                requested_vertical_speed
            }
        }
    };
    FollowerMovement {
        velocity: Velocity(Vector3d {
            x: radians.cos() * speed,
            y: vertical_speed,
            z: radians.sin() * speed,
        }),
        yaw,
        pitch,
    }
}

fn set_jump_velocity(entity: &mut GenericEntity, height: f32) {
    entity.set_velocity(Velocity(Vector3d {
        x: 0.0,
        y: f64::from(height) * JUMP_HEIGHT_VELOCITY_MULTIPLIER,
        z: 0.0,
    }));
}

fn vanilla_acceleration(entity: &GenericEntity, world: &WorldSnapshot, movement_speed: f64) -> f64 {
    if !entity.is_on_ground() {
        return VANILLA_AIR_ACCELERATION;
    }

    let position = entity.position();
    let block_below = BlockPosition::new(
        position.x().floor() as i32,
        (position.y() - 0.5000001).floor() as i32,
        position.z().floor() as i32,
    );
    let slipperiness = f64::from(world.block(block_below).friction());
    movement_speed * VANILLA_GROUND_ACCELERATION / slipperiness.powi(3)
}

fn apply_vanilla_acceleration(
    entity: &mut GenericEntity,
    world: &WorldSnapshot,
    speed: f64,
    yaw: f32,
) {
    let movement_speed = speed
        * entity.attribute_value(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        );
    let acceleration = vanilla_acceleration(entity, world, movement_speed);
    let movement_input = movement_speed.min(1.0);
    let yaw_radians = f64::from(yaw).to_radians();
    let acceleration_per_tick = acceleration * movement_input;
    let current_velocity = entity.velocity().0;
    entity.set_velocity(Velocity(Vector3d {
        x: current_velocity.x
            + -yaw_radians.sin() * acceleration_per_tick * SERVER_TICKS_PER_SECOND,
        y: current_velocity.y,
        z: current_velocity.z + yaw_radians.cos() * acceleration_per_tick * SERVER_TICKS_PER_SECOND,
    }));
}

fn vanilla_jump(entity: &mut GenericEntity) {
    if !entity.is_on_ground() {
        return;
    }

    let jump_strength = entity.attribute_value(
        Attribute::JUMP_STRENGTH.protocol_id(),
        Attribute::JUMP_STRENGTH.default_value(),
    );
    let current_velocity = entity.velocity().0;
    entity.set_velocity(Velocity(Vector3d {
        x: current_velocity.x,
        y: current_velocity
            .y
            .max(jump_strength * SERVER_TICKS_PER_SECOND),
        z: current_velocity.z,
    }));
}

fn vanilla_collision_shape_requires_jump(entity: &GenericEntity, world: &WorldSnapshot) -> bool {
    let position = entity.position();
    let block_position = BlockPosition::new(
        position.x().floor() as i32,
        position.y().floor() as i32,
        position.z().floor() as i32,
    );
    let block_state = world.block_state(block_position);
    let collision_shape = block_state.collision_shape();
    if collision_shape.is_empty() {
        return false;
    }
    let block = block_state.block();
    let is_door = block.is_in_vanilla_tag(&VANILLA_DOORS_TAG);
    let is_fence = block.is_in_vanilla_tag(&VANILLA_FENCES_TAG);
    if is_door || is_fence {
        return false;
    }
    let maximum_collision_y = collision_shape
        .iter()
        .map(|shape| shape.max_y)
        .fold(f64::NEG_INFINITY, f64::max);
    position.y() < f64::from(block_position.y) + maximum_collision_y
}

fn vanilla_wrapped_yaw(current: f32, target: f32, maximum_change: f32) -> f32 {
    let mut difference = (target - current) % 360.0;
    if difference >= 180.0 {
        difference -= 360.0;
    }
    if difference < -180.0 {
        difference += 360.0;
    }
    let mut yaw = current + difference.clamp(-maximum_change, maximum_change);
    if yaw < 0.0 {
        yaw += 360.0;
    } else if yaw > 360.0 {
        yaw -= 360.0;
    }
    yaw
}

fn same_block(left: EntityPosition, right: EntityPosition) -> bool {
    left.x().floor() == right.x().floor()
        && left.y().floor() == right.y().floor()
        && left.z().floor() == right.z().floor()
}

fn look_yaw(delta_x: f64, delta_z: f64) -> f32 {
    (delta_z.atan2(delta_x).to_degrees() - 90.0) as f32
}

fn look_pitch(delta_x: f64, delta_y: f64, delta_z: f64) -> f32 {
    (-delta_y
        .atan2((delta_x * delta_x + delta_z * delta_z).sqrt())
        .to_degrees()) as f32
}
