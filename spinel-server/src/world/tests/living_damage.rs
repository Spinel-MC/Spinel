use crate::entity::player::{Player, PlayerChunk};
use crate::entity::{Damage, Entity, EntityId, EntityPose, EntityPosition, GenericEntity};
use crate::events::entity_damage::EntityDamageEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::World;
use spinel_core::network::clientbound::play::damage_event::DamageEventPacket;
use spinel_core::network::clientbound::play::entity_sound_effect::EntitySoundEffectPacket;
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, DataType};
use spinel_registry::damage_type::DamageType;
use spinel_registry::{EntityType, Registries};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use uuid::Uuid;

static LIVING_DAMAGE_TEST_LOCK: Mutex<()> = Mutex::new(());
static LIVING_DAMAGE_TEST_ENTITY: Mutex<Option<EntityId>> = Mutex::new(None);
static LIVING_DAMAGE_EVENT_ENABLED: AtomicBool = AtomicBool::new(false);
static LIVING_DAMAGE_EVENT_CANCELLED: AtomicBool = AtomicBool::new(false);
static LIVING_DAMAGE_EVENT_AMOUNT_BITS: AtomicU32 = AtomicU32::new(0);
static LIVING_DAMAGE_EVENT_ANIMATE: AtomicBool = AtomicBool::new(true);
static LIVING_DAMAGE_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn living_damage_listener(event: &mut EntityDamageEvent, _server: &mut MinecraftServer) {
    if !LIVING_DAMAGE_EVENT_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    if *LIVING_DAMAGE_TEST_ENTITY.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    let event_entity_id = event.get_entity_id();
    if event.get_entity().get_entity_id() == event_entity_id {
        LIVING_DAMAGE_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    event.set_damage(f32::from_bits(
        LIVING_DAMAGE_EVENT_AMOUNT_BITS.load(Ordering::SeqCst),
    ));
    event.set_should_animate(LIVING_DAMAGE_EVENT_ANIMATE.load(Ordering::SeqCst));
    event.set_cancelled(LIVING_DAMAGE_EVENT_CANCELLED.load(Ordering::SeqCst));
}

#[test]
fn world_damage_entity_applies_minestom_event_mutation_and_last_damage_order() {
    let _lock = LIVING_DAMAGE_TEST_LOCK.lock().unwrap();
    reset_living_damage_test_state();
    let mut server = server_with_living_entity();
    let entity_id = tracked_living_entity_id(&server);
    *LIVING_DAMAGE_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    LIVING_DAMAGE_EVENT_ENABLED.store(true, Ordering::SeqCst);
    LIVING_DAMAGE_EVENT_AMOUNT_BITS.store(3.0f32.to_bits(), Ordering::SeqCst);
    LIVING_DAMAGE_EVENT_ANIMATE.store(false, Ordering::SeqCst);

    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    assert!(
        world
            .damage_entity(
                &Registries::new_vanilla(),
                entity_id,
                Damage::new(DamageType::GENERIC, 10.0),
            )
            .unwrap()
    );

    let entity = tracked_living_entity(&server);
    assert_eq!(entity.get_health(), 17.0);
    assert_eq!(entity.get_last_damage().unwrap().get_amount(), 3.0);
    assert_eq!(entity.get_last_damage_source(), Some("minecraft:generic"));
    let world_uuid = server.world_manager.world_uuids()[0];
    let shared_entity = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .get_entity(entity_id)
        .unwrap();
    assert_eq!(
        shared_entity
            .get_last_damage_source()
            .unwrap()
            .damage_type(),
        &DamageType::GENERIC
    );
    assert_eq!(
        shared_entity.get_last_damage_source().unwrap().get_amount(),
        3.0
    );
    assert!(LIVING_DAMAGE_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    reset_living_damage_test_state();
}

#[test]
fn world_damage_entity_cancellation_does_not_store_last_damage_or_reduce_health() {
    let _lock = LIVING_DAMAGE_TEST_LOCK.lock().unwrap();
    reset_living_damage_test_state();
    let mut server = server_with_living_entity();
    let entity_id = tracked_living_entity_id(&server);
    *LIVING_DAMAGE_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    LIVING_DAMAGE_EVENT_ENABLED.store(true, Ordering::SeqCst);
    LIVING_DAMAGE_EVENT_CANCELLED.store(true, Ordering::SeqCst);
    LIVING_DAMAGE_EVENT_AMOUNT_BITS.store(3.0f32.to_bits(), Ordering::SeqCst);

    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    assert!(
        !world
            .damage_entity(
                &Registries::new_vanilla(),
                entity_id,
                Damage::new(DamageType::GENERIC, 10.0),
            )
            .unwrap()
    );

    let entity = tracked_living_entity(&server);
    assert_eq!(entity.get_health(), 20.0);
    assert!(entity.get_last_damage().is_none());
    reset_living_damage_test_state();
}

#[test]
fn world_damage_entity_uses_player_additional_hearts_before_health_damage() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut player = Player::new(
        Uuid::new_v4(),
        "DamagePlayer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_id = player.get_entity_id();
    player.heal().unwrap();
    player.set_additional_hearts(4.0);
    world.add_entity(Entity::Player(player));

    assert!(
        world
            .damage_entity(
                &Registries::new_vanilla(),
                player_id,
                Damage::new(DamageType::GENERIC, 6.0).without_animation(),
            )
            .unwrap()
    );

    let player = world
        .get_entity(player_id)
        .and_then(|entity| match entity {
            Entity::Player(player) => Some(player),
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Projectile(_) => None,
        })
        .unwrap();
    assert_eq!(player.get_additional_hearts(), 0.0);
    assert_eq!(player.get_health(), 18.0);
    assert_eq!(player.get_last_damage().unwrap().get_amount(), 6.0);
}

#[test]
fn world_damage_entity_dispatches_damage_event_packet_to_current_viewers() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "DamageViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    viewer.set_client(&mut viewer_client);
    viewer.mark_entered_world();
    viewer.mark_chunk_sent_to_client(PlayerChunk::new(0, 0));
    let entity = Entity::Generic(positioned_living_entity());
    let entity_id = entity.get_entity_id();

    world.add_entity(entity);
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    let registries = Registries::new_vanilla();
    assert!(
        world
            .damage_entity(
                &registries,
                entity_id,
                Damage::new(DamageType::GENERIC, 2.0),
            )
            .unwrap()
    );
    let payload = viewer_client
        .queued_outbound_packet_payloads()
        .into_iter()
        .find_map(|(packet_id, payload)| {
            (packet_id == DamageEventPacket::get_id()).then_some(payload)
        })
        .unwrap();
    let packet = DamageEventPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            DamageEventPacket::get_id(),
            EntitySoundEffectPacket::get_id(),
        ]
    );
    assert_eq!(
        packet.damage_type_id,
        registries
            .dynamic_registry_id(
                &spinel_registry::DAMAGE_TYPE_REGISTRY,
                DamageType::GENERIC.key()
            )
            .unwrap()
    );
    assert_eq!(packet.target_entity_id, entity_id.get_value());
}

#[test]
fn lethal_world_damage_runs_living_death_after_damage_and_sound_packets() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "LethalDamageViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    viewer.set_client(&mut viewer_client);
    viewer.mark_entered_world();
    viewer.mark_chunk_sent_to_client(PlayerChunk::new(0, 0));
    let entity = Entity::Generic(positioned_living_entity());
    let entity_id = entity.get_entity_id();
    world.add_entity(entity);
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .damage_entity(
                &Registries::new_vanilla(),
                entity_id,
                Damage::new(DamageType::GENERIC, 20.0),
            )
            .unwrap()
    );

    assert!(world.creatures()[0].is_dead());
    assert_eq!(world.creatures()[0].get_pose(), EntityPose::Dying);
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            DamageEventPacket::get_id(),
            EntitySoundEffectPacket::get_id(),
            EntityStatusPacket::get_id(),
        ]
    );
}

fn server_with_living_entity() -> MinecraftServer {
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(positioned_living_entity()));
    server
}

fn positioned_living_entity() -> GenericEntity {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.heal();
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    entity
}

fn tracked_living_entity_id(server: &MinecraftServer) -> EntityId {
    tracked_living_entity(server).get_entity_id()
}

fn tracked_living_entity(server: &MinecraftServer) -> &GenericEntity {
    server.world_manager.worlds()[0].creatures()[0]
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

fn reset_living_damage_test_state() {
    *LIVING_DAMAGE_TEST_ENTITY.lock().unwrap() = None;
    LIVING_DAMAGE_EVENT_ENABLED.store(false, Ordering::SeqCst);
    LIVING_DAMAGE_EVENT_CANCELLED.store(false, Ordering::SeqCst);
    LIVING_DAMAGE_EVENT_AMOUNT_BITS.store(0, Ordering::SeqCst);
    LIVING_DAMAGE_EVENT_ANIMATE.store(true, Ordering::SeqCst);
    LIVING_DAMAGE_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
}
