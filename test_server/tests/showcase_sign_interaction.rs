use spinel::core::network::serverbound::play::use_item_on::UseItemOnPacket;
use spinel::network::{Client, ConnectionState, DataType, Position};
use spinel::registry::{EntityType, Identifier, Material};
use spinel::server::MinecraftServer;
use spinel::server::entity::{Entity, EntityPosition, GenericEntity, Player, PlayerHand};
use spinel::server::world::{Block, BlockPosition, ChunkPosition};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use test_server::showcase::ShowcaseSigns;
use uuid::Uuid;

#[test]
fn entity_showcase_sign_interaction_keeps_player_valid_after_spawning_entities() {
    assert_showcase_block_interaction_listener_is_registered();
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let world_id = server
        .world_manager
        .create_world(Identifier::minecraft("showcase_sign_interaction"));
    let sign_position = BlockPosition::new(3, 4, 5);
    let player_uuid = Uuid::new_v4();
    let world = server.world_manager.world_mut(world_id).unwrap();
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.set_block(sign_position, Block::OAK_SIGN).unwrap();
    (0..3).for_each(|entity_offset| {
        let mut entity = GenericEntity::new(EntityType::ARMOR_STAND);
        entity.set_position(EntityPosition::new(
            f64::from(entity_offset),
            5.0,
            0.0,
            0.0,
            0.0,
        ));
        assert!(Entity::Generic(entity).set_instance(world));
    });
    let player = Player::new(player_uuid, "SignInteraction".to_string(), 0, client.addr);
    assert!(Entity::Player(player).set_instance(world));
    assert!(matches!(
        ShowcaseSigns::command_at_position(sign_position),
        Some(test_server::showcase::ShowcaseSignCommand::Entity)
    ));

    dispatch_sign_interaction(&mut server, &mut client, sign_position, 1);

    assert_eq!(creature_count(&server, world_id), 2);
    assert_player_has_pathfinding_stick(&server, world_id, player_uuid);
}

#[test]
fn repeated_entity_showcase_sign_interactions_keep_player_valid() {
    assert_showcase_block_interaction_listener_is_registered();
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let world_id = server
        .world_manager
        .create_world(Identifier::minecraft("repeated_showcase_sign_interaction"));
    let sign_position = BlockPosition::new(3, 4, 5);
    let player_uuid = Uuid::new_v4();
    let world = server.world_manager.world_mut(world_id).unwrap();
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.set_block(sign_position, Block::OAK_SIGN).unwrap();
    let player = Player::new(
        player_uuid,
        "RepeatedInteraction".to_string(),
        0,
        client.addr,
    );
    assert!(Entity::Player(player).set_instance(world));

    (1..=8).for_each(|sequence| {
        dispatch_sign_interaction(&mut server, &mut client, sign_position, sequence);
        assert_eq!(creature_count(&server, world_id), sequence as usize * 2);
        assert_player_has_pathfinding_stick(&server, world_id, player_uuid);
    });

    assert_eq!(creature_count(&server, world_id), 16);
}

fn dispatch_sign_interaction(
    server: &mut MinecraftServer,
    client: &mut Client,
    sign_position: BlockPosition,
    sequence: i32,
) {
    let packet = UseItemOnPacket {
        hand: 0,
        block_position: Position {
            x: sign_position.x,
            y: sign_position.y,
            z: sign_position.z,
        },
        block_face: 2,
        cursor_position_x: 0.5,
        cursor_position_y: 0.5,
        cursor_position_z: 0.5,
        inside_block: false,
        hit_world_border: false,
        sequence,
    };
    let mut payload = Vec::new();
    packet.encode(&mut payload).unwrap();

    assert!(server.dispatch_packet(UseItemOnPacket::get_id(), client, payload));
}

fn assert_player_has_pathfinding_stick(
    server: &MinecraftServer,
    world_id: Uuid,
    player_uuid: Uuid,
) {
    let world = server.world_manager.world(world_id).unwrap();
    let player = world.player_by_uuid(player_uuid).unwrap();
    assert_eq!(
        player.item_in_hand(PlayerHand::Main).material(),
        &Material::STICK
    );
    assert_eq!(
        player.inventory_ref().get_item_stack(1).unwrap().material(),
        &Material::STICK
    );
    assert_eq!(
        player.inventory_ref().get_item_stack(2).unwrap().material(),
        &Material::STICK
    );
}

fn creature_count(server: &MinecraftServer, world_id: Uuid) -> usize {
    server
        .world_manager
        .world(world_id)
        .unwrap()
        .entities()
        .filter(|entity| matches!(entity, Entity::Creature(_)))
        .count()
}

fn assert_showcase_block_interaction_listener_is_registered() {
    assert!(
        spinel::events::inventory::iter::<&'static spinel::events::RegisteredListener>()
            .any(|listener| listener.event_name == "player_block_interact")
    );
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    (Client::new(stream, address), peer_stream)
}
