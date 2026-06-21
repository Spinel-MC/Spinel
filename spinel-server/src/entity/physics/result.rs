use crate::entity::EntityPosition;
use crate::world::BlockPosition;
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::BlockShapeBox;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityPhysicsResult {
    new_position: EntityPosition,
    new_velocity_per_tick: Velocity,
    is_on_ground: bool,
    collision_x: bool,
    collision_y: bool,
    collision_z: bool,
    original_delta: Velocity,
    collision_points: [Option<Vector3d>; 3],
    collision_shapes: [Option<&'static [BlockShapeBox]>; 3],
    collision_shape_positions: [Option<BlockPosition>; 3],
    has_collision: bool,
    sweep: EntitySweepResult,
    cached: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntitySweepResult {
    ratio: f64,
    normal: Vector3d,
    collision_point: Option<Vector3d>,
    collision_shape: Option<&'static [BlockShapeBox]>,
    collision_shape_position: Option<BlockPosition>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityBoundingBoxSweepResult {
    ratio: f64,
    normal: Vector3d,
    collision_point: Option<Vector3d>,
    collision_shape: Option<spinel_registry::EntityBoundingBox>,
    collision_shape_position: Option<Vector3d>,
}

impl EntityPhysicsResult {
    pub const fn new(
        new_position: EntityPosition,
        new_velocity_per_tick: Velocity,
        is_on_ground: bool,
        collision_x: bool,
        collision_y: bool,
        collision_z: bool,
        original_delta: Velocity,
        collision_points: [Option<Vector3d>; 3],
        collision_shapes: [Option<&'static [BlockShapeBox]>; 3],
        collision_shape_positions: [Option<BlockPosition>; 3],
        has_collision: bool,
        sweep: EntitySweepResult,
        cached: bool,
    ) -> Self {
        Self {
            new_position,
            new_velocity_per_tick,
            is_on_ground,
            collision_x,
            collision_y,
            collision_z,
            original_delta,
            collision_points,
            collision_shapes,
            collision_shape_positions,
            has_collision,
            sweep,
            cached,
        }
    }

    pub const fn without_collision(
        new_position: EntityPosition,
        new_velocity_per_tick: Velocity,
        original_delta: Velocity,
    ) -> Self {
        Self::new(
            new_position,
            new_velocity_per_tick,
            false,
            false,
            false,
            false,
            original_delta,
            [None; 3],
            [None; 3],
            [None; 3],
            false,
            EntitySweepResult::no_collision(),
            false,
        )
    }

    pub const fn get_new_position(self) -> EntityPosition {
        self.new_position
    }

    pub const fn get_new_velocity_per_tick(self) -> Velocity {
        self.new_velocity_per_tick
    }

    pub const fn is_on_ground(self) -> bool {
        self.is_on_ground
    }

    pub const fn has_collision_x(self) -> bool {
        self.collision_x
    }

    pub const fn has_collision_y(self) -> bool {
        self.collision_y
    }

    pub const fn has_collision_z(self) -> bool {
        self.collision_z
    }

    pub const fn get_original_delta(self) -> Velocity {
        self.original_delta
    }

    pub const fn get_collision_points(self) -> [Option<Vector3d>; 3] {
        self.collision_points
    }

    pub const fn get_collision_shapes(self) -> [Option<&'static [BlockShapeBox]>; 3] {
        self.collision_shapes
    }

    pub const fn get_collision_shape_positions(self) -> [Option<BlockPosition>; 3] {
        self.collision_shape_positions
    }

    pub const fn has_collision(self) -> bool {
        self.has_collision
    }

    pub const fn get_sweep(self) -> EntitySweepResult {
        self.sweep
    }

    pub const fn is_cached(self) -> bool {
        self.cached
    }

    pub const fn with_movement(
        self,
        new_position: EntityPosition,
        new_velocity_per_tick: Velocity,
        cached: bool,
    ) -> Self {
        Self {
            new_position,
            new_velocity_per_tick,
            cached,
            ..self
        }
    }

    pub const fn as_cached(self) -> Self {
        Self {
            cached: true,
            ..self
        }
    }
}

impl EntitySweepResult {
    pub const fn new(
        ratio: f64,
        normal: Vector3d,
        collision_point: Option<Vector3d>,
        collision_shape: Option<&'static [BlockShapeBox]>,
        collision_shape_position: Option<BlockPosition>,
    ) -> Self {
        Self {
            ratio,
            normal,
            collision_point,
            collision_shape,
            collision_shape_position,
        }
    }

    pub const fn no_collision() -> Self {
        Self::new(
            f64::MAX,
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            None,
            None,
            None,
        )
    }

    pub const fn get_ratio(self) -> f64 {
        self.ratio
    }

    pub const fn get_normal(self) -> Vector3d {
        self.normal
    }

    pub const fn get_collision_point(self) -> Option<Vector3d> {
        self.collision_point
    }

    pub const fn get_collision_shape(self) -> Option<&'static [BlockShapeBox]> {
        self.collision_shape
    }

    pub const fn get_collision_shape_position(self) -> Option<BlockPosition> {
        self.collision_shape_position
    }
}

impl EntityBoundingBoxSweepResult {
    pub const fn no_collision() -> Self {
        Self {
            ratio: f64::MAX,
            normal: Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            collision_point: None,
            collision_shape: None,
            collision_shape_position: None,
        }
    }

    pub const fn get_ratio(self) -> f64 {
        self.ratio
    }

    pub const fn get_normal(self) -> Vector3d {
        self.normal
    }

    pub const fn get_collision_point(self) -> Option<Vector3d> {
        self.collision_point
    }

    pub const fn get_collision_shape(self) -> Option<spinel_registry::EntityBoundingBox> {
        self.collision_shape
    }

    pub const fn get_collision_shape_position(self) -> Option<Vector3d> {
        self.collision_shape_position
    }

    pub(crate) fn set_collision(
        &mut self,
        ratio: f64,
        normal: Vector3d,
        collision_point: Vector3d,
        collision_shape: spinel_registry::EntityBoundingBox,
        collision_shape_position: Vector3d,
    ) {
        self.ratio = ratio;
        self.normal = normal;
        self.collision_point = Some(collision_point);
        self.collision_shape = Some(collision_shape);
        self.collision_shape_position = Some(collision_shape_position);
    }
}
