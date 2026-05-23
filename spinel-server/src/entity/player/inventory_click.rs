use crate::entity::Player;
use crate::inventory::slot_conversion::OFFHAND_SLOT;
use crate::inventory::{Click, ClickType};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;

impl Player {
    pub fn handle_click(
        &mut self,
        click: Click,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let player = self as *mut Player;
        self.apply_click(click, player, server, client)
    }

    pub fn left_click(
        &mut self,
        slot: i32,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        self.handle_click(Click::Left(slot), server, client)
    }

    pub fn right_click(
        &mut self,
        slot: i32,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        self.handle_click(Click::Right(slot), server, client)
    }

    pub fn shift_click(
        &mut self,
        slot: i32,
        button: i32,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let click = if button == 0 {
            Click::LeftShift(slot)
        } else {
            Click::RightShift(slot)
        };
        self.handle_click(click, server, client)
    }

    pub fn change_held(
        &mut self,
        slot: i32,
        key: i32,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if key == 40 || key == OFFHAND_SLOT {
            return self.handle_click(Click::OffhandSwap(slot), server, client);
        }
        self.handle_click(
            Click::HotbarSwap {
                hotbar_slot: key,
                slot,
            },
            server,
            client,
        )
    }

    pub fn middle_click(
        &mut self,
        slot: i32,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        self.handle_click(Click::Middle(slot), server, client)
    }

    pub fn drop_click(
        &mut self,
        all: bool,
        slot: i32,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        self.handle_click(Click::DropSlot { slot, all }, server, client)
    }

    pub fn dragging(
        &mut self,
        slots: Vec<i32>,
        button: i32,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let player = self as *mut Player;
        match button {
            2 => self.apply_drag(
                slots,
                ClickType::LeftDragging,
                crate::entity::player::click_actions::DragDistribution::Even,
                player,
                server,
                client,
            ),
            6 => self.apply_drag(
                slots,
                ClickType::RightDragging,
                crate::entity::player::click_actions::DragDistribution::Single,
                player,
                server,
                client,
            ),
            10 => self.handle_click(Click::MiddleDrag(slots), server, client),
            _ => false,
        }
    }

    pub fn double_click(
        &mut self,
        slot: i32,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        self.handle_click(Click::Double(slot), server, client)
    }
}

#[cfg(test)]
mod tests {
    use super::Player;
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
            .inventory()
            .set_cursor_item(ItemStack::of(Material::DIAMOND).with_amount(60));
        player
            .inventory()
            .set_item_stack(0, ItemStack::of(Material::DIAMOND).with_amount(3));
        player
            .inventory()
            .set_item_stack(1, ItemStack::of(Material::DIAMOND).with_amount(5));
        let mut server = MinecraftServer::new();
        let mut client = test_client();

        assert!(player.double_click(0, &mut server, &mut client));
        assert_eq!(player.inventory_ref().cursor_item().amount(), 64);
        assert_eq!(player.inventory_ref().item_stack(0).unwrap().amount(), 3);
        assert_eq!(player.inventory_ref().item_stack(1).unwrap().amount(), 1);
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
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
        let addr = listener.local_addr().unwrap();
        let stream = std::net::TcpStream::connect(addr).unwrap();
        let _ = listener.accept().unwrap();
        Client::new(stream, addr)
    }
}
