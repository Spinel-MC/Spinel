use crate::entity::Player;
use crate::inventory::{Inventory, InventoryType};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_registry::{ItemStack, Material};
use spinel_utils::component::Component;
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn double_click_with_open_inventory_collects_player_inventory_before_open_inventory_when_player_slot_was_clicked()
 {
    let mut player = test_player();
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    inventory.set_item_stack(0, ItemStack::of(Material::DIAMOND).with_amount(32));
    player.open_inventory(inventory);
    player
        .get_inventory()
        .set_item_stack(1, ItemStack::of(Material::DIAMOND).with_amount(20));
    player
        .get_inventory()
        .set_item_stack(2, ItemStack::of(Material::DIAMOND).with_amount(20));

    let cursor = ItemStack::of(Material::DIAMOND).with_amount(40);
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;
    let updated_cursor =
        player.collect_double_click_items(9, cursor, player_ptr, &mut server, &mut client);

    assert_eq!(updated_cursor.amount(), 64);
    assert!(player.get_inventory_ref().get_item_stack(1).unwrap().is_air());
    assert_eq!(player.get_inventory_ref().get_item_stack(2).unwrap().amount(), 16);
    assert_eq!(
        player
            .get_opened_inventory()
            .unwrap()
            .get_item_stack(0)
            .unwrap()
            .amount(),
        32
    );
}

#[test]
fn double_click_with_open_inventory_collects_open_inventory_before_player_inventory_when_open_slot_was_clicked()
 {
    let mut player = test_player();
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    inventory.set_item_stack(1, ItemStack::of(Material::DIAMOND).with_amount(20));
    inventory.set_item_stack(2, ItemStack::of(Material::DIAMOND).with_amount(20));
    player.open_inventory(inventory);
    player
        .get_inventory()
        .set_item_stack(1, ItemStack::of(Material::DIAMOND).with_amount(32));

    let cursor = ItemStack::of(Material::DIAMOND).with_amount(40);
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    let player_ptr = &mut player as *mut Player;
    let updated_cursor =
        player.collect_double_click_items(0, cursor, player_ptr, &mut server, &mut client);

    assert_eq!(updated_cursor.amount(), 64);
    assert!(
        player
            .get_opened_inventory()
            .unwrap()
            .get_item_stack(1)
            .unwrap()
            .is_air()
    );
    assert_eq!(
        player
            .get_opened_inventory()
            .unwrap()
            .get_item_stack(2)
            .unwrap()
            .amount(),
        16
    );
    assert_eq!(player.get_inventory_ref().get_item_stack(1).unwrap().amount(), 32);
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
