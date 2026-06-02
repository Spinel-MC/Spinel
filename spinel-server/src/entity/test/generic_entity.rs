use crate::entity::{EntityId, EntityPosition, GenericEntity};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::entity_animation::EntityAnimation;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::update_attributes::{
    EntityAttributeModifier, UpdateAttributesPacket,
};
use spinel_network::ConnectionState;
use spinel_network::types::{TeleportFlags, Vector3d, Velocity};
use spinel_registry::EntityType;
use spinel_utils::component::events::HoverEvent;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicI32, Ordering};

#[test]
fn generic_entity_teleport_overloads_update_position_velocity_chunks_and_flags() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let position = EntityPosition::new(1.0, 64.0, 2.0, 90.0, 45.0);
    let velocity = Velocity(Vector3d {
        x: 0.1,
        y: 0.2,
        z: 0.3,
    });

    let plain_teleport = entity.teleport(position);
    assert_eq!(plain_teleport.position(), position);
    assert_eq!(entity.position(), position);

    let velocity_teleport = entity.teleport_with_velocity(position, velocity);
    assert_eq!(velocity_teleport.velocity(), velocity);
    assert_eq!(entity.velocity(), velocity);

    let chunk_teleport = entity.teleport_with_chunks_and_flags(
        position,
        Some(vec![1, 2, 3]),
        TeleportFlags::absolute(),
    );
    assert_eq!(chunk_teleport.chunks(), Some([1, 2, 3].as_slice()));
    assert_eq!(
        chunk_teleport.flags().bitmask(),
        TeleportFlags::absolute().bitmask()
    );

    let full_teleport = entity.teleport_with_velocity_chunks_and_flags(
        EntityPosition::new(4.0, 70.0, 5.0, 180.0, 0.0),
        velocity,
        Some(vec![4]),
        TeleportFlags::absolute(),
    );
    assert_eq!(full_teleport.position().x(), 4.0);
    assert_eq!(full_teleport.chunks(), Some([4].as_slice()));
}

#[test]
fn generic_entity_refresh_position_packet_control_overloads_update_owned_position() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);

    entity.refresh_position_with_packet_controls(
        EntityPosition::new(1.0, 2.0, 3.0, 4.0, 5.0),
        true,
        false,
    );
    assert_eq!(entity.position().x(), 1.0);

    entity.refresh_position_ignoring_view(EntityPosition::new(6.0, 7.0, 8.0, 9.0, 10.0), true);
    assert_eq!(entity.position().x(), 6.0);
    assert_eq!(entity.previous_position().x(), 1.0);
}

#[test]
fn generic_entity_from_client_swing_overloads_preserve_hand_animation() {
    let entity = GenericEntity::new(EntityType::ZOMBIE);

    assert_eq!(
        entity.swing_main_hand_from_client(true).animation,
        EntityAnimation::SwingMainArm
    );
    assert_eq!(
        entity.swing_off_hand_from_client(false).animation,
        EntityAnimation::SwingOffHand
    );
}

#[test]
fn generic_entity_hover_event_uses_type_uuid_and_custom_name() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_custom_name(Some(spinel_utils::component::text::TextComponent::literal(
        "Zombie",
    )));

    let HoverEvent::ShowEntity(hover_entity) = entity.as_hover_event() else {
        panic!("expected show entity hover event");
    };

    assert_eq!(hover_entity.entity_type, "minecraft:zombie");
    assert_eq!(hover_entity.id, entity.uuid().to_string());
    assert!(hover_entity.name.is_some());
}

#[test]
fn generic_entity_snapshot_updater_receives_mutable_snapshot_copy() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let position = EntityPosition::new(3.0, 64.0, 9.0, 45.0, 10.0);
    entity.teleport(position);

    let snapshot = entity.update_snapshot(|snapshot| {
        assert_eq!(snapshot.entity_id(), entity.entity_id());
        assert_eq!(snapshot.entity_type(), EntityType::ZOMBIE);
        assert_eq!(snapshot.position(), position);
    });

    assert_eq!(snapshot.uuid(), entity.uuid());
    assert_eq!(snapshot.position(), position);
}

#[test]
fn generic_entity_event_node_pose_kill_viewer_packets_and_occlusion_match_minestom_surface() {
    static POSE_EVENT_ENTITY_ID: AtomicI32 = AtomicI32::new(0);
    let (mut client, _peer_stream) = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity
        .attribute(1, 20.0)
        .add_modifier(EntityAttributeModifier::attack_speed(
            spinel_network::types::Identifier::minecraft("test_modifier"),
            1.0,
        ));

    entity.event_node().listen("EntityPoseEvent", |entity_id| {
        POSE_EVENT_ENTITY_ID.store(entity_id.value(), Ordering::SeqCst);
    });
    entity.set_pose(2);

    assert_eq!(
        POSE_EVENT_ENTITY_ID.load(Ordering::SeqCst),
        entity.entity_id().value()
    );
    assert_eq!(entity.event_node().listener_count("EntityPoseEvent"), 1);
    assert!(entity.update_new_viewer(&mut client).is_ok());
    assert!(
        client
            .queued_outbound_packet_ids()
            .contains(&SetEquipmentPacket::get_id())
    );
    assert!(
        client
            .queued_outbound_packet_ids()
            .contains(&UpdateAttributesPacket::get_id())
    );
    assert!(entity.add_leashed_entity(EntityId::from_raw(44)));
    assert!(entity.update_old_viewer(&mut client).is_ok());
    assert!(
        client
            .queued_outbound_packet_ids()
            .contains(&RemoveEntitiesPacket::get_id())
    );
    assert!(!entity.is_occluded());
    assert!(entity.kill());
    assert!(entity.is_dead());
    assert_eq!(entity.pose(), 6);
}

fn test_client() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let client_stream = TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    (Client::new(client_stream, addr), peer_stream)
}
