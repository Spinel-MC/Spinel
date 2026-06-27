use crate::entity::{Entity, EntityPosition, GenericEntity, Player};
use crate::network::client::instance::Client;
use crate::world::World;
use spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::EntityType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use uuid::Uuid;

#[test]
fn add_passenger_reparents_and_updates_both_entity_sides() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let previous_vehicle = positioned_entity(EntityType::COW, 0.0, 64.0, 0.0);
    let previous_vehicle_id = previous_vehicle.get_entity_id();
    let vehicle = positioned_entity(EntityType::PIG, 10.0, 70.0, 12.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 30.0, 40.0, 50.0);
    let passenger_id = passenger.get_entity_id();
    world.add_entity(previous_vehicle);
    world.add_entity(vehicle);
    world.add_entity(passenger);

    assert!(
        world
            .add_passenger(previous_vehicle_id, passenger_id)
            .unwrap()
    );
    assert!(world.add_passenger(vehicle_id, passenger_id).unwrap());

    assert!(
        world
            .entity_by_id(previous_vehicle_id)
            .unwrap()
            .get_passengers()
            .is_empty()
    );
    assert!(
        world
            .entity_by_id(vehicle_id)
            .unwrap()
            .get_passengers()
            .contains(&passenger_id)
    );
    let passenger = world.entity_by_id(passenger_id).unwrap();
    assert_eq!(passenger.get_vehicle(), Some(vehicle_id));
    assert_eq!(
        passenger.get_position(),
        EntityPosition::new(
            10.0,
            70.0 + EntityType::PIG.get_height() * 0.75,
            12.0,
            0.0,
            0.0
        )
    );
}

#[test]
fn passenger_relation_rejects_self_and_immediate_vehicle_cycle() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    world.add_entity(vehicle);
    world.add_entity(passenger);

    assert!(!world.add_passenger(vehicle_id, vehicle_id).unwrap());
    assert!(world.add_passenger(vehicle_id, passenger_id).unwrap());
    assert!(!world.add_passenger(passenger_id, vehicle_id).unwrap());
}

#[test]
fn remove_passenger_clears_both_entity_sides_and_is_idempotent() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    world.add_entity(vehicle);
    world.add_entity(passenger);
    world.add_passenger(vehicle_id, passenger_id).unwrap();

    assert!(world.remove_passenger(vehicle_id, passenger_id).unwrap());
    assert!(!world.remove_passenger(vehicle_id, passenger_id).unwrap());
    assert!(
        world
            .entity_by_id(vehicle_id)
            .unwrap()
            .get_passengers()
            .is_empty()
    );
    assert_eq!(
        world.entity_by_id(passenger_id).unwrap().get_vehicle(),
        None
    );
}

#[test]
fn taking_an_entity_detaches_its_vehicle_and_passenger_relations() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    world.add_entity(vehicle);
    world.add_entity(passenger);
    world.add_passenger(vehicle_id, passenger_id).unwrap();

    world.take_entity(vehicle_id).unwrap();

    assert_eq!(
        world.entity_by_id(passenger_id).unwrap().get_vehicle(),
        None
    );
}

#[test]
fn taking_a_passenger_removes_it_from_its_vehicle() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    world.add_entity(vehicle);
    world.add_entity(passenger);
    world.add_passenger(vehicle_id, passenger_id).unwrap();

    world.take_entity(passenger_id).unwrap();

    assert!(
        world
            .entity_by_id(vehicle_id)
            .unwrap()
            .get_passengers()
            .is_empty()
    );
}

#[test]
fn passenger_mutation_sends_vehicle_packet_before_passenger_position_sync() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(vehicle);
    world.add_entity(passenger);
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    world.add_passenger(vehicle_id, passenger_id).unwrap();
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            SetPassengersPacket::get_id(),
            EntityPositionSyncPacket::get_id(),
        ]
    );
    viewer_client.discard_queued_outbound_packets();

    world.remove_passenger(vehicle_id, passenger_id).unwrap();
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            SetPassengersPacket::get_id(),
            EntityPositionSyncPacket::get_id(),
        ]
    );
}

#[test]
fn vehicle_movement_recursively_refreshes_passenger_chain_positions() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    let nested_passenger = positioned_entity(EntityType::COW, 0.0, 64.0, 0.0);
    let nested_passenger_id = nested_passenger.get_entity_id();
    world.add_entity(vehicle);
    world.add_entity(passenger);
    world.add_entity(nested_passenger);
    world.add_passenger(vehicle_id, passenger_id).unwrap();
    world
        .add_passenger(passenger_id, nested_passenger_id)
        .unwrap();

    assert!(
        world
            .move_generic_entity(
                vehicle_id,
                EntityPosition::new(40.0, 80.0, 24.0, 0.0, 0.0),
                true,
            )
            .unwrap()
    );

    let passenger_position = EntityPosition::new(
        40.0,
        80.0 + EntityType::PIG.get_height() * 0.75,
        24.0,
        0.0,
        0.0,
    );
    assert_eq!(
        world.entity_by_id(passenger_id).unwrap().get_position(),
        passenger_position
    );
    assert_eq!(
        world
            .entity_by_id(nested_passenger_id)
            .unwrap()
            .get_position(),
        EntityPosition::new(
            40.0,
            passenger_position.get_y() + EntityType::ZOMBIE.get_height(),
            24.0,
            0.0,
            0.0,
        )
    );
}

#[test]
fn direct_world_position_mutation_recursively_refreshes_passenger_chain_positions() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    let nested_passenger = positioned_entity(EntityType::COW, 0.0, 64.0, 0.0);
    let nested_passenger_id = nested_passenger.get_entity_id();
    world.add_entity(vehicle);
    world.add_entity(passenger);
    world.add_entity(nested_passenger);
    world.add_passenger(vehicle_id, passenger_id).unwrap();
    world
        .add_passenger(passenger_id, nested_passenger_id)
        .unwrap();

    assert!(world.set_entity_position(vehicle_id, EntityPosition::new(8.0, 90.0, -6.0, 0.0, 0.0)));

    let passenger_position = EntityPosition::new(
        8.0,
        90.0 + EntityType::PIG.get_height() * 0.75,
        -6.0,
        0.0,
        0.0,
    );
    assert_eq!(
        world.entity_by_id(passenger_id).unwrap().get_position(),
        passenger_position
    );
    assert_eq!(
        world
            .entity_by_id(nested_passenger_id)
            .unwrap()
            .get_position(),
        EntityPosition::new(
            8.0,
            passenger_position.get_y() + EntityType::ZOMBIE.get_height(),
            -6.0,
            0.0,
            0.0,
        )
    );
}

#[test]
fn reparenting_a_vehicle_with_existing_passengers_refreshes_the_whole_chain() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let vehicle = positioned_entity(EntityType::PIG, 20.0, 70.0, 10.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    let nested_passenger = positioned_entity(EntityType::COW, 0.0, 64.0, 0.0);
    let nested_passenger_id = nested_passenger.get_entity_id();
    world.add_entity(passenger);
    world.add_entity(nested_passenger);
    world
        .add_passenger(passenger_id, nested_passenger_id)
        .unwrap();
    world.add_entity(vehicle);

    assert!(world.add_passenger(vehicle_id, passenger_id).unwrap());

    let passenger_position = EntityPosition::new(
        20.0,
        70.0 + EntityType::PIG.get_height() * 0.75,
        10.0,
        0.0,
        0.0,
    );
    assert_eq!(
        world.entity_by_id(passenger_id).unwrap().get_position(),
        passenger_position
    );
    assert_eq!(
        world
            .entity_by_id(nested_passenger_id)
            .unwrap()
            .get_position(),
        EntityPosition::new(
            20.0,
            passenger_position.get_y() + EntityType::ZOMBIE.get_height(),
            10.0,
            0.0,
            0.0,
        )
    );
}

#[test]
fn visibility_spawns_passenger_chain_before_reverse_passenger_packets() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let viewer_id = viewer.get_entity_id();
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    let nested_passenger = positioned_entity(EntityType::COW, 0.0, 64.0, 0.0);
    let nested_passenger_id = nested_passenger.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(vehicle);
    world.add_entity(passenger);
    world.add_entity(nested_passenger);
    world.add_passenger(vehicle_id, passenger_id).unwrap();
    world
        .add_passenger(passenger_id, nested_passenger_id)
        .unwrap();
    viewer_client.discard_queued_outbound_packets();

    world.process_pending_entity_visibility_refreshes().unwrap();

    let packet_ids = viewer_client.queued_outbound_packet_ids();
    assert_eq!(
        packet_ids
            .iter()
            .filter(|packet_id| **packet_id == SpawnEntityPacket::get_id())
            .count(),
        3
    );
    assert_eq!(
        packet_ids
            .iter()
            .filter(|packet_id| **packet_id == SetPassengersPacket::get_id())
            .count(),
        2
    );
    let first_passenger_packet_index = packet_ids
        .iter()
        .position(|packet_id| *packet_id == SetPassengersPacket::get_id())
        .unwrap();
    let last_spawn_packet_index = packet_ids
        .iter()
        .rposition(|packet_id| *packet_id == SpawnEntityPacket::get_id())
        .unwrap();
    assert!(last_spawn_packet_index < first_passenger_packet_index);
    assert!(
        [vehicle_id, passenger_id, nested_passenger_id]
            .into_iter()
            .all(|entity_id| world
                .entity_by_id(entity_id)
                .unwrap()
                .get_view()
                .is_viewer(viewer_id))
    );
}

#[test]
fn visibility_hides_vehicle_and_passenger_chain_recursively() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let viewer_id = viewer.get_entity_id();
    let vehicle = positioned_entity(EntityType::PIG, 0.0, 64.0, 0.0);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = positioned_entity(EntityType::ZOMBIE, 0.0, 64.0, 0.0);
    let passenger_id = passenger.get_entity_id();
    let nested_passenger = positioned_entity(EntityType::COW, 0.0, 64.0, 0.0);
    let nested_passenger_id = nested_passenger.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(vehicle);
    world.add_entity(passenger);
    world.add_entity(nested_passenger);
    world.add_passenger(vehicle_id, passenger_id).unwrap();
    world
        .add_passenger(passenger_id, nested_passenger_id)
        .unwrap();
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    world
        .move_generic_entity(
            vehicle_id,
            EntityPosition::new(16.0 * 20.0, 64.0, 0.0, 0.0, 0.0),
            true,
        )
        .unwrap();
    viewer_client.discard_queued_outbound_packets();

    world.process_pending_entity_visibility_refreshes().unwrap();

    assert_eq!(
        viewer_client
            .queued_outbound_packet_ids()
            .into_iter()
            .filter(|packet_id| *packet_id == RemoveEntitiesPacket::get_id())
            .count(),
        3
    );
    assert!(
        [vehicle_id, passenger_id, nested_passenger_id]
            .into_iter()
            .all(|entity_id| !world
                .entity_by_id(entity_id)
                .unwrap()
                .get_view()
                .is_viewer(viewer_id))
    );
}

fn positioned_entity(entity_type: EntityType, x: f64, y: f64, z: f64) -> Entity {
    let mut entity = GenericEntity::new(entity_type);
    entity.set_position(EntityPosition::new(x, y, z, 0.0, 0.0));
    Entity::Generic(entity)
}

fn entered_player(client: &mut Client) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        "PassengerViewer".to_owned(),
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
