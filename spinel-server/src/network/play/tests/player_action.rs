use crate::entity::{Player, TimedPotionEffect};
use crate::network::client::instance::Client;
use crate::network::play::block_break_calculation;
use crate::network::play::player_action::should_prevent_breaking;
use crate::world::Block;
use spinel_core::entity::game_mode::GameMode;
use spinel_network::types::Identifier;
use spinel_registry::data_components::vanilla_components::{CAN_BREAK, TOOL};
use spinel_registry::mob_effect::MobEffect;
use spinel_registry::{
    BlockPredicate, BlockPredicates, DataComponentPredicates, ItemStack, Material, Registries,
    Tool, ToolRule,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use uuid::Uuid;

#[test]
fn digging_prevention_matches_minestom_game_mode_item_component_rules() {
    let registries = Registries::new_vanilla();
    let mut player = Player::new(Uuid::nil(), "Digger".to_string(), 0, test_client().addr);

    player.set_game_mode(GameMode::Spectator);
    assert!(should_prevent_breaking(
        &player,
        Block::STONE,
        None,
        &registries
    ));

    player.set_game_mode(GameMode::Adventure);
    assert!(should_prevent_breaking(
        &player,
        Block::STONE,
        None,
        &registries
    ));

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
    assert!(!should_prevent_breaking(
        &player,
        Block::STONE,
        None,
        &registries
    ));

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
    assert!(should_prevent_breaking(
        &player,
        Block::STONE,
        None,
        &registries
    ));
}

#[test]
fn survival_break_ticks_match_minestom_hardness_tool_effect_and_environment_branches() {
    let registries = Registries::new_vanilla();
    let mut player = Player::new(Uuid::nil(), "Digger".to_string(), 0, test_client().addr);
    player.set_on_ground(true);

    assert_eq!(
        block_break_calculation::break_ticks(Block::STONE, &player, &registries, false),
        150
    );
    assert_eq!(
        block_break_calculation::break_ticks(Block::BEDROCK, &player, &registries, false),
        block_break_calculation::UNBREAKABLE
    );

    player.set_item_in_hand(
        crate::entity::PlayerHand::Main,
        ItemStack::of(Material::DIAMOND_PICKAXE),
    );
    assert_eq!(
        block_break_calculation::break_ticks(Block::STONE, &player, &registries, false),
        6
    );

    let haste_id = registries.mob_effect_id(&MobEffect::HASTE).unwrap();
    player.add_effect(TimedPotionEffect::new(haste_id, 1, 200, 0, 0));
    assert_eq!(
        block_break_calculation::break_ticks(Block::STONE, &player, &registries, false),
        5
    );

    player.clear_effects();
    let fatigue_id = registries
        .mob_effect_id(&MobEffect::MINING_FATIGUE)
        .unwrap();
    player.add_effect(TimedPotionEffect::new(fatigue_id, 0, 200, 0, 0));
    assert_eq!(
        block_break_calculation::break_ticks(Block::STONE, &player, &registries, false),
        19
    );

    player.clear_effects();
    player.set_on_ground(false);
    assert_eq!(
        block_break_calculation::break_ticks(Block::STONE, &player, &registries, false),
        29
    );

    player.set_on_ground(true);
    assert_eq!(
        block_break_calculation::break_ticks(Block::STONE, &player, &registries, true),
        29
    );
}

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = TcpStream::connect(addr).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    Client::new(stream, addr)
}
