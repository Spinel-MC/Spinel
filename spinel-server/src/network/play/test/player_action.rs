use crate::entity::{Entity, EntityPosition, Player};
use crate::network::client::instance::Client;
use crate::network::play::player_action::{
    correct_player_after_failed_digging, should_prevent_breaking,
};
use crate::server::MinecraftServer;
use crate::world::{Block, BlockPosition, ChunkPosition};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::data_components::vanilla_components::{CAN_BREAK, TOOL};
use spinel_registry::{
    BlockPredicate, BlockPredicates, DataComponentPredicates, ItemStack, Material, Registries,
    Tool, ToolRule,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use uuid::Uuid;

#[test]
fn failed_digging_correction_teleports_player_when_block_is_under_position() {
    let mut server = MinecraftServer::new();
    let world = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut player = Player::new(Uuid::nil(), "Digger".to_string(), 0, client.addr);
    player.set_client(&mut client);
    player.set_position(EntityPosition::new(1.5, 65.0, 1.5, 0.0, 0.0));
    server
        .world_manager
        .world_mut(world)
        .unwrap()
        .load_chunk(ChunkPosition::new(0, 0))
        .unwrap();
    server
        .world_manager
        .world_mut(world)
        .unwrap()
        .set_block(BlockPosition::new(1, 64, 1), Block::STONE)
        .unwrap();
    server
        .world_manager
        .add_entity(world, Entity::Player(player));

    assert!(correct_player_after_failed_digging(
        BlockPosition::new(1, 64, 1),
        &mut server,
        &mut client,
    ));

    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![SyncPlayerPositionPacket::get_id()]
    );
}

#[test]
fn digging_prevention_matches_minestom_game_mode_item_component_rules() {
    let registries = Registries::new_vanilla();
    let mut player = Player::new(Uuid::nil(), "Digger".to_string(), 0, test_client().addr);

    player.set_game_mode(GameMode::Spectator);
    assert!(should_prevent_breaking(&player, Block::STONE, &registries));

    player.set_game_mode(GameMode::Adventure);
    assert!(should_prevent_breaking(&player, Block::STONE, &registries));

    let can_break_stone = BlockPredicates::new(vec![BlockPredicate::new(
        Some(spinel_registry::RegistryTagReference::direct(vec![
            Identifier::minecraft("stone"),
        ])),
        None,
        None,
        DataComponentPredicates,
    )]);
    player.set_item_in_hand(
        crate::entity::PlayerHand::Main,
        ItemStack::of(Material::DIAMOND_PICKAXE).with(CAN_BREAK, can_break_stone),
    );
    assert!(!should_prevent_breaking(&player, Block::STONE, &registries));

    player.set_game_mode(GameMode::Creative);
    player.set_item_in_hand(
        crate::entity::PlayerHand::Main,
        ItemStack::of(Material::DIAMOND_SWORD).with(
            TOOL,
            Tool::new(
                vec![ToolRule::new(
                    spinel_registry::RegistryTagReference::empty(),
                    None,
                    None,
                )],
                1.0,
                1,
                false,
            ),
        ),
    );
    assert!(should_prevent_breaking(&player, Block::STONE, &registries));
}

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = TcpStream::connect(addr).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    Client::new(stream, addr)
}
