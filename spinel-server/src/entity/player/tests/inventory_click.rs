use crate::entity::Player;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_registry::{ItemStack, Material};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn public_double_click_collects_matching_stacks_into_cursor() {
    let mut player = test_player();
    player
        .get_inventory()
        .set_cursor_item(ItemStack::of(Material::DIAMOND).with_amount(60));
    player
        .get_inventory()
        .set_item_stack(0, ItemStack::of(Material::DIAMOND).with_amount(3));
    player
        .get_inventory()
        .set_item_stack(1, ItemStack::of(Material::DIAMOND).with_amount(5));
    let mut server = MinecraftServer::new();
    let mut client = test_client();

    assert!(player.double_click(0, &mut server, &mut client));
    assert_eq!(player.get_inventory_ref().cursor_item().amount(), 64);
    assert_eq!(
        player
            .get_inventory_ref()
            .get_item_stack(0)
            .unwrap()
            .amount(),
        3
    );
    assert_eq!(
        player
            .get_inventory_ref()
            .get_item_stack(1)
            .unwrap()
            .amount(),
        1
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
