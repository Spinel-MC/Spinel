use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_registry::{EntityBoundingBox, EntityType};
use spinel_server::entity::ai::{EntityAiGroupBuilder, GoalSelector, TargetSelector};
use spinel_server::entity::pathfinding::{
    FlyingNodeFollower, GroundNodeFollower, Navigator, NoPhysicsNodeFollower, NodeFollower,
    NodeGenerator, PathGenerator, PathNode, PathNodeType, PathState, VanillaGroundNodeFollower,
    WaterNodeFollower, WaterNodeGenerator,
};
use spinel_server::entity::{CreatureEntity, Entity, EntityPosition, GenericEntity};
use spinel_server::world::{Block, BlockPosition, Chunk, ChunkPosition, World};
use std::collections::HashSet;

const MINESTOM_GROUND_JUMP_VELOCITY: f64 = 10.0;

#[test]
fn ground_follower_applies_minestom_displacement_immediately() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = GroundNodeFollower;
    let mut entity = grounded_zombie();
    let start = entity.position();
    let target = EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);
    let minestom_position = minestom_ground_position(start, target, speed, target);

    follower.move_towards(&mut entity, &snapshot, target, speed, target);

    assert_eq!(entity.position(), minestom_position);
    assert_eq!(
        entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    );
}

#[test]
fn ground_follower_applies_minestom_body_and_head_view() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = GroundNodeFollower;
    let mut entity = grounded_zombie();
    let target = EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0);
    let look_at = EntityPosition::new(4.5, 68.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);
    let minestom_position = minestom_ground_position(entity.position(), target, speed, look_at);

    follower.move_towards(&mut entity, &snapshot, target, speed, look_at);

    assert_eq!(entity.position().yaw(), minestom_position.yaw());
    assert_eq!(entity.position().head_yaw(), minestom_position.head_yaw());
    assert_eq!(entity.position().pitch(), minestom_position.pitch());
}

#[test]
fn ground_follower_resolves_collision_during_the_minestom_move() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(1, 65, 0), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let follower = GroundNodeFollower;
    let mut entity = grounded_zombie();
    let target = EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);

    follower.move_towards(&mut entity, &snapshot, target, speed, target);

    assert!(entity.position().x() <= 0.7);
    assert_eq!(
        entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    );
}

#[test]
fn ground_follower_jump_replaces_velocity_with_minestom_impulse() {
    let follower = GroundNodeFollower;
    let mut world = pathfinding_world();
    let creature = CreatureEntity::new(EntityType::ZOMBIE);
    let creature_id = creature.entity_id();
    world.add_entity(Entity::Creature(creature));
    world
        .set_entity_velocity(
            creature_id,
            Velocity(Vector3d {
                x: 3.0,
                y: 0.0,
                z: -2.0,
            }),
        )
        .unwrap();
    let entity = world.creature_by_id_mut(creature_id).unwrap();
    entity.set_on_ground(true);

    follower.jump(
        entity,
        Some(EntityPosition::new(1.5, 66.0, 0.5, 0.0, 0.0)),
        Some(EntityPosition::new(2.5, 66.0, 0.5, 0.0, 0.0)),
    );

    assert_eq!(
        entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: MINESTOM_GROUND_JUMP_VELOCITY,
            z: 0.0,
        })
    );
}

#[test]
fn ground_follower_jump_is_a_minestom_no_op_while_airborne() {
    let follower = GroundNodeFollower;
    let mut world = pathfinding_world();
    let creature = CreatureEntity::new(EntityType::ZOMBIE);
    let creature_id = creature.entity_id();
    world.add_entity(Entity::Creature(creature));
    let airborne_velocity = Velocity(Vector3d {
        x: 3.0,
        y: -1.0,
        z: -2.0,
    });
    world
        .set_entity_velocity(creature_id, airborne_velocity)
        .unwrap();
    let entity = world.creature_by_id_mut(creature_id).unwrap();
    entity.set_on_ground(false);

    follower.jump(
        entity,
        Some(EntityPosition::new(1.5, 66.0, 0.5, 0.0, 0.0)),
        Some(EntityPosition::new(2.5, 66.0, 0.5, 0.0, 0.0)),
    );

    assert_eq!(entity.velocity(), airborne_velocity);
}

#[test]
fn ground_follower_exposes_minestom_custom_jump_height() {
    let follower = GroundNodeFollower;
    let mut entity = grounded_zombie();

    follower.jump_with_height(&mut entity, 2.0);

    assert_eq!(
        entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        })
    );
}

#[test]
fn no_physics_follower_matches_minestom_position_and_view() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = NoPhysicsNodeFollower;
    let mut entity = grounded_zombie();
    let start = entity.position();
    let target = EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0);
    let look_at = EntityPosition::new(4.5, 68.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);
    let minestom_position = minestom_ground_position(start, target, speed, look_at);

    follower.move_towards(&mut entity, &snapshot, target, speed, look_at);

    assert_eq!(entity.position(), minestom_position);
}

#[test]
fn no_physics_follower_jump_replaces_velocity_with_minestom_impulse() {
    let follower = NoPhysicsNodeFollower;
    let mut world = pathfinding_world();
    let creature = CreatureEntity::new(EntityType::ZOMBIE);
    let creature_id = creature.entity_id();
    world.add_entity(Entity::Creature(creature));
    world
        .set_entity_velocity(
            creature_id,
            Velocity(Vector3d {
                x: 3.0,
                y: 0.0,
                z: -2.0,
            }),
        )
        .unwrap();
    let entity = world.creature_by_id_mut(creature_id).unwrap();
    entity.set_on_ground(true);

    follower.jump(
        entity,
        Some(EntityPosition::new(1.5, 66.0, 0.5, 0.0, 0.0)),
        Some(EntityPosition::new(2.5, 66.0, 0.5, 0.0, 0.0)),
    );

    assert_eq!(
        entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: MINESTOM_GROUND_JUMP_VELOCITY,
            z: 0.0,
        })
    );
}

#[test]
fn no_physics_follower_exposes_minestom_custom_jump_height() {
    let follower = NoPhysicsNodeFollower;
    let mut entity = grounded_zombie();

    follower.jump_with_height(&mut entity, 2.0);

    assert_eq!(
        entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        })
    );
}

#[test]
fn no_physics_follower_jumps_toward_a_higher_minestom_target() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = NoPhysicsNodeFollower;
    let mut entity = grounded_zombie();
    let start = entity.position();
    let target = EntityPosition::new(4.5, 66.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);
    let minestom_position = minestom_ground_position(start, target, speed, target);

    follower.move_towards(&mut entity, &snapshot, target, speed, target);

    assert_eq!(entity.position(), minestom_position);
    assert_eq!(
        entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: MINESTOM_GROUND_JUMP_VELOCITY,
            z: 0.0,
        })
    );
}

#[test]
fn creature_navigation_moves_and_turns_on_the_first_world_tick() {
    let mut world = pathfinding_world();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    creature.set_on_ground(true);
    let creature_id = creature.entity_id();
    world.add_entity(Entity::Creature(creature));
    let snapshot = world.update_snapshot();
    let creature = world.creature_by_id_mut(creature_id).unwrap();
    let start = creature.position();
    assert!(creature.set_path_to_default(
        &snapshot,
        Some(EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0)),
    ));

    world.tick();

    let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    assert!(creature.position().x() > start.x());
    assert_eq!(creature.position().yaw().round() as i32, -90);
    assert_eq!(creature.position().head_yaw(), creature.position().yaw());
    assert_eq!(creature.position().pitch(), 0.0);
}

#[test]
fn creature_ai_path_start_moves_on_the_same_minestom_tick() {
    let mut world = pathfinding_world();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    creature.set_on_ground(true);
    creature.add_ai_group(
        EntityAiGroupBuilder::default()
            .add_goal_selector(StartPathGoal {
                goal: EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
                has_started: false,
            })
            .build(),
    );
    let creature_id = creature.entity_id();
    let start = creature.position();
    world.add_entity(Entity::Creature(creature));

    world.tick();

    let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    assert!(creature.position().x() > start.x());
}

#[test]
fn creature_ai_path_reset_prevents_movement_on_the_same_minestom_tick() {
    let mut world = pathfinding_world();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    creature.set_on_ground(true);
    creature.add_ai_group(
        EntityAiGroupBuilder::default()
            .add_goal_selector(ResetPathGoal {
                goal: EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
                has_started: false,
            })
            .build(),
    );
    let creature_id = creature.entity_id();
    let start = creature.position();
    world.add_entity(Entity::Creature(creature));

    world.tick();

    let Entity::Creature(creature) = world.entity_by_id(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    assert_eq!(creature.position(), start);
    assert_eq!(creature.navigator().goal_position(), None);
}

#[test]
fn navigator_moves_toward_current_node_while_facing_next_node() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = grounded_zombie();
    let start = entity.position();
    let current_target = EntityPosition::new(1.5, 65.0, 0.5, 0.0, 0.0);
    let next_target = EntityPosition::new(1.5, 65.0, 2.5, 0.0, 0.0);
    let mut navigator = navigator_following_nodes(
        &snapshot,
        &mut entity,
        EntityPosition::new(4.5, 65.0, 4.5, 0.0, 0.0),
        vec![
            PathNode::new(current_target, 0.0, 0.0, PathNodeType::Walk),
            PathNode::new(next_target, 0.0, 0.0, PathNodeType::Walk),
        ],
    );
    let speed = GroundNodeFollower.movement_speed(&entity);
    let minestom_position = minestom_ground_position(start, current_target, speed, next_target);

    navigator.tick(&mut entity, &snapshot, false);

    assert_eq!(entity.position(), minestom_position);
}

#[test]
fn navigator_faces_current_node_before_a_minestom_repath_sentinel() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = grounded_zombie();
    let start = entity.position();
    let current_target = EntityPosition::new(1.5, 65.0, 2.5, 0.0, 0.0);
    let mut navigator = navigator_following_nodes(
        &snapshot,
        &mut entity,
        EntityPosition::new(4.5, 65.0, 4.5, 0.0, 0.0),
        vec![
            PathNode::new(current_target, 0.0, 0.0, PathNodeType::Walk),
            PathNode::new(EntityPosition::default(), 0.0, 0.0, PathNodeType::Repath),
        ],
    );
    let speed = GroundNodeFollower.movement_speed(&entity);
    let minestom_position = minestom_ground_position(start, current_target, speed, current_target);

    navigator.tick(&mut entity, &snapshot, false);

    assert_eq!(entity.position(), minestom_position);
}

#[test]
fn navigator_advances_after_reaching_the_current_minestom_node_block() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut entity = grounded_zombie();
    let current_target = EntityPosition::new(0.8, 65.0, 0.5, 0.0, 0.0);
    let next_target = EntityPosition::new(2.5, 65.0, 0.5, 0.0, 0.0);
    let mut navigator = navigator_following_nodes(
        &snapshot,
        &mut entity,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
        vec![
            PathNode::new(current_target, 0.0, 0.0, PathNodeType::Walk),
            PathNode::new(next_target, 0.0, 0.0, PathNodeType::Walk),
        ],
    );

    navigator.tick(&mut entity, &snapshot, false);

    assert_eq!(navigator.path().unwrap().current(), Some(next_target));
}

#[test]
fn navigator_jumps_only_for_the_current_minestom_jump_node() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let current_target = EntityPosition::new(1.5, 66.0, 0.5, 0.0, 0.0);
    let next_target = EntityPosition::new(2.5, 66.0, 0.5, 0.0, 0.0);
    let goal = EntityPosition::new(4.5, 66.0, 0.5, 0.0, 0.0);
    let mut walking_entity = grounded_zombie();
    let mut walking_navigator = navigator_following_nodes(
        &snapshot,
        &mut walking_entity,
        goal,
        vec![
            PathNode::new(current_target, 0.0, 0.0, PathNodeType::Walk),
            PathNode::new(next_target, 0.0, 0.0, PathNodeType::Walk),
        ],
    );
    let mut jumping_entity = grounded_zombie();
    let mut jumping_navigator = navigator_following_nodes(
        &snapshot,
        &mut jumping_entity,
        goal,
        vec![
            PathNode::new(current_target, 0.0, 0.0, PathNodeType::Jump),
            PathNode::new(next_target, 0.0, 0.0, PathNodeType::Walk),
        ],
    );

    walking_navigator.tick(&mut walking_entity, &snapshot, false);
    jumping_navigator.tick(&mut jumping_entity, &snapshot, false);

    assert_eq!(
        walking_entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    );
    assert_eq!(
        jumping_entity.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: MINESTOM_GROUND_JUMP_VELOCITY,
            z: 0.0,
        })
    );
}

#[test]
fn follower_view_is_encoded_in_body_and_head_packets() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = GroundNodeFollower;
    let mut entity = grounded_zombie();
    let previous_position = entity.position();
    let target = EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0);
    let look_at = EntityPosition::new(4.5, 68.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);

    follower.move_towards(&mut entity, &snapshot, target, speed, look_at);

    let movement_packet = entity.position_and_rotation_delta_packet(previous_position, true);
    let rotation_packet = entity.rotation_packet(true);
    let head_look_packet = entity.head_look_packet();
    assert_ne!(movement_packet.delta_x, 0);
    assert_eq!(movement_packet.yaw.0, entity.position().yaw());
    assert_eq!(movement_packet.pitch.0, entity.position().pitch());
    assert_eq!(rotation_packet.yaw.0, entity.position().yaw());
    assert_eq!(rotation_packet.pitch.0, entity.position().pitch());
    assert_eq!(head_look_packet.head_yaw.0, entity.position().head_yaw());
}

#[test]
fn flying_follower_matches_minestom_position_and_view() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = FlyingNodeFollower;
    let mut entity = grounded_zombie();
    let start = entity.position();
    let target = EntityPosition::new(4.5, 67.0, 0.5, 0.0, 0.0);
    let look_at = EntityPosition::new(4.5, 68.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);
    let minestom_position = minestom_flying_position(start, target, speed, look_at);

    follower.move_towards(&mut entity, &snapshot, target, speed, look_at);

    assert_eq!(entity.position(), minestom_position);
}

#[test]
fn water_follower_halves_minestom_speed_in_liquid() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(0, 65, 0), Block::WATER)
        .unwrap();
    let snapshot = world.update_snapshot();
    let follower = WaterNodeFollower;
    let mut entity = grounded_zombie();
    let start = entity.position();
    let target = EntityPosition::new(4.5, 67.0, 0.5, 0.0, 0.0);
    let look_at = EntityPosition::new(4.5, 68.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);
    let minestom_position = minestom_flying_position(start, target, speed * 0.5, look_at);

    follower.move_towards(&mut entity, &snapshot, target, speed, look_at);

    assert_eq!(entity.position(), minestom_position);
}

#[test]
fn water_follower_keeps_minestom_speed_outside_liquid() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = WaterNodeFollower;
    let mut entity = grounded_zombie();
    let start = entity.position();
    let target = EntityPosition::new(4.5, 67.0, 0.5, 0.0, 0.0);
    let look_at = EntityPosition::new(4.5, 68.0, 0.5, 0.0, 0.0);
    let speed = follower.movement_speed(&entity);
    let minestom_position = minestom_flying_position(start, target, speed, look_at);

    follower.move_towards(&mut entity, &snapshot, target, speed, look_at);

    assert_eq!(entity.position(), minestom_position);
}

#[test]
fn minestom_water_generator_rejects_water_through_default_direct_movement_check() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(0, 65, 1), Block::WATER)
        .unwrap();
    let snapshot = world.update_snapshot();
    let current = PathNode::new(
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        0.0,
        0.0,
        PathNodeType::Swim,
    );

    let nodes = WaterNodeGenerator.walkable(
        &snapshot,
        &HashSet::new(),
        &current,
        EntityPosition::new(0.5, 65.0, 1.5, 0.0, 0.0),
        EntityType::ZOMBIE.bounding_box(),
    );

    assert!(nodes.is_empty());
}

#[test]
fn minestom_point_invalid_includes_a_block_touching_the_exact_box_top() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(0, 67, 0), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();

    assert!(WaterNodeGenerator.point_is_invalid(
        &snapshot,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        EntityBoundingBox::new(0.6, 2.0, 0.6),
    ));
}

#[test]
fn minestom_point_invalid_applies_the_bounding_box_offset() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(0, 68, 0), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let offset_box = EntityBoundingBox::new(0.6, 2.0, 0.6).with_offset(-0.3, 1.0, -0.3);

    assert!(WaterNodeGenerator.point_is_invalid(
        &snapshot,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        offset_box,
    ));
}

#[test]
fn minestom_path_generation_uses_the_parent_chain_supplied_by_the_node_generator() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let path = PathGenerator::generate(
        &snapshot,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        EntityPosition::new(1.5, 65.0, 0.5, 0.0, 0.0),
        EntityType::ZOMBIE.bounding_box(),
        true,
        0.8,
        50.0,
        20.0,
        &ParentlessNodeGenerator,
        None,
    );

    assert_eq!(path.state(), PathState::Invalid);
    assert!(path.nodes().is_empty());
}

#[test]
fn minestom_negative_completion_distance_does_not_complete_a_path_request() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let mut navigator = Navigator::default();

    assert!(navigator.set_path_to(
        &snapshot,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
        Some(EntityPosition::new(1.5, 65.0, 0.5, 0.0, 0.0)),
        EntityType::ZOMBIE.bounding_box(),
        true,
        -2.0,
        50.0,
        20.0,
        None,
    ));
}

#[test]
fn vanilla_ground_follower_uses_move_control_turning_and_ground_acceleration_constants() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = VanillaGroundNodeFollower::default();
    let mut entity = grounded_zombie();
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 90.0, 0.0));
    let start = entity.position();
    let speed = follower.movement_speed(&entity);

    follower.move_towards(
        &mut entity,
        &snapshot,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
        speed,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
    );

    let expected_acceleration_per_tick =
        0.23000000417232513 * (0.23000000417232513 * 0.21600002 / 0.6_f64.powi(3));
    assert_eq!(entity.position().x(), start.x());
    assert_eq!(entity.position().z(), start.z());
    assert_eq!(entity.position().yaw(), 0.0);
    assert!(entity.velocity().0.x.abs() < 0.000001);
    assert!((entity.velocity().0.z - expected_acceleration_per_tick * 20.0).abs() < 0.000001);
}

#[test]
fn vanilla_ground_follower_uses_air_control_and_vanilla_jump_strength() {
    let world = pathfinding_world();
    let snapshot = world.update_snapshot();
    let follower = VanillaGroundNodeFollower::default();
    let mut entity = grounded_zombie();

    follower.jump(
        &mut entity,
        Some(EntityPosition::new(0.5, 66.0, 0.5, 0.0, 0.0)),
        None,
    );
    assert!((entity.velocity().0.y - 0.41999998688697815 * 20.0).abs() < 0.000001);

    let follower = VanillaGroundNodeFollower::default();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let speed = follower.movement_speed(&entity);
    follower.move_towards(
        &mut entity,
        &snapshot,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
        speed,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
    );

    assert!((entity.velocity().0.x - 0.23000000417232513 * 0.02 * 20.0).abs() < 0.000001);
}

#[test]
fn vanilla_ground_follower_jumps_when_occupying_a_non_door_non_fence_collision_shape() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(0, 65, 0), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let follower = VanillaGroundNodeFollower::default();
    let mut entity = grounded_zombie();
    let speed = follower.movement_speed(&entity);

    follower.move_towards(
        &mut entity,
        &snapshot,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
        speed,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
    );

    assert!((entity.velocity().0.y - 0.41999998688697815 * 20.0).abs() < 0.000001);
}

#[test]
fn vanilla_ground_follower_does_not_auto_jump_from_door_or_fence_shapes() {
    let follower = VanillaGroundNodeFollower::default();
    for block in [Block::OAK_DOOR, Block::OAK_FENCE] {
        let mut world = pathfinding_world();
        world
            .set_block(BlockPosition::new(0, 65, 0), block)
            .unwrap();
        let snapshot = world.update_snapshot();
        let mut entity = grounded_zombie();
        let speed = follower.movement_speed(&entity);

        follower.move_towards(
            &mut entity,
            &snapshot,
            EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
            speed,
            EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
        );

        assert_eq!(entity.velocity().0.y, 0.0);
    }
}

#[test]
fn vanilla_ground_follower_preserves_yaw_during_move_control_jumping_state() {
    let mut world = pathfinding_world();
    world
        .set_block(BlockPosition::new(0, 65, 0), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let follower = VanillaGroundNodeFollower::default();
    let mut entity = grounded_zombie();
    let speed = follower.movement_speed(&entity);

    follower.move_towards(
        &mut entity,
        &snapshot,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
        speed,
        EntityPosition::new(4.5, 65.0, 0.5, 0.0, 0.0),
    );
    let jumping_yaw = entity.position().yaw();
    entity.set_on_ground(false);

    follower.move_towards(
        &mut entity,
        &snapshot,
        EntityPosition::new(-4.5, 65.0, 0.5, 0.0, 0.0),
        speed,
        EntityPosition::new(-4.5, 65.0, 0.5, 0.0, 0.0),
    );

    assert_eq!(entity.position().yaw(), jumping_yaw);
}

#[test]
fn minestom_and_vanilla_followers_move_simultaneously_in_one_world() {
    let mut world = pathfinding_world();
    let mut minestom_zombie = CreatureEntity::new(EntityType::ZOMBIE);
    minestom_zombie.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    let minestom_id = minestom_zombie.entity_id();
    let mut vanilla_zombie = CreatureEntity::new(EntityType::ZOMBIE);
    vanilla_zombie.set_position(EntityPosition::new(0.5, 65.0, 3.5, 0.0, 0.0));
    vanilla_zombie
        .navigator_mut()
        .set_node_follower(VanillaGroundNodeFollower::default());
    let vanilla_id = vanilla_zombie.entity_id();
    assert!(world.add_entity(Entity::Creature(minestom_zombie)));
    assert!(world.add_entity(Entity::Creature(vanilla_zombie)));
    let snapshot = world.update_snapshot();
    let minestom_start = world.entity_by_id(minestom_id).unwrap().position();
    let vanilla_start = world.entity_by_id(vanilla_id).unwrap().position();

    assert!(
        world
            .creature_by_id_mut(minestom_id)
            .unwrap()
            .set_path_to_default(
                &snapshot,
                Some(EntityPosition::new(6.5, 65.0, 0.5, 0.0, 0.0)),
            )
    );
    assert!(
        world
            .creature_by_id_mut(vanilla_id)
            .unwrap()
            .set_path_to_default(
                &snapshot,
                Some(EntityPosition::new(6.5, 65.0, 3.5, 0.0, 0.0)),
            )
    );

    world.tick();
    world.tick();

    assert!(world.entity_by_id(minestom_id).unwrap().position().x() > minestom_start.x());
    assert!(world.entity_by_id(vanilla_id).unwrap().position().x() > vanilla_start.x());
}

#[test]
fn vanilla_navigation_acceleration_moves_the_entity_on_the_same_world_tick() {
    let mut world = pathfinding_world();
    let mut vanilla_zombie = CreatureEntity::new(EntityType::ZOMBIE);
    vanilla_zombie.set_position(EntityPosition::new(0.5, 65.0, 3.5, 0.0, 0.0));
    vanilla_zombie
        .navigator_mut()
        .set_node_follower(VanillaGroundNodeFollower::default());
    let vanilla_id = vanilla_zombie.entity_id();
    assert!(world.add_entity(Entity::Creature(vanilla_zombie)));
    let snapshot = world.update_snapshot();
    let start = world.entity_by_id(vanilla_id).unwrap().position();

    assert!(
        world
            .creature_by_id_mut(vanilla_id)
            .unwrap()
            .set_path_to_default(
                &snapshot,
                Some(EntityPosition::new(6.5, 65.0, 3.5, 0.0, 0.0)),
            )
    );

    world.tick();

    assert!(world.entity_by_id(vanilla_id).unwrap().position().x() > start.x());
}

#[test]
fn path_node_display_matches_minestom_enum_spelling() {
    let node_types = [
        (PathNodeType::Walk, "WALK"),
        (PathNodeType::Jump, "JUMP"),
        (PathNodeType::Fall, "FALL"),
        (PathNodeType::Climb, "CLIMB"),
        (PathNodeType::ClimbWall, "CLIMB_WALL"),
        (PathNodeType::Swim, "SWIM"),
        (PathNodeType::Fly, "FLY"),
        (PathNodeType::Repath, "REPATH"),
    ];

    for (node_type, expected_type) in node_types {
        let node = PathNode::new(
            EntityPosition::new(1.0, 2.0, 3.0, 0.0, 0.0),
            4.0,
            5.0,
            node_type,
        );

        assert_eq!(
            node.to_string(),
            format!("PNode{{point=1, 2, 3, d=9, type={expected_type}}}")
        );
    }
}

fn grounded_zombie() -> GenericEntity {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    entity.set_on_ground(true);
    entity
}

struct ParentlessNodeGenerator;

impl NodeGenerator for ParentlessNodeGenerator {
    fn walkable(
        &self,
        _world: &spinel_server::world::WorldSnapshot,
        _visited: &HashSet<PathNode>,
        current: &PathNode,
        goal: EntityPosition,
        _bounding_box: EntityBoundingBox,
    ) -> Vec<PathNode> {
        if current.block_coordinates() != (0, 65, 0) {
            return Vec::new();
        }

        vec![PathNode::new(goal, 1.0, 0.0, PathNodeType::Walk)]
    }

    fn has_gravity_snap(&self) -> bool {
        false
    }

    fn gravity_snap(
        &self,
        _world: &spinel_server::world::WorldSnapshot,
        position: EntityPosition,
        _bounding_box: EntityBoundingBox,
        _maximum_fall: i32,
    ) -> Option<f64> {
        Some(position.y())
    }
}

fn navigator_following_nodes(
    world: &spinel_server::world::WorldSnapshot,
    entity: &mut GenericEntity,
    goal: EntityPosition,
    nodes: Vec<PathNode>,
) -> Navigator {
    let start = entity.position();
    let mut navigator = Navigator::default();
    assert!(navigator.set_path_to(
        world,
        start,
        Some(goal),
        entity.bounding_box(),
        true,
        0.1,
        50.0,
        20.0,
        None,
    ));
    navigator.tick(entity, world, false);
    entity.set_position(start);
    navigator
        .path_mut()
        .unwrap()
        .set_state(PathState::Following);
    let path_nodes = navigator.nodes_mut().unwrap();
    path_nodes.clear();
    path_nodes.extend(nodes);
    navigator
}

fn minestom_ground_position(
    start: EntityPosition,
    target: EntityPosition,
    speed: f64,
    look_at: EntityPosition,
) -> EntityPosition {
    let delta_x = target.x() - start.x();
    let delta_y = target.y() - start.y();
    let delta_z = target.z() - start.z();
    let distance_squared = delta_x * delta_x + delta_y * delta_y + delta_z * delta_z;
    let movement_speed = speed.min(distance_squared);
    let movement_radians = delta_z.atan2(delta_x);
    let movement_x = movement_radians.cos() * movement_speed;
    let movement_z = movement_radians.sin() * movement_speed;
    let look_delta_x = look_at.x() - start.x();
    let look_delta_y = look_at.y() - start.y();
    let look_delta_z = look_at.z() - start.z();
    let yaw = (look_delta_z.atan2(look_delta_x).to_degrees() - 90.0) as f32;
    let pitch = (-look_delta_y
        .atan2((look_delta_x * look_delta_x + look_delta_z * look_delta_z).sqrt())
        .to_degrees()) as f32;

    start
        .offset(movement_x, 0.0, movement_z)
        .with_view(yaw, pitch)
}

fn minestom_flying_position(
    start: EntityPosition,
    target: EntityPosition,
    speed: f64,
    look_at: EntityPosition,
) -> EntityPosition {
    let delta_x = target.x() - start.x();
    let delta_y = target.y() - start.y();
    let delta_z = target.z() - start.z();
    let distance_squared = delta_x * delta_x + delta_y * delta_y + delta_z * delta_z;
    let movement_speed = speed.min(distance_squared);
    let movement_radians = delta_z.atan2(delta_x);
    let movement_x = movement_radians.cos() * movement_speed;
    let movement_z = movement_radians.sin() * movement_speed;
    let requested_movement_y = delta_y.signum() * 0.5 * movement_speed;
    let movement_y = if requested_movement_y.abs().min(delta_y.abs()) == delta_y.abs() {
        delta_y
    } else {
        requested_movement_y
    };
    let look_delta_x = look_at.x() - start.x();
    let look_delta_y = look_at.y() - start.y();
    let look_delta_z = look_at.z() - start.z();
    let yaw = (look_delta_z.atan2(look_delta_x).to_degrees() - 90.0) as f32;
    let pitch = (-look_delta_y
        .atan2((look_delta_x * look_delta_x + look_delta_z * look_delta_z).sqrt())
        .to_degrees()) as f32;

    start
        .offset(movement_x, movement_y, movement_z)
        .with_view(yaw, pitch)
}

fn pathfinding_world() -> World {
    let mut world = World::new(Identifier::minecraft("entity_ai_pathfinding_parity_gaps"));
    world.set_chunk_supplier(|chunk_position| {
        let mut chunk = Chunk::new_with_generation(chunk_position, false);
        let floor_section = chunk.section_mut(4).unwrap();
        (0..256).for_each(|block_index| {
            floor_section.block_palette_mut().set(
                block_index,
                spinel_server::world::Block::STONE.default_state(),
            );
        });
        chunk
    });
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world
}

struct StartPathGoal {
    goal: EntityPosition,
    has_started: bool,
}

impl GoalSelector for StartPathGoal {
    fn should_start(
        &mut self,
        _creature: &CreatureEntity,
        _world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        !self.has_started
    }

    fn start(
        &mut self,
        creature: &mut CreatureEntity,
        world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.has_started = creature.set_path_to_default(world, Some(self.goal));
    }

    fn tick(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
        _time: u64,
    ) {
    }

    fn should_end(
        &mut self,
        _creature: &CreatureEntity,
        _world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        false
    }

    fn end(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
    }
}

struct ResetPathGoal {
    goal: EntityPosition,
    has_started: bool,
}

impl GoalSelector for ResetPathGoal {
    fn should_start(
        &mut self,
        _creature: &CreatureEntity,
        _world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        !self.has_started
    }

    fn start(
        &mut self,
        creature: &mut CreatureEntity,
        world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.has_started = creature.set_path_to_default(world, Some(self.goal));
    }

    fn tick(
        &mut self,
        creature: &mut CreatureEntity,
        _world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
        _time: u64,
    ) {
        creature.navigator_mut().reset();
    }

    fn should_end(
        &mut self,
        _creature: &CreatureEntity,
        _world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        false
    }

    fn end(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &spinel_server::world::WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
    }
}
