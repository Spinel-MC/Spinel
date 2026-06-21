use crate::entity::EntityPosition;
use crate::world::{BlockPosition, ChunkPosition, WorldSnapshot};
use spinel_network::types::math::Vector3d;
use spinel_registry::EntityBoundingBox;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PerfectControlState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sprint: bool,
    pub sneak: bool,
    pub yaw_delta: f32,
    pub pitch_delta: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PerfectMotionState {
    pub position: EntityPosition,
    pub velocity: Vector3d,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
    pub horizontal_collision: bool,
    pub tick: u32,
    pub control_state: PerfectControlState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PerfectMotionSimulator {
    movement_speed: f64,
    sprint_multiplier: f64,
    sneak_multiplier: f64,
    ground_acceleration: f64,
    air_acceleration: f64,
    gravity: f64,
    horizontal_drag: f64,
    vertical_drag: f64,
    terminal_velocity: f64,
    jump_velocity: f64,
}

impl Default for PerfectMotionSimulator {
    fn default() -> Self {
        Self {
            movement_speed: 0.1,
            sprint_multiplier: 1.3,
            sneak_multiplier: 0.3,
            ground_acceleration: 1.0,
            air_acceleration: 0.2,
            gravity: 0.08,
            horizontal_drag: 0.91,
            vertical_drag: 0.98,
            terminal_velocity: -3.92,
            jump_velocity: 0.42,
        }
    }
}

impl PerfectMotionState {
    pub fn new(position: EntityPosition) -> Self {
        Self {
            position,
            velocity: Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            yaw: position.get_yaw(),
            pitch: position.get_pitch(),
            on_ground: false,
            horizontal_collision: false,
            tick: 0,
            control_state: PerfectControlState::default(),
        }
    }
}

impl PerfectMotionSimulator {
    pub fn tick(
        self,
        world: &WorldSnapshot,
        bounding_box: EntityBoundingBox,
        state: PerfectMotionState,
        control: PerfectControlState,
    ) -> PerfectMotionState {
        let yaw = state.yaw + control.yaw_delta;
        let pitch = (state.pitch + control.pitch_delta).clamp(-90.0, 90.0);
        let velocity = self.accelerated_velocity(state, control, yaw);
        let moved = self.move_with_collision(world, bounding_box, state.position, velocity);
        PerfectMotionState {
            position: moved.position.with_view(yaw, pitch),
            velocity: moved.velocity,
            yaw,
            pitch,
            on_ground: moved.on_ground,
            horizontal_collision: moved.horizontal_collision,
            tick: state.tick + 1,
            control_state: control,
        }
    }

    fn accelerated_velocity(
        self,
        state: PerfectMotionState,
        control: PerfectControlState,
        yaw: f32,
    ) -> Vector3d {
        let mut velocity = state.velocity;
        if control.jump && state.on_ground {
            velocity.y = self.jump_velocity;
            if control.sprint {
                let radians = (yaw as f64).to_radians();
                velocity.x -= radians.sin() * 0.2;
                velocity.z += radians.cos() * 0.2;
            }
        }
        let movement = control_movement(control);
        let acceleration = if state.on_ground {
            self.ground_acceleration
        } else {
            self.air_acceleration
        };
        let speed = self.control_speed(control) * acceleration;
        let rotated = rotated_input(movement, yaw, speed);
        velocity.x += rotated.x;
        velocity.z += rotated.z;
        velocity.y = (velocity.y - self.gravity).max(self.terminal_velocity);
        velocity.x *= self.horizontal_drag;
        velocity.z *= self.horizontal_drag;
        velocity.y *= self.vertical_drag;
        velocity
    }

    fn control_speed(self, control: PerfectControlState) -> f64 {
        if control.sneak {
            return self.movement_speed * self.sneak_multiplier;
        }
        if control.sprint {
            return self.movement_speed * self.sprint_multiplier;
        }
        self.movement_speed
    }

    fn move_with_collision(
        self,
        world: &WorldSnapshot,
        bounding_box: EntityBoundingBox,
        position: EntityPosition,
        velocity: Vector3d,
    ) -> MoveResult {
        let after_x = position.get_offset(velocity.x, 0.0, 0.0);
        let x_blocked = !position_is_simulatable(world, after_x, bounding_box);
        let position = if x_blocked { position } else { after_x };
        let after_z = position.get_offset(0.0, 0.0, velocity.z);
        let z_blocked = !position_is_simulatable(world, after_z, bounding_box);
        let position = if z_blocked { position } else { after_z };
        let after_y = position.get_offset(0.0, velocity.y, 0.0);
        let y_blocked = !position_is_simulatable(world, after_y, bounding_box);
        let on_ground = y_blocked && velocity.y < 0.0;
        let position = if y_blocked { position } else { after_y };
        MoveResult {
            position,
            velocity: Vector3d {
                x: if x_blocked { 0.0 } else { velocity.x },
                y: if y_blocked { 0.0 } else { velocity.y },
                z: if z_blocked { 0.0 } else { velocity.z },
            },
            on_ground,
            horizontal_collision: x_blocked || z_blocked,
        }
    }
}

struct MoveResult {
    position: EntityPosition,
    velocity: Vector3d,
    on_ground: bool,
    horizontal_collision: bool,
}

fn control_movement(control: PerfectControlState) -> Vector3d {
    let forward: f64 = if control.forward { 1.0 } else { 0.0 };
    let backward: f64 = if control.backward { 1.0 } else { 0.0 };
    let left: f64 = if control.left { 1.0 } else { 0.0 };
    let right: f64 = if control.right { 1.0 } else { 0.0 };
    let strafe = left - right;
    let forward = forward - backward;
    let length = (strafe * strafe + forward * forward).sqrt();
    if length == 0.0 {
        return Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    Vector3d {
        x: strafe / length,
        y: 0.0,
        z: forward / length,
    }
}

fn rotated_input(input: Vector3d, yaw: f32, speed: f64) -> Vector3d {
    let radians = (yaw as f64).to_radians();
    let sin = radians.sin();
    let cos = radians.cos();
    Vector3d {
        x: (input.x * cos - input.z * sin) * speed,
        y: 0.0,
        z: (input.z * cos + input.x * sin) * speed,
    }
}

pub(crate) fn position_is_simulatable(
    world: &WorldSnapshot,
    position: EntityPosition,
    bounding_box: EntityBoundingBox,
) -> bool {
    if !world.get_world_border().contains(position.get_x(), position.get_z()) {
        return false;
    }
    let minimum_x = (position.get_x() + bounding_box.minimum_x()).floor() as i32;
    let maximum_x = (position.get_x() + bounding_box.maximum_x()).floor() as i32;
    let minimum_y = (position.get_y() + bounding_box.minimum_y()).floor() as i32;
    let maximum_y = (position.get_y() + bounding_box.maximum_y()).ceil() as i32 - 1;
    let minimum_z = (position.get_z() + bounding_box.minimum_z()).floor() as i32;
    let maximum_z = (position.get_z() + bounding_box.maximum_z()).floor() as i32;
    let dimension = world.cached_dimension_type();
    let maximum_dimension_y = dimension.min_y + dimension.height - 1;
    if minimum_y < dimension.min_y || maximum_y > maximum_dimension_y {
        return false;
    }
    !(minimum_x..=maximum_x).any(|x| {
        (minimum_y..=maximum_y).any(|y| {
            (minimum_z..=maximum_z).any(|z| {
                let block_position = BlockPosition::new(x, y, z);
                !world.is_chunk_loaded(ChunkPosition::from(block_position))
                    || world.block(block_position).is_solid()
            })
        })
    })
}
