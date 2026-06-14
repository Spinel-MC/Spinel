use crate::entity::Player;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_registry::{ItemStack, Material};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn client_close_for_window_zero_refreshes_player_inventory_window() {
    let mut player = test_player();
    player
        .inventory()
        .set_item_stack(36, ItemStack::of(Material::DIAMOND));
    let mut server = MinecraftServer::new();
    let mut client = test_client();

    assert!(player.close_inventory_window_with_client(true, 0, &mut server, &mut client));
    assert!(player.opened_inventory().is_none());
    assert_eq!(
        player.inventory_ref().item_stack(36).unwrap().material(),
        &Material::DIAMOND
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

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let _ = listener.accept().unwrap();
    Client::new(stream, addr)
}
