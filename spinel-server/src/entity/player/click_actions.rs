use crate::entity::Player;
use crate::events::inventory_click::InventoryClickEvent;
use crate::inventory::{ClickType, InventoryClickProcessor};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::entity::game_mode::GameMode;
use spinel_registry::ItemStack;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DragDistribution {
    Even,
    Single,
}

impl Player {
    pub(super) fn apply_slot_click(
        &mut self,
        slot: i32,
        click_type: ClickType,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(clicked) = self.item_at(slot) else {
            return false;
        };
        let cursor = self.inventory_ref().cursor_item().clone();
        let result = match click_type {
            ClickType::LeftClick => InventoryClickProcessor::left_click(clicked, cursor),
            ClickType::RightClick => InventoryClickProcessor::right_click(clicked, cursor),
            _ => return false,
        };
        if result.is_cancelled() {
            return false;
        }
        self.set_item_at_with_change_event(slot, result.clicked().clone(), player, server, client);
        self.inventory().set_cursor_item(result.cursor().clone());
        self.dispatch_inventory_click(
            slot,
            click_type,
            result.clicked(),
            result.cursor(),
            player,
            server,
            client,
        )
    }

    pub(super) fn apply_held_swap(
        &mut self,
        slot: i32,
        held_slot: i32,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(clicked) = self.item_at(slot) else {
            return false;
        };
        let held_item_slot = self.player_inventory_slot_in_click_window(held_slot);
        let held_item = self.item_at(held_item_slot).unwrap_or_else(ItemStack::air);
        let cursor_is_held =
            self.opened_inventory().is_none() && !self.inventory_ref().cursor_item().is_air();
        if cursor_is_held {
            return false;
        }
        let result = InventoryClickProcessor::change_held(clicked, held_item);
        self.set_item_at_with_change_event(slot, result.clicked().clone(), player, server, client);
        self.set_item_at_with_change_event(
            held_item_slot,
            result.cursor().clone(),
            player,
            server,
            client,
        );
        self.dispatch_inventory_click(
            slot,
            ClickType::ChangeHeld,
            result.clicked(),
            result.cursor(),
            player,
            server,
            client,
        )
    }

    pub(super) fn apply_slot_drop(
        &mut self,
        slot: i32,
        all: bool,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(clicked) = self.item_at(slot) else {
            return false;
        };
        if clicked.is_air() {
            return true;
        }
        let dropped_item = if all {
            clicked.clone()
        } else {
            clicked.with_amount(1)
        };
        if !self.drop_item(dropped_item, server, client) {
            return false;
        }
        let updated_item = if all {
            ItemStack::air()
        } else {
            clicked.consume(1)
        };
        self.set_item_at_with_change_event(slot, updated_item.clone(), player, server, client);
        self.dispatch_inventory_click(
            slot,
            ClickType::Drop,
            &updated_item,
            &clicked,
            player,
            server,
            client,
        )
    }

    pub(super) fn drop_cursor(
        &mut self,
        all: bool,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let cursor = self.inventory_ref().cursor_item().clone();
        if cursor.is_air() {
            return true;
        }
        let dropped_item = if all {
            cursor.clone()
        } else {
            cursor.with_amount(1)
        };
        if !self.drop_item(dropped_item, server, client) {
            return false;
        }
        let updated_cursor = if all {
            ItemStack::air()
        } else {
            cursor.consume(1)
        };
        self.inventory().set_cursor_item(updated_cursor);
        self.dispatch_inventory_click(
            -999,
            ClickType::Drop,
            &ItemStack::air(),
            &cursor,
            player,
            server,
            client,
        )
    }

    pub(super) fn apply_double_click(
        &mut self,
        slot: i32,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(clicked) = self.item_at(slot) else {
            return false;
        };
        let cursor = self.inventory_ref().cursor_item().clone();
        if cursor.is_air() {
            return true;
        }
        let updated_cursor = self.collect_double_click_items(slot, cursor, player, server, client);
        self.inventory().set_cursor_item(updated_cursor.clone());
        let click_was_dispatched = self.dispatch_inventory_click(
            slot,
            ClickType::DoubleClick,
            &clicked,
            &updated_cursor,
            player,
            server,
            client,
        );
        click_was_dispatched && self.resync_inventory_after_bulk_click(client)
    }

    pub(super) fn apply_drag(
        &mut self,
        slots: Vec<i32>,
        click_type: ClickType,
        drag_distribution: DragDistribution,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let cursor = self.inventory_ref().cursor_item().clone();
        if cursor.is_air() || slots.is_empty() {
            return true;
        }
        let amount = match drag_distribution {
            DragDistribution::Even => 1.max(cursor.amount() / slots.len() as i32),
            DragDistribution::Single => 1,
        };
        let remaining_cursor = slots.into_iter().fold(cursor, |remaining, slot| {
            let cursor_before_slot = remaining.clone();
            let remaining_after_slot =
                self.drag_into_slot(slot, remaining, amount, player, server, client);
            if remaining_after_slot.amount() != cursor_before_slot.amount() {
                let clicked_item = self.item_at(slot).unwrap_or_else(ItemStack::air);
                self.dispatch_inventory_click(
                    slot,
                    click_type,
                    &clicked_item,
                    &cursor_before_slot,
                    player,
                    server,
                    client,
                );
            }
            remaining_after_slot
        });
        self.inventory().set_cursor_item(remaining_cursor);
        self.resync_inventory_after_bulk_click(client)
    }

    pub(super) fn apply_middle_click(&mut self, slot: i32) -> bool {
        let Some(clicked) = self.item_at(slot) else {
            return false;
        };
        if self.game_mode() != GameMode::Creative || clicked.is_air() {
            return false;
        }
        self.inventory()
            .set_cursor_item(clicked.with_amount(clicked.max_stack_size()));
        true
    }

    pub(super) fn dispatch_inventory_click(
        &mut self,
        slot: i32,
        click_type: ClickType,
        clicked_item: &ItemStack,
        cursor_item: &ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let in_open_inventory = self.slot_is_in_open_inventory(slot);
        InventoryClickEvent::new(
            player,
            in_open_inventory,
            self.window_slot(slot),
            click_type,
            clicked_item.clone(),
            cursor_item.clone(),
        )
        .dispatch(server, client);
        true
    }
}
