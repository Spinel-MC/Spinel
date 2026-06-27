use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity};
use crate::events::entity_shoot::EntityShootEvent;
use crate::events::projectile_collide::ProjectileCollideEvent;
use crate::events::projectile_collide_with_block::ProjectileCollideWithBlockEvent;
use crate::events::projectile_collide_with_entity::ProjectileCollideWithEntityEvent;
use crate::events::projectile_uncollide::ProjectileUncollideEvent;
use crate::server::MinecraftServer;
use crate::world::{Block, BlockPosition, World};
use spinel_macros::event_listener;
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::{EntityType, Identifier};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static PROJECTILE_TEST_LOCK: Mutex<()> = Mutex::new(());
static PROJECTILE_TEST_ID: Mutex<Option<EntityId>> = Mutex::new(None);
static PROJECTILE_TARGET_ID: Mutex<Option<EntityId>> = Mutex::new(None);
static PROJECTILE_SHOOT_CANCELLED: AtomicBool = AtomicBool::new(false);
static PROJECTILE_BLOCK_COLLISION_CANCELLED: AtomicBool = AtomicBool::new(false);
static PROJECTILE_BLOCK_COLLISION_COUNT: AtomicU32 = AtomicU32::new(0);
static PROJECTILE_ENTITY_COLLISION_COUNT: AtomicU32 = AtomicU32::new(0);
static PROJECTILE_SHARED_COLLISION_COUNT: AtomicU32 = AtomicU32::new(0);
static PROJECTILE_SHARED_COLLISION_CANCELLED: AtomicBool = AtomicBool::new(false);
static PROJECTILE_UNCOLLIDE_COUNT: AtomicU32 = AtomicU32::new(0);
static PROJECTILE_SHOOT_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[event_listener]
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

#[event_listener]
fn projectile_block_collision_listener(
    event: &mut ProjectileCollideWithBlockEvent,
    _server: &mut MinecraftServer,
) {
    if *PROJECTILE_TEST_ID.lock().unwrap() != Some(event.get_projectile_id()) {
        return;
    }
    PROJECTILE_BLOCK_COLLISION_COUNT.fetch_add(1, Ordering::SeqCst);
    event.set_cancelled(PROJECTILE_BLOCK_COLLISION_CANCELLED.load(Ordering::SeqCst));
}

#[event_listener]
fn projectile_entity_collision_listener(
    event: &mut ProjectileCollideWithEntityEvent,
    _server: &mut MinecraftServer,
) {
    if *PROJECTILE_TEST_ID.lock().unwrap() != Some(event.get_projectile_id()) {
        return;
    }
    *PROJECTILE_TARGET_ID.lock().unwrap() = Some(event.get_target_id());
    PROJECTILE_ENTITY_COLLISION_COUNT.fetch_add(1, Ordering::SeqCst);
}

#[event_listener]
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

#[event_listener]
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
    let mut world = World::new(Identifier::minecraft("projectile_shoot"));
    world.use_server_event_dispatcher(server_ptr);
    let mut shooter = GenericEntity::new(EntityType::ZOMBIE);
    shooter.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let shooter_id = shooter.get_entity_id();
    world.add_entity(Entity::Generic(shooter));
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
fn projectile_tick_samples_its_path_and_sticks_in_a_solid_block() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new(Identifier::minecraft("projectile_collision"));
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
    assert!(projectile.get_position().get_x() >= 1.0);
    assert!(projectile.get_position().get_x() < 2.0);
    assert_eq!(PROJECTILE_BLOCK_COLLISION_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(PROJECTILE_SHARED_COLLISION_COUNT.load(Ordering::SeqCst), 1);
    reset_projectile_event_state();
}

#[test]
fn shared_projectile_collision_listener_can_cancel_concrete_block_collision() {
    let _lock = PROJECTILE_TEST_LOCK.lock().unwrap();
    reset_projectile_event_state();
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut world = World::new(Identifier::minecraft("shared_projectile_collision"));
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
    let mut world = World::new(Identifier::minecraft("projectile_uncollision"));
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
    let mut world = World::new(Identifier::minecraft("projectile_entity_collision"));
    world.use_server_event_dispatcher(server_ptr);
    world
        .load_chunk(crate::world::ChunkPosition::new(0, 0))
        .unwrap();
    let mut target = GenericEntity::new(EntityType::ZOMBIE);
    target.set_position(EntityPosition::new(1.25, 64.0, 0.5, 0.0, 0.0));
    target.set_no_gravity(true);
    let target_id = target.get_entity_id();
    world.add_entity(Entity::Generic(target));
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
                Entity::Generic(target) => target.get_intersects_box_at(
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

    let projectile_position = projectile_entity(&world, projectile_id).get_position();
    let target_position = world.get_entity(target_id).unwrap().get_position();
    assert!(
        world
            .get_entity(target_id)
            .is_some_and(|target| match target {
                Entity::Generic(target) => target.get_intersects_box_at(
                    projectile_position.as_vector(),
                    EntityType::ARROW.get_bounding_box(),
                ),
                _ => false,
            }),
        "projectile={projectile_position:?} target={target_position:?}"
    );
    assert!(PROJECTILE_ENTITY_COLLISION_COUNT.load(Ordering::SeqCst) >= 1);
    assert!(PROJECTILE_SHARED_COLLISION_COUNT.load(Ordering::SeqCst) >= 1);
    assert_eq!(*PROJECTILE_TARGET_ID.lock().unwrap(), Some(target_id));
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
    PROJECTILE_SHOOT_CANCELLED.store(false, Ordering::SeqCst);
    PROJECTILE_BLOCK_COLLISION_CANCELLED.store(false, Ordering::SeqCst);
    PROJECTILE_BLOCK_COLLISION_COUNT.store(0, Ordering::SeqCst);
    PROJECTILE_ENTITY_COLLISION_COUNT.store(0, Ordering::SeqCst);
    PROJECTILE_SHARED_COLLISION_COUNT.store(0, Ordering::SeqCst);
    PROJECTILE_SHARED_COLLISION_CANCELLED.store(false, Ordering::SeqCst);
    PROJECTILE_UNCOLLIDE_COUNT.store(0, Ordering::SeqCst);
    PROJECTILE_SHOOT_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
}
