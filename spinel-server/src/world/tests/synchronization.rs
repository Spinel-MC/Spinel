use crate::entity::pathfinding::{PathNode, PathNodeType, PathRequest, PathState};
use crate::entity::{Entity, EntityCreature, EntityPosition, GenericEntity, Player};
use crate::network::client::instance::Client;
use crate::world::{Block, BlockPosition, ChunkPosition, World};
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_network::ConnectionState;
use spinel_network::DataType;
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_registry::{EntityType, Registries};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use uuid::Uuid;

#[test]
fn world_tick_sends_scheduled_position_then_velocity_and_preserves_interval() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let mut target = GenericEntity::new(EntityType::ZOMBIE);
    target.set_position(EntityPosition::new(8.0, 64.0, 4.0, 0.0, 0.0));
    target.set_synchronization_ticks(2);
    target.synchronize_next_tick();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    let registries = Registries::new_vanilla();

    world.tick_with_registries(&registries);

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            EntityPositionSyncPacket::get_id(),
            EntityVelocityPacket::get_id(),
        ]
    );
    viewer_client.discard_queued_outbound_packets();
    world.tick_with_registries(&registries);
    assert!(viewer_client.queued_outbound_packet_ids().is_empty());
    world.tick_with_registries(&registries);
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            EntityPositionSyncPacket::get_id(),
            EntityVelocityPacket::get_id(),
        ]
    );
}

#[test]
fn scheduled_position_synchronization_delta_uses_displacement_from_last_synchronized_position() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let mut target = GenericEntity::new(EntityType::ZOMBIE);
    target.set_position(EntityPosition::new(8.0, 64.0, 4.0, 0.0, 0.0));
    target.set_no_gravity(true);
    target.set_has_physics(false);
    let target_id = target.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    let target = match world.get_entity_mut(target_id).unwrap() {
        Entity::Generic(target) => target,
        _ => panic!("target must remain a generic entity"),
    };
    target.set_position(EntityPosition::new(8.0, 70.0, 4.0, 0.0, 0.0));
    target.set_velocity(Velocity(Vector3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }));
    target.synchronize_next_tick();

    world.tick_with_registries(&Registries::new_vanilla());

    let synchronization_packet_payload = viewer_client
        .queued_outbound_packet_payloads()
        .into_iter()
        .find_map(|(packet_id, payload)| {
            (packet_id == EntityPositionSyncPacket::get_id()).then_some(payload)
        })
        .expect("scheduled synchronization packet must be queued");
    let synchronization_packet =
        EntityPositionSyncPacket::decode(&mut Cursor::new(synchronization_packet_payload)).unwrap();

    assert_eq!(synchronization_packet.position.y, 70.0);
    assert_eq!(synchronization_packet.delta.y, 6.0);
}

#[test]
fn scheduled_position_synchronization_is_suppressed_for_passengers() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let vehicle = GenericEntity::new(EntityType::PIG);
    let vehicle_id = vehicle.get_entity_id();
    let passenger = GenericEntity::new(EntityType::ZOMBIE);
    let passenger_id = passenger.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(vehicle));
    world.add_entity(Entity::Generic(passenger));
    world.add_passenger(vehicle_id, passenger_id).unwrap();
    if let Some(Entity::Generic(passenger)) = world.get_entity_mut(passenger_id) {
        passenger.synchronize_next_tick();
    }
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    world.tick_with_registries(&Registries::new_vanilla());

    assert!(viewer_client.queued_outbound_packet_ids().is_empty());
}

#[test]
fn ordinary_physics_movement_sends_relative_packet_for_non_synchronization_only_entity() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world
        .load_chunk(crate::world::ChunkPosition::new(0, 0))
        .unwrap();
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let mut target = GenericEntity::new(EntityType::ZOMBIE);
    target.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    target.set_no_gravity(true);
    target.set_velocity(Velocity(Vector3d {
        x: 20.0,
        y: 0.0,
        z: 0.0,
    }));
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    world.tick_with_registries(&Registries::new_vanilla());

    assert!(
        viewer_client
            .queued_outbound_packet_ids()
            .contains(&EntityPositionAndRotationPacket::get_id())
    );
}

#[test]
fn creature_pathfinding_tick_sends_body_and_head_rotation_to_viewers() {
    let mut world = World::new(Identifier::minecraft("pathfinding_synchronization"));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    for block_x in 0..=5 {
        world
            .set_block(BlockPosition::new(block_x, 64, 0), Block::STONE)
            .unwrap();
    }
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    creature.set_on_ground(true);
    let creature_id = creature.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Creature(creature));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    {
        let Entity::Creature(creature) = world.get_entity_mut(creature_id).unwrap() else {
            panic!("creature entity must preserve its subtype");
        };
        assert!(
            creature
                .set_path_to(PathRequest::from(EntityPosition::new(
                    4.5, 65.0, 0.5, 0.0, 0.0
                )))
                .unwrap()
        );
    }

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            EntityPositionAndRotationPacket::get_id(),
            EntityHeadLookPacket::get_id(),
            SetEntityDataPacket::get_id(),
        ]
    );
}

#[test]
fn creature_pathfinding_jump_sends_velocity_after_minestom_ground_collision_tick() {
    let mut world = World::new(Identifier::minecraft("pathfinding_jump_velocity"));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    for block_x in 0..=3 {
        world
            .set_block(BlockPosition::new(block_x, 64, 0), Block::STONE)
            .unwrap();
    }
    world
        .set_block(BlockPosition::new(1, 65, 0), Block::STONE)
        .unwrap();
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let mut creature = EntityCreature::new(EntityType::ZOMBIE);
    creature.set_position(EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0));
    creature.set_on_ground(true);
    let creature_id = creature.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Creature(creature));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    {
        let Entity::Creature(creature) = world.get_entity_mut(creature_id).unwrap() else {
            panic!("creature entity must preserve its subtype");
        };
        assert!(
            creature
                .set_path_to(
                    PathRequest::from(EntityPosition::new(2.5, 66.0, 0.5, 0.0, 0.0))
                        .with_minimum_distance(0.1),
                )
                .unwrap()
        );
        let path = creature.get_navigator_mut().get_path_mut().unwrap();
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
    }

    world.tick_with_registries(&Registries::new_vanilla());
    assert!(!viewer_client
        .queued_outbound_packet_ids()
        .contains(&EntityVelocityPacket::get_id()));
    viewer_client.discard_queued_outbound_packets();

    world.tick_with_registries(&Registries::new_vanilla());

    let packet_ids = viewer_client.queued_outbound_packet_ids();
    let Entity::Creature(creature) = world.get_entity(creature_id).unwrap() else {
        panic!("creature entity must preserve its subtype");
    };
    assert!(
        packet_ids.contains(&EntityVelocityPacket::get_id()),
        "queued packet ids: {packet_ids:?}, velocity: {:?}, on_ground: {}",
        creature.get_velocity(),
        creature.is_on_ground()
    );
}
#[test]
fn synchronization_only_entity_suppresses_ordinary_physics_movement_packet() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world
        .load_chunk(crate::world::ChunkPosition::new(0, 0))
        .unwrap();
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let mut target = GenericEntity::new(EntityType::ITEM);
    let target_id = target.get_entity_id();
    target.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    target.set_no_gravity(true);
    target.set_velocity(Velocity(Vector3d {
        x: 20.0,
        y: 0.0,
        z: 0.0,
    }));
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(
        world.get_entity(target_id).unwrap().get_position().get_x(),
        1.9999989999999999
    );
    assert!(
        !viewer_client
            .queued_outbound_packet_ids()
            .contains(&EntityPositionAndRotationPacket::get_id())
    );
}

#[test]
fn switch_entity_type_resends_entity_to_existing_viewers_in_minestom_order() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let target = GenericEntity::new(EntityType::ZOMBIE);
    let target_id = target.get_entity_id();
    let original_bounding_box = target.get_bounding_box();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .switch_entity_type(target_id, EntityType::ARROW)
            .unwrap()
    );

    let target = world.get_entity(target_id).unwrap();
    assert_eq!(target.get_entity_type(), EntityType::ARROW);
    assert_eq!(target.get_bounding_box(), original_bounding_box);
    assert_eq!(
        &viewer_client.queued_outbound_packet_ids()[..2],
        &[RemoveEntitiesPacket::get_id(), SpawnEntityPacket::get_id()]
    );
}

#[test]
fn switch_player_visual_entity_type_uses_entity_destroy_and_spawn_without_player_list_packets() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let target = Player::new(
        Uuid::new_v4(),
        "SwitchedPlayer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25566),
    );
    let target_id = target.get_entity_id();
    let original_bounding_box = target.get_bounding_box();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Player(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .switch_entity_type(target_id, EntityType::ARROW)
            .unwrap()
    );

    let target = world.get_entity(target_id).unwrap();
    let packet_ids = viewer_client.queued_outbound_packet_ids();
    assert_eq!(target.get_entity_type(), EntityType::ARROW);
    assert_eq!(target.get_bounding_box(), original_bounding_box);
    let Entity::Player(target) = target else {
        panic!("target must stay owned by the player entity variant");
    };
    assert!(target.has_entity_collision());
    assert!(!target.can_prevent_block_placement());
    assert_eq!(
        &packet_ids[..2],
        &[RemoveEntitiesPacket::get_id(), SpawnEntityPacket::get_id()]
    );
    assert!(!packet_ids.contains(&PlayerInfoRemovePacket::get_id()));
    assert!(!packet_ids.contains(&PlayerInfoUpdatePacket::get_id()));
}

fn entered_player(client: &mut Client) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        "SynchronizationViewer".to_owned(),
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
