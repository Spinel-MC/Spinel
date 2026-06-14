use super::super::click_actions::DragDistribution;
use crate::entity::Player;
use crate::inventory::slot_conversion::OFFHAND_SLOT;
use crate::inventory::{ClickType, Inventory, InventoryType};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_registry::{ItemStack, Material};
use spinel_utils::component::Component;
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn player_inventory_offhand_swap_works_without_open_inventory() {
    let mut player = test_player();
    player
        .inventory()
        .set_item_stack(0, ItemStack::of(Material::DIAMOND));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_held_swap(0, OFFHAND_SLOT, player_ptr, &mut server, &mut client));
    assert!(player.inventory_ref().item_stack(0).unwrap().is_air());
    assert_eq!(
        player
            .inventory_ref()
            .item_stack(OFFHAND_SLOT as usize)
            .unwrap()
            .material(),
        &Material::DIAMOND
    );
}

#[test]
fn open_inventory_hotbar_swap_uses_player_inventory_hotbar_slot() {
    let mut player = test_player();
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    inventory.set_item_stack(0, ItemStack::of(Material::EMERALD));
    player.open_inventory(inventory);
    player
        .inventory()
        .set_item_stack(0, ItemStack::of(Material::DIAMOND));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_held_swap(0, 0, player_ptr, &mut server, &mut client));
    assert_eq!(
        player
            .opened_inventory()
            .unwrap()
            .item_stack(0)
            .unwrap()
            .material(),
        &Material::DIAMOND
    );
    assert_eq!(
        player.inventory_ref().item_stack(0).unwrap().material(),
        &Material::EMERALD
    );
}

#[test]
fn open_inventory_offhand_swap_uses_player_inventory_offhand_slot() {
    let mut player = test_player();
    let inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    player.open_inventory(inventory);
    player
        .inventory()
        .set_item_stack(0, ItemStack::of(Material::DIAMOND));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_held_swap(9, OFFHAND_SLOT, player_ptr, &mut server, &mut client));
    assert!(player.inventory_ref().item_stack(0).unwrap().is_air());
    assert_eq!(
        player
            .inventory_ref()
            .item_stack(OFFHAND_SLOT as usize)
            .unwrap()
            .material(),
        &Material::DIAMOND
    );
}

#[test]
fn open_inventory_shift_click_stacks_before_filling_empty_slots() {
    let mut player = test_player();
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    inventory.set_item_stack(0, ItemStack::of(Material::DIAMOND).with_amount(60));
    player.open_inventory(inventory);
    player
        .inventory()
        .set_item_stack(9, ItemStack::of(Material::DIAMOND).with_amount(10));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_shift_click(18, player_ptr, &mut server, &mut client));
    assert!(player.inventory_ref().item_stack(9).unwrap().is_air());
    assert_eq!(
        player
            .opened_inventory()
            .unwrap()
            .item_stack(0)
            .unwrap()
            .amount(),
        64
    );
    assert_eq!(
        player
            .opened_inventory()
            .unwrap()
            .item_stack(1)
            .unwrap()
            .amount(),
        6
    );
}

#[test]
fn dragging_across_open_and_player_inventory_slots_updates_cursor_and_slots() {
    let mut player = test_player();
    let inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    player.open_inventory(inventory);
    player
        .inventory()
        .set_cursor_item(ItemStack::of(Material::DIAMOND).with_amount(10));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_drag(
        vec![0, 9],
        ClickType::LeftDragging,
        DragDistribution::Even,
        player_ptr,
        &mut server,
        &mut client,
    ));
    assert_eq!(
        player
            .opened_inventory()
            .unwrap()
            .item_stack(0)
            .unwrap()
            .amount(),
        5
    );
    assert_eq!(player.inventory_ref().item_stack(0).unwrap().amount(), 5);
    assert!(player.inventory_ref().cursor_item().is_air());
}

#[test]
fn dragging_more_slots_than_cursor_items_places_only_available_items() {
    let mut player = test_player();
    player
        .inventory()
        .set_cursor_item(ItemStack::of(Material::DIAMOND).with_amount(2));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_drag(
        vec![0, 1, 2, 3, 4],
        ClickType::LeftDragging,
        DragDistribution::Even,
        player_ptr,
        &mut server,
        &mut client,
    ));
    assert_eq!(player.inventory_ref().item_stack(0).unwrap().amount(), 1);
    assert_eq!(player.inventory_ref().item_stack(1).unwrap().amount(), 1);
    assert!(player.inventory_ref().item_stack(2).unwrap().is_air());
    assert!(player.inventory_ref().item_stack(3).unwrap().is_air());
    assert!(player.inventory_ref().item_stack(4).unwrap().is_air());
    assert!(player.inventory_ref().cursor_item().is_air());
}

#[test]
fn dragging_overstacked_cursor_to_empty_slot_moves_the_requested_amount() {
    let mut player = test_player();
    player
        .inventory()
        .set_cursor_item(ItemStack::of(Material::ENDER_PEARL).with_amount(20));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_drag(
        vec![0],
        ClickType::LeftDragging,
        DragDistribution::Even,
        player_ptr,
        &mut server,
        &mut client,
    ));
    assert_eq!(player.inventory_ref().item_stack(0).unwrap().amount(), 20);
    assert!(player.inventory_ref().cursor_item().is_air());
}

#[test]
fn left_click_respects_extracted_vanilla_max_stack_size() {
    let mut player = test_player();
    player
        .inventory()
        .set_item_stack(0, ItemStack::of(Material::ENDER_PEARL).with_amount(10));
    player
        .inventory()
        .set_cursor_item(ItemStack::of(Material::ENDER_PEARL).with_amount(10));
    let mut server = MinecraftServer::new();
    let mut client = test_client();

    assert!(player.left_click(0, &mut server, &mut client));
    assert_eq!(player.inventory_ref().item_stack(0).unwrap().amount(), 16);
    assert_eq!(player.inventory_ref().cursor_item().amount(), 4);
}

fn test_player() -> Player {
    Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    )
}

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let _ = listener.accept().unwrap();
    Client::new(stream, addr)
}
