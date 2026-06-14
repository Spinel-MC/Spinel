use crate::entity::Player;
use crate::events::inventory_click::InventoryClickEvent;
use crate::events::inventory_item_change::InventoryItemChangeEvent;
use crate::inventory::ClickType;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_registry::ItemStack;

impl Player {
    pub(super) fn item_at(&self, slot: i32) -> Option<ItemStack> {
        if slot < 0 {
            return None;
        }
        let open_inventory_size = self.open_inventory_size();
        if self.opened_inventory().is_some() && slot < open_inventory_size {
            return self
                .opened_inventory()
                .and_then(|inventory| inventory.item_stack(slot as usize))
                .cloned();
        }
        self.inventory_ref()
            .item_stack((slot - open_inventory_size) as usize)
            .cloned()
    }

    pub(super) fn set_item_at(&mut self, slot: i32, item_stack: ItemStack) -> bool {
        if slot < 0 {
            return false;
        }
        let open_inventory_size = self.open_inventory_size();
        if self.opened_inventory().is_some() && slot < open_inventory_size {
            return self
                .opened_inventory_mut()
                .is_some_and(|inventory| inventory.set_item_stack(slot as usize, item_stack));
        }
        self.inventory()
            .set_item_stack((slot - open_inventory_size) as usize, item_stack)
    }

    pub(super) fn set_item_at_with_change_event(
        &mut self,
        slot: i32,
        item_stack: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(previous_item) = self.item_at(slot) else {
            return false;
        };
        if previous_item == item_stack {
            return true;
        }
        let in_open_inventory = self.slot_is_in_open_inventory(slot);
        if !self.set_item_at(slot, item_stack.clone()) {
            return false;
        }
        if !in_open_inventory {
            self.update_inventory_slot_attributes(
                slot - self.open_inventory_size(),
                &previous_item,
                &item_stack,
            );
        }
        InventoryItemChangeEvent::new(
            player,
            in_open_inventory,
            self.window_slot(slot),
            previous_item,
            item_stack,
        )
        .dispatch(server, client);
        let _ = self.sync_slot(slot, client);
        if self.slot_is_held_main_hand(slot) {
            let _ = self.sync_main_hand_attributes(client);
        }
        true
    }

    pub(super) fn collect_double_click_items(
        &mut self,
        excluded_slot: i32,
        cursor: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        if cursor.amount() >= cursor.max_stack_size() {
            return cursor;
        }
        let requested_item = cursor.with_amount(cursor.max_stack_size() - cursor.amount());
        let remaining_item = self.double_click_slots(excluded_slot).into_iter().fold(
            requested_item,
            |remaining_item, slot| {
                self.take_similar_item_from_slot(slot, remaining_item, player, server, client)
            },
        );
        let collected_amount = cursor.max_stack_size() - cursor.amount() - remaining_item.amount();
        cursor.with_amount(cursor.amount() + collected_amount)
    }

    pub(super) fn drag_into_slot(
        &mut self,
        slot: i32,
        cursor: ItemStack,
        amount: i32,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        let Some(clicked) = self.item_at(slot) else {
            return cursor;
        };
        if !clicked.is_air() && !clicked.is_similar(&cursor) {
            return cursor;
        }
        let accepted_amount = if clicked.is_air() {
            amount.min(cursor.amount())
        } else {
            amount
                .min(cursor.amount())
                .min(cursor.max_stack_size() - clicked.amount())
        };
        if accepted_amount <= 0 {
            return cursor;
        }
        let updated_slot = if clicked.is_air() {
            cursor.with_amount(accepted_amount)
        } else {
            clicked.with_amount(clicked.amount() + accepted_amount)
        };
        self.set_item_at_with_change_event(slot, updated_slot, player, server, client);
        cursor.consume(accepted_amount)
    }

    pub(super) fn open_inventory_size(&self) -> i32 {
        self.opened_inventory()
            .map(|inventory| inventory.inventory_type().size() as i32)
            .unwrap_or(0)
    }

    pub(super) fn slot_is_in_open_inventory(&self, slot: i32) -> bool {
        self.opened_inventory().is_some() && slot >= 0 && slot < self.open_inventory_size()
    }

    pub(super) fn window_slot(&self, slot: i32) -> i32 {
        if !self.slot_is_in_open_inventory(slot) {
            return slot - self.open_inventory_size();
        }
        slot
    }

    pub(super) fn player_inventory_slot_in_click_window(&self, player_inventory_slot: i32) -> i32 {
        if self.opened_inventory().is_none() {
            return player_inventory_slot;
        }
        self.open_inventory_size() + player_inventory_slot
    }

    pub(super) fn resync_inventory_after_bulk_click(&self, client: &mut Client) -> bool {
        let _ = self.sync_inventory(client);
        let _ = self.sync_open_inventory_contents(client);
        true
    }

    pub(super) fn move_item_to_slots(
        &mut self,
        item_stack: ItemStack,
        target_slots: impl Iterator<Item = i32>,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        let target_slots = target_slots.collect::<Vec<_>>();
        let occupied_target_slots = target_slots
            .iter()
            .copied()
            .filter(|target_slot| {
                self.item_at(*target_slot)
                    .is_some_and(|target_item| !target_item.is_air())
            })
            .collect::<Vec<_>>();
        let empty_target_slots = target_slots
            .into_iter()
            .filter(|target_slot| {
                self.item_at(*target_slot)
                    .is_some_and(|target_item| target_item.is_air())
            })
            .collect::<Vec<_>>();
        let remaining_item =
            occupied_target_slots
                .into_iter()
                .fold(item_stack, |remaining_item, target_slot| {
                    self.move_item_to_slot(remaining_item, target_slot, player, server, client)
                });
        empty_target_slots
            .into_iter()
            .fold(remaining_item, |remaining_item, target_slot| {
                self.move_item_to_slot(remaining_item, target_slot, player, server, client)
            })
    }

    fn double_click_slots(&self, clicked_slot: i32) -> Vec<i32> {
        let open_inventory_size = self.open_inventory_size();
        let player_inventory_start = open_inventory_size;
        let player_inventory_end = open_inventory_size + self.inventory_ref().inner_size() as i32;
        if self.opened_inventory().is_none() {
            return (0..self.inventory_ref().inner_size() as i32)
                .filter(|slot| *slot != clicked_slot)
                .collect();
        }
        if clicked_slot < open_inventory_size {
            return (0..open_inventory_size)
                .chain(player_inventory_start..player_inventory_end)
                .filter(|slot| *slot != clicked_slot)
                .collect();
        }
        (player_inventory_start..player_inventory_end)
            .chain(0..open_inventory_size)
            .filter(|slot| *slot != clicked_slot)
            .collect()
    }

    fn take_similar_item_from_slot(
        &mut self,
        slot: i32,
        remaining_item: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        if remaining_item.is_air() {
            return remaining_item;
        }
        let Some(item_stack) = self.item_at(slot) else {
            return remaining_item;
        };
        if !remaining_item.is_similar(&item_stack) {
            return remaining_item;
        }
        let removed_amount = remaining_item.amount().min(item_stack.amount());
        self.set_item_at_with_change_event(
            slot,
            item_stack.consume(removed_amount),
            player,
            server,
            client,
        );
        InventoryClickEvent::new(
            player,
            self.slot_is_in_open_inventory(slot),
            self.window_slot(slot),
            ClickType::DoubleClick,
            self.item_at(slot).unwrap_or_else(ItemStack::air),
            self.inventory_ref().cursor_item().clone(),
        )
        .dispatch(server, client);
        remaining_item.consume(removed_amount)
    }

    fn move_item_to_slot(
        &mut self,
        item_stack: ItemStack,
        slot: i32,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        if item_stack.is_air() {
            return item_stack;
        }
        let Some(target_item) = self.item_at(slot) else {
            return item_stack;
        };
        if !target_item.is_air() && !target_item.is_similar(&item_stack) {
            return item_stack;
        }
        let available_amount = item_stack.max_stack_size() - target_item.amount();
        let moved_amount = item_stack.amount().min(available_amount);
        if moved_amount <= 0 {
            return item_stack;
        }
        let updated_target = if target_item.is_air() {
            item_stack.with_amount(moved_amount)
        } else {
            target_item.with_amount(target_item.amount() + moved_amount)
        };
        self.set_item_at_with_change_event(slot, updated_target.clone(), player, server, client);
        InventoryClickEvent::new(
            player,
            self.slot_is_in_open_inventory(slot),
            self.window_slot(slot),
            ClickType::ShiftClick,
            updated_target,
            self.inventory_ref().cursor_item().clone(),
        )
        .dispatch(server, client);
        item_stack.consume(moved_amount)
    }
}
