use crate::entity::{Entity, EntityPosition, GenericEntity, Player};
use crate::network::client::instance::Client;
use crate::world::World;
use spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket;
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::EntityType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use uuid::Uuid;

#[test]
fn set_leash_holder_updates_both_sides_and_sends_attach_packet() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let holder = GenericEntity::new(EntityType::PIG);
    let holder_id = holder.entity_id();
    let leashed = GenericEntity::new(EntityType::COW);
    let leashed_id = leashed.entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(holder));
    world.add_entity(Entity::Generic(leashed));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(world.set_leash_holder(leashed_id, Some(holder_id)).unwrap());

    assert_eq!(
        world.entity_by_id(leashed_id).unwrap().leash_holder(),
        Some(holder_id)
    );
    assert!(
        world
            .entity_by_id(holder_id)
            .unwrap()
            .leashed_entities()
            .contains(&leashed_id)
    );
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![AttachEntityPacket::get_id()]
    );
    viewer_client.discard_queued_outbound_packets();

    assert!(world.set_leash_holder(leashed_id, None).unwrap());
    assert_eq!(world.entity_by_id(leashed_id).unwrap().leash_holder(), None);
    assert!(
        world
            .entity_by_id(holder_id)
            .unwrap()
            .leashed_entities()
            .is_empty()
    );
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![AttachEntityPacket::get_id()]
    );
}

#[test]
fn removing_a_leash_holder_detaches_every_leashed_entity() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let holder = GenericEntity::new(EntityType::PIG);
    let holder_id = holder.entity_id();
    let leashed = GenericEntity::new(EntityType::COW);
    let leashed_id = leashed.entity_id();
    world.add_entity(Entity::Generic(holder));
    world.add_entity(Entity::Generic(leashed));
    world.set_leash_holder(leashed_id, Some(holder_id)).unwrap();

    world.take_entity(holder_id).unwrap();

    assert_eq!(world.entity_by_id(leashed_id).unwrap().leash_holder(), None);
}

#[test]
fn viewer_spawn_and_removal_emit_leash_attach_and_detach_side_effects() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let holder = GenericEntity::new(EntityType::PIG);
    let holder_id = holder.entity_id();
    let leashed = GenericEntity::new(EntityType::COW);
    let leashed_id = leashed.entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(holder));
    world.add_entity(Entity::Generic(leashed));
    world.set_leash_holder(leashed_id, Some(holder_id)).unwrap();
    viewer_client.discard_queued_outbound_packets();

    world.process_pending_entity_visibility_refreshes().unwrap();

    let spawn_packet_ids = viewer_client.queued_outbound_packet_ids();
    let first_attach_index = spawn_packet_ids
        .iter()
        .position(|packet_id| *packet_id == AttachEntityPacket::get_id())
        .unwrap();
    let next_head_look_index = spawn_packet_ids
        .iter()
        .enumerate()
        .find(|(packet_index, packet_id)| {
            *packet_index > first_attach_index && **packet_id == EntityHeadLookPacket::get_id()
        })
        .map(|(packet_index, _)| packet_index)
        .unwrap();
    assert!(first_attach_index < next_head_look_index);
    viewer_client.discard_queued_outbound_packets();
    world
        .move_generic_entity(
            holder_id,
            EntityPosition::new(16.0 * 20.0, 64.0, 0.0, 0.0, 0.0),
            true,
        )
        .unwrap();
    viewer_client.discard_queued_outbound_packets();

    world.process_pending_entity_visibility_refreshes().unwrap();

    let packet_ids = viewer_client.queued_outbound_packet_ids();
    assert_eq!(packet_ids.first(), Some(&AttachEntityPacket::get_id()));
    assert_eq!(packet_ids.last(), Some(&RemoveEntitiesPacket::get_id()));
}

fn entered_player(client: &mut Client) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        "LeashViewer".to_owned(),
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
