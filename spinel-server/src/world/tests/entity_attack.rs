use crate::entity::{CreatureEntity, Entity, EntityId, EntityPosition, GenericEntity, Player};
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
    let attack_pair = (event.entity_id(), event.target_id());
    if *ENTITY_ATTACK_TEST_PAIR.lock().unwrap() != Some(attack_pair) {
        return;
    }
    ENTITY_ATTACK_EVENT_LOG.lock().unwrap().push(attack_pair);
}

#[test]
fn creature_attack_entity_dispatches_entity_attack_event_without_client_context() {
    let _lock = ENTITY_ATTACK_TEST_LOCK.lock().unwrap();
    reset_entity_attack_state();
    let mut server = MinecraftServer::new();
    let mut world = event_world("creature_attack_event", &mut server);
    let creature = CreatureEntity::new(EntityType::ZOMBIE);
    let target = GenericEntity::new(EntityType::PLAYER);
    let creature_id = creature.entity_id();
    let target_id = target.entity_id();
    *ENTITY_ATTACK_TEST_PAIR.lock().unwrap() = Some((creature_id, target_id));
    world.add_entity(Entity::Creature(creature));
    world.add_entity(Entity::Generic(target));

    assert!(
        world
            .creature_attack_entity(creature_id, target_id, false)
            .unwrap()
    );

    assert_eq!(
        ENTITY_ATTACK_EVENT_LOG.lock().unwrap().as_slice(),
        [(creature_id, target_id)]
    );
}

#[test]
fn creature_attack_entity_swings_main_hand_before_dispatching_attack_event() {
    let _lock = ENTITY_ATTACK_TEST_LOCK.lock().unwrap();
    reset_entity_attack_state();
    let mut server = MinecraftServer::new();
    let mut world = event_world("creature_attack_swing", &mut server);
    let mut viewer_client = queued_client();
    let viewer = entered_player("AttackViewer", &mut viewer_client);
    let viewer_id = viewer.entity_id();
    let mut creature = CreatureEntity::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    let target = GenericEntity::new(EntityType::PLAYER);
    let creature_id = creature.entity_id();
    let target_id = target.entity_id();
    *ENTITY_ATTACK_TEST_PAIR.lock().unwrap() = Some((creature_id, target_id));
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Creature(creature));
    world.add_entity(Entity::Generic(target));
    world.add_entity_viewer(creature_id, viewer_id).unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .creature_attack_entity(creature_id, target_id, true)
            .unwrap()
    );

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![EntityAnimationPacket::get_id()]
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
