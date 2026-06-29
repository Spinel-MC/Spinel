use crate::entity::player::{Player, PlayerChunk};
use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity, TimedPotionEffect};
use crate::events::entity_potion_add::EntityPotionAddEvent;
use crate::events::entity_potion_remove::EntityPotionRemoveEvent;
use crate::events::entity_tick::EntityTickEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_macros::event_listener;
use spinel_network::ConnectionState;
use spinel_network::types::{Identifier, Particle};
use spinel_registry::{EntityType, MobEffect, Registries};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use uuid::Uuid;

static LIVING_EFFECT_TEST_LOCK: Mutex<()> = Mutex::new(());
static LIVING_EFFECT_TEST_ENTITY: Mutex<Option<EntityId>> = Mutex::new(None);
static LIVING_EFFECT_ADD_CANCELLED: AtomicBool = AtomicBool::new(false);
static LIVING_EFFECT_REMOVE_COUNT: AtomicU32 = AtomicU32::new(0);
static LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);
static LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);
static LIVING_EFFECT_TICK_EVENT_OBSERVED_ACTIVE_EFFECT: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn living_effect_add_listener(event: &mut EntityPotionAddEvent, _server: &mut MinecraftServer) {
    if *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    let event_entity_id = event.get_entity_id();
    if event.get_entity().get_entity_id() == event_entity_id {
        LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    event.set_cancelled(LIVING_EFFECT_ADD_CANCELLED.load(Ordering::SeqCst));
}

#[event_listener]
fn living_effect_remove_listener(
    event: &mut EntityPotionRemoveEvent,
    _server: &mut MinecraftServer,
) {
    if *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    let event_entity_id = event.get_entity_id();
    if event.get_entity().get_entity_id() == event_entity_id {
        LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    LIVING_EFFECT_REMOVE_COUNT.fetch_add(1, Ordering::SeqCst);
}

#[event_listener]
fn living_effect_tick_listener(event: &mut EntityTickEvent, _server: &mut MinecraftServer) {
    let Some(test_entity_id) = *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() else {
        return;
    };
    let entity = event.get_entity();
    if entity.get_entity_id() != test_entity_id {
        return;
    }
    LIVING_EFFECT_TICK_EVENT_OBSERVED_ACTIVE_EFFECT.store(
        entity.get_effect(&MobEffect::SPEED).is_some(),
        Ordering::SeqCst,
    );
}
#[test]
fn world_add_entity_effect_cancellation_preserves_effect_state_and_sends_no_packet() {
    let _lock = LIVING_EFFECT_TEST_LOCK.lock().unwrap();
    reset_living_effect_test_state();
    let (mut server, entity_id, viewer_client) = server_with_living_entity_and_viewer();
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    LIVING_EFFECT_ADD_CANCELLED.store(true, Ordering::SeqCst);
    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    assert!(
        !world
            .add_entity_effect(
                entity_id,
                TimedPotionEffect::new(MobEffect::SPEED, 0, 2, 40, 6, 0)
            )
            .unwrap()
    );
    assert!(
        !world
            .get_entity(entity_id)
            .unwrap()
            .get_effect(&MobEffect::SPEED)
            .is_some()
    );
    assert!(viewer_client.queued_outbound_packet_ids().is_empty());
    assert!(LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    reset_living_effect_test_state();
}

#[test]
fn world_add_entity_effect_replacement_removes_previous_effect_before_add_packet() {
    let _lock = LIVING_EFFECT_TEST_LOCK.lock().unwrap();
    reset_living_effect_test_state();
    let (mut server, entity_id, viewer_client) = server_with_living_entity_and_viewer();
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    assert!(
        world
            .add_entity_effect(
                entity_id,
                TimedPotionEffect::new(MobEffect::SPEED, 0, 0, 40, 0, 0)
            )
            .unwrap()
    );
    assert!(
        world
            .add_entity_effect(
                entity_id,
                TimedPotionEffect::new(MobEffect::SPEED, 0, 3, 60, 0, 0)
            )
            .unwrap()
    );

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            EntityEffectPacket::get_id(),
            RemoveEntityEffectPacket::get_id(),
            EntityEffectPacket::get_id(),
        ]
    );
    assert_eq!(LIVING_EFFECT_REMOVE_COUNT.load(Ordering::SeqCst), 1);
    assert!(LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert!(LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert_eq!(
        world
            .get_entity(entity_id)
            .unwrap()
            .get_effect(&MobEffect::SPEED)
            .unwrap()
            .get_amplifier(),
        3
    );
    reset_living_effect_test_state();
}

#[test]
fn world_clear_entity_effects_dispatches_one_remove_packet_and_event_per_effect() {
    let _lock = LIVING_EFFECT_TEST_LOCK.lock().unwrap();
    reset_living_effect_test_state();
    let (mut server, entity_id, viewer_client) = server_with_living_entity_and_viewer();
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    assert!(
        world
            .add_entity_effect(
                entity_id,
                TimedPotionEffect::new(MobEffect::SPEED, 0, 0, 40, 0, 0)
            )
            .unwrap()
    );
    assert!(
        world
            .add_entity_effect(
                entity_id,
                TimedPotionEffect::new(MobEffect::SLOWNESS, 1, 0, 40, 0, 0)
            )
            .unwrap()
    );

    assert_eq!(world.clear_entity_effects(entity_id).unwrap(), 2);

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            EntityEffectPacket::get_id(),
            EntityEffectPacket::get_id(),
            RemoveEntityEffectPacket::get_id(),
            RemoveEntityEffectPacket::get_id(),
        ]
    );
    assert_eq!(LIVING_EFFECT_REMOVE_COUNT.load(Ordering::SeqCst), 2);
    assert!(
        world
            .get_entity(entity_id)
            .unwrap()
            .get_active_effects()
            .is_empty()
    );
    reset_living_effect_test_state();
}
#[test]
fn world_living_tick_expires_effect_and_dispatches_remove_packet_and_event() {
    let _lock = LIVING_EFFECT_TEST_LOCK.lock().unwrap();
    reset_living_effect_test_state();
    let (mut server, entity_id, viewer_client) = server_with_living_entity_and_viewer();
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    assert!(
        world
            .add_entity_effect(
                entity_id,
                TimedPotionEffect::new(MobEffect::SPEED, 0, 0, 1, 0, 0)
            )
            .unwrap()
    );

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            EntityEffectPacket::get_id(),
            RemoveEntityEffectPacket::get_id(),
        ]
    );
    assert_eq!(LIVING_EFFECT_REMOVE_COUNT.load(Ordering::SeqCst), 1);
    assert!(LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert!(LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert!(
        world
            .get_entity(entity_id)
            .unwrap()
            .get_effect(&MobEffect::SPEED)
            .is_none()
    );
    reset_living_effect_test_state();
}

#[test]
fn world_entity_tick_event_observes_effect_before_same_tick_expiry() {
    let _lock = LIVING_EFFECT_TEST_LOCK.lock().unwrap();
    reset_living_effect_test_state();
    let (mut server, entity_id, _viewer_client) = server_with_living_entity_and_viewer();
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    assert!(
        world
            .add_entity_effect(
                entity_id,
                TimedPotionEffect::new(MobEffect::SPEED, 0, 0, 1, 0, 0),
            )
            .unwrap()
    );

    world.tick_with_registries(&Registries::new_vanilla());

    assert!(LIVING_EFFECT_TICK_EVENT_OBSERVED_ACTIVE_EFFECT.load(Ordering::SeqCst));
    assert!(
        world
            .get_entity(entity_id)
            .unwrap()
            .get_effect(&MobEffect::SPEED)
            .is_none()
    );
    reset_living_effect_test_state();
}
#[test]
fn timed_potion_effect_flags_packets_and_duration_boundaries_match_minestom_potion() {
    let flags = TimedPotionEffect::AMBIENT_FLAG
        | TimedPotionEffect::PARTICLES_FLAG
        | TimedPotionEffect::ICON_FLAG
        | TimedPotionEffect::BLEND_FLAG;
    let effect = TimedPotionEffect::new(MobEffect::INSTANT_HEALTH, 5, 2, 120, flags, 10);

    assert!(effect.is_ambient());
    assert!(effect.has_particles());
    assert!(effect.has_icon());
    assert!(effect.has_blend());
    assert!(!effect.is_expired_at(129));
    assert!(effect.is_expired_at(130));

    let zero_duration_effect = TimedPotionEffect::new(MobEffect::INSTANT_DAMAGE, 6, 0, 0, 0, 10);
    assert!(zero_duration_effect.is_expired_at(10));

    let infinite_effect = TimedPotionEffect::new(
        MobEffect::JUMP_BOOST,
        7,
        0,
        TimedPotionEffect::INFINITE_DURATION,
        0,
        10,
    );
    assert!(!infinite_effect.is_expired_at(10));
    assert!(!infinite_effect.is_expired_at(u64::MAX));

    let add_packet = effect.get_packet(EntityId::from_raw(42));
    assert_eq!(add_packet.entity_id, 42);
    assert_eq!(add_packet.effect_id, 5);
    assert_eq!(add_packet.amplifier, 2);
    assert_eq!(add_packet.duration_ticks, 120);
    assert_eq!(add_packet.flags, flags);

    let remove_packet = effect.remove_packet(EntityId::from_raw(42));
    assert_eq!(remove_packet.entity_id, 42);
    assert_eq!(remove_packet.effect_id, 5);
}

#[test]
fn runtime_effect_collection_does_not_mutate_living_effect_metadata() {
    let effect_particle = Particle::effect();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_effect_particles(vec![effect_particle.clone()]);
    entity.set_potion_effect_ambient(true);

    entity.add_effect(TimedPotionEffect::new(MobEffect::SPEED, 0, 0, 40, 0, 0));

    assert!(entity.has_effect(&MobEffect::SPEED));
    assert_eq!(entity.get_effect_particles(), vec![effect_particle]);
    assert!(entity.is_potion_effect_ambient());
}
fn server_with_living_entity_and_viewer() -> (MinecraftServer, EntityId, Box<Client>) {
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut viewer_client = Box::new(queued_client());
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "EffectViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    viewer.set_client(&mut viewer_client);
    viewer.mark_entered_world();
    viewer.mark_chunk_sent_to_client(PlayerChunk::new(0, 0));
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    let entity_id = entity.get_entity_id();
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.add_entity(Entity::Generic(entity));
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    (server, entity_id, viewer_client)
}

fn queued_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    let mut client = Client::new(stream, addr);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client
}

fn reset_living_effect_test_state() {
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = None;
    LIVING_EFFECT_ADD_CANCELLED.store(false, Ordering::SeqCst);
    LIVING_EFFECT_REMOVE_COUNT.store(0, Ordering::SeqCst);
    LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
    LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
    LIVING_EFFECT_TICK_EVENT_OBSERVED_ACTIVE_EFFECT.store(false, Ordering::SeqCst);
}
