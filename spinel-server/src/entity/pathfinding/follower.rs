use crate::entity::physics::simulate_collision;
use crate::entity::{EntityPosition, GenericEntity};
use crate::world::{BlockPosition, WorldSnapshot};
use spinel_network::types::math::Vector3d;
use spinel_network::types::velocity::Velocity;
use spinel_registry::registry_values::attribute::Attribute;

const SERVER_TICKS_PER_SECOND: f64 = 20.0;

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
}

#[derive(Default)]
pub struct GroundNodeFollower;

#[derive(Default)]
pub struct FlyingNodeFollower;

#[derive(Default)]
pub struct WaterNodeFollower;

#[derive(Default)]
pub struct NoPhysicsNodeFollower;

impl NodeFollower for GroundNodeFollower {
    fn move_towards(
        &self,
        entity: &mut GenericEntity,
        world: &WorldSnapshot,
        target: EntityPosition,
        speed: f64,
        look_at: EntityPosition,
    ) {
        accelerate_ground_movement(entity, world, target, speed, look_at);
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
        set_jump_velocity(entity);
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
            ViewDirection::Target,
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
            ViewDirection::Target,
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
            set_jump_velocity(entity);
        }
        let movement = movement_towards(
            current,
            target,
            speed,
            look_at,
            VerticalMovement::None,
            ViewDirection::Horizontal,
        );
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
        set_jump_velocity(entity);
    }
}

#[derive(Clone, Copy)]
enum VerticalMovement {
    None,
    TowardsTarget,
}

#[derive(Clone, Copy)]
enum ViewDirection {
    Horizontal,
    Target,
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
    view_direction: ViewDirection,
) {
    let current = entity.position();
    let movement = movement_towards(
        current,
        target,
        speed,
        look_at,
        vertical_movement,
        view_direction,
    );
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

fn accelerate_ground_movement(
    entity: &mut GenericEntity,
    world: &WorldSnapshot,
    target: EntityPosition,
    speed: f64,
    look_at: EntityPosition,
) {
    let current = entity.position();
    let delta_x = target.x() - current.x();
    let delta_z = target.z() - current.z();
    let radians = delta_z.atan2(delta_x);
    let block_below = BlockPosition::new(
        current.x().floor() as i32,
        (current.y() - 0.5000001).floor() as i32,
        current.z().floor() as i32,
    );
    let slipperiness = f64::from(world.block(block_below).friction());
    let acceleration_per_tick = if entity.is_on_ground() {
        speed * speed * 0.21600002 / slipperiness.powi(3)
    } else {
        speed * 0.02
    };
    let current_velocity = entity.velocity();
    entity.set_velocity(Velocity(Vector3d {
        x: current_velocity.0.x + radians.cos() * acceleration_per_tick * SERVER_TICKS_PER_SECOND,
        y: current_velocity.0.y,
        z: current_velocity.0.z + radians.sin() * acceleration_per_tick * SERVER_TICKS_PER_SECOND,
    }));
    entity.set_position(current.with_view(
        look_yaw(look_at.x() - current.x(), look_at.z() - current.z()),
        0.0,
    ));
}

fn movement_towards(
    current: EntityPosition,
    target: EntityPosition,
    speed: f64,
    look_at: EntityPosition,
    vertical_movement: VerticalMovement,
    view_direction: ViewDirection,
) -> FollowerMovement {
    let delta_x = target.x() - current.x();
    let delta_y = target.y() - current.y();
    let delta_z = target.z() - current.z();
    let distance_squared = delta_x * delta_x + delta_y * delta_y + delta_z * delta_z;
    let speed = speed.min(distance_squared);
    let radians = delta_z.atan2(delta_x);
    let yaw = look_yaw(look_at.x() - current.x(), look_at.z() - current.z());
    let pitch = match view_direction {
        ViewDirection::Horizontal => 0.0,
        ViewDirection::Target => look_pitch(
            look_at.x() - current.x(),
            look_at.y() - current.y(),
            look_at.z() - current.z(),
        ),
    };
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

fn set_jump_velocity(entity: &mut GenericEntity) {
    let jump_strength = entity.attribute_value(
        Attribute::JUMP_STRENGTH.protocol_id(),
        Attribute::JUMP_STRENGTH.default_value(),
    );
    let current_velocity = entity.velocity();
    entity.set_velocity(Velocity(Vector3d {
        x: current_velocity.0.x,
        y: jump_strength * SERVER_TICKS_PER_SECOND,
        z: current_velocity.0.z,
    }));
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
