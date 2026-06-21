use crate::entity::Player;
use crate::inventory::slot_conversion::OFFHAND_SLOT;
use crate::inventory::{Inventory, InventoryType};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::container_click::ContainerClickPacket;
use spinel_network::types::{Array, ItemStackHash};
use spinel_registry::{ItemStack, Material};
use spinel_utils::component::Component;
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn player_inventory_shift_click_moves_hotbar_to_inventory() {
    let mut player = test_player();
    player
        .get_inventory()
        .set_item_stack(0, ItemStack::of(Material::DIAMOND));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
    assert!(player.get_inventory_ref().get_item_stack(0).unwrap().is_air());
    assert_eq!(
        player.get_inventory_ref().get_item_stack(9).unwrap().material(),
        &Material::DIAMOND
    );
}

#[test]
fn player_inventory_shift_click_moves_inventory_to_hotbar() {
    let mut player = test_player();
    player
        .get_inventory()
        .set_item_stack(9, ItemStack::of(Material::DIAMOND));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_shift_click(9, player_ptr, &mut server, &mut client));
    assert!(player.get_inventory_ref().get_item_stack(9).unwrap().is_air());
    assert_eq!(
        player.get_inventory_ref().get_item_stack(0).unwrap().material(),
        &Material::DIAMOND
    );
}

#[test]
fn player_inventory_shift_click_moves_offhand_to_inventory_before_hotbar() {
    let mut player = test_player();
    player
        .get_inventory()
        .set_item_stack(OFFHAND_SLOT as usize, ItemStack::of(Material::DIAMOND));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_shift_click(OFFHAND_SLOT, player_ptr, &mut server, &mut client));
    assert!(
        player
            .get_inventory_ref()
            .get_item_stack(OFFHAND_SLOT as usize)
            .unwrap()
            .is_air()
    );
    assert_eq!(
        player.get_inventory_ref().get_item_stack(9).unwrap().material(),
        &Material::DIAMOND
    );
}

#[test]
fn open_inventory_shift_click_uses_minestom_transfer_order() {
    let mut player = test_player();
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    inventory.set_item_stack(0, ItemStack::of(Material::DIAMOND));
    player.open_inventory(inventory);
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
    assert!(
        player
            .get_opened_inventory()
            .unwrap()
            .get_item_stack(0)
            .unwrap()
            .is_air()
    );
    assert_eq!(
        player.get_inventory_ref().get_item_stack(8).unwrap().material(),
        &Material::DIAMOND
    );
}

#[test]
fn open_inventory_consecutive_shift_clicks_use_the_current_open_window() {
    let mut player = test_player();
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    inventory.set_item_stack(0, ItemStack::of(Material::DIAMOND));
    inventory.set_item_stack(1, ItemStack::of(Material::EMERALD));
    inventory.set_item_stack(2, ItemStack::of(Material::GOLD_INGOT));
    player.open_inventory(inventory);
    let container_id = player.get_opened_inventory().unwrap().id();
    let mut server = MinecraftServer::new();
    let mut client = test_client();

    assert!(player.handle_container_click(
        &container_click_packet(0, 1, 0, container_id),
        &mut server,
        &mut client,
    ));
    assert!(player.handle_container_click(
        &container_click_packet(1, 1, 0, container_id),
        &mut server,
        &mut client,
    ));
    assert!(player.handle_container_click(
        &container_click_packet(2, 1, 0, container_id),
        &mut server,
        &mut client,
    ));

    assert!(
        player
            .get_opened_inventory()
            .unwrap()
            .get_item_stack(0)
            .unwrap()
            .is_air()
    );
    assert!(
        player
            .get_opened_inventory()
            .unwrap()
            .get_item_stack(1)
            .unwrap()
            .is_air()
    );
    assert!(
        player
            .get_opened_inventory()
            .unwrap()
            .get_item_stack(2)
            .unwrap()
            .is_air()
    );
    assert_eq!(
        player.get_inventory_ref().get_item_stack(8).unwrap().material(),
        &Material::DIAMOND
    );
    assert_eq!(
        player.get_inventory_ref().get_item_stack(7).unwrap().material(),
        &Material::EMERALD
    );
    assert_eq!(
        player.get_inventory_ref().get_item_stack(6).unwrap().material(),
        &Material::GOLD_INGOT
    );
}

#[test]
fn open_inventory_rejects_clicks_for_a_different_window_id() {
    let mut player = test_player();
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    inventory.set_item_stack(0, ItemStack::of(Material::DIAMOND));
    player.open_inventory(inventory);
    let container_id = player.get_opened_inventory().unwrap().id() + 1;
    let mut server = MinecraftServer::new();
    let mut client = test_client();

    assert!(!player.handle_container_click(
        &container_click_packet(0, 1, 0, container_id),
        &mut server,
        &mut client,
    ));
    assert_eq!(
        player
            .get_opened_inventory()
            .unwrap()
            .get_item_stack(0)
            .unwrap()
            .material(),
        &Material::DIAMOND
    );
    assert!(player.get_inventory_ref().get_item_stack(8).unwrap().is_air());
}

#[test]
fn player_inventory_shift_click_equips_registry_backed_head_items() {
    let mut player = test_player();
    player
        .get_inventory()
        .set_item_stack(0, ItemStack::of(Material::CARVED_PUMPKIN));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
    assert!(player.get_inventory_ref().get_item_stack(0).unwrap().is_air());
    assert_eq!(
        player
            .get_inventory_ref()
            .get_equipment(crate::entity::EquipmentSlot::Helmet, player.get_held_slot())
            .material(),
        &Material::CARVED_PUMPKIN
    );
}

#[test]
fn player_inventory_shift_click_equips_extracted_component_helmet() {
    let mut player = test_player();
    player
        .get_inventory()
        .set_item_stack(0, ItemStack::of(Material::DIAMOND_HELMET));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
    assert!(player.get_inventory_ref().get_item_stack(0).unwrap().is_air());
    assert_eq!(
        player
            .get_inventory_ref()
            .get_equipment(crate::entity::EquipmentSlot::Helmet, player.get_held_slot())
            .material(),
        &Material::DIAMOND_HELMET
    );
}

#[test]
fn window_zero_hotbar_shift_click_equips_extracted_component_helmet() {
    let mut player = test_player();
    player
        .get_inventory()
        .set_item_stack(0, ItemStack::of(Material::DIAMOND_HELMET));
    let mut server = MinecraftServer::new();
    let mut client = test_client();

    assert!(player.handle_container_click(
        &container_click_packet(36, 1, 0, 0),
        &mut server,
        &mut client
    ));
    assert!(player.get_inventory_ref().get_item_stack(0).unwrap().is_air());
    assert_eq!(
        player
            .get_inventory_ref()
            .get_equipment(crate::entity::EquipmentSlot::Helmet, player.get_held_slot())
            .material(),
        &Material::DIAMOND_HELMET
    );
}

#[test]
fn player_inventory_shift_click_equips_registry_backed_chest_items() {
    let mut player = test_player();
    player
        .get_inventory()
        .set_item_stack(0, ItemStack::of(Material::ELYTRA));
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;

    assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
    assert!(player.get_inventory_ref().get_item_stack(0).unwrap().is_air());
    assert_eq!(
        player
            .get_inventory_ref()
            .get_equipment(crate::entity::EquipmentSlot::Chestplate, player.get_held_slot())
            .material(),
        &Material::ELYTRA
    );
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

fn container_click_packet(
    slot: i16,
    click_type: i32,
    button: i8,
    container_id: i32,
) -> ContainerClickPacket {
    ContainerClickPacket {
        container_id,
        state_id: 0,
        slot,
        button,
        click_type,
        changed_slots: Array(Vec::new()),
        carried_item: ItemStackHash::from_item_stack(&ItemStack::air()),
    }
}
