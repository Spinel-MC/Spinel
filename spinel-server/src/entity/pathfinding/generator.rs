use crate::entity::EntityPosition;
use crate::entity::pathfinding::{PathNode, PathNodeType};
use crate::entity::physics::simulate_collision;
use crate::world::{Block, BlockPosition, WorldSnapshot};
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::EntityBoundingBox;
use std::collections::HashSet;

const MAXIMUM_GROUND_FALL_DISTANCE: i32 = 5;
const PRECISE_MOVEMENT_EPSILON: f64 = 0.000001;

pub trait NodeGenerator: Send {
    fn walkable(
        &self,
        world: &WorldSnapshot,
        visited: &HashSet<PathNode>,
        current: &PathNode,
        goal: EntityPosition,
        bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode>;

    fn has_gravity_snap(&self) -> bool;

    fn gravity_snap(
        &self,
        world: &WorldSnapshot,
        position: EntityPosition,
        bounding_box: EntityBoundingBox,
        maximum_fall: i32,
    ) -> Option<f64>;

    fn can_move_towards(
        &self,
        world: &WorldSnapshot,
        start: EntityPosition,
        end: EntityPosition,
        bounding_box: EntityBoundingBox,
    ) -> bool {
        if world.block(position_block(end)) != Block::AIR {
            return false;
        }
        movement_has_no_collision(world, start, end, bounding_box)
    }

    fn point_is_invalid(
        &self,
        world: &WorldSnapshot,
        position: EntityPosition,
        bounding_box: EntityBoundingBox,
    ) -> bool {
        occupancy_positions(position, bounding_box)
            .into_iter()
            .any(|block_position| world.block(block_position).is_solid())
    }

    fn heuristic(&self, node: EntityPosition, target: EntityPosition) -> f64 {
        node.get_distance_squared(target).sqrt()
    }
}

#[derive(Default)]
pub struct GroundNodeGenerator;

#[derive(Default)]
pub struct PreciseGroundNodeGenerator;

#[derive(Default)]
pub struct FlyingNodeGenerator;

#[derive(Default)]
pub struct WaterNodeGenerator;

impl NodeGenerator for GroundNodeGenerator {
    fn walkable(
        &self,
        world: &WorldSnapshot,
        visited: &HashSet<PathNode>,
        current: &PathNode,
        goal: EntityPosition,
        bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode> {
        ground_neighbors(
            self,
            world,
            visited,
            current,
            goal,
            bounding_box,
            GroundNeighborStrategy::Normal,
        )
    }

    fn has_gravity_snap(&self) -> bool {
        true
    }

    fn gravity_snap(
        &self,
        world: &WorldSnapshot,
        position: EntityPosition,
        bounding_box: EntityBoundingBox,
        maximum_fall: i32,
    ) -> Option<f64> {
        let centered_position = EntityPosition::new(
            position.get_x().floor() + 0.5,
            position.get_y().floor(),
            position.get_z().floor() + 0.5,
            position.get_yaw(),
            position.get_pitch(),
        );
        (1..=maximum_fall).find_map(|fall_distance| {
            footprint_positions(centered_position, bounding_box, -fall_distance)
                .into_iter()
                .find(|block_position| world.block(*block_position).is_solid())
                .map(|block_position| f64::from(block_position.y + 1))
        })
    }
}

impl NodeGenerator for PreciseGroundNodeGenerator {
    fn walkable(
        &self,
        world: &WorldSnapshot,
        visited: &HashSet<PathNode>,
        current: &PathNode,
        goal: EntityPosition,
        bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode> {
        ground_neighbors(
            self,
            world,
            visited,
            current,
            goal,
            bounding_box,
            GroundNeighborStrategy::Precise,
        )
    }

    fn has_gravity_snap(&self) -> bool {
        true
    }

    fn gravity_snap(
        &self,
        world: &WorldSnapshot,
        position: EntityPosition,
        bounding_box: EntityBoundingBox,
        _maximum_fall: i32,
    ) -> Option<f64> {
        let centered_position = EntityPosition::new(
            position.get_x().floor() + 0.5,
            position.get_y(),
            position.get_z().floor() + 0.5,
            position.get_yaw(),
            position.get_pitch(),
        );
        Some(
            simulate_collision(
                centered_position,
                Velocity(Vector3d {
                    x: 0.0,
                    y: -(MAXIMUM_GROUND_FALL_DISTANCE as f64),
                    z: 0.0,
                }),
                bounding_box,
                world,
                None,
            )
            .get_new_position()
            .get_y(),
        )
    }

    fn can_move_towards(
        &self,
        world: &WorldSnapshot,
        start: EntityPosition,
        end: EntityPosition,
        bounding_box: EntityBoundingBox,
    ) -> bool {
        movement_has_no_collision(
            world,
            start.get_offset(0.0, PRECISE_MOVEMENT_EPSILON, 0.0),
            end.get_offset(0.0, PRECISE_MOVEMENT_EPSILON, 0.0),
            bounding_box,
        )
    }
}

impl NodeGenerator for FlyingNodeGenerator {
    fn walkable(
        &self,
        world: &WorldSnapshot,
        visited: &HashSet<PathNode>,
        current: &PathNode,
        goal: EntityPosition,
        bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode> {
        flying_neighbors(current.get_position(), bounding_box, 0.5)
            .into_iter()
            .filter(|neighbor| {
                self.can_move_towards(
                    world,
                    current.get_position(),
                    neighbor.position,
                    bounding_box,
                )
            })
            .map(|neighbor| {
                let mut candidate_node = PathNode::new(
                    neighbor.position,
                    current.get_cost() + neighbor.cost,
                    self.heuristic(neighbor.position, goal),
                    PathNodeType::Fly,
                );
                candidate_node.set_parent(Some(current.clone()));
                candidate_node
            })
            .filter(|candidate| !visited.contains(candidate))
            .collect()
    }

    fn has_gravity_snap(&self) -> bool {
        false
    }

    fn gravity_snap(
        &self,
        _world: &WorldSnapshot,
        position: EntityPosition,
        _bounding_box: EntityBoundingBox,
        _maximum_fall: i32,
    ) -> Option<f64> {
        Some(position.get_y())
    }
}

impl NodeGenerator for WaterNodeGenerator {
    fn walkable(
        &self,
        world: &WorldSnapshot,
        visited: &HashSet<PathNode>,
        current: &PathNode,
        goal: EntityPosition,
        bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode> {
        water_neighbors(world, current.get_position(), bounding_box)
            .into_iter()
            .filter(|neighbor| {
                self.can_move_towards(
                    world,
                    current.get_position(),
                    neighbor.position,
                    bounding_box,
                )
            })
            .map(|neighbor| {
                let mut candidate_node = PathNode::new(
                    neighbor.position,
                    current.get_cost() + neighbor.cost,
                    self.heuristic(neighbor.position, goal),
                    PathNodeType::Fly,
                );
                candidate_node.set_parent(Some(current.clone()));
                candidate_node
            })
            .filter(|candidate| !visited.contains(candidate))
            .collect()
    }

    fn has_gravity_snap(&self) -> bool {
        false
    }

    fn gravity_snap(
        &self,
        _world: &WorldSnapshot,
        position: EntityPosition,
        _bounding_box: EntityBoundingBox,
        _maximum_fall: i32,
    ) -> Option<f64> {
        Some(position.get_y())
    }
}

fn horizontal_step_size(bounding_box: EntityBoundingBox) -> i32 {
    (bounding_box.get_width() / 2.0).floor().max(1.0) as i32
}

#[derive(Clone, Copy)]
enum GroundNeighborStrategy {
    Normal,
    Precise,
}

impl GroundNeighborStrategy {
    const fn candidate_y(self, current: EntityPosition) -> f64 {
        match self {
            Self::Normal => current.get_y().floor(),
            Self::Precise => current.get_y(),
        }
    }

    const fn jump_cost_increment(self) -> f64 {
        match self {
            Self::Normal => 0.2,
            Self::Precise => 0.8,
        }
    }

    const fn should_resnap_walk(self) -> bool {
        matches!(self, Self::Precise)
    }
}

fn ground_neighbors(
    generator: &dyn NodeGenerator,
    world: &WorldSnapshot,
    visited: &HashSet<PathNode>,
    current: &PathNode,
    goal: EntityPosition,
    bounding_box: EntityBoundingBox,
    strategy: GroundNeighborStrategy,
) -> Vec<PathNode> {
    let current_position = current.get_position();
    let candidate_y = strategy.candidate_y(current_position);
    let step_size = horizontal_step_size(bounding_box);
    let mut nearby_nodes = Vec::new();
    for x_offset in -step_size..=step_size {
        for z_offset in -step_size..=step_size {
            if x_offset == 0 && z_offset == 0 {
                continue;
            }
            let cost = ((x_offset * x_offset + z_offset * z_offset) as f64).sqrt() * 0.98;
            let candidate_x = current_position.get_x().floor() + 0.5 + f64::from(x_offset);
            let candidate_z = current_position.get_z().floor() + 0.5 + f64::from(z_offset);
            let floor_candidate = EntityPosition::new(
                candidate_x,
                candidate_y,
                candidate_z,
                current_position.get_yaw(),
                current_position.get_pitch(),
            );
            let Some(floor_y) = generator.gravity_snap(
                world,
                floor_candidate,
                bounding_box,
                MAXIMUM_GROUND_FALL_DISTANCE,
            ) else {
                continue;
            };
            let floor_candidate = EntityPosition::new(
                candidate_x,
                floor_y,
                candidate_z,
                current_position.get_yaw(),
                current_position.get_pitch(),
            );
            let floor_candidate = if strategy.should_resnap_walk() {
                let Some(resnapped_y) = generator.gravity_snap(
                    world,
                    floor_candidate,
                    bounding_box,
                    MAXIMUM_GROUND_FALL_DISTANCE,
                ) else {
                    continue;
                };
                EntityPosition::new(
                    floor_candidate.get_x(),
                    resnapped_y,
                    floor_candidate.get_z(),
                    floor_candidate.get_yaw(),
                    floor_candidate.get_pitch(),
                )
            } else {
                floor_candidate
            };
            if let Some(walk_node) = create_ground_walk_node(
                generator,
                world,
                visited,
                current,
                goal,
                bounding_box,
                floor_candidate,
                cost,
            ) {
                nearby_nodes.push(walk_node);
            }
            let jump_candidate = EntityPosition::new(
                candidate_x,
                candidate_y + 1.0,
                candidate_z,
                current_position.get_yaw(),
                current_position.get_pitch(),
            );
            let Some(jump_y) = generator.gravity_snap(
                world,
                jump_candidate,
                bounding_box,
                MAXIMUM_GROUND_FALL_DISTANCE,
            ) else {
                continue;
            };
            let jump_candidate = EntityPosition::new(
                candidate_x,
                jump_y,
                candidate_z,
                current_position.get_yaw(),
                current_position.get_pitch(),
            );
            if same_block(floor_candidate, jump_candidate) {
                continue;
            }
            if let Some(jump_node) = create_ground_jump_node(
                generator,
                world,
                visited,
                current,
                goal,
                bounding_box,
                jump_candidate,
                cost + strategy.jump_cost_increment(),
            ) {
                nearby_nodes.push(jump_node);
            }
        }
    }
    nearby_nodes
}

fn create_ground_walk_node(
    generator: &dyn NodeGenerator,
    world: &WorldSnapshot,
    visited: &HashSet<PathNode>,
    current: &PathNode,
    goal: EntityPosition,
    bounding_box: EntityBoundingBox,
    candidate: EntityPosition,
    cost: f64,
) -> Option<PathNode> {
    let current_position = current.get_position();
    let vertical_delta = candidate.get_y() - current_position.get_y();
    let is_fall = vertical_delta.abs() > PRECISE_MOVEMENT_EPSILON && vertical_delta < 0.0;
    let movement_target = if is_fall {
        if -vertical_delta > f64::from(MAXIMUM_GROUND_FALL_DISTANCE) {
            return None;
        }
        EntityPosition::new(
            candidate.get_x(),
            current_position.get_y(),
            candidate.get_z(),
            candidate.get_yaw(),
            candidate.get_pitch(),
        )
    } else if vertical_delta.abs() <= PRECISE_MOVEMENT_EPSILON {
        EntityPosition::new(
            candidate.get_x(),
            current_position.get_y(),
            candidate.get_z(),
            candidate.get_yaw(),
            candidate.get_pitch(),
        )
    } else {
        candidate
    };
    if !generator.can_move_towards(world, current_position, movement_target, bounding_box) {
        return None;
    }
    let candidate_node = path_node_with_parent(
        candidate,
        current.get_cost() + cost,
        generator.heuristic(candidate, goal),
        if is_fall {
            PathNodeType::Fall
        } else {
            PathNodeType::Walk
        },
        current,
    );
    (!visited.contains(&candidate_node)).then_some(candidate_node)
}

fn create_ground_jump_node(
    generator: &dyn NodeGenerator,
    world: &WorldSnapshot,
    visited: &HashSet<PathNode>,
    current: &PathNode,
    goal: EntityPosition,
    bounding_box: EntityBoundingBox,
    candidate: EntityPosition,
    cost: f64,
) -> Option<PathNode> {
    let current_position = current.get_position();
    let vertical_delta = candidate.get_y() - current_position.get_y();
    if vertical_delta.abs() < PRECISE_MOVEMENT_EPSILON || vertical_delta > 2.0 {
        return None;
    }
    let candidate_coordinates = block_coordinates(candidate);
    let current_coordinates = current.get_block_coordinates();
    if candidate_coordinates.0 != current_coordinates.0
        && candidate_coordinates.2 != current_coordinates.2
    {
        return None;
    }
    if generator.point_is_invalid(world, candidate, bounding_box) {
        return None;
    }
    let start_headroom = current_position.get_offset(0.0, 1.0, 0.0);
    if generator.point_is_invalid(world, start_headroom, bounding_box) {
        return None;
    }
    let candidate_node = path_node_with_parent(
        candidate,
        current.get_cost() + cost,
        generator.heuristic(candidate, goal),
        PathNodeType::Jump,
        current,
    );
    (!visited.contains(&candidate_node)).then_some(candidate_node)
}

fn path_node_with_parent(
    position: EntityPosition,
    cost: f64,
    heuristic: f64,
    node_type: PathNodeType,
    parent: &PathNode,
) -> PathNode {
    let mut candidate_node = PathNode::new(position, cost, heuristic, node_type);
    candidate_node.set_parent(Some(parent.clone()));
    candidate_node
}

fn movement_has_no_collision(
    world: &WorldSnapshot,
    start: EntityPosition,
    end: EntityPosition,
    bounding_box: EntityBoundingBox,
) -> bool {
    let movement = Velocity(Vector3d {
        x: end.get_x() - start.get_x(),
        y: end.get_y() - start.get_y(),
        z: end.get_z() - start.get_z(),
    });
    let collision = simulate_collision(start, movement, bounding_box, world, None);
    !collision.has_collision_x() && !collision.has_collision_y() && !collision.has_collision_z()
}

fn same_block(left: EntityPosition, right: EntityPosition) -> bool {
    block_coordinates(left) == block_coordinates(right)
}

struct FlightNeighbor {
    position: EntityPosition,
    cost: f64,
}

fn water_neighbors(
    world: &WorldSnapshot,
    position: EntityPosition,
    bounding_box: EntityBoundingBox,
) -> Vec<FlightNeighbor> {
    let step_size = horizontal_step_size(bounding_box);
    let mut neighbors = Vec::new();
    for x_offset in -step_size..=step_size {
        for z_offset in -step_size..=step_size {
            if x_offset == 0 && z_offset == 0 {
                continue;
            }
            let cost = ((x_offset * x_offset + z_offset * z_offset) as f64).sqrt() * 0.98;
            [
                centered_block_offset(position, x_offset, 0.0, z_offset),
                centered_block_offset(position, x_offset, 1.5, z_offset),
                centered_block_offset(position, x_offset, -0.5, z_offset),
            ]
            .into_iter()
            .filter(|candidate| world.block(position_block(*candidate)) == Block::WATER)
            .for_each(|candidate| {
                neighbors.push(FlightNeighbor {
                    position: candidate,
                    cost,
                });
            });
        }
    }
    let block_above = EntityPosition::new(
        position.get_x(),
        position.get_y().floor() + 1.5,
        position.get_z(),
        position.get_yaw(),
        position.get_pitch(),
    );
    if world.block(position_block(block_above)) == Block::WATER {
        neighbors.push(FlightNeighbor {
            position,
            cost: 2.0,
        });
    }
    let block_below = EntityPosition::new(
        position.get_x(),
        position.get_y().floor() - 0.5,
        position.get_z(),
        position.get_yaw(),
        position.get_pitch(),
    );
    if world.block(position_block(block_below)) == Block::WATER {
        neighbors.push(FlightNeighbor {
            position: block_below,
            cost: 2.0,
        });
    }
    neighbors
}

fn flying_neighbors(
    position: EntityPosition,
    bounding_box: EntityBoundingBox,
    current_level_y_offset: f64,
) -> Vec<FlightNeighbor> {
    let step_size = horizontal_step_size(bounding_box);
    let mut neighbors = Vec::new();
    for x_offset in -step_size..=step_size {
        for z_offset in -step_size..=step_size {
            if x_offset == 0 && z_offset == 0 {
                continue;
            }
            let cost = ((x_offset * x_offset + z_offset * z_offset) as f64).sqrt() * 0.98;
            neighbors.push(FlightNeighbor {
                position: centered_block_offset(
                    position,
                    x_offset,
                    current_level_y_offset,
                    z_offset,
                ),
                cost,
            });
            neighbors.push(FlightNeighbor {
                position: centered_block_offset(position, x_offset, 1.5, z_offset),
                cost,
            });
            neighbors.push(FlightNeighbor {
                position: centered_block_offset(position, x_offset, -0.5, z_offset),
                cost,
            });
        }
    }
    neighbors.push(FlightNeighbor {
        position: EntityPosition::new(
            position.get_x(),
            position.get_y().floor() + 1.5,
            position.get_z(),
            position.get_yaw(),
            position.get_pitch(),
        ),
        cost: 2.0,
    });
    neighbors.push(FlightNeighbor {
        position: EntityPosition::new(
            position.get_x(),
            position.get_y().floor() - 0.5,
            position.get_z(),
            position.get_yaw(),
            position.get_pitch(),
        ),
        cost: 2.0,
    });
    neighbors
}

fn centered_block_offset(
    position: EntityPosition,
    x_offset: i32,
    y_offset: f64,
    z_offset: i32,
) -> EntityPosition {
    EntityPosition::new(
        position.get_x().floor() + 0.5 + x_offset as f64,
        position.get_y().floor() + y_offset,
        position.get_z().floor() + 0.5 + z_offset as f64,
        position.get_yaw(),
        position.get_pitch(),
    )
}

fn block_coordinates(position: EntityPosition) -> (i32, i32, i32) {
    (
        position.get_x().floor() as i32,
        position.get_y().floor() as i32,
        position.get_z().floor() as i32,
    )
}

fn position_block(position: EntityPosition) -> BlockPosition {
    BlockPosition::new(
        position.get_x().floor() as i32,
        position.get_y().floor() as i32,
        position.get_z().floor() as i32,
    )
}

fn occupancy_positions(
    position: EntityPosition,
    bounding_box: EntityBoundingBox,
) -> Vec<BlockPosition> {
    let minimum_x = (position.get_x() + bounding_box.minimum_x()).floor() as i32;
    let maximum_x = (position.get_x() + bounding_box.maximum_x()).floor() as i32;
    let minimum_y = (position.get_y() + bounding_box.minimum_y()).floor() as i32;
    let maximum_y = (position.get_y() + bounding_box.maximum_y()).floor() as i32;
    let minimum_z = (position.get_z() + bounding_box.minimum_z()).floor() as i32;
    let maximum_z = (position.get_z() + bounding_box.maximum_z()).floor() as i32;
    (minimum_x..=maximum_x)
        .flat_map(|x| {
            (minimum_y..=maximum_y).flat_map(move |y| {
                (minimum_z..=maximum_z).map(move |z| BlockPosition::new(x, y, z))
            })
        })
        .collect()
}

fn footprint_positions(
    position: EntityPosition,
    bounding_box: EntityBoundingBox,
    y_offset: i32,
) -> Vec<BlockPosition> {
    let minimum_x = (position.get_x() + bounding_box.minimum_x()).floor() as i32;
    let maximum_x = (position.get_x() + bounding_box.maximum_x()).floor() as i32;
    let minimum_z = (position.get_z() + bounding_box.minimum_z()).floor() as i32;
    let maximum_z = (position.get_z() + bounding_box.maximum_z()).floor() as i32;
    let y = (position.get_y() + bounding_box.minimum_y()).floor() as i32 + y_offset;
    (minimum_x..=maximum_x)
        .flat_map(|x| (minimum_z..=maximum_z).map(move |z| BlockPosition::new(x, y, z)))
        .collect()
}
