use crate::entity::pathfinding::{
    GroundNodeFollower, GroundNodeGenerator, NodeFollower, NodeFollowerPhysicsTiming,
    NodeGenerator, Path, PathGenerator, PathNode, PathNodeType, PathRequest, PathState,
    SetPathToError,
};
use crate::entity::{EntityPosition, GenericEntity};
use crate::world::{ChunkPosition, WorldSnapshot};
use spinel_registry::EntityBoundingBox;

const DEFAULT_MAXIMUM_DISTANCE: f64 = 50.0;
const DEFAULT_VARIANCE: f64 = 20.0;
const RESET_MINIMUM_DISTANCE: f64 = 0.0;

pub struct Navigator {
    path: Option<Path>,
    pending_path: Option<Path>,
    node_generator: Box<dyn NodeGenerator>,
    node_follower: Box<dyn NodeFollower>,
    goal: Option<EntityPosition>,
    pending_goal: Option<EntityPosition>,
    minimum_distance: f64,
}

impl Default for Navigator {
    fn default() -> Self {
        Self {
            path: None,
            pending_path: None,
            node_generator: Box::new(GroundNodeGenerator),
            node_follower: Box::new(GroundNodeFollower),
            goal: None,
            pending_goal: None,
            minimum_distance: RESET_MINIMUM_DISTANCE,
        }
    }
}

impl Navigator {
    pub fn set_node_generator(&mut self, node_generator: impl NodeGenerator + 'static) {
        self.node_generator = Box::new(node_generator);
    }

    pub fn set_node_follower(&mut self, node_follower: impl NodeFollower + 'static) {
        self.node_follower = Box::new(node_follower);
    }

    pub fn get_physics_timing(&self) -> NodeFollowerPhysicsTiming {
        self.node_follower.physics_timing()
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_ref()
    }

    pub fn get_path_mut(&mut self) -> Option<&mut Path> {
        self.path.as_mut()
    }

    pub fn get_nodes(&self) -> Option<&[PathNode]> {
        self.path.as_ref().map(Path::get_nodes)
    }

    pub fn get_nodes_mut(&mut self) -> Option<&mut Vec<PathNode>> {
        self.path.as_mut().map(Path::get_nodes_mut)
    }

    pub const fn get_path_position(&self) -> Option<EntityPosition> {
        self.goal
    }

    pub fn goal_position(&self) -> Option<EntityPosition> {
        self.goal
    }

    pub fn state(&self) -> PathState {
        self.path
            .as_ref()
            .map(Path::get_state)
            .unwrap_or(PathState::Invalid)
    }

    pub fn reset(&mut self) {
        if let Some(path) = self.path.as_mut() {
            path.set_state(PathState::Invalid);
        }
        self.path = None;
        self.pending_path = None;
        self.goal = None;
        self.pending_goal = None;
        self.minimum_distance = RESET_MINIMUM_DISTANCE;
    }

    pub fn is_complete(&self, position: EntityPosition) -> bool {
        if self.path.is_none() {
            return true;
        }
        let Some(goal) = self.goal else {
            return true;
        };
        same_block(position, goal)
    }

    pub fn set_path_to(
        &mut self,
        world: &WorldSnapshot,
        start: EntityPosition,
        bounding_box: EntityBoundingBox,
        is_on_ground: bool,
        mut request: PathRequest,
    ) -> Result<bool, SetPathToError> {
        let Some(destination) = request.get_destination() else {
            self.reset();
            return Ok(false);
        };
        if !world
            .get_world_border()
            .contains(destination.get_x(), destination.get_z())
        {
            return Err(SetPathToError::TargetOutsideWorldBorder {
                target: destination,
            });
        }
        if !world.is_chunk_loaded(ChunkPosition::from(destination)) {
            return Err(SetPathToError::TargetChunkUnloaded {
                target: destination,
            });
        }

        let minimum_distance = request
            .get_minimum_distance()
            .unwrap_or_else(|| default_minimum_distance(bounding_box));
        if start.get_distance_squared(destination) <= minimum_distance * minimum_distance
            || same_block(start, destination)
        {
            if let Some(on_complete) = request.take_on_complete() {
                on_complete();
            }
            self.reset();
            return Ok(false);
        }

        let path = PathGenerator::generate(
            world,
            start,
            destination,
            bounding_box,
            is_on_ground,
            minimum_distance,
            request
                .get_maximum_distance()
                .unwrap_or(DEFAULT_MAXIMUM_DISTANCE),
            request.get_variance().unwrap_or(DEFAULT_VARIANCE),
            self.node_generator.as_ref(),
            request.take_on_complete(),
        );
        self.minimum_distance = minimum_distance;
        if self.path.is_some() {
            self.pending_path = Some(path);
            self.pending_goal = Some(destination);
        } else {
            self.path = Some(path);
            self.goal = Some(destination);
        }
        Ok(true)
    }

    pub fn tick(
        &mut self,
        entity: &mut GenericEntity,
        world: &WorldSnapshot,
        entity_is_dead: bool,
    ) {
        if entity_is_dead {
            return;
        }
        self.promote_pending_path_if_possible();

        {
            let Some(path) = self.path.as_mut() else {
                return;
            };
            match path.get_state() {
                PathState::Computed | PathState::BestEffort => {
                    trim_nodes_before_current_block(path, entity.get_position());
                    path.set_state(PathState::Following);
                }
                PathState::Following => {}
                PathState::Terminating => return,
                PathState::Invalid | PathState::Terminated | PathState::Calculating => return,
            }
        }
        if self.entity_reached_goal(entity.get_position()) {
            if let Some(path) = self.path.as_mut() {
                path.complete();
            }
            self.path = None;
            self.node_follower.stop_following_path(entity);
            return;
        }
        let Some(path) = self.path.as_mut() else {
            return;
        };

        if self
            .node_follower
            .should_advance_reached_node_before_moving()
        {
            let Some(target) = path.get_current() else {
                path.set_state(PathState::Invalid);
                return;
            };
            if self.node_follower.is_at_point(entity, target) {
                path.advance();
            }
        }

        if path.get_current().is_none() || path.get_current_type() == Some(PathNodeType::Repath) {
            self.regenerate_path(world, entity);
            return;
        }

        let Some(target) = path.get_current() else {
            path.set_state(PathState::Invalid);
            return;
        };
        let Some(look_at) = path.get_next_position() else {
            path.set_state(PathState::Invalid);
            return;
        };
        let target_type = path.get_current_type();
        let speed = self.node_follower.movement_speed(entity);
        self.node_follower
            .move_towards(entity, world, target, speed, look_at);
        if self.node_follower.is_at_point(entity, target) {
            path.advance();
            return;
        }
        if target_type == Some(PathNodeType::Jump)
            && self.node_follower.should_execute_path_node_jump()
        {
            self.node_follower.jump(entity, Some(target), Some(look_at));
        }
    }

    fn entity_reached_goal(&self, position: EntityPosition) -> bool {
        self.goal.is_some_and(|goal| {
            position.get_distance_squared(goal) < self.minimum_distance * self.minimum_distance
        })
    }

    fn promote_pending_path_if_possible(&mut self) {
        if self.state() == PathState::Terminating {
            return;
        }
        if let Some(pending_path) = self.pending_path.take() {
            self.path = Some(pending_path);
            self.goal = self.pending_goal.take();
        }
    }

    fn regenerate_path(&mut self, world: &WorldSnapshot, entity: &GenericEntity) {
        let Some(goal) = self.goal else {
            self.path = None;
            return;
        };
        let Some(path) = self.path.as_ref() else {
            return;
        };
        let maximum_distance = path.get_maximum_distance();
        let variance = path.get_variance();
        self.path = Some(PathGenerator::generate(
            world,
            entity.get_position(),
            goal,
            entity.get_bounding_box(),
            entity.is_on_ground(),
            self.minimum_distance,
            maximum_distance,
            variance,
            self.node_generator.as_ref(),
            None,
        ));
    }
}

fn same_block(first: EntityPosition, second: EntityPosition) -> bool {
    first.get_x().floor() == second.get_x().floor()
        && first.get_y().floor() == second.get_y().floor()
        && first.get_z().floor() == second.get_z().floor()
}

fn default_minimum_distance(bounding_box: EntityBoundingBox) -> f64 {
    let width = bounding_box.get_width();
    let depth = bounding_box.depth();
    ((width * width) + (depth * depth)).sqrt() / 2.0
}

fn trim_nodes_before_current_block(path: &mut Path, position: EntityPosition) {
    let Some(index) = path
        .get_nodes()
        .iter()
        .position(|node| same_block(node.get_position(), position))
    else {
        return;
    };
    path.remove_nodes_before(index);
}
