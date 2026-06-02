use crate::entity::EntityPosition;
use crate::entity::player::{Player, PlayerSkin};
use crate::entity::{Entity, TimedPotionEffect};
use crate::network::client::instance::Client;
use crate::world::{ChunkPosition, World};
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_network::ConnectionState;
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_registry::{Attribute, EntityType, Registries};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use uuid::Uuid;

#[test]
fn automatic_visibility_removes_stale_viewer_relationship_after_entity_moves_out_of_range() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client, "Viewer");
    let target = Entity::new(EntityType::ZOMBIE);
    let target_id = target.entity_id();
    let viewer_id = viewer.entity_id();

    world.add_entity(target);
    world.add_entity(Entity::Player(viewer));
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

    assert!(
        !world
            .entity_by_id(target_id)
            .unwrap()
            .view()
            .is_viewer(viewer_id)
    );
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
    target.attribute(Attribute::MAX_HEALTH);
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
    let target_id = target.entity_id();

    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Player(target));
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
    let target_id = target.entity_id();

    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Player(far_player));
    world.add_entity(Entity::Player(target));
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
        ]
    );
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
