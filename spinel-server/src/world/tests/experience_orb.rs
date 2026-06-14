use crate::entity::metadata::definitions;
use crate::entity::player::Player;
use crate::entity::{Entity, EntityId, EntityPosition, ExperienceOrb};
use crate::events::pickup_experience::PickupExperienceEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::World;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_macros::event_listener;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_network::{ConnectionState, DataType};
use spinel_registry::Registries;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI16, Ordering};
use uuid::Uuid;

static EXPERIENCE_PICKUP_TEST_LOCK: Mutex<()> = Mutex::new(());
static EXPERIENCE_PICKUP_CANCELLED: AtomicBool = AtomicBool::new(false);
static EXPERIENCE_PICKUP_COUNT: AtomicI16 = AtomicI16::new(0);

#[event_listener]
fn experience_pickup_listener(event: &mut PickupExperienceEvent, _server: &mut MinecraftServer) {
    EXPERIENCE_PICKUP_COUNT.store(event.experience_count(), Ordering::SeqCst);
    event.set_experience_count(27);
    event.set_cancelled(EXPERIENCE_PICKUP_CANCELLED.load(Ordering::SeqCst));
}

#[test]
fn experience_orb_spawn_count_and_world_query_use_the_dedicated_owner() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let position = EntityPosition::new(3.0, 64.0, 5.0, 0.0, 0.0);

    let entity_id = world.spawn_experience_orb(12, position).unwrap();

    let experience_orb = world.experience_orbs().into_iter().next().unwrap();
    assert_eq!(experience_orb.entity_id(), entity_id);
    assert_eq!(experience_orb.experience_count(), 12);
    assert_eq!(experience_orb.spawn_packet().data, 12);
    assert_eq!(
        experience_orb
            .metadata()
            .value(&definitions::experience_orb::value()),
        MetadataValue::VarInt(12)
    );
    assert_eq!(experience_orb.bounding_box().width(), 0.5);
    assert_eq!(experience_orb.bounding_box().height(), 0.5);
    assert_eq!(experience_orb.bounding_box().depth(), 0.5);
}

#[test]
fn changing_experience_count_removes_and_respawns_the_orb_for_existing_viewers() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer", 0.0);
    let viewer_id = viewer.entity_id();
    let mut experience_orb = ExperienceOrb::new(3);
    experience_orb.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
    experience_orb.view_mut().set_auto_viewable(false);
    let experience_orb_id = experience_orb.entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::ExperienceOrb(experience_orb));
    world
        .add_entity_viewer(experience_orb_id, viewer_id)
        .unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .set_experience_orb_count(experience_orb_id, 19)
            .unwrap()
    );

    let packet_ids = viewer_client.queued_outbound_packet_ids();
    assert_eq!(packet_ids[0], RemoveEntitiesPacket::get_id());
    assert_eq!(packet_ids[1], SpawnEntityPacket::get_id());
    let spawn_payload = viewer_client
        .queued_outbound_packet_payloads()
        .into_iter()
        .find_map(|(packet_id, payload)| {
            (packet_id == SpawnEntityPacket::get_id()).then_some(payload)
        })
        .unwrap();
    let spawn_packet = SpawnEntityPacket::decode(&mut spawn_payload.as_slice()).unwrap();
    assert_eq!(spawn_packet.data, 19);
    assert_eq!(
        match world.entity_by_id(experience_orb_id).unwrap() {
            Entity::ExperienceOrb(experience_orb) => experience_orb
                .metadata()
                .value(&definitions::experience_orb::value()),
            _ => MetadataValue::VarInt(0),
        },
        MetadataValue::VarInt(19)
    );
    assert!(
        world
            .entity_by_id(experience_orb_id)
            .unwrap()
            .view()
            .is_viewer(viewer_id)
    );
}

#[test]
fn experience_orb_targets_the_nearest_player_and_applies_attraction() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_world_age(100).unwrap();
    let player = positioned_player("Target", 4.0);
    let player_id = player.entity_id();
    let experience_orb_id = world
        .spawn_experience_orb(1, EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0))
        .unwrap();
    world.add_entity(Entity::Player(player));

    world.tick_with_registries(&Registries::new_vanilla());

    let experience_orb = experience_orb(&world, experience_orb_id);
    assert_eq!(experience_orb.target(), Some(player_id));
    assert!(experience_orb.velocity().0.x > 0.0);
    assert_eq!(experience_orb.position().x(), 0.0);
}

#[test]
fn nearest_spectator_is_selected_then_cleared_without_falling_back_to_another_player() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_world_age(100).unwrap();
    let mut spectator = positioned_player("Spectator", 2.0);
    spectator.set_game_mode(GameMode::Spectator);
    let normal_player = positioned_player("Normal", 4.0);
    let experience_orb_id = world
        .spawn_experience_orb(1, EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0))
        .unwrap();
    world.add_entity(Entity::Player(spectator));
    world.add_entity(Entity::Player(normal_player));

    world.tick_with_registries(&Registries::new_vanilla());

    let experience_orb = experience_orb(&world, experience_orb_id);
    assert_eq!(experience_orb.target(), None);
    assert_eq!(
        experience_orb.velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    );
}

#[test]
fn experience_orb_motion_matches_minestom_gravity_drag_and_ground_bounce() {
    let mut experience_orb = ExperienceOrb::new(1);
    experience_orb.set_no_gravity(true);
    experience_orb.set_on_ground(true);
    experience_orb.set_velocity(Velocity(Vector3d {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }));

    experience_orb.apply_gravity();
    experience_orb.apply_drag();

    let velocity = experience_orb.velocity().0;
    assert!((velocity.x - 0.588).abs() < f64::EPSILON);
    assert!((velocity.y - -0.6174).abs() < f64::EPSILON);
    assert!((velocity.z - 0.588).abs() < f64::EPSILON);
}

#[test]
fn pickup_event_cancellation_and_player_cooldown_match_minestom() {
    let _lock = EXPERIENCE_PICKUP_TEST_LOCK.lock().unwrap();
    EXPERIENCE_PICKUP_CANCELLED.store(true, Ordering::SeqCst);
    EXPERIENCE_PICKUP_COUNT.store(0, Ordering::SeqCst);
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    world.add_entity(Entity::Player(positioned_player("Collector", 0.0)));
    let experience_orb_id = world
        .spawn_experience_orb(9, EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0))
        .unwrap();

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(EXPERIENCE_PICKUP_COUNT.load(Ordering::SeqCst), 9);
    assert!(world.entity_by_id(experience_orb_id).is_some());
    EXPERIENCE_PICKUP_CANCELLED.store(false, Ordering::SeqCst);
    (0..9).for_each(|_| world.tick_with_registries(&Registries::new_vanilla()));
    assert!(world.entity_by_id(experience_orb_id).is_some());

    world.tick_with_registries(&Registries::new_vanilla());

    assert!(world.entity_by_id(experience_orb_id).is_none());
    EXPERIENCE_PICKUP_CANCELLED.store(false, Ordering::SeqCst);
    EXPERIENCE_PICKUP_COUNT.store(0, Ordering::SeqCst);
}

fn experience_orb(world: &World, entity_id: EntityId) -> &ExperienceOrb {
    match world.entity_by_id(entity_id).unwrap() {
        Entity::ExperienceOrb(experience_orb) => experience_orb,
        _ => panic!("entity is not an experience orb"),
    }
}

fn positioned_player(username: &str, x: f64) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        username.to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_position(EntityPosition::new(x, 64.0, 0.0, 0.0, 0.0));
    player
}

fn entered_player(client: &mut Client, username: &str, x: f64) -> Player {
    let mut player = Player::new(Uuid::new_v4(), username.to_owned(), 0, client.addr);
    player.set_position(EntityPosition::new(x, 64.0, 0.0, 0.0, 0.0));
    player.set_client(client);
    player.mark_entered_world();
    player
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
