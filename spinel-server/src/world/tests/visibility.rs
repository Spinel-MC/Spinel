use crate::entity::EntityPosition;
use crate::entity::player::{Player, PlayerSkin};
use crate::entity::{Entity, TimedPotionEffect};
use crate::network::client::instance::Client;
use crate::world::{ChunkPosition, World};
use spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_position::EntityPositionPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_network::{ConnectionState, DataType};
use spinel_registry::{Attribute, EntityType, Registries};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use uuid::Uuid;

#[test]
fn manual_viewer_add_requires_an_active_entity_and_rejects_self_view() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let inactive_entity = Entity::new(EntityType::ZOMBIE);
    let inactive_entity_id = inactive_entity.get_entity_id();
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let viewer_id = viewer.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .add_entity_viewer(inactive_entity_id, viewer_id)
            .is_err()
    );
    assert!(!world.add_entity_viewer(viewer_id, viewer_id).unwrap());
    assert!(viewer_client.queued_outbound_packet_ids().is_empty());
}

#[test]
fn manual_viewer_add_and_remove_match_minestom_packets_and_no_op_edges() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let viewer_id = viewer.get_entity_id();
    let mut holder = Entity::new(EntityType::PIG);
    holder.get_view_mut().set_auto_viewable(false);
    let holder_id = holder.get_entity_id();
    let mut leashed = Entity::new(EntityType::COW);
    leashed.get_view_mut().set_auto_viewable(false);
    let leashed_id = leashed.get_entity_id();
    let mut passenger = Entity::new(EntityType::ZOMBIE);
    passenger.get_view_mut().set_auto_viewable(false);
    let passenger_id = passenger.get_entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(holder);
    world.add_entity(leashed);
    world.add_entity(passenger);
    world.set_leash_holder(leashed_id, Some(holder_id)).unwrap();
    world.add_passenger(holder_id, passenger_id).unwrap();
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    world.add_entity_viewer(leashed_id, viewer_id).unwrap();
    world.add_entity_viewer(passenger_id, viewer_id).unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(world.add_entity_viewer(holder_id, viewer_id).unwrap());
    assert!(!world.add_entity_viewer(holder_id, viewer_id).unwrap());
    let add_packet_ids = viewer_client.queued_outbound_packet_ids();
    let attach_index = add_packet_ids
        .iter()
        .position(|packet_id| *packet_id == AttachEntityPacket::get_id())
        .unwrap();
    let metadata_index = add_packet_ids
        .iter()
        .position(|packet_id| *packet_id == SetEntityDataPacket::get_id())
        .unwrap();
    let head_look_index = add_packet_ids
        .iter()
        .position(|packet_id| *packet_id == EntityHeadLookPacket::get_id())
        .unwrap();
    assert!(metadata_index < attach_index);
    assert!(attach_index < head_look_index);
    viewer_client.discard_queued_outbound_packets();

    assert!(world.remove_entity_viewer(holder_id, viewer_id).unwrap());
    assert!(!world.remove_entity_viewer(holder_id, viewer_id).unwrap());
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![AttachEntityPacket::get_id(), RemoveEntitiesPacket::get_id()]
    );
    assert!(!world.get_entity(holder_id).unwrap().get_view().is_viewer(viewer_id));
    assert!(
        world
            .get_entity(passenger_id)
            .unwrap()
            .get_view()
            .is_viewer(viewer_id)
    );
}

#[test]
fn viewable_rule_update_refreshes_automatic_visibility_immediately() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let viewer_id = viewer.get_entity_id();
    let target = Entity::new(EntityType::ZOMBIE);
    let target_id = target.get_entity_id();

    world.add_entity(target);
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    assert!(world.get_entity(target_id).unwrap().get_view().is_viewer(viewer_id));
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .update_entity_viewable_rule(target_id, |_| false)
            .unwrap()
    );

    assert!(!world.get_entity(target_id).unwrap().get_view().is_viewer(viewer_id));
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![RemoveEntitiesPacket::get_id()]
    );
    viewer_client.discard_queued_outbound_packets();

    assert!(world.clear_entity_viewable_rule(target_id).unwrap());

    assert!(world.get_entity(target_id).unwrap().get_view().is_viewer(viewer_id));
    assert!(
        viewer_client
            .queued_outbound_packet_ids()
            .contains(&SpawnEntityPacket::get_id())
    );
}

#[test]
fn viewer_rule_update_refreshes_player_automatic_visibility_immediately() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let viewer_id = viewer.get_entity_id();
    let target = Entity::new(EntityType::ZOMBIE);
    let target_id = target.get_entity_id();

    world.add_entity(target);
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    assert!(world.get_entity(target_id).unwrap().get_view().is_viewer(viewer_id));
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .update_entity_viewer_rule(viewer_id, |_| false)
            .unwrap()
    );

    assert!(!world.get_entity(target_id).unwrap().get_view().is_viewer(viewer_id));
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![RemoveEntitiesPacket::get_id()]
    );
    viewer_client.discard_queued_outbound_packets();

    assert!(world.clear_entity_viewer_rule(viewer_id).unwrap());

    assert!(world.get_entity(target_id).unwrap().get_view().is_viewer(viewer_id));
    assert!(
        viewer_client
            .queued_outbound_packet_ids()
            .contains(&SpawnEntityPacket::get_id())
    );
}

#[test]
fn automatic_visibility_removes_stale_viewer_relationship_after_entity_moves_out_of_range() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let target = Entity::new(EntityType::ZOMBIE);
    let target_id = target.get_entity_id();
    let viewer_id = viewer.get_entity_id();

    world.add_entity(target);
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    world.load_chunk(ChunkPosition::new(7, 0)).unwrap();
    world
        .move_player(
            &mut viewer_client,
            16.0 * 7.0,
            64.0,
            0.0,
            true,
            &Registries::new_vanilla(),
        )
        .unwrap();

    assert!(!world.get_entity(target_id).unwrap().get_view().is_viewer(viewer_id));
    let packet_ids = viewer_client.queued_outbound_packet_ids();
    assert_eq!(packet_ids.last(), Some(&RemoveEntitiesPacket::get_id()));
    assert_eq!(
        packet_ids
            .iter()
            .filter(|packet_id| **packet_id == RemoveEntitiesPacket::get_id())
            .count(),
        1
    );
}

#[test]
fn player_spawn_snapshot_sends_living_packets_in_minestom_order() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let mut target = entered_player_without_client("Target");
    target.get_attribute(Attribute::MAX_HEALTH);
    target.add_effect(TimedPotionEffect::new(1, 2, 40, 0, 0));
    target.set_velocity(Velocity(Vector3d {
        x: 0.25,
        y: 0.5,
        z: 0.75,
    }));
    target.add_passenger(crate::entity::EntityId::from_raw(999));

    world.add_entity(Entity::Player(viewer));
    viewer_client.discard_queued_outbound_packets();
    world.add_entity(Entity::Player(target));
    world.process_pending_entity_visibility_refreshes().unwrap();

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            PlayerInfoUpdatePacket::get_id(),
            SpawnEntityPacket::get_id(),
            EntityVelocityPacket::get_id(),
            SetEntityDataPacket::get_id(),
            EntityHeadLookPacket::get_id(),
            SetEquipmentPacket::get_id(),
            UpdateAttributesPacket::get_id(),
            EntityEffectPacket::get_id(),
            SetPassengersPacket::get_id(),
        ]
    );
}

#[test]
fn vanished_player_refresh_removes_player_info_before_entity_removal() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let target = entered_player_without_client("Target");
    let target_id = target.get_entity_id();

    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Player(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(world.set_player_vanished(target_id, true).unwrap());

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            PlayerInfoRemovePacket::get_id(),
            RemoveEntitiesPacket::get_id(),
        ]
    );
}

#[test]
fn skin_refresh_replays_player_to_current_viewers_only() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let mut far_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let mut far_player = entered_player(&mut far_client, "Far");
    far_player.set_position(EntityPosition::new(256.0, 64.0, 0.0, 0.0, 0.0));
    let target = entered_player_without_client("Target");
    let target_id = target.get_entity_id();

    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Player(far_player));
    world.add_entity(Entity::Player(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    far_client.discard_queued_outbound_packets();

    assert!(
        world
            .set_player_skin(
                target_id,
                Some(PlayerSkin::new("textures", Some("signature".to_owned())))
            )
            .unwrap()
    );

    assert_eq!(far_client.queued_outbound_packet_ids(), Vec::<i32>::new());
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            PlayerInfoRemovePacket::get_id(),
            RemoveEntitiesPacket::get_id(),
            PlayerInfoUpdatePacket::get_id(),
            SpawnEntityPacket::get_id(),
            SetEntityDataPacket::get_id(),
            EntityHeadLookPacket::get_id(),
            SetEquipmentPacket::get_id(),
            UpdateAttributesPacket::get_id(),
        ]
    );
}

#[test]
fn add_entity_defers_spawn_packets_until_pending_visibility_refresh() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let target = Entity::new(EntityType::ZOMBIE);

    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    world.add_entity(target);

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        Vec::<i32>::new()
    );

    world.process_pending_entity_visibility_refreshes().unwrap();

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            SpawnEntityPacket::get_id(),
            SetEntityDataPacket::get_id(),
            EntityHeadLookPacket::get_id(),
            SetEquipmentPacket::get_id(),
            UpdateAttributesPacket::get_id(),
        ]
    );
}

#[test]
fn joining_player_receives_existing_generic_entity_replay_packets() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut target = Entity::new(EntityType::ZOMBIE);
    target.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    let target_id = target.get_entity_id();
    world.add_entity(target);
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    world.add_entity(Entity::Player(viewer));

    world.process_pending_entity_visibility_refreshes().unwrap();

    let spawn_payload = viewer_client
        .queued_outbound_packet_payloads()
        .into_iter()
        .find_map(|(packet_id, payload)| {
            (packet_id == SpawnEntityPacket::get_id()).then_some(payload)
        })
        .unwrap();
    let spawn_packet = SpawnEntityPacket::decode(&mut spawn_payload.as_slice()).unwrap();
    assert_eq!(spawn_packet.entity_id, target_id.get_value());
}

#[test]
fn second_player_receives_first_player_relative_movement_packet() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut moving_client = queued_client();
    let mut viewer_client = queued_client();
    let mut moving_player = entered_player(&mut moving_client, "Moving");
    moving_player.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let moving_player_id = moving_player.get_entity_id();
    let mut viewer = entered_player(&mut viewer_client, "Viewer");
    viewer.set_position(EntityPosition::new(2.0, 64.0, 0.0, 0.0, 0.0));
    world.add_entity(Entity::Player(moving_player));
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    moving_client.discard_queued_outbound_packets();
    viewer_client.discard_queued_outbound_packets();

    world
        .move_player(
            &mut moving_client,
            1.0,
            64.0,
            0.0,
            true,
            &Registries::new_vanilla(),
        )
        .unwrap();

    let movement_payload = viewer_client
        .queued_outbound_packet_payloads()
        .into_iter()
        .find_map(|(packet_id, payload)| {
            (packet_id == EntityPositionAndRotationPacket::get_id()).then_some(payload)
        })
        .unwrap();
    let movement_packet =
        EntityPositionAndRotationPacket::decode(&mut movement_payload.as_slice()).unwrap();
    assert_eq!(movement_packet.entity_id, moving_player_id.get_value());
    assert_eq!(
        movement_packet.delta_x,
        EntityPositionPacket::delta(1.0, 0.0)
    );
    assert_eq!(movement_packet.delta_y, 0);
    assert_eq!(movement_packet.delta_z, 0);
}

#[test]
fn tracker_scopes_chunk_and_entity_packets_to_actual_viewers() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let mut far_client = queued_client();
    let mut viewer = entered_player(&mut viewer_client, "Viewer");
    viewer.mark_chunk_sent_to_client(crate::entity::player::PlayerChunk::new(0, 0));
    let mut far_player = entered_player(&mut far_client, "Far");
    far_player.set_position(EntityPosition::new(256.0, 64.0, 0.0, 0.0, 0.0));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Player(far_player));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    far_client.discard_queued_outbound_packets();

    let mut target = Entity::new(EntityType::ZOMBIE);
    target.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    let target_id = target.get_entity_id();
    world.add_entity(target);
    world.process_pending_entity_visibility_refreshes().unwrap();

    assert!(
        viewer_client
            .queued_outbound_packet_ids()
            .contains(&SpawnEntityPacket::get_id())
    );
    assert!(far_client.queued_outbound_packet_ids().is_empty());
    viewer_client.discard_queued_outbound_packets();

    assert!(
        world
            .move_generic_entity(
                target_id,
                EntityPosition::new(2.0, 64.0, 1.0, 15.0, 0.0),
                true,
            )
            .unwrap()
    );

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        [
            EntityPositionAndRotationPacket::get_id(),
            EntityHeadLookPacket::get_id()
        ]
    );
    assert!(far_client.queued_outbound_packet_ids().is_empty());
    viewer_client.discard_queued_outbound_packets();

    assert!(world.take_entity(target_id).is_some());

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        [RemoveEntitiesPacket::get_id()]
    );
    assert!(far_client.queued_outbound_packet_ids().is_empty());
    viewer_client.discard_queued_outbound_packets();

    assert!(world.unload_chunk(ChunkPosition::new(0, 0)).unwrap());

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        [ForgetLevelChunkPacket::get_id()]
    );
    assert!(far_client.queued_outbound_packet_ids().is_empty());
}

fn entered_player(client: &mut Client, username: &str) -> Player {
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

fn entered_player_without_client(username: &str) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        username.to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
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
