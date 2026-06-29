use crate::entity::pathfinding::perfect::{
    PerfectControlState, PerfectMotionSimulator, PerfectMotionState, PerfectPathBudget,
    PerfectPathPlanner, PerfectPathRequest, PerfectPathTermination,
};
use crate::entity::pathfinding::{
    FlyingNodeGenerator, GroundNodeFollower, GroundNodeGenerator, Navigator, NoPhysicsNodeFollower,
    NodeFollower, NodeGenerator, Path, PathGenerator, PathNode, PathNodeType, PathRequest,
    PathState, PreciseGroundNodeGenerator, SetPathToError, WaterNodeGenerator,
};
use crate::entity::physics::simulate_collision;
use crate::entity::{Entity, EntityCreature, EntityPosition, GenericEntity};
use crate::world::{Block, BlockPosition, BlockState, Chunk, ChunkPosition, World, WorldBorder};
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_registry::dimension_type::DimensionType;
use spinel_registry::registry_values::attribute::Attribute;
use spinel_registry::{EntityBoundingBox, EntityType};
use std::collections::{BTreeMap, HashSet};
use std::path::{Path as FsPath, PathBuf};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::{env, fs};

#[test]
fn path_state_owner_records_every_minestom_state_transition() {
    let path = Path::new(40.0, 8.0, None);
    let states = [
        PathState::Calculating,
        PathState::Following,
        PathState::Terminating,
        PathState::Terminated,
        PathState::Computed,
        PathState::BestEffort,
        PathState::Invalid,
    ];

    for state in states {
        path.set_state(state);
        assert_eq!(path.get_state(), state);
    }
}

#[test]
fn path_exposes_minestom_modifiable_node_collection() {
    let mut path = Path::new(32.0, 0.0, None);
    path.get_nodes_mut().push(PathNode::new(
        EntityPosition::new(1.0, 2.0, 3.0, 0.0, 0.0),
        4.0,
        5.0,
        PathNodeType::Walk,
    ));

    assert_eq!(path.get_nodes().len(), 1);
    assert_eq!(
        path.get_current(),
        Some(EntityPosition::new(1.0, 2.0, 3.0, 0.0, 0.0))
    );
}

#[test]
fn public_path_generator_exposes_minestom_generation_entry_point() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let path = PathGenerator::generate(
        &snapshot,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        EntityPosition::new(3.5, 65.0, 0.5, 0.0, 0.0),
        EntityType::ZOMBIE.get_bounding_box(),
        true,
        0.5,
        50.0,
        20.0,
        &GroundNodeGenerator,
        None,
    );

    assert_eq!(path.get_state(), PathState::Computed);
    assert!(!path.get_nodes().is_empty());
}

#[test]
fn path_generator_preserves_minestom_sub_millistep_later_candidate_ties() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let expanded_nodes = Arc::new(Mutex::new(Vec::new()));
    let generator = PriorityTieNodeGenerator {
        expanded_nodes: expanded_nodes.clone(),
    };

    PathGenerator::generate(
        &snapshot,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        EntityPosition::new(8.5, 65.0, 0.5, 0.0, 0.0),
        EntityBoundingBox::new(0.6, 1.8, 0.6),
        true,
        0.8,
        16.0,
        16.0,
        &generator,
        None,
    );

    assert_eq!(
        expanded_nodes.lock().unwrap().as_slice(),
        &[(0, 65, 0), (2, 65, 0), (1, 65, 0)]
    );
}

#[test]
fn navigator_exposes_minestom_modifiable_node_collection() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut navigator = Navigator::default();
    let start = EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0);

    assert!(navigator
        .set_path_to(
            &snapshot,
            start,
            EntityType::ZOMBIE.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(3.5, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());
    navigator.get_nodes_mut().unwrap().clear();

    assert!(navigator.get_nodes().unwrap().is_empty());
}

#[test]
fn navigator_default_request_uses_minestom_bounding_box_distance_and_path_limits() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut navigator = Navigator::default();
    let start = EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0);

    assert!(navigator
        .set_path_to(
            &snapshot,
            start,
            EntityType::ZOMBIE.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(1.1, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());
    let path = navigator.get_path().unwrap();
    assert_eq!(path.get_maximum_distance(), 50.0);
    assert_eq!(path.get_variance(), 20.0);
}
#[test]
fn path_node_identity_uses_floored_block_coordinates() {
    let first = PathNode::new(
        EntityPosition::new(1.1, 64.9, -2.1, 0.0, 0.0),
        1.0,
        2.0,
        PathNodeType::Walk,
    );
    let second = PathNode::new(
        EntityPosition::new(1.9, 64.1, -2.9, 90.0, 20.0),
        99.0,
        88.0,
        PathNodeType::Jump,
    );
    let negative_neighbor = PathNode::new(
        EntityPosition::new(-0.1, 64.0, -2.0, 0.0, 0.0),
        1.0,
        2.0,
        PathNodeType::Walk,
    );
    let mut nodes = HashSet::new();

    nodes.insert(first.clone());
    nodes.insert(second.clone());
    nodes.insert(negative_neighbor);

    assert_eq!(first, second);
    assert_eq!(nodes.len(), 2);
}

#[test]
fn path_node_owner_exposes_generation_mutation_points() {
    let mut node = PathNode::new(
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        1.0,
        2.0,
        PathNodeType::Walk,
    );
    let next_position = EntityPosition::new(1.5, 66.0, 0.5, 30.0, 10.0);

    node.set_position(next_position);
    node.set_cost(3.0);
    node.set_heuristic(4.0);
    node.set_node_type(PathNodeType::Jump);
    node.set_parent(Some(PathNode::new(
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        0.0,
        0.0,
        PathNodeType::Walk,
    )));

    assert_eq!(node.get_position(), next_position);
    assert_eq!(node.get_cost(), 3.0);
    assert_eq!(node.get_heuristic(), 4.0);
    assert_eq!(node.get_node_type(), PathNodeType::Jump);
    assert_eq!(node.get_parent_coordinates(), Some((0, 65, 0)));
}

#[test]
fn navigator_rejects_unloaded_and_out_of_border_targets() {
    let mut world = pathfinding_world();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();
    let unloaded_goal = EntityPosition::new(32.5, 65.0, 0.5, 0.0, 0.0);

    assert!(matches!(
        navigator.set_path_to(
            &world.update_snapshot(),
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(unloaded_goal).with_minimum_distance(0.5),
        ),
        Err(SetPathToError::TargetChunkUnloaded { target }) if target == unloaded_goal
    ));

    world
        .set_world_border(WorldBorder::DEFAULT.with_center(100_000_000.0, 100_000_000.0))
        .unwrap();
    let loaded_goal = EntityPosition::new(5.5, 65.0, 0.5, 0.0, 0.0);
    assert!(matches!(
        navigator.set_path_to(
            &world.update_snapshot(),
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(loaded_goal).with_minimum_distance(0.5),
        ),
        Err(SetPathToError::TargetOutsideWorldBorder { target }) if target == loaded_goal
    ));
}

#[test]
fn navigator_completes_same_block_and_minimum_distance_requests_immediately() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();
    let same_block_completed = Arc::new(AtomicBool::new(false));
    let same_block_completed_for_callback = Arc::clone(&same_block_completed);

    assert!(!navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(0.9, 65.0, 0.9, 0.0, 0.0))
                .with_minimum_distance(0.1)
                .on_complete(move || {
                    same_block_completed_for_callback.store(true, Ordering::SeqCst);
                }),
        )
        .unwrap());
    assert!(same_block_completed.load(Ordering::SeqCst));

    let minimum_distance_completed = Arc::new(AtomicBool::new(false));
    let minimum_distance_completed_for_callback = Arc::clone(&minimum_distance_completed);
    assert!(!navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(2.5, 65.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(5.0)
                .on_complete(move || {
                    minimum_distance_completed_for_callback.store(true, Ordering::SeqCst);
                }),
        )
        .unwrap());
    assert!(minimum_distance_completed.load(Ordering::SeqCst));
}

#[test]
fn navigator_follows_computed_path_and_runs_completion_once() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let completed = Arc::new(AtomicBool::new(false));
    let completed_for_callback = Arc::clone(&completed);
    let mut navigator = Navigator::default();

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(5.5, 65.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(0.5)
                .on_complete(move || {
                    completed_for_callback.store(true, Ordering::SeqCst);
                }),
        )
        .unwrap());
    assert!(matches!(
        navigator.state(),
        PathState::Computed | PathState::BestEffort
    ));
    assert!(!navigator.is_complete(entity.get_position()));

    for _ in 0..200 {
        entity.movement_tick(&snapshot);
        navigator.tick(&mut entity, &snapshot, false);
        if completed.load(Ordering::SeqCst) {
            break;
        }
    }

    assert!(completed.load(Ordering::SeqCst));
    assert!(navigator.is_complete(entity.get_position()));
}

#[test]
fn navigator_reset_and_no_physics_follower_match_public_lifecycle() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();
    navigator.set_node_follower(NoPhysicsNodeFollower);

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(3.5, 65.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(0.5),
        )
        .unwrap());
    navigator.tick(&mut entity, &snapshot, false);
    assert!(entity.get_position().get_x() > 0.5);

    navigator.reset();
    assert_eq!(navigator.state(), PathState::Invalid);
    assert_eq!(navigator.goal_position(), None);
    assert!(navigator.get_nodes().is_none());
}

#[test]
fn navigator_preserves_active_path_until_replacement_promotes_on_tick() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(3.5, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());
    navigator.tick(&mut entity, &snapshot, false);
    let active_goal_before_replacement = navigator
        .get_nodes()
        .unwrap()
        .last()
        .unwrap()
        .get_position();

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(5.5, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());
    let active_goal_during_replacement = navigator
        .get_nodes()
        .unwrap()
        .last()
        .unwrap()
        .get_position();
    navigator.tick(&mut entity, &snapshot, false);
    let active_goal_after_replacement = navigator
        .get_nodes()
        .unwrap()
        .last()
        .unwrap()
        .get_position();

    assert_eq!(
        active_goal_during_replacement,
        active_goal_before_replacement
    );
    assert_eq!(active_goal_after_replacement.get_x().floor() as i32, 5);
    assert_eq!(active_goal_after_replacement.get_y().floor() as i32, 65);
    assert_eq!(active_goal_after_replacement.get_z().floor() as i32, 0);
}

#[test]
fn navigator_waits_while_computing_path_is_terminating() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(3.5, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());
    navigator
        .get_path_mut()
        .unwrap()
        .set_state(PathState::Terminating);

    navigator.tick(&mut entity, &snapshot, false);

    assert_eq!(navigator.state(), PathState::Terminating);
    assert!(navigator.get_nodes().is_some());
}

#[test]
fn navigator_uses_replaced_node_generator_and_preserves_best_effort_state() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();
    navigator.set_node_generator(SingleBestEffortNodeGenerator);

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(10.5, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());

    assert_eq!(navigator.state(), PathState::BestEffort);
    assert_eq!(
        navigator
            .get_nodes()
            .unwrap()
            .last()
            .unwrap()
            .get_block_coordinates(),
        (2, 65, 0)
    );
}

#[test]
fn path_generation_invalidates_repath_rooted_at_start() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();
    navigator.set_node_generator(FrontierBudgetNodeGenerator);

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(15.5, 65.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(0.5)
                .with_maximum_distance(5.0),
        )
        .unwrap());

    assert_eq!(navigator.state(), PathState::Invalid);
    assert!(navigator.get_nodes().is_some_and(<[PathNode]>::is_empty));
}

#[test]
fn navigator_recomputes_after_repath_current_node() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());
    navigator.tick(&mut entity, &snapshot, false);
    let path = navigator.get_path_mut().unwrap();
    path.set_state(PathState::Following);
    path.set_nodes(vec![
        PathNode::new(entity.get_position(), 0.0, 0.0, PathNodeType::Repath),
        PathNode::new(EntityPosition::default(), 0.0, 0.0, PathNodeType::Walk),
    ]);

    navigator.tick(&mut entity, &snapshot, false);

    assert_ne!(
        navigator.get_path().unwrap().get_current_type(),
        Some(PathNodeType::Repath)
    );
}

#[test]
fn navigator_marks_path_invalid_without_next_target() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let mut navigator = Navigator::default();

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());
    navigator.tick(&mut entity, &snapshot, false);
    let path = navigator.get_path_mut().unwrap();
    path.set_state(PathState::Following);
    path.set_nodes(vec![PathNode::new(
        entity.get_position().get_offset(1.0, 0.0, 0.0),
        0.0,
        0.0,
        PathNodeType::Walk,
    )]);

    navigator.tick(&mut entity, &snapshot, false);

    assert_eq!(navigator.state(), PathState::Invalid);
}

#[test]
fn navigator_moves_toward_jump_node_before_executing_jump_like_minestom() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    entity.set_on_ground(true);
    let follower_events = Arc::new(Mutex::new(Vec::new()));
    let mut navigator = Navigator::default();
    navigator.set_node_follower(RecordingNodeFollower {
        events: Arc::clone(&follower_events),
    });

    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(4.5, 66.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(0.25),
        )
        .unwrap());
    let path = navigator.get_path_mut().unwrap();
    path.set_state(PathState::Following);
    path.set_nodes(vec![
        PathNode::new(
            EntityPosition::new(1.5, 66.0, 0.5, 0.0, 0.0),
            0.0,
            0.0,
            PathNodeType::Jump,
        ),
        PathNode::new(
            EntityPosition::new(2.5, 66.0, 0.5, 0.0, 0.0),
            0.0,
            0.0,
            PathNodeType::Walk,
        ),
    ]);

    navigator.tick(&mut entity, &snapshot, false);

    assert_eq!(
        follower_events.lock().unwrap().as_slice(),
        &["move", "jump"]
    );
}

#[test]
fn dead_entity_does_not_tick_navigation() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let start = entity.get_position();
    let mut navigator = Navigator::default();
    assert!(navigator
        .set_path_to(
            &snapshot,
            entity.get_position(),
            entity.get_bounding_box(),
            true,
            PathRequest::from(EntityPosition::new(3.5, 65.0, 0.5, 0.0, 0.0)),
        )
        .unwrap());

    navigator.tick(&mut entity, &snapshot, true);

    assert_eq!(entity.get_position(), start);
    assert!(navigator.get_nodes().is_some());
}

#[test]
fn creature_rejects_path_requests_before_world_assignment() {
    let mut world = pathfinding_world();
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature
        .get_entity_mut()
        .get_attribute(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        )
        .set_base_value(Attribute::MOVEMENT_SPEED.default_value());
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));

    assert!(matches!(
        creature.set_path_to(PathRequest::from(EntityPosition::new(
            3.5, 65.0, 0.5, 0.0, 0.0
        ))),
        Err(SetPathToError::EntityHasNoWorld)
    ));

    let creature_id = creature.get_entity_id();
    creature.set_world(&mut world);
    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };

    assert!(creature
        .set_path_to(PathRequest::from(EntityPosition::new(
            3.5, 65.0, 0.5, 0.0, 0.0
        )))
        .unwrap());
}

#[test]
fn entity_creature_set_path_to_owns_pathfinding_request() {
    let mut world = pathfinding_world();
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature
        .get_entity_mut()
        .get_attribute(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        )
        .set_base_value(Attribute::MOVEMENT_SPEED.default_value());
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let creature_id = creature.get_entity_id();
    creature.set_world(&mut world);
    let destination = EntityPosition::new(3.5, 65.0, 0.5, 0.0, 0.0);
    {
        let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
            panic!("creature entity must preserve its subtype");
        };

        assert!(creature
            .set_path_to(PathRequest::from(destination))
            .unwrap());
    }
    world.tick();
    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };

    assert_eq!(creature.get_navigator().goal_position(), Some(destination));
}

#[test]
fn entity_creature_set_path_to_observes_blocks_placed_after_spawn() {
    let mut world = pathfinding_world();
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature
        .get_entity_mut()
        .get_attribute(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        )
        .set_base_value(Attribute::MOVEMENT_SPEED.default_value());
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    creature.set_on_ground(true);
    let creature_id = creature.get_entity_id();
    creature.set_world(&mut world);
    world
        .set_block(BlockPosition::new(1, 65, 0), Block::STONE)
        .unwrap();
    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };

    assert!(creature
        .set_path_to(
            PathRequest::from(EntityPosition::new(2.5, 65.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(0.35)
        )
        .unwrap());
    assert!(creature
        .get_navigator()
        .get_nodes()
        .unwrap()
        .iter()
        .any(|node| node.get_node_type() == PathNodeType::Jump));
}
#[test]
fn world_tick_pathfinding_zombie_jumps_over_one_block_obstruction() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(1, 65, 0), Block::STONE)
        .unwrap();
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature
        .get_entity_mut()
        .get_attribute(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        )
        .set_base_value(Attribute::MOVEMENT_SPEED.default_value());
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    creature.set_on_ground(true);
    let creature_id = creature.get_entity_id();
    world.add_entity(Entity::Creature(creature));
    let snapshot = world.update_snapshot();
    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    let start_position = creature.get_position();
    let bounding_box = creature.get_bounding_box();
    assert!(creature
        .get_navigator_mut()
        .set_path_to(
            &snapshot,
            start_position,
            bounding_box,
            true,
            PathRequest::from(EntityPosition::new(2.5, 65.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(0.35),
        )
        .unwrap());
    assert!(creature
        .get_navigator()
        .get_nodes()
        .unwrap()
        .iter()
        .any(|node| node.get_node_type() == PathNodeType::Jump));

    let mut crossed_obstruction_while_airborne = false;
    for _ in 0..80 {
        world.tick();
        let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
            panic!("creature entity must preserve its subtype");
        };
        let position = creature.get_position();
        crossed_obstruction_while_airborne |=
            position.get_x() >= 1.0 && position.get_x() <= 2.0 && position.get_y() > 65.25;
    }

    let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    let final_position = creature.get_position();
    assert!(
        crossed_obstruction_while_airborne,
        "zombie never became airborne over obstruction; final position: {:?}",
        final_position
    );
    assert!(
        final_position.get_x() > 2.0,
        "zombie did not reach the far side of obstruction: {:?}",
        final_position
    );
}

#[test]
fn ground_generator_emits_walk_fall_and_non_diagonal_jump_nodes() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(9, 65, 8), Block::STONE)
        .unwrap();
    world
        .set_block(BlockPosition::new(8, 64, 9), Block::AIR)
        .unwrap();
    world
        .set_block(BlockPosition::new(8, 63, 9), Block::STONE)
        .unwrap();
    world
        .set_block(BlockPosition::new(9, 65, 9), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let generator = GroundNodeGenerator;
    let current = PathNode::new(
        EntityPosition::new(8.5, 65.0, 8.5, 0.0, 0.0),
        0.0,
        0.0,
        PathNodeType::Walk,
    );
    let nodes = generator.walkable(
        &snapshot,
        &HashSet::new(),
        &current,
        EntityPosition::new(11.5, 65.0, 8.5, 0.0, 0.0),
        EntityType::PLAYER.get_bounding_box(),
    );

    assert!(nodes
        .iter()
        .any(|node| node.get_node_type() == PathNodeType::Walk));
    assert!(nodes.iter().any(|node| {
        node.get_node_type() == PathNodeType::Walk && node.get_block_coordinates() == (7, 65, 7)
    }));
    assert!(nodes
        .iter()
        .any(|node| node.get_node_type() == PathNodeType::Fall));
    assert!(nodes
        .iter()
        .any(|node| node.get_node_type() == PathNodeType::Jump));
    assert!(!nodes.iter().any(|node| {
        node.get_node_type() == PathNodeType::Walk && node.get_block_coordinates() == (9, 65, 8)
    }));
    assert!(!nodes.iter().any(|node| {
        node.get_node_type() == PathNodeType::Jump && node.get_block_coordinates() == (9, 66, 9)
    }));
    assert_eq!(
        generator.gravity_snap(
            &snapshot,
            EntityPosition::new(8.5, 70.0, 8.5, 0.0, 0.0),
            EntityType::PLAYER.get_bounding_box(),
            10,
        ),
        Some(65.0)
    );
}

#[test]
fn node_generators_reject_movement_through_intermediate_wall() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(1, 65, 0), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let start = EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0);
    let end = EntityPosition::new(2.5, 65.0, 0.5, 0.0, 0.0);
    let bounding_box = EntityType::PLAYER.get_bounding_box();

    assert!(!GroundNodeGenerator.can_move_towards(&snapshot, start, end, bounding_box,));
    assert!(!PreciseGroundNodeGenerator.can_move_towards(&snapshot, start, end, bounding_box,));
}

#[test]
fn precise_ground_generator_uses_collision_shape_for_gravity_snap() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(8, 64, 8), Block::STONE_SLAB)
        .unwrap();
    let snapshot = world.update_snapshot();
    let position = EntityPosition::new(8.5, 65.0, 8.5, 0.0, 0.0);
    let bounding_box = EntityType::PLAYER.get_bounding_box();

    assert_eq!(
        GroundNodeGenerator.gravity_snap(&snapshot, position, bounding_box, 5),
        Some(65.0)
    );
    let precise_y = PreciseGroundNodeGenerator
        .gravity_snap(&snapshot, position, bounding_box, 5)
        .unwrap();
    assert!((precise_y - 64.5).abs() < 0.00001);
}

#[test]
fn precise_ground_generator_rejects_bottom_slab_walk_without_step_fallback() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(9, 65, 8), Block::STONE_SLAB)
        .unwrap();
    let snapshot = world.update_snapshot();
    let generator = PreciseGroundNodeGenerator;
    let current = PathNode::new(
        EntityPosition::new(8.5, 65.0, 8.5, 0.0, 0.0),
        0.0,
        0.0,
        PathNodeType::Walk,
    );
    let nodes = generator.walkable(
        &snapshot,
        &HashSet::new(),
        &current,
        EntityPosition::new(9.5, 65.5, 8.5, 0.0, 0.0),
        EntityType::ZOMBIE.get_bounding_box(),
    );

    assert!(!nodes.iter().any(|node| {
        node.get_node_type() == PathNodeType::Walk
            && node.get_block_coordinates() == (9, 65, 8)
            && (node.get_position().get_y() - 65.5).abs() < 0.000001
    }));
}
#[test]
fn collision_sweep_allows_clear_horizontal_floor_travel() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let start = EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0);
    let collision = simulate_collision(
        start,
        Velocity(Vector3d {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
        EntityType::PLAYER.get_bounding_box(),
        &snapshot,
        None,
    );

    assert!(
        !collision.has_collision_x(),
        "clear floor travel collided: {collision:?}"
    );
}

#[test]
fn flying_generator_emits_minestom_neighbor_shape() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let generator = FlyingNodeGenerator;
    let current = PathNode::new(
        EntityPosition::new(8.5, 70.0, 8.5, 0.0, 0.0),
        0.0,
        0.0,
        PathNodeType::Fly,
    );
    let nodes = generator.walkable(
        &snapshot,
        &HashSet::new(),
        &current,
        EntityPosition::new(9.5, 70.0, 9.5, 0.0, 0.0),
        EntityType::PLAYER.get_bounding_box(),
    );

    assert_eq!(nodes.len(), 26);
    assert!(nodes
        .iter()
        .all(|node| node.get_node_type() == PathNodeType::Fly));
    assert!(nodes
        .iter()
        .any(|node| node.get_block_coordinates() == (9, 70, 9)));
    assert!(nodes
        .iter()
        .any(|node| node.get_block_coordinates() == (8, 71, 8)));
    assert!(nodes
        .iter()
        .any(|node| node.get_block_coordinates() == (8, 69, 8)));
}

#[test]
fn water_generator_matches_minestom_default_direct_movement_rejection() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(8, 70, 9), Block::WATER)
        .unwrap();
    world
        .set_block(BlockPosition::new(9, 70, 8), Block::WATER_CAULDRON)
        .unwrap();
    let snapshot = world.update_snapshot();
    let generator = WaterNodeGenerator;
    let current = PathNode::new(
        EntityPosition::new(8.5, 70.0, 8.5, 0.0, 0.0),
        0.0,
        0.0,
        PathNodeType::Swim,
    );
    let nodes = generator.walkable(
        &snapshot,
        &HashSet::new(),
        &current,
        EntityPosition::new(8.5, 70.0, 9.5, 0.0, 0.0),
        EntityType::PLAYER.get_bounding_box(),
    );

    assert!(nodes.is_empty());
}

#[test]
fn ground_follower_applies_minestom_speed_rotation_collision_and_jump_behavior() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = GroundNodeFollower;
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    entity.set_on_ground(true);
    let movement_speed = follower.movement_speed(&entity);

    follower.move_towards(
        &mut entity,
        &snapshot,
        EntityPosition::new(1.5, 65.0, 0.5, 0.0, 0.0),
        movement_speed,
        EntityPosition::new(1.5, 66.0, 0.5, 0.0, 0.0),
    );

    assert!(entity.get_position().get_x() > 0.5);
    assert_eq!(entity.get_position().get_yaw().round() as i32, -90);
    assert_eq!(entity.get_position().get_pitch().round() as i32, -45);
    assert!(follower.is_at_point(&entity, entity.get_position()));

    let mut blocked_world = pathfinding_world();
    blocked_world
        .set_block(BlockPosition::new(1, 65, 0), Block::STONE)
        .unwrap();
    let blocked_snapshot = blocked_world.update_snapshot();
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    entity.set_velocity(Velocity(Vector3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }));
    entity.set_on_ground(true);
    let blocked_start = entity.get_position();
    follower.move_towards(
        &mut entity,
        &blocked_snapshot,
        EntityPosition::new(1.5, 65.0, 0.5, 0.0, 0.0),
        movement_speed,
        EntityPosition::new(1.5, 65.0, 0.5, 0.0, 0.0),
    );
    assert!(entity.get_position().get_x() >= blocked_start.get_x());
    assert!(entity.get_position().get_x() < 1.0);

    entity.set_on_ground(true);
    follower.jump(
        &mut entity,
        Some(EntityPosition::new(1.5, 66.0, 0.5, 0.0, 0.0)),
        Some(EntityPosition::new(2.5, 66.0, 0.5, 0.0, 0.0)),
    );
    assert_eq!(entity.get_velocity().0.x, 0.0);
    assert_eq!(entity.get_velocity().0.y, 10.0);
    assert_eq!(entity.get_velocity().0.z, 0.0);
}

#[test]
fn node_follower_uses_living_entity_default_and_minestom_non_living_fallback() {
    let follower = GroundNodeFollower;
    let living_entity = GenericEntity::new(EntityType::ZOMBIE);
    let non_living_entity = GenericEntity::new(EntityType::ITEM);

    assert!((follower.movement_speed(&living_entity) - 0.23000000417232513).abs() < f64::EPSILON);
    assert_eq!(follower.movement_speed(&non_living_entity), 0.1);
}

#[test]
fn ground_follower_uses_extracted_minecraft_zombie_speed_as_minestom_displacement() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = GroundNodeFollower;
    let mut zombie = GenericEntity::new(EntityType::ZOMBIE);
    zombie.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    zombie.set_on_ground(true);
    let movement_speed = follower.movement_speed(&zombie);

    follower.move_towards(
        &mut zombie,
        &snapshot,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
        movement_speed,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
    );

    assert!((zombie.get_position().get_x() - (0.5 + movement_speed)).abs() < 0.000_001);
    assert!((movement_speed - 0.23000000417232513).abs() < f64::EPSILON);
}

#[test]
fn minestom_straight_short_and_fractional_negative_path_fixtures_are_valid() {
    let straight_world = pathfinding_world_in_chunk_range(-1, 1, -1, 1);
    assert_valid_default_path(
        &straight_world,
        EntityPosition::new(0.0, 65.0, 0.0, 0.0, 0.0),
        EntityPosition::new(0.0, 65.0, 10.0, 0.0, 0.0),
        EntityType::ZOMBIE.get_bounding_box(),
    );
    assert_valid_default_path(
        &straight_world,
        EntityPosition::new(0.0, 65.0, 0.0, 0.0, 0.0),
        EntityPosition::new(2.0, 65.0, 2.0, 0.0, 0.0),
        EntityType::ZOMBIE.get_bounding_box(),
    );

    let negative_world = pathfinding_world_in_chunk_range(1, 3, -4, -2);
    assert_valid_default_path(
        &negative_world,
        EntityPosition::new(43.972731367054266, 65.0, -39.89155139999369, 0.0, 0.0),
        EntityPosition::new(43.5, 65.0, -41.5, 0.0, 0.0),
        EntityType::ZOMBIE.get_bounding_box(),
    );
}

#[test]
fn minestom_tall_and_blocked_path_fixtures_are_valid() {
    let mut tall_world = pathfinding_world_in_chunk_range(-1, 1, -1, 1);
    tall_world
        .set_block(BlockPosition::new(1, 71, 7), Block::STONE)
        .unwrap();
    assert_valid_default_path(
        &tall_world,
        EntityPosition::new(0.0, 65.0, 0.0, 0.0, 0.0),
        EntityPosition::new(0.0, 65.0, 10.0, 0.0, 0.0),
        EntityBoundingBox::new(3.0, 6.5, 3.0),
    );

    let mut blocked_world = pathfinding_world_in_chunk_range(-1, 1, -1, 1);
    for x in -6..=7 {
        blocked_world
            .set_block(BlockPosition::new(x, 65, 5), Block::STONE)
            .unwrap();
        blocked_world
            .set_block(BlockPosition::new(x, 66, 5), Block::STONE)
            .unwrap();
    }
    assert_valid_default_path(
        &blocked_world,
        EntityPosition::new(0.0, 65.0, 0.0, 0.0, 0.0),
        EntityPosition::new(0.0, 65.0, 10.0, 0.0, 0.0),
        EntityBoundingBox::new(4.6, 5.8, 4.6),
    );
}

#[test]
fn perfect_motion_simulation_is_deterministic_and_uses_legal_jump_state() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let simulator = PerfectMotionSimulator::default();
    let mut grounded = PerfectMotionState::new(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    grounded.on_ground = true;
    let airborne = PerfectMotionState {
        on_ground: false,
        ..grounded
    };
    let control = PerfectControlState {
        forward: true,
        jump: true,
        sprint: true,
        ..PerfectControlState::default()
    };
    let first = simulator.tick(
        &snapshot,
        EntityType::PLAYER.get_bounding_box(),
        grounded,
        control,
    );
    let second = simulator.tick(
        &snapshot,
        EntityType::PLAYER.get_bounding_box(),
        grounded,
        control,
    );
    let airborne_result = simulator.tick(
        &snapshot,
        EntityType::PLAYER.get_bounding_box(),
        airborne,
        control,
    );

    assert_eq!(first, second);
    assert!(first.velocity.y > airborne_result.velocity.y);
}

#[test]
fn perfect_motion_collision_blocks_intermediate_state() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(0, 65, 1), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let simulator = PerfectMotionSimulator::default();
    let mut state = PerfectMotionState::new(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    state.on_ground = true;
    state.velocity.z = 1.0;
    let control = PerfectControlState {
        forward: true,
        sprint: true,
        ..PerfectControlState::default()
    };

    let next = simulator.tick(
        &snapshot,
        EntityType::PLAYER.get_bounding_box(),
        state,
        control,
    );

    assert!(next.position.get_z() < 1.0);
    assert!(next.horizontal_collision);
}

#[test]
fn perfect_planner_returns_success_cancelled_and_budget_results() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut start = PerfectMotionState::new(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    start.on_ground = true;
    let planner = PerfectPathPlanner::default();
    let budget = PerfectPathBudget {
        maximum_expanded_states: 2_000,
        maximum_ticks: 40,
    };

    let success = planner.plan(PerfectPathRequest {
        world: &snapshot,
        bounding_box: EntityType::PLAYER.get_bounding_box(),
        start,
        destination: EntityPosition::new(0.5, 65.0, 2.0, 0.0, 0.0),
        destination_radius: 1.0,
        budget,
        is_cancelled: &|| false,
    });

    assert_eq!(success.termination, PerfectPathTermination::Success);
    assert!(success.plan.unwrap().controls.len() <= budget.maximum_ticks as usize);

    let cancelled = planner.plan(PerfectPathRequest {
        world: &snapshot,
        bounding_box: EntityType::PLAYER.get_bounding_box(),
        start,
        destination: EntityPosition::new(0.5, 65.0, 2.0, 0.0, 0.0),
        destination_radius: 1.0,
        budget,
        is_cancelled: &|| true,
    });
    assert_eq!(cancelled.termination, PerfectPathTermination::Cancelled);

    let exhausted = planner.plan(PerfectPathRequest {
        world: &snapshot,
        bounding_box: EntityType::PLAYER.get_bounding_box(),
        start,
        destination: EntityPosition::new(0.5, 65.0, 15.0, 0.0, 0.0),
        destination_radius: 0.2,
        budget: PerfectPathBudget {
            maximum_expanded_states: 1,
            maximum_ticks: 40,
        },
        is_cancelled: &|| false,
    });
    assert_eq!(
        exhausted.termination,
        PerfectPathTermination::ExpandedStateBudget
    );
}

#[test]
fn creature_owns_navigation_and_world_ticks_advance_it() {
    let mut world = pathfinding_world();
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature
        .get_entity_mut()
        .get_attribute(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        )
        .set_base_value(Attribute::MOVEMENT_SPEED.default_value());
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let initial_snapshot = world.update_snapshot();
    let initial_position = creature.get_position();
    let bounding_box = creature.get_bounding_box();
    assert!(creature
        .get_navigator_mut()
        .set_path_to(
            &initial_snapshot,
            initial_position,
            bounding_box,
            true,
            PathRequest::from(EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(0.5),
        )
        .unwrap());
    let creature_id = creature.get_entity_id();
    world.add_entity(Entity::Creature(creature));

    let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    assert_eq!(creature.get_navigator().state(), PathState::Invalid);

    let snapshot = world.update_snapshot();
    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    let start_x = creature.get_position().get_x();
    let start_position = creature.get_position();
    let bounding_box = creature.get_bounding_box();
    assert!(creature
        .get_navigator_mut()
        .set_path_to(
            &snapshot,
            start_position,
            bounding_box,
            true,
            PathRequest::from(EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0))
                .with_minimum_distance(0.5),
        )
        .unwrap());

    world.tick();
    world.tick();

    let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    assert!(creature.get_position().get_x() > start_x);
    assert_eq!(creature.ticks(), 2);
}

#[test]
fn perfect_planner_rejects_unloaded_border_and_dimension_bounds() {
    let planner = PerfectPathPlanner::default();
    let budget = PerfectPathBudget {
        maximum_expanded_states: 2_000,
        maximum_ticks: 40,
    };
    let mut start = PerfectMotionState::new(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    start.on_ground = true;
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let unloaded = planner.plan(PerfectPathRequest {
        world: &snapshot,
        bounding_box: EntityType::PLAYER.get_bounding_box(),
        start,
        destination: EntityPosition::new(32.5, 65.0, 0.5, 0.0, 0.0),
        destination_radius: 1.0,
        budget,
        is_cancelled: &|| false,
    });
    assert_eq!(unloaded.termination, PerfectPathTermination::Unreachable);
    assert_eq!(unloaded.expanded_states, 0);

    let mut bordered_world = pathfinding_world();
    bordered_world
        .set_world_border(WorldBorder::new(2.0, 0.0, 0.0, 0, 0, 2).unwrap())
        .unwrap();
    let bordered_snapshot = bordered_world.update_snapshot();
    let outside_border = planner.plan(PerfectPathRequest {
        world: &bordered_snapshot,
        bounding_box: EntityType::PLAYER.get_bounding_box(),
        start,
        destination: EntityPosition::new(5.5, 65.0, 0.5, 0.0, 0.0),
        destination_radius: 1.0,
        budget,
        is_cancelled: &|| false,
    });
    assert_eq!(
        outside_border.termination,
        PerfectPathTermination::Unreachable
    );
    assert_eq!(outside_border.expanded_states, 0);

    let dimension = DimensionType::builder().vertical_bounds(0, 10, 10).build();
    let mut short_world = World::new_with_dimension(
        Identifier::minecraft("short_dimension"),
        DimensionType::OVERWORLD,
        dimension,
    );
    short_world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let short_snapshot = short_world.update_snapshot();
    let outside_dimension = planner.plan(PerfectPathRequest {
        world: &short_snapshot,
        bounding_box: EntityType::PLAYER.get_bounding_box(),
        start,
        destination: EntityPosition::new(1.5, 65.0, 0.5, 0.0, 0.0),
        destination_radius: 1.0,
        budget,
        is_cancelled: &|| false,
    });
    assert_eq!(
        outside_dimension.termination,
        PerfectPathTermination::Unreachable
    );
    assert_eq!(outside_dimension.expanded_states, 0);
}

#[test]
fn perfect_planner_is_deterministic_and_replays_to_predicted_state() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut start = PerfectMotionState::new(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    start.on_ground = true;
    let planner = PerfectPathPlanner::default();
    let budget = PerfectPathBudget {
        maximum_expanded_states: 2_000,
        maximum_ticks: 40,
    };
    let destination = EntityPosition::new(0.5, 65.0, 2.0, 0.0, 0.0);

    let first = planner.plan(PerfectPathRequest {
        world: &snapshot,
        bounding_box: EntityType::PLAYER.get_bounding_box(),
        start,
        destination,
        destination_radius: 1.0,
        budget,
        is_cancelled: &|| false,
    });
    let second = planner.plan(PerfectPathRequest {
        world: &snapshot,
        bounding_box: EntityType::PLAYER.get_bounding_box(),
        start,
        destination,
        destination_radius: 1.0,
        budget,
        is_cancelled: &|| false,
    });
    let plan = first.plan.unwrap();
    let simulator = PerfectMotionSimulator::default();
    let replayed = plan.controls.iter().fold(start, |motion_state, control| {
        simulator.tick(
            &snapshot,
            EntityType::PLAYER.get_bounding_box(),
            motion_state,
            *control,
        )
    });

    assert_eq!(second.plan.unwrap().controls, plan.controls);
    assert_eq!(replayed, plan.final_state);
}

#[test]
fn minestom_follower_accepts_new_path_after_long_jump_landing() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(1, 65, 0), Block::STONE)
        .unwrap();
    world
        .set_block(BlockPosition::new(2, 65, 0), Block::STONE)
        .unwrap();
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature
        .get_entity_mut()
        .get_attribute(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        )
        .set_base_value(Attribute::MOVEMENT_SPEED.default_value());
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    creature.set_on_ground(true);
    let creature_id = creature.get_entity_id();
    world.add_entity(Entity::Creature(creature));
    {
        let snapshot = world.update_snapshot();
        let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
            panic!("creature entity must preserve its subtype");
        };
        let start_position = creature.get_position();
        let bounding_box = creature.get_bounding_box();
        assert!(creature
            .get_navigator_mut()
            .set_path_to(
                &snapshot,
                start_position,
                bounding_box,
                true,
                PathRequest::from(EntityPosition::new(4.5, 66.0, 0.5, 0.0, 0.0))
                    .with_minimum_distance(0.35),
            )
            .unwrap());
    }
    for _ in 0..120 {
        world.tick();
    }
    {
        let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
            panic!("creature entity must preserve its subtype");
        };
        assert!(creature
            .set_path_to(PathRequest::from(EntityPosition::new(
                0.5, 65.0, 0.5, 0.0, 0.0
            )))
            .unwrap());
    }
    world.tick();
    let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };

    assert_ne!(creature.get_navigator().state(), PathState::Invalid);
    assert!(creature
        .get_navigator()
        .get_nodes()
        .is_some_and(|nodes| !nodes.is_empty()));
}
#[test]
fn ground_generator_treats_epsilon_above_ground_landing_as_flat_ground() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(1, 65, 0), Block::STONE)
        .unwrap();
    world
        .set_block(BlockPosition::new(2, 65, 0), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let current = PathNode::new(
        EntityPosition::new(4.150000062584876, 65.00000000002315, 0.5, -90.0, 21.760849),
        0.0,
        0.0,
        PathNodeType::Walk,
    );
    let neighbors = GroundNodeGenerator.walkable(
        &snapshot,
        &HashSet::new(),
        &current,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        EntityType::ZOMBIE.get_bounding_box(),
    );

    assert!(!neighbors.is_empty());
}

#[test]
fn minestom_default_creature_trace_has_no_per_tick_deviation_across_obstacles() {
    let minestom_trace = minestom_reference_trace();
    let spinel_trace = spinel_reference_trace();

    assert_eq!(
        spinel_trace.keys().collect::<Vec<_>>(),
        minestom_trace.keys().collect::<Vec<_>>()
    );
    for (scenario_name, expected_scenario) in &minestom_trace {
        let actual_scenario = spinel_trace
            .get(scenario_name)
            .unwrap_or_else(|| panic!("missing Spinel trace scenario {scenario_name}"));
        assert_eq!(
            actual_scenario.accepted, expected_scenario.accepted,
            "path acceptance deviated for {scenario_name}"
        );
        assert_eq!(
            actual_scenario.ticks.len(),
            expected_scenario.ticks.len(),
            "tick count deviated for {scenario_name}"
        );
        expected_scenario
            .ticks
            .iter()
            .zip(actual_scenario.ticks.iter())
            .for_each(|(expected_tick, actual_tick)| {
                assert_trace_tick_has_no_deviation(scenario_name, expected_tick, actual_tick);
            });
    }
}

#[derive(Clone, Debug)]
struct PathfindingTraceScenario {
    accepted: bool,
    ticks: Vec<PathfindingTraceTick>,
}

#[derive(Clone, Debug)]
struct PathfindingTraceTick {
    tick: usize,
    x: f64,
    y: f64,
    z: f64,
    yaw: f64,
    pitch: f64,
    velocity_x: f64,
    velocity_y: f64,
    velocity_z: f64,
    is_on_ground: bool,
    state: String,
}

#[derive(Clone, Copy)]
enum PathfindingTraceScenarioKind {
    Flat,
    SingleBlock,
    BottomSlab,
    TopSlab,
    Stair,
    TwoHighWall,
    SlabDestination,
}

impl PathfindingTraceScenarioKind {
    const ALL: &'static [(Self, &'static str)] = &[
        (Self::Flat, "flat"),
        (Self::SingleBlock, "single_block"),
        (Self::BottomSlab, "bottom_slab"),
        (Self::TopSlab, "top_slab"),
        (Self::Stair, "stair"),
        (Self::TwoHighWall, "two_high_wall"),
        (Self::SlabDestination, "slab_destination"),
    ];

    const fn goal(self) -> EntityPosition {
        match self {
            Self::Flat => EntityPosition::new(5.5, 65.0, 0.5, 0.0, 0.0),
            Self::SingleBlock
            | Self::BottomSlab
            | Self::TopSlab
            | Self::Stair
            | Self::TwoHighWall => EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
            Self::SlabDestination => EntityPosition::new(1.5, 66.0, 1.5, 0.0, 0.0),
        }
    }

    const fn start(self) -> EntityPosition {
        match self {
            Self::SlabDestination => EntityPosition::new(5.5, 65.0, 1.5, 0.0, 0.0),
            _ => EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        }
    }

    const fn trace_ticks(self) -> usize {
        match self {
            Self::SlabDestination => 400,
            _ => 90,
        }
    }
    fn place_blocks(self, world: &mut World) {
        match self {
            Self::Flat => {}
            Self::SingleBlock => {
                world
                    .set_block(BlockPosition::new(2, 65, 0), Block::STONE)
                    .unwrap();
            }
            Self::BottomSlab => {
                world
                    .set_block(BlockPosition::new(2, 65, 0), Block::STONE_SLAB)
                    .unwrap();
            }
            Self::TopSlab => {
                world
                    .set_block_state(
                        BlockPosition::new(2, 65, 0),
                        block_state_with_property(Block::STONE_SLAB, "type", "top"),
                    )
                    .unwrap();
            }
            Self::Stair => {
                world
                    .set_block_state(
                        BlockPosition::new(2, 65, 0),
                        block_state_with_property(Block::OAK_STAIRS, "facing", "east"),
                    )
                    .unwrap();
            }
            Self::TwoHighWall => {
                world
                    .set_block(BlockPosition::new(2, 65, 0), Block::STONE)
                    .unwrap();
                world
                    .set_block(BlockPosition::new(2, 66, 0), Block::STONE)
                    .unwrap();
            }
            Self::SlabDestination => {
                (0..4)
                    .flat_map(|block_x| (0..4).map(move |block_z| (block_x, block_z)))
                    .for_each(|(block_x, block_z)| {
                        world
                            .set_block(
                                BlockPosition::new(block_x, 65, block_z),
                                Block::OAK_SLAB,
                            )
                            .unwrap();
                    });
            }
        }
    }
}

fn minestom_reference_trace() -> BTreeMap<String, PathfindingTraceScenario> {
    let output = Command::new(gradle_executable())
        .arg("-q")
        .arg("pathfindingTrace")
        .current_dir(minestom_pathfinding_showcase_dir())
        .output()
        .unwrap_or_else(|error| panic!("failed to run Minestom pathfinding trace task: {error}"));
    if output.status.success() {
        return parse_minestom_trace(&String::from_utf8_lossy(&output.stdout));
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if minestom_jar_is_locked_by_running_showcase(&stderr) {
        return minestom_reference_trace_from_running_showcase_classpath();
    }

    panic!(
        "Minestom pathfinding trace task failed:
{stderr}"
    );
}

fn minestom_jar_is_locked_by_running_showcase(stderr: &str) -> bool {
    stderr.contains("Unable to delete file") && stderr.contains("minestom-dev.jar") && cfg!(windows)
}

fn minestom_reference_trace_from_running_showcase_classpath(
) -> BTreeMap<String, PathfindingTraceScenario> {
    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(running_showcase_trace_command())
        .current_dir(workspace_root())
        .output()
        .unwrap_or_else(|error| {
            panic!("failed to run Minestom trace from locked showcase classpath: {error}")
        });
    if !output.status.success() {
        panic!(
            "Minestom trace fallback failed:
{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    parse_minestom_trace(&String::from_utf8_lossy(&output.stdout))
}

fn running_showcase_trace_command() -> String {
    let workspace = workspace_root().to_string_lossy().replace('\'', "''");
    format!(
        r#"$process = Get-CimInstance Win32_Process -Filter "name = 'java.exe' or name = 'javaw.exe'" | Where-Object {{ $_.CommandLine -like '*showcase.PathfindingShowcaseServer*' -and $_.CommandLine -like '*{workspace}*' }} | Select-Object -First 1; if ($null -eq $process) {{ Write-Error 'No running Minestom showcase process is available for locked-jar trace fallback'; exit 64 }}; $commandLine = $process.CommandLine; $classpathStart = $commandLine.IndexOf('-cp ') + 4; $classpathEnd = $commandLine.IndexOf(' showcase.PathfindingShowcaseServer'); if ($classpathStart -lt 4 -or $classpathEnd -le $classpathStart) {{ Write-Error 'The running Minestom showcase command line did not expose a classpath'; exit 65 }}; $classpath = $commandLine.Substring($classpathStart, $classpathEnd - $classpathStart); & 'C:\Program Files\Eclipse Adoptium\jdk-25.0.2.10-hotspot\bin\java.exe' -cp $classpath showcase.PathfindingTraceDumper"#
    )
}

fn spinel_reference_trace() -> BTreeMap<String, PathfindingTraceScenario> {
    PathfindingTraceScenarioKind::ALL
        .iter()
        .map(|(scenario, scenario_name)| {
            ((*scenario_name).to_string(), spinel_trace_for(*scenario))
        })
        .collect()
}

fn spinel_trace_for(scenario: PathfindingTraceScenarioKind) -> PathfindingTraceScenario {
    let mut world = minestom_trace_world();
    scenario.place_blocks(&mut world);
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature
        .get_entity_mut()
        .get_attribute(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        )
        .set_base_value(Attribute::MOVEMENT_SPEED.default_value());
    creature.set_position(scenario.start());
    let creature_id = creature.get_entity_id();
    world.add_entity(Entity::Creature(creature));
    let snapshot = world.update_snapshot();
    let Entity::Creature(creature) = world.entity_by_id_mut(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    let accepted = creature
        .set_path_to_in_world(&snapshot, PathRequest::from(scenario.goal()))
        .unwrap();
    let mut ticks = vec![trace_tick_from_creature(0, creature)];
    for tick in 1..=scenario.trace_ticks() {
        world.tick();
        let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
            panic!("creature entity must preserve its subtype");
        };
        ticks.push(trace_tick_from_creature(tick, creature));
    }
    PathfindingTraceScenario { accepted, ticks }
}

fn minestom_trace_world() -> World {
    let mut world = pathfinding_world_in_chunk_range(-2, 2, -2, 2);
    for x in -32..=32 {
        for z in -32..=32 {
            world
                .set_block(BlockPosition::new(x, 64, z), Block::STONE)
                .unwrap();
        }
    }
    world
}

fn trace_tick_from_creature(tick: usize, creature: &EntityCreature) -> PathfindingTraceTick {
    let position = creature.get_position();
    let velocity = creature.get_velocity().0;
    PathfindingTraceTick {
        tick,
        x: position.get_x(),
        y: position.get_y(),
        z: position.get_z(),
        yaw: f64::from(position.get_yaw()),
        pitch: f64::from(position.get_pitch()),
        velocity_x: velocity.x,
        velocity_y: velocity.y,
        velocity_z: velocity.z,
        is_on_ground: creature.is_on_ground(),
        state: minestom_state_name(creature.get_navigator().state()).to_string(),
    }
}

fn parse_minestom_trace(trace: &str) -> BTreeMap<String, PathfindingTraceScenario> {
    let mut scenarios = BTreeMap::new();
    for line in trace.lines().filter(|line| line.starts_with("scenario\t")) {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 || fields[2] != "accepted" {
            panic!("malformed Minestom scenario line: {line}");
        }
        scenarios.insert(
            fields[1].to_string(),
            PathfindingTraceScenario {
                accepted: parse_bool(fields[3], line),
                ticks: Vec::new(),
            },
        );
    }
    for line in trace.lines().filter(|line| line.starts_with("tick\t")) {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 13 {
            panic!("malformed Minestom tick line: {line}");
        }
        let scenario = scenarios
            .get_mut(fields[1])
            .unwrap_or_else(|| panic!("tick referenced missing scenario: {line}"));
        scenario.ticks.push(PathfindingTraceTick {
            tick: parse_usize(fields[2], line),
            x: parse_f64(fields[3], line),
            y: parse_f64(fields[4], line),
            z: parse_f64(fields[5], line),
            yaw: parse_f64(fields[6], line),
            pitch: parse_f64(fields[7], line),
            velocity_x: parse_f64(fields[8], line),
            velocity_y: parse_f64(fields[9], line),
            velocity_z: parse_f64(fields[10], line),
            is_on_ground: parse_bool(fields[11], line),
            state: fields[12].to_string(),
        });
    }
    scenarios
}

fn assert_trace_tick_has_no_deviation(
    scenario_name: &str,
    expected: &PathfindingTraceTick,
    actual: &PathfindingTraceTick,
) {
    assert_eq!(
        actual.tick, expected.tick,
        "tick index deviated for {scenario_name}"
    );
    assert_float_has_no_deviation(scenario_name, expected.tick, "x", expected.x, actual.x);
    assert_float_has_no_deviation(scenario_name, expected.tick, "y", expected.y, actual.y);
    assert_float_has_no_deviation(scenario_name, expected.tick, "z", expected.z, actual.z);
    assert_float_has_no_deviation(
        scenario_name,
        expected.tick,
        "yaw",
        expected.yaw,
        actual.yaw,
    );
    assert_float_has_no_deviation(
        scenario_name,
        expected.tick,
        "pitch",
        expected.pitch,
        actual.pitch,
    );
    assert_float_has_no_deviation(
        scenario_name,
        expected.tick,
        "velocity_x",
        expected.velocity_x,
        actual.velocity_x,
    );
    assert_float_has_no_deviation(
        scenario_name,
        expected.tick,
        "velocity_y",
        expected.velocity_y,
        actual.velocity_y,
    );
    assert_float_has_no_deviation(
        scenario_name,
        expected.tick,
        "velocity_z",
        expected.velocity_z,
        actual.velocity_z,
    );
    assert_eq!(
        actual.is_on_ground, expected.is_on_ground,
        "ground flag deviated for {scenario_name} tick {}",
        expected.tick
    );
    assert_eq!(
        actual.state, expected.state,
        "navigator state deviated for {scenario_name} tick {}",
        expected.tick
    );
}

fn assert_float_has_no_deviation(
    scenario_name: &str,
    tick: usize,
    field: &str,
    expected: f64,
    actual: f64,
) {
    assert_eq!(
        actual.to_bits(),
        expected.to_bits(),
        "{field} deviated for {scenario_name} tick {tick}: expected {expected}, actual {actual}"
    );
}

fn minestom_state_name(state: PathState) -> &'static str {
    match state {
        PathState::Calculating => "CALCULATING",
        PathState::Following => "FOLLOWING",
        PathState::Terminating => "TERMINATING",
        PathState::Terminated => "TERMINATED",
        PathState::Computed => "COMPUTED",
        PathState::BestEffort => "BEST_EFFORT",
        PathState::Invalid => "INVALID",
    }
}

fn block_state_with_property(block: Block, property: &str, value: &str) -> BlockState {
    block
        .default_state()
        .with_property(property, value)
        .unwrap_or_else(|| panic!("{block:?} must support {property}={value}"))
}

fn parse_usize(value: &str, line: &str) -> usize {
    value
        .parse()
        .unwrap_or_else(|error| panic!("failed to parse usize in {line}: {error}"))
}

fn parse_f64(value: &str, line: &str) -> f64 {
    value
        .parse()
        .unwrap_or_else(|error| panic!("failed to parse f64 in {line}: {error}"))
}

fn parse_bool(value: &str, line: &str) -> bool {
    value
        .parse()
        .unwrap_or_else(|error| panic!("failed to parse bool in {line}: {error}"))
}

fn minestom_pathfinding_showcase_dir() -> PathBuf {
    workspace_root().join("minestom-pathfinding-showcase")
}

fn gradle_executable() -> PathBuf {
    if let Some(configured_gradle) = env::var_os("SPINEL_GRADLE") {
        return PathBuf::from(configured_gradle);
    }
    if cfg!(windows) {
        return gradle_wrapper_distribution_executable()
            .unwrap_or_else(|| PathBuf::from("gradle.bat"));
    }
    PathBuf::from("gradle")
}

fn gradle_wrapper_distribution_executable() -> Option<PathBuf> {
    let user_home = env::var_os("USERPROFILE").or_else(|| env::var_os("HOME"))?;
    let distribution_root = PathBuf::from(user_home)
        .join(".gradle")
        .join("wrapper")
        .join("dists");
    let mut pending = vec![distribution_root];
    let mut candidates = Vec::new();
    while let Some(directory) = pending.pop() {
        let Ok(entries) = fs::read_dir(directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            if path.file_name().and_then(|name| name.to_str()) == Some("gradle.bat") {
                candidates.push(path);
            }
        }
    }
    candidates.sort();
    candidates.pop()
}

fn workspace_root() -> PathBuf {
    FsPath::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(FsPath::parent)
        .unwrap_or_else(|| panic!("could not resolve SpinelandSteel workspace root"))
        .to_path_buf()
}

fn pathfinding_world() -> World {
    pathfinding_world_in_chunk_range(0, 0, 0, 0)
}

fn pathfinding_world_in_chunk_range(
    minimum_chunk_x: i32,
    maximum_chunk_x: i32,
    minimum_chunk_z: i32,
    maximum_chunk_z: i32,
) -> World {
    let mut world = World::new(Identifier::minecraft("pathfinding"));
    world.set_chunk_supplier(|chunk_position| {
        let mut chunk = Chunk::new_with_generation(chunk_position, false);
        let floor_section = chunk.section_mut(4).unwrap();
        (0..256).for_each(|block_index| {
            floor_section
                .block_palette_mut()
                .set(block_index, Block::STONE.default_state());
        });
        chunk
    });
    for chunk_x in minimum_chunk_x..=maximum_chunk_x {
        for chunk_z in minimum_chunk_z..=maximum_chunk_z {
            world
                .load_chunk(ChunkPosition::new(chunk_x, chunk_z))
                .unwrap();
        }
    }
    world
}

fn assert_valid_default_path(
    world: &World,
    start: EntityPosition,
    goal: EntityPosition,
    bounding_box: EntityBoundingBox,
) {
    let snapshot = world.update_snapshot();
    let mut navigator = Navigator::default();

    assert!(
        navigator
            .set_path_to(
                &snapshot,
                start,
                bounding_box,
                true,
                PathRequest::from(goal)
            )
            .unwrap(),
        "path request was rejected from {start:?} to {goal:?}"
    );
    let nodes = navigator.get_nodes().unwrap();
    assert!(
        !nodes.is_empty(),
        "empty path from {start:?} to {goal:?} in state {:?}",
        navigator.state()
    );
    assert!(nodes.iter().all(|node| {
        let (block_x, block_y, block_z) = node.get_block_coordinates();
        !snapshot
            .block(BlockPosition::new(block_x, block_y, block_z))
            .is_solid()
    }));
}

struct RecordingNodeFollower {
    events: Arc<Mutex<Vec<&'static str>>>,
}

impl NodeFollower for RecordingNodeFollower {
    fn move_towards(
        &self,
        _entity: &mut GenericEntity,
        _world: &crate::world::WorldSnapshot,
        _target: EntityPosition,
        _speed: f64,
        _look_at: EntityPosition,
    ) {
        self.events.lock().unwrap().push("move");
    }

    fn jump(
        &self,
        _entity: &mut GenericEntity,
        _point: Option<EntityPosition>,
        _target: Option<EntityPosition>,
    ) {
        self.events.lock().unwrap().push("jump");
    }

    fn is_at_point(&self, _entity: &GenericEntity, _point: EntityPosition) -> bool {
        false
    }

    fn movement_speed(&self, _entity: &GenericEntity) -> f64 {
        1.0
    }
}

struct SingleBestEffortNodeGenerator;

impl NodeGenerator for SingleBestEffortNodeGenerator {
    fn walkable(
        &self,
        _world: &crate::world::WorldSnapshot,
        visited: &HashSet<PathNode>,
        current: &PathNode,
        goal: EntityPosition,
        _bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode> {
        if current.get_block_coordinates() != (0, 65, 0) {
            return Vec::new();
        }
        let position = EntityPosition::new(2.5, 65.0, 0.5, 0.0, 0.0);
        let mut node = PathNode::new(
            position,
            1.0,
            self.heuristic(position, goal),
            PathNodeType::Walk,
        );
        node.set_parent(Some(current.clone()));
        if visited.contains(&node) {
            return Vec::new();
        }
        vec![node]
    }

    fn has_gravity_snap(&self) -> bool {
        false
    }

    fn gravity_snap(
        &self,
        _world: &crate::world::WorldSnapshot,
        _position: EntityPosition,
        _bounding_box: EntityBoundingBox,
        _maximum_fall: i32,
    ) -> Option<f64> {
        None
    }
}

struct PriorityTieNodeGenerator {
    expanded_nodes: Arc<Mutex<Vec<(i32, i32, i32)>>>,
}

impl NodeGenerator for PriorityTieNodeGenerator {
    fn walkable(
        &self,
        _world: &crate::world::WorldSnapshot,
        _visited: &HashSet<PathNode>,
        current: &PathNode,
        _goal: EntityPosition,
        _bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode> {
        self.expanded_nodes
            .lock()
            .unwrap()
            .push(current.get_block_coordinates());
        if current.get_block_coordinates() != (0, 65, 0) {
            return Vec::new();
        }
        vec![
            PathNode::new(
                EntityPosition::new(1.5, 65.0, 0.5, 0.0, 0.0),
                1.0011,
                0.0,
                PathNodeType::Walk,
            ),
            PathNode::new(
                EntityPosition::new(2.5, 65.0, 0.5, 0.0, 0.0),
                1.0009,
                0.0,
                PathNodeType::Walk,
            ),
        ]
    }

    fn has_gravity_snap(&self) -> bool {
        false
    }

    fn gravity_snap(
        &self,
        _world: &crate::world::WorldSnapshot,
        _position: EntityPosition,
        _bounding_box: EntityBoundingBox,
        _maximum_fall: i32,
    ) -> Option<f64> {
        None
    }
}

struct FrontierBudgetNodeGenerator;

impl NodeGenerator for FrontierBudgetNodeGenerator {
    fn walkable(
        &self,
        _world: &crate::world::WorldSnapshot,
        _visited: &HashSet<PathNode>,
        current: &PathNode,
        goal: EntityPosition,
        _bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode> {
        if current.get_block_coordinates() != (0, 65, 0) {
            return Vec::new();
        }
        (-4..=4)
            .flat_map(|offset_x| {
                (-4..=4).filter_map(move |offset_z| {
                    let position = EntityPosition::new(
                        f64::from(offset_x) + 0.5,
                        65.0,
                        f64::from(offset_z) + 0.5,
                        0.0,
                        0.0,
                    );
                    (position.get_distance_squared(current.get_position()) <= 25.0).then(|| {
                        let mut node = PathNode::new(
                            position,
                            1.0,
                            self.heuristic(position, goal),
                            PathNodeType::Walk,
                        );
                        node.set_parent(Some(current.clone()));
                        node
                    })
                })
            })
            .collect()
    }

    fn has_gravity_snap(&self) -> bool {
        false
    }

    fn gravity_snap(
        &self,
        _world: &crate::world::WorldSnapshot,
        _position: EntityPosition,
        _bounding_box: EntityBoundingBox,
        _maximum_fall: i32,
    ) -> Option<f64> {
        None
    }
}
