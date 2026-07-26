use crate::entity::{Entity, EntityId, EntityPosition, LivingEntity, ProjectilePhysics};
use crate::events::entity_shoot::EntityShootEvent;
use crate::events::projectile_collide::ProjectileCollideEvent;
use crate::events::projectile_collide_with_block::ProjectileCollideWithBlockEvent;
use crate::events::projectile_collide_with_entity::ProjectileCollideWithEntityEvent;
use crate::events::projectile_uncollide::ProjectileUncollideEvent;
use crate::server::MinecraftServer;
use crate::world::{Block, BlockPosition, ChunkPosition, World};
use spinel_macros::fn_event_listener;
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::{EntityType, Identifier};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static PROJECTILE_TEST_LOCK: Mutex<()> = Mutex::new(());
static PROJECTILE_TEST_ID: Mutex<Option<EntityId>> = Mutex::new(None);
static PROJECTILE_TARGET_ID: Mutex<Option<EntityId>> = Mutex::new(None);
static PROJECTILE_BLOCK_COLLISION_POSITION: Mutex<Option<EntityPosition>> = Mutex::new(None);
static PROJECTILE_ENTITY_COLLISION_POSITION: Mutex<Option<EntityPosition>> = Mutex::new(None);
static PROJECTILE_SHOOT_CANCELLED: AtomicBool = AtomicBool::new(false);
static PROJECTILE_BLOCK_COLLISION_CANCELLED: AtomicBool = AtomicBool::new(false);
static PROJECTILE_BLOCK_COLLISION_COUNT: AtomicU32 = AtomicU32::new(0);
static PROJECTILE_ENTITY_COLLISION_COUNT: AtomicU32 = AtomicU32::new(0);
static PROJECTILE_SHARED_COLLISION_COUNT: AtomicU32 = AtomicU32::new(0);
static PROJECTILE_SHARED_COLLISION_CANCELLED: AtomicBool = AtomicBool::new(false);
static PROJECTILE_UNCOLLIDE_COUNT: AtomicU32 = AtomicU32::new(0);
static PROJECTILE_SHOOT_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[fn_event_listener]
fn projectile_shoot_listener(event: &mut EntityShootEvent, _server: &mut MinecraftServer) {
    if *PROJECTILE_TEST_ID.lock().unwrap() != Some(event.get_projectile_id()) {
        return;
    }
    let shooter_id = event.get_shooter_id();
    let projectile_id = event.get_projectile_id();
    let event_entity_id = event.get_entity().get_entity_id();
    let event_projectile_id = event.get_projectile().get_entity_id();
    if event_entity_id == shooter_id && event_projectile_id == projectile_id {
        PROJECTILE_SHOOT_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    event.set_power(0.5);
    event.set_spread(0.0);
    event.set_cancelled(PROJECTILE_SHOOT_CANCELLED.load(Ordering::SeqCst));
}

#[fn_event_listener]
fn projectile_block_collision_listener(
    event: &mut ProjectileCollideWithBlockEvent,
    _server: &mut MinecraftServer,
) {
    if *PROJECTILE_TEST_ID.lock().unwrap() != Some(event.get_projectile_id()) {
        return;
    }
    *PROJECTILE_BLOCK_COLLISION_POSITION.lock().unwrap() = Some(event.get_collision_position());
    PROJECTILE_BLOCK_COLLISION_COUNT.fetch_add(1, Ordering::SeqCst);
    event.set_cancelled(PROJECTILE_BLOCK_COLLISION_CANCELLED.load(Ordering::SeqCst));
}

#[fn_event_listener]
fn projectile_entity_collision_listener(
    event: &mut ProjectileCollideWithEntityEvent,
    _server: &mut MinecraftServer,
) {
    if *PROJECTILE_TEST_ID.lock().unwrap() != Some(event.get_projectile_id()) {
        return;
    }
    *PROJECTILE_TARGET_ID.lock().unwrap() = Some(event.get_target_id());
    *PROJECTILE_ENTITY_COLLISION_POSITION.lock().unwrap() = Some(event.get_collision_position());
    PROJECTILE_ENTITY_COLLISION_COUNT.fetch_add(1, Ordering::SeqCst);
}

#[fn_event_listener]
fn projectile_collision_listener(
    event: &mut ProjectileCollideEvent,
    _server: &mut MinecraftServer,
) {
    if *PROJECTILE_TEST_ID.lock().unwrap() != Some(event.get_projectile_id()) {
        return;
    }
    PROJECTILE_SHARED_COLLISION_COUNT.fetch_add(1, Ordering::SeqCst);
    event.set_cancelled(PROJECTILE_SHARED_COLLISION_CANCELLED.load(Ordering::SeqCst));
}

#[fn_event_listener]
fn projectile_uncollide_listener(
    event: &mut ProjectileUncollideEvent,
    _server: &mut MinecraftServer,
) {
    if *PROJECTILE_TEST_ID.lock().unwrap() != Some(event.get_projectile_id()) {
        return;
    }
    PROJECTILE_UNCOLLIDE_COUNT.fetch_add(1, Ordering::SeqCst);
}

#[test]
fn projectile_shoot_event_can_mutate_power_and_cancel_the_shot() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("projectile_shoot"),
    );
    world.use_server_event_dispatcher(server_ptr);
    let mut shooter = LivingEntity::new(EntityType::ZOMBIE);
    shooter.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let shooter_id = shooter.get_entity_id();
    world.add_entity(Entity::Living(shooter));
    let projectile_id = world
        .spawn_projectile(
            Some(shooter_id),
            EntityType::ARROW,
            EntityPosition::new(0.0, 65.0, 0.0, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);

    assert!(world.shoot_projectile(
        projectile_id,
        EntityPosition::new(0.0, 65.0, 10.0, 0.0, 0.0),
        1.0,
        4.0,
    ));
    let shot_projectile = projectile_entity(&world, projectile_id);
    let speed = shot_projectile.get_velocity().0.x.mul_add(
        shot_projectile.get_velocity().0.x,
        shot_projectile.get_velocity().0.y.mul_add(
            shot_projectile.get_velocity().0.y,
            shot_projectile.get_velocity().0.z.powi(2),
        ),
    );
    assert!((speed.sqrt() - 10.0).abs() < 0.000001);
    assert!(PROJECTILE_SHOOT_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));

    PROJECTILE_SHOOT_CANCELLED.store(true, Ordering::SeqCst);
    assert!(!world.shoot_projectile(
        projectile_id,
        EntityPosition::new(0.0, 65.0, 10.0, 0.0, 0.0),
        1.0,
        0.0,
    ));
    assert!(projectile_entity(&world, projectile_id).is_removed());
    reset_projectile_event_state();
}

#[test]
fn projectile_tick_raycast_hits_block_surface_and_sticks() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("projectile_collision"),
    );
    world.use_server_event_dispatcher(server_ptr);
    world
        .set_block(BlockPosition::new(1, 64, 0), Block::STONE)
        .unwrap();
    let projectile_id = world
        .spawn_projectile(
            None,
            EntityType::ARROW,
            EntityPosition::new(0.5, 64.0, 0.5, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);
    let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
        panic!("spawned projectile must remain a projectile");
    };
    projectile.set_no_gravity(true);
    projectile.set_velocity(Velocity(Vector3d {
        x: 20.0,
        y: 0.0,
        z: 0.0,
    }));

    world.tick();

    let Some(Entity::Projectile(projectile)) = world.get_entity(projectile_id) else {
        panic!("spawned projectile must remain in the world");
    };
    assert!(projectile.get_was_stuck());
    assert!(projectile.is_on_ground());
    assert!(projectile.has_no_gravity());
    assert_eq!(
        projectile.get_velocity().0,
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    );
    assert!((projectile.get_position().get_x() - 1.0).abs() < 0.000001);
    let collision_position = PROJECTILE_BLOCK_COLLISION_POSITION.lock().unwrap().unwrap();
    assert!((collision_position.get_x() - 1.0).abs() < 0.000001);
    assert_eq!(PROJECTILE_BLOCK_COLLISION_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(PROJECTILE_SHARED_COLLISION_COUNT.load(Ordering::SeqCst), 1);
    reset_projectile_event_state();
}

#[test]
fn projectile_tick_hits_ground_surface_before_block_collision_event() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("projectile_ground_surface_collision"),
    );
    world.use_server_event_dispatcher(server_ptr);
    world
        .set_block(BlockPosition::new(0, 63, 0), Block::STONE)
        .unwrap();
    let projectile_id = world
        .spawn_projectile(
            None,
            EntityType::SPLASH_POTION,
            EntityPosition::new(0.5, 64.5, 0.5, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);
    let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
        panic!("spawned projectile must remain a projectile");
    };
    projectile.set_no_gravity(true);
    projectile.set_velocity(Velocity(Vector3d {
        x: 0.0,
        y: -20.0,
        z: 0.0,
    }));

    world.tick();

    let collision_position = PROJECTILE_BLOCK_COLLISION_POSITION.lock().unwrap().unwrap();
    assert!((collision_position.get_y() - 64.0).abs() < 0.000001);
    assert!(
        (projectile_entity(&world, projectile_id)
            .get_position()
            .get_y()
            - 64.0)
            .abs()
            < 0.000001
    );
    reset_projectile_event_state();
}

#[test]
fn shared_projectile_collision_listener_can_cancel_concrete_block_collision() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("shared_projectile_collision"),
    );
    world.use_server_event_dispatcher(server_ptr);
    world
        .set_block(BlockPosition::new(1, 64, 0), Block::STONE)
        .unwrap();
    let projectile_id = world
        .spawn_projectile(
            None,
            EntityType::ARROW,
            EntityPosition::new(0.5, 64.0, 0.5, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);
    PROJECTILE_SHARED_COLLISION_CANCELLED.store(true, Ordering::SeqCst);
    let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
        panic!("spawned projectile must remain a projectile");
    };
    projectile.set_no_gravity(true);
    projectile.set_velocity(Velocity(Vector3d {
        x: 20.0,
        y: 0.0,
        z: 0.0,
    }));

    world.tick();

    assert!(!projectile_entity(&world, projectile_id).get_was_stuck());
    assert!(PROJECTILE_SHARED_COLLISION_COUNT.load(Ordering::SeqCst) >= 1);
    reset_projectile_event_state();
}

#[test]
fn stuck_projectile_uncollides_after_its_block_is_removed() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("projectile_uncollision"),
    );
    world.use_server_event_dispatcher(server_ptr);
    let collision_block = BlockPosition::new(1, 64, 0);
    world.set_block(collision_block, Block::STONE).unwrap();
    let projectile_id = world
        .spawn_projectile(
            None,
            EntityType::ARROW,
            EntityPosition::new(0.5, 64.0, 0.5, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);
    let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
        panic!("spawned projectile must remain a projectile");
    };
    projectile.set_no_gravity(true);
    projectile.set_velocity(Velocity(Vector3d {
        x: 20.0,
        y: 0.0,
        z: 0.0,
    }));
    world.tick();
    world.set_block(collision_block, Block::AIR).unwrap();

    world.tick();

    let Some(Entity::Projectile(projectile)) = world.get_entity(projectile_id) else {
        panic!("spawned projectile must remain in the world");
    };
    assert!(!projectile.get_was_stuck());
    assert!(!projectile.is_on_ground());
    assert!(!projectile.has_no_gravity());
    assert_eq!(PROJECTILE_UNCOLLIDE_COUNT.load(Ordering::SeqCst), 1);
    reset_projectile_event_state();
}

#[test]
fn projectile_tick_emits_entity_collision_for_living_targets() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("projectile_entity_collision"),
    );
    world.use_server_event_dispatcher(server_ptr);
    world
        .load_chunk(crate::world::ChunkPosition::new(0, 0))
        .unwrap();
    let target_x = 1.25;
    let mut target = LivingEntity::new(EntityType::ZOMBIE);
    target.set_position(EntityPosition::new(target_x, 64.0, 0.5, 0.0, 0.0));
    target.set_no_gravity(true);
    let target_id = target.get_entity_id();
    world.add_entity(Entity::Living(target));
    let projectile_id = world
        .spawn_projectile(
            None,
            EntityType::ARROW,
            EntityPosition::new(0.25, 64.0, 0.5, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);
    let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
        panic!("spawned projectile must remain a projectile");
    };
    projectile.set_no_gravity(true);
    projectile.set_velocity(Velocity(Vector3d {
        x: 20.0,
        y: 0.0,
        z: 0.0,
    }));
    assert!(
        world
            .get_entity(target_id)
            .is_some_and(|target| match target {
                Entity::Living(target) => target.get_intersects_box_at(
                    Vector3d {
                        x: 0.75,
                        y: 64.0,
                        z: 0.5,
                    },
                    EntityType::ARROW.get_bounding_box(),
                ),
                _ => false,
            })
    );
    assert!(
        world
            .chunk_entities(crate::world::ChunkPosition::new(0, 0))
            .iter()
            .any(|entity| entity.get_entity_id() == target_id)
    );

    world.tick();

    assert!(PROJECTILE_ENTITY_COLLISION_COUNT.load(Ordering::SeqCst) >= 1);
    assert!(PROJECTILE_SHARED_COLLISION_COUNT.load(Ordering::SeqCst) >= 1);
    assert_eq!(*PROJECTILE_TARGET_ID.lock().unwrap(), Some(target_id));
    let collision_position = PROJECTILE_ENTITY_COLLISION_POSITION
        .lock()
        .unwrap()
        .unwrap();
    assert!(collision_position.get_x() < target_x);
    reset_projectile_event_state();
}

#[test]
fn projectile_collision_chooses_nearest_entity_before_later_block() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("projectile_entity_before_block"),
    );
    world.use_server_event_dispatcher(server_ptr);
    world
        .load_chunk(crate::world::ChunkPosition::new(0, 0))
        .unwrap();
    world
        .set_block(BlockPosition::new(3, 64, 0), Block::STONE)
        .unwrap();
    let mut target = LivingEntity::new(EntityType::ZOMBIE);
    target.set_position(EntityPosition::new(1.25, 64.0, 0.5, 0.0, 0.0));
    target.set_no_gravity(true);
    let target_id = target.get_entity_id();
    world.add_entity(Entity::Living(target));
    let projectile_id = world
        .spawn_projectile(
            None,
            EntityType::ARROW,
            EntityPosition::new(0.25, 64.0, 0.5, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);
    let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
        panic!("spawned projectile must remain a projectile");
    };
    projectile.set_no_gravity(true);
    projectile.set_velocity(Velocity(Vector3d {
        x: 80.0,
        y: 0.0,
        z: 0.0,
    }));

    world.tick();

    assert_eq!(PROJECTILE_ENTITY_COLLISION_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(PROJECTILE_BLOCK_COLLISION_COUNT.load(Ordering::SeqCst), 0);
    assert_eq!(*PROJECTILE_TARGET_ID.lock().unwrap(), Some(target_id));
    reset_projectile_event_state();
}

#[test]
fn projectile_does_not_collide_with_shooter_until_it_leaves_shooter_collision_range() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("projectile_shooter_collision_range"),
    );
    world.use_server_event_dispatcher(server_ptr);
    world
        .load_chunk(crate::world::ChunkPosition::new(0, 0))
        .unwrap();
    let mut shooter = LivingEntity::new(EntityType::ZOMBIE);
    shooter.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
    shooter.set_no_gravity(true);
    let shooter_id = shooter.get_entity_id();
    world.add_entity(Entity::Living(shooter));
    let projectile_id = world
        .spawn_projectile(
            Some(shooter_id),
            EntityType::SPLASH_POTION,
            EntityPosition::new(0.0, 65.52, 0.0, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);
    let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
        panic!("spawned projectile must remain a projectile");
    };
    projectile.set_no_gravity(true);
    projectile.set_velocity(Velocity(Vector3d {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    }));

    (0..5).for_each(|_| world.tick());

    assert_eq!(PROJECTILE_ENTITY_COLLISION_COUNT.load(Ordering::SeqCst), 0);
    assert_eq!(*PROJECTILE_TARGET_ID.lock().unwrap(), None);
    reset_projectile_event_state();
}

#[test]
fn splash_potion_motion_matches_vanilla_throwable_motion_for_multiple_heights_and_velocities() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("splash_potion_vanilla_motion"),
    );
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.load_chunk(ChunkPosition::new(-1, 0)).unwrap();
    let motion_cases = [
        VanillaThrowableMotionCase::new(
            80.0,
            Vector3d {
                x: 0.0,
                y: -0.1,
                z: 0.2,
            },
        ),
        VanillaThrowableMotionCase::new(
            86.40769762399721,
            Vector3d {
                x: 0.0,
                y: 0.379906000122078,
                z: 1.0717206860770312,
            },
        ),
        VanillaThrowableMotionCase::new(
            92.0,
            Vector3d {
                x: 0.15,
                y: 0.0,
                z: 0.5,
            },
        ),
        VanillaThrowableMotionCase::new(
            104.0,
            Vector3d {
                x: -0.2,
                y: -0.25,
                z: 0.8,
            },
        ),
    ];

    motion_cases.into_iter().for_each(|motion_case| {
        let projectile_id = world
            .spawn_projectile(
                None,
                EntityType::SPLASH_POTION,
                EntityPosition::new(0.5, motion_case.starting_y, 0.5, 0.0, 0.0),
            )
            .unwrap();
        let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
            panic!("spawned projectile must remain a projectile");
        };
        projectile.set_physics(ProjectilePhysics::VanillaPhysics);
        projectile.set_velocity(motion_case.starting_velocity_per_tick.as_velocity());
        let spawn_packet_velocity = projectile.spawn_packet().velocity.0;
        assert_vector_approximately_eq(
            spawn_packet_velocity,
            motion_case.starting_velocity_per_tick,
        );

        let mut expected_position = projectile.get_position();
        let mut expected_velocity_per_tick = motion_case.starting_velocity_per_tick;
        let mut expected_yaw = expected_position.get_yaw();
        let mut expected_pitch = expected_position.get_pitch();
        (0..6).for_each(|_| {
            world.tick();
            expected_velocity_per_tick =
                expected_velocity_per_tick.next_splash_potion_velocity_per_tick();
            (expected_yaw, expected_pitch) = vanilla_throwable_projectile_view(
                expected_yaw,
                expected_pitch,
                expected_velocity_per_tick,
            );
            expected_position = expected_position
                .get_offset(
                    expected_velocity_per_tick.x,
                    expected_velocity_per_tick.y,
                    expected_velocity_per_tick.z,
                )
                .with_view(expected_yaw, expected_pitch);
            let projectile = projectile_entity(&world, projectile_id);
            assert_position_approximately_eq(projectile.get_position(), expected_position);
            assert_view_approximately_eq(projectile.get_position(), expected_position);
            assert_vector_approximately_eq(
                projectile.get_velocity().0,
                expected_velocity_per_tick.as_server_velocity_vector(),
            );
        });
        world.remove_entity(projectile_id);
    });
    reset_projectile_event_state();
}

#[test]
fn splash_potion_ground_collision_tick_matches_vanilla_surface_crossing() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("splash_potion_vanilla_ground_crossing"),
    );
    world.use_server_event_dispatcher(server_ptr);
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world
        .set_block(BlockPosition::new(0, 63, 0), Block::STONE)
        .unwrap();
    let projectile_id = world
        .spawn_projectile(
            None,
            EntityType::SPLASH_POTION,
            EntityPosition::new(0.5, 66.0, 0.5, 0.0, 0.0),
        )
        .unwrap();
    *PROJECTILE_TEST_ID.lock().unwrap() = Some(projectile_id);
    let Some(Entity::Projectile(projectile)) = world.get_entity_mut(projectile_id) else {
        panic!("spawned projectile must remain a projectile");
    };
    projectile.set_physics(ProjectilePhysics::VanillaPhysics);
    let starting_velocity_per_tick = Vector3d {
        x: 0.0,
        y: -0.1,
        z: 0.0,
    };
    projectile.set_velocity(starting_velocity_per_tick.as_velocity());
    let expected_collision_tick =
        vanilla_splash_potion_ground_collision_tick(66.0, starting_velocity_per_tick.y, 64.0);

    (1..expected_collision_tick).for_each(|_| {
        world.tick();
        assert_eq!(PROJECTILE_BLOCK_COLLISION_COUNT.load(Ordering::SeqCst), 0);
    });
    world.tick();

    assert_eq!(PROJECTILE_BLOCK_COLLISION_COUNT.load(Ordering::SeqCst), 1);
    let collision_position = PROJECTILE_BLOCK_COLLISION_POSITION.lock().unwrap().unwrap();
    assert!((collision_position.get_y() - 64.0).abs() < 0.000001);
    reset_projectile_event_state();
}

fn projectile_entity(world: &World, projectile_id: EntityId) -> &crate::entity::ProjectileEntity {
    let Some(Entity::Projectile(projectile)) = world.get_entity(projectile_id) else {
        panic!("projectile must remain in the world");
    };
    projectile
}

fn reset_projectile_event_state() {
    *PROJECTILE_TEST_ID.lock().unwrap() = None;
    *PROJECTILE_TARGET_ID.lock().unwrap() = None;
    *PROJECTILE_BLOCK_COLLISION_POSITION.lock().unwrap() = None;
    *PROJECTILE_ENTITY_COLLISION_POSITION.lock().unwrap() = None;
    PROJECTILE_SHOOT_CANCELLED.store(false, Ordering::SeqCst);
    PROJECTILE_BLOCK_COLLISION_CANCELLED.store(false, Ordering::SeqCst);
    PROJECTILE_BLOCK_COLLISION_COUNT.store(0, Ordering::SeqCst);
    PROJECTILE_ENTITY_COLLISION_COUNT.store(0, Ordering::SeqCst);
    PROJECTILE_SHARED_COLLISION_COUNT.store(0, Ordering::SeqCst);
    PROJECTILE_SHARED_COLLISION_CANCELLED.store(false, Ordering::SeqCst);
    PROJECTILE_UNCOLLIDE_COUNT.store(0, Ordering::SeqCst);
    PROJECTILE_SHOOT_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
}

struct VanillaThrowableMotionCase {
    starting_y: f64,
    starting_velocity_per_tick: Vector3d,
}

impl VanillaThrowableMotionCase {
    const fn new(starting_y: f64, starting_velocity_per_tick: Vector3d) -> Self {
        Self {
            starting_y,
            starting_velocity_per_tick,
        }
    }
}

trait VanillaThrowableVector {
    fn next_splash_potion_velocity_per_tick(self) -> Self;
    fn as_velocity(self) -> Velocity;
    fn as_server_velocity_vector(self) -> Vector3d;
}

impl VanillaThrowableVector for Vector3d {
    fn next_splash_potion_velocity_per_tick(self) -> Self {
        Self {
            x: self.x * (0.99_f32 as f64),
            y: (self.y - 0.05) * (0.99_f32 as f64),
            z: self.z * (0.99_f32 as f64),
        }
    }

    fn as_velocity(self) -> Velocity {
        Velocity(self.as_server_velocity_vector())
    }

    fn as_server_velocity_vector(self) -> Vector3d {
        Vector3d {
            x: self.x * 20.0,
            y: self.y * 20.0,
            z: self.z * 20.0,
        }
    }
}

fn vanilla_splash_potion_ground_collision_tick(
    starting_y: f64,
    starting_velocity_y_per_tick: f64,
    ground_surface_y: f64,
) -> u64 {
    let mut position_y = starting_y;
    let mut velocity_y_per_tick = starting_velocity_y_per_tick;
    for tick in 1..100 {
        velocity_y_per_tick = (velocity_y_per_tick - 0.05) * (0.99_f32 as f64);
        let next_position_y = position_y + velocity_y_per_tick;
        if next_position_y <= ground_surface_y {
            return tick;
        }
        position_y = next_position_y;
    }
    panic!("expected splash potion to reach the ground surface");
}

fn vanilla_throwable_projectile_view(
    previous_yaw: f32,
    previous_pitch: f32,
    velocity_per_tick: Vector3d,
) -> (f32, f32) {
    let target_yaw = velocity_per_tick.x.atan2(velocity_per_tick.z).to_degrees() as f32;
    let target_pitch = velocity_per_tick
        .y
        .atan2(velocity_per_tick.x.hypot(velocity_per_tick.z))
        .to_degrees() as f32;
    (
        vanilla_throwable_projectile_rotation(previous_yaw, target_yaw),
        vanilla_throwable_projectile_rotation(previous_pitch, target_pitch),
    )
}

fn vanilla_throwable_projectile_rotation(previous_rotation: f32, target_rotation: f32) -> f32 {
    let mut wrapped_previous_rotation = previous_rotation;
    while target_rotation - wrapped_previous_rotation < -180.0 {
        wrapped_previous_rotation -= 360.0;
    }
    while target_rotation - wrapped_previous_rotation >= 180.0 {
        wrapped_previous_rotation += 360.0;
    }
    wrapped_previous_rotation + 0.2 * (target_rotation - wrapped_previous_rotation)
}

fn assert_position_approximately_eq(
    actual_position: EntityPosition,
    expected_position: EntityPosition,
) {
    assert!(
        (actual_position.get_x() - expected_position.get_x()).abs() < 0.000001,
        "actual x {} expected x {}",
        actual_position.get_x(),
        expected_position.get_x()
    );
    assert!(
        (actual_position.get_y() - expected_position.get_y()).abs() < 0.000001,
        "actual y {} expected y {}",
        actual_position.get_y(),
        expected_position.get_y()
    );
    assert!(
        (actual_position.get_z() - expected_position.get_z()).abs() < 0.000001,
        "actual z {} expected z {}",
        actual_position.get_z(),
        expected_position.get_z()
    );
}

fn assert_view_approximately_eq(
    actual_position: EntityPosition,
    expected_position: EntityPosition,
) {
    assert!(
        (actual_position.get_yaw() - expected_position.get_yaw()).abs() < 0.000001,
        "actual yaw {} expected yaw {}",
        actual_position.get_yaw(),
        expected_position.get_yaw()
    );
    assert!(
        (actual_position.get_pitch() - expected_position.get_pitch()).abs() < 0.000001,
        "actual pitch {} expected pitch {}",
        actual_position.get_pitch(),
        expected_position.get_pitch()
    );
}

fn assert_vector_approximately_eq(actual_vector: Vector3d, expected_vector: Vector3d) {
    assert!(
        (actual_vector.x - expected_vector.x).abs() < 0.000001,
        "actual x {} expected x {}",
        actual_vector.x,
        expected_vector.x
    );
    assert!(
        (actual_vector.y - expected_vector.y).abs() < 0.000001,
        "actual y {} expected y {}",
        actual_vector.y,
        expected_vector.y
    );
    assert!(
        (actual_vector.z - expected_vector.z).abs() < 0.000001,
        "actual z {} expected z {}",
        actual_vector.z,
        expected_vector.z
    );
}
