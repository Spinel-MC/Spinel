use crate::entity::{Entity, EntityCreature, EntityId, EntityPosition, GenericEntity, Player};
use crate::events::entity_attack::EntityAttackEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::World;
use spinel_core::network::clientbound::play::entity_animation::EntityAnimationPacket;
use spinel_macros::event_listener;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::EntityType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use uuid::Uuid;

static ENTITY_ATTACK_TEST_LOCK: Mutex<()> = Mutex::new(());
static ENTITY_ATTACK_TEST_PAIR: Mutex<Option<(EntityId, EntityId)>> = Mutex::new(None);
static ENTITY_ATTACK_EVENT_LOG: Mutex<Vec<(EntityId, EntityId)>> = Mutex::new(Vec::new());

#[event_listener]
fn creature_attack_listener(event: &mut EntityAttackEvent, _server: &mut MinecraftServer) {
    let attack_pair = (event.get_entity_id(), event.get_target_id());
    if *ENTITY_ATTACK_TEST_PAIR.lock().unwrap() != Some(attack_pair) {
        return;
    }
    ENTITY_ATTACK_EVENT_LOG.lock().unwrap().push(attack_pair);
}

#[test]
fn entity_creature_attack_dispatches_entity_attack_event_without_client_context() {
    let _lock = ENTITY_ATTACK_TEST_LOCK.lock().unwrap();
    reset_entity_attack_state();
    let mut server = MinecraftServer::new();
    let mut world = event_world("creature_attack_event", &mut server);
    let creature = EntityCreature::new(EntityType::ZOMBIE);
    let target = Entity::Generic(GenericEntity::new(EntityType::PLAYER));
    let creature_id = creature.get_entity_id();
    let target_id = target.get_entity_id();
    *ENTITY_ATTACK_TEST_PAIR.lock().unwrap() = Some((creature_id, target_id));
    creature.set_world(&mut world);

    let Some(Entity::Creature(creature)) = world.get_entity_mut(creature_id) else {
        panic!("creature must be assigned to the test world");
    };
    creature.attack(&target).unwrap();

    assert_eq!(
        ENTITY_ATTACK_EVENT_LOG.lock().unwrap().as_slice(),
        [(creature_id, target_id)]
    );

    world.tick();

    assert_eq!(
        ENTITY_ATTACK_EVENT_LOG.lock().unwrap().as_slice(),
        [(creature_id, target_id)]
    );
}

#[test]
fn entity_creature_attack_with_swing_swings_main_hand_before_dispatching_attack_event() {
    let _lock = ENTITY_ATTACK_TEST_LOCK.lock().unwrap();
    reset_entity_attack_state();
    let mut server = MinecraftServer::new();
    let world = event_world("creature_attack_swing", &mut server);
    let world_uuid = world.uuid();
    server.world_manager.register_world(world);
    let mut world = server.world_manager.world_mut(world_uuid).unwrap();
    let mut viewer_client = queued_client();
    let viewer = entered_player("AttackViewer", &mut viewer_client);
    let viewer_id = viewer.get_entity_id();
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    let target = Entity::Generic(GenericEntity::new(EntityType::PLAYER));
    let creature_id = creature.get_entity_id();
    let target_id = target.get_entity_id();
    *ENTITY_ATTACK_TEST_PAIR.lock().unwrap() = Some((creature_id, target_id));
    Entity::Player(viewer).set_world(&mut world);
    creature.set_world(&mut world);
    world.add_entity_viewer(creature_id, viewer_id).unwrap();
    viewer_client.discard_queued_outbound_packets();

    let Some(Entity::Creature(creature)) = world.get_entity_mut(creature_id) else {
        panic!("creature must be assigned to the test world");
    };
    creature.attack_with_swing(&target).unwrap();

    assert_eq!(
        ENTITY_ATTACK_EVENT_LOG.lock().unwrap().as_slice(),
        [(creature_id, target_id)]
    );

    assert_eq!(
        viewer_client.queued_outbound_packet_ids().first().copied(),
        Some(EntityAnimationPacket::get_id())
    );
    assert_eq!(
        ENTITY_ATTACK_EVENT_LOG.lock().unwrap().as_slice(),
        [(creature_id, target_id)]
    );
}

fn event_world(name: &str, server: &mut MinecraftServer) -> World {
    let mut world = World::new(Identifier::minecraft(name));
    world.use_server_event_dispatcher(server as *mut MinecraftServer as usize);
    world
}

fn entered_player(username: &str, client: &mut Client) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        username.to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), client.addr.port()),
    );
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

fn reset_entity_attack_state() {
    *ENTITY_ATTACK_TEST_PAIR.lock().unwrap() = None;
    ENTITY_ATTACK_EVENT_LOG.lock().unwrap().clear();
}
