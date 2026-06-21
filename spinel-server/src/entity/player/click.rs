use crate::entity::Player;
use crate::entity::player::click_actions::DragDistribution;
use crate::events::inventory_pre_click::InventoryPreClickEvent;
use crate::inventory::{Click, ClickType};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::serverbound::play::container_click::ContainerClickPacket;
use spinel_network::types::ItemStackHash;

impl Player {
    pub(crate) fn handle_container_click(
        &mut self,
        packet: &ContainerClickPacket,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(container_size) = self.clicked_container_size(packet.container_id) else {
            return false;
        };
        let Some(mut click) = self
            .get_click_preprocessor()
            .process_click(packet, container_size)
        else {
            return false;
        };
        if self.click_is_creative_only(&click) {
            return self.resync_inventory(client);
        }
        let player = self as *mut Player;
        self.set_did_close_inventory(false);
        let mut window_click = click
            .clone()
            .to_window(container_size.map(|size| size as i32));
        let mut event = InventoryPreClickEvent::new(
            player,
            window_click.in_open_inventory(),
            window_click.click().clone(),
        );
        event.dispatch(server, client);
        if self.did_close_inventory() {
            self.set_did_close_inventory(false);
            return self.resync_inventory(client);
        }
        if event.is_cancelled() {
            return self.resync_inventory(client);
        }
        window_click.set_click(event.click().clone());
        click = window_click.from_window(container_size.map(|size| size as i32));
        let handled = self.apply_click(click.clone(), player, server, client);
        if !handled {
            return self.resync_inventory(client);
        }
        if self.cursor_mismatch(packet) {
            return self.sync_cursor(client).is_ok();
        }
        true
    }

    fn clicked_container_size(&self, container_id: i32) -> Option<Option<usize>> {
        if container_id == 0 {
            return Some(None);
        }
        let inventory = self.get_opened_inventory()?;
        if inventory.id() != container_id {
            return None;
        }
        Some(Some(inventory.inventory_type().size()))
    }

    fn click_is_creative_only(&self, click: &Click) -> bool {
        self.get_game_mode() != GameMode::Creative
            && self
                .click_preprocessor
                .is_creative_click(click, !self.get_inventory_ref().cursor_item().is_air())
    }

    fn resync_inventory(&self, client: &mut Client) -> bool {
        let inventory_is_synced = self.sync_inventory(client).is_ok();
        let open_inventory_is_synced = self.sync_open_inventory_contents(client).is_ok();
        inventory_is_synced && open_inventory_is_synced
    }

    pub(super) fn apply_click(
        &mut self,
        click: Click,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        match click {
            Click::Left(slot) => {
                self.apply_slot_click(slot, ClickType::LeftClick, player, server, client)
            }
            Click::Right(slot) => {
                self.apply_slot_click(slot, ClickType::RightClick, player, server, client)
            }
            Click::HotbarSwap { hotbar_slot, slot } => {
                self.apply_held_swap(slot, hotbar_slot, player, server, client)
            }
            Click::OffhandSwap(slot) => self.apply_held_swap(
                slot,
                crate::inventory::slot_conversion::OFFHAND_SLOT,
                player,
                server,
                client,
            ),
            Click::DropSlot { slot, all } => {
                self.apply_slot_drop(slot, all, player, server, client)
            }
            Click::LeftDropCursor => self.drop_cursor(true, player, server, client),
            Click::RightDropCursor => self.drop_cursor(false, player, server, client),
            Click::LeftShift(slot) | Click::RightShift(slot) => {
                self.apply_shift_click(slot, player, server, client)
            }
            Click::Double(slot) => self.apply_double_click(slot, player, server, client),
            Click::LeftDrag(slots) => self.apply_drag(
                slots,
                ClickType::LeftDragging,
                DragDistribution::Even,
                player,
                server,
                client,
            ),
            Click::RightDrag(slots) => self.apply_drag(
                slots,
                ClickType::RightDragging,
                DragDistribution::Single,
                player,
                server,
                client,
            ),
            Click::Middle(slot) => self.apply_middle_click(slot),
            Click::MiddleDrag(_) | Click::MiddleDropCursor => true,
        }
    }

    fn cursor_mismatch(&self, packet: &ContainerClickPacket) -> bool {
        ItemStackHash::from_item_stack(self.get_inventory_ref().cursor_item()) != packet.carried_item
    }
}
