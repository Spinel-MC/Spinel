use crate::entity::{Entity, EntityPose, Player};
use crate::network::client::instance::Client;
use crate::world::{Block, BlockPosition, ChunkPosition, World};
use spinel_network::types::Identifier;
use spinel_registry::Registries;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use uuid::Uuid;

#[test]
fn player_pose_fit_accepts_sneaking_and_rejects_standing_under_loaded_ceiling() {
    let (mut world, client, player_uuid, _peer_stream) = world_with_player_at_fractional_y();

    world
        .set_block(BlockPosition::new(0, 66, 0), Block::STONE)
        .unwrap();

    assert!(world.set_player_pose(player_uuid, EntityPose::Sneaking));
    assert_eq!(
        world.player_by_uuid(player_uuid).unwrap().get_pose(),
        EntityPose::Sneaking
    );
    assert!(!world.set_player_pose(player_uuid, EntityPose::Standing));
    assert_eq!(
        world.player_by_uuid(player_uuid).unwrap().get_pose(),
        EntityPose::Sneaking
    );

    drop(client);
}

#[test]
fn player_pose_fit_ignores_missing_chunks_like_minestom() {
    let (mut world, client, player_uuid, _peer_stream) = world_with_player_without_loaded_chunks();

    assert!(world.set_player_pose(player_uuid, EntityPose::Standing));
    assert_eq!(
        world.player_by_uuid(player_uuid).unwrap().get_pose(),
        EntityPose::Standing
    );

    drop(client);
}

fn world_with_player_without_loaded_chunks() -> (World, Client, Uuid, TcpStream) {
    let (client, peer_stream) = test_client_pair();
    let player_uuid = Uuid::new_v4();
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut player = Player::new(player_uuid, "Pose".to_owned(), 0, client.addr);
    player.mark_entered_world();
    world.add_entity(Entity::Player(player));
    (world, client, player_uuid, peer_stream)
}

fn world_with_player_at_fractional_y() -> (World, Client, Uuid, TcpStream) {
    let (mut client, peer_stream) = test_client_pair();
    let player_uuid = Uuid::new_v4();
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let mut player = Player::new(player_uuid, "Pose".to_owned(), 0, client.addr);
    player.mark_entered_world();
    world.add_entity(Entity::Player(player));
    world
        .move_player(
            &mut client,
            0.5,
            64.3,
            0.5,
            true,
            &Registries::new_vanilla(),
        )
        .unwrap();
    (world, client, player_uuid, peer_stream)
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    (Client::new(stream, addr), peer_stream)
}
