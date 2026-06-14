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
