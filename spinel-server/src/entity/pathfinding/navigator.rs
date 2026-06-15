use crate::entity::EntityPosition;
use crate::entity::GenericEntity;
use crate::entity::pathfinding::{
    GroundNodeFollower, GroundNodeGenerator, NodeFollower, NodeFollowerPhysicsTiming,
    NodeGenerator, Path, PathGenerator, PathNode, PathNodeType, PathState,
};
use crate::world::{ChunkPosition, WorldSnapshot};
use spinel_registry::EntityBoundingBox;

pub struct Navigator {
    goal_position: Option<EntityPosition>,
    path: Option<Path>,
    computing_path: Option<Path>,
    minimum_distance: f64,
    node_generator: Box<dyn NodeGenerator>,
    node_follower: Box<dyn NodeFollower>,
}

impl Default for Navigator {
    fn default() -> Self {
        Self {
            goal_position: None,
            path: None,
            computing_path: None,
            minimum_distance: 0.0,
            node_generator: Box::<GroundNodeGenerator>::default(),
            node_follower: Box::<GroundNodeFollower>::default(),
        }
    }
}

impl Navigator {
    pub fn set_path_to_default(
        &mut self,
        world: &WorldSnapshot,
        start: EntityPosition,
        goal: Option<EntityPosition>,
        bounding_box: EntityBoundingBox,
        is_on_ground: bool,
    ) -> bool {
        let center_to_corner = (bounding_box.width() * bounding_box.width()
            + bounding_box.depth() * bounding_box.depth())
        .sqrt()
            / 2.0;
        self.set_path_to_with_completion(
            world,
            start,
            goal,
            bounding_box,
            is_on_ground,
            center_to_corner,
            None,
        )
    }

    pub fn set_path_to_with_completion(
        &mut self,
        world: &WorldSnapshot,
        start: EntityPosition,
        goal: Option<EntityPosition>,
        bounding_box: EntityBoundingBox,
        is_on_ground: bool,
        minimum_distance: f64,
        on_complete: Option<Box<dyn FnOnce() + Send>>,
    ) -> bool {
        self.set_path_to(
            world,
            start,
            goal,
            bounding_box,
            is_on_ground,
            minimum_distance,
            50.0,
            20.0,
            on_complete,
        )
    }

    pub fn state(&self) -> PathState {
        match (&self.path, &self.computing_path) {
            (Some(path), _) => path.state(),
            (None, Some(path)) => path.state(),
            (None, None) => PathState::Invalid,
        }
    }

    pub fn set_path_to(
        &mut self,
        world: &WorldSnapshot,
        start: EntityPosition,
        goal: Option<EntityPosition>,
        bounding_box: EntityBoundingBox,
        is_on_ground: bool,
        minimum_distance: f64,
        maximum_distance: f64,
        variance: f64,
        on_complete: Option<Box<dyn FnOnce() + Send>>,
    ) -> bool {
        let Some(goal) = goal else {
            self.path = None;
            return false;
        };
        if !world.world_border().contains(goal.x(), goal.z())
            || !world.is_chunk_loaded(ChunkPosition::from(goal))
        {
            return false;
        }
        self.minimum_distance = minimum_distance;
        if start.distance_squared(goal).sqrt() < minimum_distance || same_block(start, goal) {
            if let Some(on_complete) = on_complete {
                on_complete();
            }
            return false;
        }
        if let Some(computing_path) = self.computing_path.as_mut() {
            computing_path.set_state(PathState::Terminating);
        }
        self.computing_path = Some(PathGenerator::generate(
            world,
            start,
            goal,
            bounding_box,
            is_on_ground,
            minimum_distance,
            maximum_distance,
            variance,
            self.node_generator.as_ref(),
            on_complete,
        ));
        self.goal_position = Some(goal);
        true
    }

    pub const fn goal_position(&self) -> Option<EntityPosition> {
        self.goal_position
    }

    pub fn reset(&mut self) {
        if let Some(path) = self.path.as_mut() {
            path.set_state(PathState::Terminating);
        }
        if let Some(path) = self.computing_path.as_mut() {
            path.set_state(PathState::Terminating);
        }
        self.goal_position = None;
        self.path = None;
        self.computing_path = None;
    }

    pub fn is_complete(&self, position: EntityPosition) -> bool {
        if self.path.is_none() {
            return true;
        }
        self.goal_position
            .is_none_or(|goal_position| same_block(position, goal_position))
    }

    pub fn nodes(&self) -> Option<&[PathNode]> {
        self.path
            .as_ref()
            .or(self.computing_path.as_ref())
            .map(Path::nodes)
    }

    pub fn nodes_mut(&mut self) -> Option<&mut Vec<PathNode>> {
        self.path
            .as_mut()
            .or(self.computing_path.as_mut())
            .map(Path::nodes_mut)
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref().or(self.computing_path.as_ref())
    }

    pub const fn path_position(&self) -> Option<EntityPosition> {
        self.goal_position
    }

    pub fn path_mut(&mut self) -> Option<&mut Path> {
        self.path.as_mut().or(self.computing_path.as_mut())
    }

    pub fn set_node_generator(&mut self, node_generator: impl NodeGenerator + 'static) {
        self.node_generator = Box::new(node_generator);
    }

    pub fn set_node_follower(&mut self, node_follower: impl NodeFollower + 'static) {
        self.node_follower = Box::new(node_follower);
    }

    pub fn physics_timing(&self) -> NodeFollowerPhysicsTiming {
        self.node_follower.physics_timing()
    }

    pub fn tick(
        &mut self,
        entity: &mut GenericEntity,
        world: &WorldSnapshot,
        entity_is_dead: bool,
    ) {
        let Some(goal_position) = self.goal_position else {
            return;
        };
        if entity_is_dead {
            return;
        }
        if self
            .computing_path
            .as_ref()
            .is_some_and(|path| matches!(path.state(), PathState::Computed | PathState::BestEffort))
        {
            self.path = self.computing_path.take();
        }
        let Some(path) = self.path.as_mut() else {
            return;
        };
        if matches!(path.state(), PathState::Computed | PathState::BestEffort) {
            path.set_state(PathState::Following);
            if let Some(index) = path
                .nodes()
                .iter()
                .position(|node| same_block(node.position(), entity.position()))
            {
                path.remove_nodes_before(index);
            }
        }
        if path.state() != PathState::Following {
            return;
        }
        if entity.position().distance_squared(goal_position).sqrt() < self.minimum_distance {
            path.complete();
            self.path = None;
            return;
        }
        if path
            .current_type()
            .is_none_or(|node_type| node_type == PathNodeType::Repath)
        {
            if self
                .computing_path
                .as_ref()
                .is_some_and(|path| path.state() == PathState::Calculating)
            {
                return;
            }
            self.computing_path = Some(PathGenerator::generate(
                world,
                entity.position(),
                goal_position,
                entity.bounding_box(),
                entity.is_on_ground(),
                self.minimum_distance,
                path.maximum_distance(),
                path.variance(),
                self.node_generator.as_ref(),
                None,
            ));
            return;
        };
        let Some(current_target) = path.current() else {
            return;
        };
        let Some(next_target) = path.next_position() else {
            path.set_state(PathState::Invalid);
            return;
        };
        let next_is_repath = next_target.distance_squared(EntityPosition::default()) == 0.0;
        let look_at = if next_is_repath {
            current_target
        } else {
            next_target
        };
        self.node_follower.move_towards(
            entity,
            world,
            current_target,
            self.node_follower.movement_speed(entity),
            look_at,
        );
        if self.node_follower.is_at_point(entity, current_target) {
            path.advance();
        } else if path.current_type() == Some(PathNodeType::Jump) {
            self.node_follower
                .jump(entity, Some(current_target), Some(next_target));
        }
    }
}

fn same_block(left: EntityPosition, right: EntityPosition) -> bool {
    left.x().floor() == right.x().floor()
        && left.y().floor() == right.y().floor()
        && left.z().floor() == right.z().floor()
}
