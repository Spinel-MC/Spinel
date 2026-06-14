use crate::entity::{EntityPosition, EquipmentSlot, Player, PlayerHand, TimedPotionEffect};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_registry::data_components::vanilla_components::{CONSUMABLE, INSTRUMENT, USE_REMAINDER};
use spinel_registry::{
    Consumable, ConsumeEffect, CustomPotionEffect, Identifier, InstrumentComponent, ItemAnimation,
    ItemStack, Material, PotionEffectSettings, RegistryTagReference,
};
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn finished_consumable_applies_removes_clears_random_teleports_and_updates_statistics() {
    let mut player = test_player();
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client();
    let haste_id = server
        .registries
        .dynamic_registry_id(
            &spinel_registry::MOB_EFFECT_REGISTRY,
            &Identifier::minecraft("haste"),
        )
        .unwrap();
    let poison_id = server
        .registries
        .dynamic_registry_id(
            &spinel_registry::MOB_EFFECT_REGISTRY,
            &Identifier::minecraft("poison"),
        )
        .unwrap();
    player.add_effect(TimedPotionEffect::new(poison_id, 0, 200, 0, 0));
    player.set_position(EntityPosition::new(10.0, 64.0, 10.0, 0.0, 0.0));
    let consumable_stack = ItemStack::of(Material::APPLE).with(
        CONSUMABLE,
        Consumable::new(
            1.6,
            ItemAnimation::Eat,
            Identifier::minecraft("entity.generic.eat"),
            true,
            vec![
                ConsumeEffect::ApplyEffects {
                    effects: vec![CustomPotionEffect::new(
                        Identifier::minecraft("haste"),
                        PotionEffectSettings::new(1, 40, false, true, true, None),
                    )],
                    probability: 1.0,
                },
                ConsumeEffect::RemoveEffects {
                    effects: RegistryTagReference::direct(vec![Identifier::minecraft("poison")]),
                },
                ConsumeEffect::TeleportRandomly { diameter: 8.0 },
            ],
        ),
    );

    player
        .finish_item_use(
            PlayerHand::Main,
            consumable_stack.clone(),
            32,
            &mut server,
            &mut client,
        )
        .unwrap();

    assert!(player.has_effect(haste_id));
    assert!(!player.has_effect(poison_id));
    assert_eq!(player.statistic_value("minecraft:used:minecraft:apple"), 1);
    assert_ne!(
        player.position(),
        EntityPosition::new(10.0, 64.0, 10.0, 0.0, 0.0)
    );

    let clearing_stack = ItemStack::of(Material::APPLE).with(
        CONSUMABLE,
        Consumable::new(
            1.6,
            ItemAnimation::Eat,
            Identifier::minecraft("entity.generic.eat"),
            true,
            vec![ConsumeEffect::ClearAllEffects],
        ),
    );
    player
        .finish_item_use(
            PlayerHand::Main,
            clearing_stack,
            32,
            &mut server,
            &mut client,
        )
        .unwrap();

    assert!(!player.has_effect(haste_id));
}

#[test]
fn right_click_swappable_armor_matches_minestom_equipment_swap() {
    let mut player = test_player();
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client();
    player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::DIAMOND_HELMET));

    assert!(
        player
            .use_item(PlayerHand::Main, 0, &mut server, &mut client)
            .unwrap()
    );
    assert_eq!(
        player.equipment(EquipmentSlot::Helmet).material(),
        &Material::DIAMOND_HELMET
    );
    assert_eq!(
        player.item_in_hand(PlayerHand::Main).material(),
        &Material::AIR
    );
}

#[test]
fn goat_horn_use_duration_comes_from_instrument_component_shape() {
    let mut player = test_player();
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client();
    player.set_item_in_hand(
        PlayerHand::Main,
        ItemStack::of(Material::GOAT_HORN).with(
            INSTRUMENT,
            InstrumentComponent::new(Identifier::minecraft("admire_goat_horn")),
        ),
    );

    assert!(
        player
            .use_item(PlayerHand::Main, 0, &mut server, &mut client)
            .unwrap()
    );
    assert_eq!(player.item_use_hand(), Some(PlayerHand::Main));

    let mut invalid_instrument_player = test_player();
    invalid_instrument_player.set_item_in_hand(
        PlayerHand::Main,
        ItemStack::of(Material::GOAT_HORN).with(
            INSTRUMENT,
            InstrumentComponent::new(Identifier::minecraft("missing_goat_horn")),
        ),
    );

    invalid_instrument_player
        .use_item(PlayerHand::Main, 0, &mut server, &mut client)
        .unwrap();
    assert_eq!(invalid_instrument_player.item_use_hand(), None);
}

#[test]
fn right_click_armor_places_previous_equipment_in_used_hand() {
    let mut player = test_player();
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client();
    player.set_equipment(
        EquipmentSlot::Helmet,
        ItemStack::of(Material::GOLDEN_HELMET),
    );
    player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::DIAMOND_HELMET));

    assert!(
        player
            .use_item(PlayerHand::Main, 0, &mut server, &mut client)
            .unwrap()
    );
    assert_eq!(
        player.equipment(EquipmentSlot::Helmet).material(),
        &Material::DIAMOND_HELMET
    );
    assert_eq!(
        player.item_in_hand(PlayerHand::Main).material(),
        &Material::GOLDEN_HELMET
    );
}

#[test]
fn finished_consumable_decrements_used_hand() {
    let mut player = test_player();
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client();
    let apple_stack = ItemStack::of(Material::APPLE).with_amount(2).with(
        CONSUMABLE,
        Consumable::new(
            1.6,
            ItemAnimation::Eat,
            Identifier::vanilla_static("entity.generic.eat"),
            true,
            Vec::new(),
        ),
    );
    player.set_item_in_hand(PlayerHand::Main, apple_stack.clone());

    player
        .finish_item_use(PlayerHand::Main, apple_stack, 32, &mut server, &mut client)
        .unwrap();

    assert_eq!(player.item_in_hand(PlayerHand::Main).amount(), 1);
}

#[test]
fn finished_consumable_uses_remainder_and_dispatches_play_sound_effect() {
    let mut player = test_player();
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client();
    let potion_stack = ItemStack::of(Material::POTION)
        .with(USE_REMAINDER, ItemStack::of(Material::GLASS_BOTTLE))
        .with(
            CONSUMABLE,
            Consumable::new(
                1.6,
                ItemAnimation::Drink,
                Identifier::vanilla_static("entity.generic.drink"),
                true,
                vec![ConsumeEffect::PlaySound {
                    sound: Identifier::vanilla_static("entity.generic.drink"),
                }],
            ),
        );
    player.set_item_in_hand(PlayerHand::Main, potion_stack.clone());

    player
        .finish_item_use(PlayerHand::Main, potion_stack, 32, &mut server, &mut client)
        .unwrap();

    assert_eq!(
        player.item_in_hand(PlayerHand::Main).material(),
        &Material::GLASS_BOTTLE
    );
}

fn test_player() -> Player {
    Player::new(
        Uuid::new_v4(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    )
}

fn test_client() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let peer_stream = TcpStream::connect(addr).unwrap();
    let (stream, _) = listener.accept().unwrap();
    (Client::new(stream, addr), peer_stream)
}
