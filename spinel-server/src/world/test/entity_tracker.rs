use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity, Player};
use crate::world::{ChunkPosition, EntityTrackerTarget, World};
use spinel_network::types::Identifier;
use spinel_registry::EntityType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn tracker_registration_indexes_players_and_generic_entities() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let player = Player::new(
        Uuid::new_v4(),
        "TrackerPlayer".to_string(),
        0,
        local_address(1),
    );
    let player_id = player.entity_id();
    let player_uuid = player.uuid();
    let generic = positioned_entity(EntityType::ZOMBIE, 24.0, 64.0, 8.0);
    let generic_id = generic.entity_id();
    let generic_uuid = generic.uuid();

    world.add_entity(Entity::Player(player));
    world.add_entity(Entity::Generic(generic));

    assert_eq!(
        world.entity_tracker().entity_by_id(player_id),
        Some(player_id)
    );
    assert_eq!(
        world.entity_tracker().entity_by_uuid(generic_uuid),
        Some(generic_id)
    );
    assert_eq!(
        world.entity_by_uuid(player_uuid).map(Entity::entity_id),
        Some(player_id)
    );
    assert!(
        ids(world
            .entity_tracker()
            .entities(EntityTrackerTarget::Entities))
        .contains(&generic_id)
    );
    assert!(
        ids(world
            .entity_tracker()
            .entities(EntityTrackerTarget::Players))
        .contains(&player_id)
    );
}

#[test]
fn tracker_movement_updates_chunk_membership() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let entity = positioned_entity(EntityType::ZOMBIE, 1.0, 64.0, 1.0);
    let entity_id = entity.entity_id();

    world.add_entity(Entity::Generic(entity));
    assert_eq!(world.chunk_entities(ChunkPosition::new(0, 0)).len(), 1);

    world
        .entity_tracker_mut()
        .move_entity(entity_id, EntityPosition::new(33.0, 64.0, 1.0, 0.0, 0.0));

    assert!(world.chunk_entities(ChunkPosition::new(0, 0)).is_empty());
    assert_eq!(world.chunk_entities(ChunkPosition::new(2, 0)).len(), 1);
}

#[test]
fn nearby_entity_queries_filter_by_distance() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let near_entity = positioned_entity(EntityType::ZOMBIE, 3.0, 64.0, 4.0);
    let near_entity_id = near_entity.entity_id();
    let far_entity = positioned_entity(EntityType::ZOMBIE, 40.0, 64.0, 40.0);

    world.add_entity(Entity::Generic(near_entity));
    world.add_entity(Entity::Generic(far_entity));

    let nearby_ids = ids_from_entities(
        world.nearby_entities(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0), 8.0),
    );

    assert_eq!(nearby_ids, vec![near_entity_id]);
}

#[test]
fn chunk_range_queries_include_entities_in_chunk_square() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let center_entity = positioned_entity(EntityType::ZOMBIE, 1.0, 64.0, 1.0);
    let adjacent_entity = positioned_entity(EntityType::ZOMBIE, 18.0, 64.0, 1.0);
    let outside_entity = positioned_entity(EntityType::ZOMBIE, 48.0, 64.0, 1.0);
    let center_id = center_entity.entity_id();
    let adjacent_id = adjacent_entity.entity_id();

    world.add_entity(Entity::Generic(center_entity));
    world.add_entity(Entity::Generic(adjacent_entity));
    world.add_entity(Entity::Generic(outside_entity));

    let chunk_range_ids = ids(world.entity_tracker().nearby_entities_by_chunk_range(
        EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0),
        1,
        EntityTrackerTarget::Entities,
    ));

    assert_eq!(chunk_range_ids, ids(vec![center_id, adjacent_id]));
}

#[test]
fn item_and_experience_orb_targets_use_vanilla_entity_types() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let item = positioned_entity(EntityType::ITEM, 1.0, 64.0, 1.0);
    let item_id = item.entity_id();
    let experience_orb = positioned_entity(EntityType::EXPERIENCE_ORB, 2.0, 64.0, 1.0);
    let experience_orb_id = experience_orb.entity_id();

    world.add_entity(Entity::Generic(item));
    world.add_entity(Entity::Generic(experience_orb));

    assert_eq!(
        ids(world.entity_tracker().entities(EntityTrackerTarget::Items)),
        vec![item_id]
    );
    assert_eq!(
        ids(world
            .entity_tracker()
            .entities(EntityTrackerTarget::ExperienceOrbs)),
        vec![experience_orb_id]
    );
    assert_eq!(world.experience_orbs().len(), 1);
}

#[test]
fn creature_queries_include_living_generic_entities_only() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let player = Player::new(
        Uuid::new_v4(),
        "CreatureFilter".to_string(),
        0,
        local_address(2),
    );
    let zombie = positioned_entity(EntityType::ZOMBIE, 1.0, 64.0, 1.0);
    let zombie_id = zombie.entity_id();
    let item = positioned_entity(EntityType::ITEM, 2.0, 64.0, 1.0);

    world.add_entity(Entity::Player(player));
    world.add_entity(Entity::Generic(zombie));
    world.add_entity(Entity::Generic(item));

    assert_eq!(
        world
            .creatures()
            .into_iter()
            .map(GenericEntity::entity_id)
            .collect::<Vec<_>>(),
        vec![zombie_id]
    );
}

#[test]
fn chunk_load_and_unload_manage_tracker_partitions_and_generic_entities() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(0, 0);
    let entity = positioned_entity(EntityType::ZOMBIE, 1.0, 64.0, 1.0);
    let entity_id = entity.entity_id();

    world.load_chunk(chunk_position).unwrap();
    world.add_entity(Entity::Generic(entity));

    assert!(world.entity_tracker().has_chunk_partition(chunk_position));
    assert_eq!(world.chunk_entities(chunk_position).len(), 1);

    world.unload_chunk(chunk_position).unwrap();

    assert!(!world.entity_tracker().has_chunk_partition(chunk_position));
    assert!(world.entity_tracker().entity_by_id(entity_id).is_none());
    assert!(world.chunk_entities(chunk_position).is_empty());
}

#[test]
fn viewable_chunk_players_returns_players_in_world_view_distance() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_view_distance(2);
    let close_player = positioned_player("CloseViewer", local_address(3), 16.0, 64.0, 16.0);
    let close_player_id = close_player.entity_id();
    let far_player = positioned_player("FarViewer", local_address(4), 96.0, 64.0, 96.0);

    world.add_entity(Entity::Player(close_player));
    world.add_entity(Entity::Player(far_player));

    assert_eq!(
        world
            .viewable_chunk_players(ChunkPosition::new(0, 0))
            .into_iter()
            .map(Player::entity_id)
            .collect::<Vec<_>>(),
        vec![close_player_id]
    );
}

fn positioned_entity(entity_type: EntityType, x: f64, y: f64, z: f64) -> GenericEntity {
    let mut entity = GenericEntity::new(entity_type);
    entity.set_position(EntityPosition::new(x, y, z, 0.0, 0.0));
    entity
}

fn positioned_player(username: &str, address: SocketAddr, x: f64, y: f64, z: f64) -> Player {
    let mut player = Player::new(Uuid::new_v4(), username.to_string(), 0, address);
    player.set_position(EntityPosition::new(x, y, z, 0.0, 0.0));
    player
}

fn local_address(port: u16) -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)
}

fn ids(mut entity_ids: Vec<EntityId>) -> Vec<EntityId> {
    entity_ids.sort();
    entity_ids
}

fn ids_from_entities(entities: Vec<&Entity>) -> Vec<EntityId> {
    ids(entities.into_iter().map(Entity::entity_id).collect())
}
