use crate::entity::{EquipmentSlot, PlayerSpawnPoint};
use crate::events::inventory_close::InventoryCloseEvent;
use crate::events::item_drop::ItemDropEvent;
use crate::events::player_change_held_slot::PlayerChangeHeldSlotEvent;
use crate::events::player_swap_item::PlayerSwapItemEvent;
use crate::inventory::{ClickPreprocessor, Inventory, PlayerInventory};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::container_close::ContainerClosePacket;
use spinel_core::network::clientbound::play::set_held_slot::SetHeldSlotPacket;
use spinel_network::ConnectionState;
use spinel_registry::ItemStack;
use std::collections::BTreeSet;

use super::hand::PlayerHand;
use super::state::Player;

impl Player {
    pub fn set_respawn_point(&mut self, respawn_point: PlayerSpawnPoint) {
        self.respawn_point = respawn_point;
    }

    pub fn get_respawn_point(&self) -> PlayerSpawnPoint {
        self.respawn_point
    }

    pub fn get_inventory(&mut self) -> &mut PlayerInventory {
        &mut self.inventory
    }

    pub fn get_inventory_ref(&self) -> &PlayerInventory {
        &self.inventory
    }

    pub fn add_item_stack(&mut self, item_stack: ItemStack) {
        let _ = self.inventory.add_item_stack(item_stack);
        let _ = self.sync_dirty_player_inventory_slots();
    }

    pub fn add_item_stacks(&mut self, item_stacks: Vec<ItemStack>) {
        let _ = self.inventory.add_item_stacks(item_stacks);
        let _ = self.sync_dirty_player_inventory_slots();
    }

    pub(super) fn sync_dirty_player_inventory_slots(&mut self) -> bool {
        let Some(client) = self.client else {
            return false;
        };
        let client = unsafe { &mut *(client as *mut Client) };
        if client.state != ConnectionState::Play {
            return false;
        }
        let dirty_slots = self.inventory.drain_dirty_slots();
        if dirty_slots.is_empty() {
            return true;
        }
        dirty_slots.into_iter().all(|dirty_slot| {
            self.sync_player_inventory_slot(dirty_slot as i32, client)
                .is_ok()
        })
    }

    pub fn open_inventory(&mut self, inventory: Inventory) {
        self.open_inventory = Some(inventory);
        let Some(client) = self.client else {
            return;
        };
        let client = unsafe { &mut *(client as *mut Client) };
        let _ = self.sync_open_inventory(client);
    }

    pub fn close_inventory(&mut self) {
        self.click_preprocessor.clear_cache();
        self.open_inventory = None;
    }

    pub(crate) fn close_inventory_window_with_client(
        &mut self,
        from_client: bool,
        window_id: i32,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if window_id == self.get_inventory_ref().window_id() {
            self.close_inventory();
            return self.sync_player_inventory_window_contents(client).is_ok();
        }
        let Some(open_inventory) = self.get_opened_inventory().cloned() else {
            return false;
        };
        if window_id != open_inventory.window_id() {
            return false;
        }
        let mut event = InventoryCloseEvent::new(self as *mut Player, open_inventory, from_client);
        event.dispatch(server, client);
        if !from_client {
            self.did_close_inventory = true;
        }
        let cursor_item = self.get_inventory_ref().cursor_item().clone();
        self.close_inventory();
        self.get_inventory().set_cursor_item(ItemStack::air());
        if !cursor_item.is_air() && !self.drop_item(cursor_item.clone(), server, client) {
            let _ = self.get_inventory().add_item_stack(cursor_item);
        }
        let player_inventory_window_is_synced =
            self.sync_player_inventory_window_contents(client).is_ok();
        if !from_client {
            let packet_result = ContainerClosePacket {
                container_id: event.get_inventory().id().into(),
            }
            .dispatch(client)
            .is_ok();
            self.did_close_inventory = false;
            return packet_result && player_inventory_window_is_synced;
        }
        self.did_close_inventory = false;
        player_inventory_window_is_synced
    }

    pub fn get_opened_inventory(&self) -> Option<&Inventory> {
        self.open_inventory.as_ref()
    }

    pub fn get_anvil_rename_text(&self) -> Option<&str> {
        self.anvil_rename_text.as_deref()
    }

    pub fn set_anvil_rename_text(&mut self, anvil_rename_text: impl Into<String>) {
        self.anvil_rename_text = Some(anvil_rename_text.into());
    }

    pub fn get_debug_subscriptions(&self) -> &BTreeSet<i32> {
        &self.debug_subscriptions
    }

    pub fn set_debug_subscriptions(&mut self, debug_subscriptions: BTreeSet<i32>) {
        self.debug_subscriptions = debug_subscriptions;
    }

    pub(crate) fn get_opened_inventory_mut(&mut self) -> Option<&mut Inventory> {
        self.open_inventory.as_mut()
    }

    pub fn get_click_preprocessor(&mut self) -> &mut ClickPreprocessor {
        &mut self.click_preprocessor
    }

    pub fn get_did_close_inventory(&self) -> bool {
        self.did_close_inventory
    }

    pub fn set_did_close_inventory(&mut self, did_close_inventory: bool) {
        self.did_close_inventory = did_close_inventory;
    }

    pub fn get_held_slot(&self) -> i32 {
        self.held_slot
    }

    pub fn set_held_slot(&mut self, held_slot: i32) -> bool {
        if !(0..=8).contains(&held_slot) {
            return false;
        }
        self.held_slot = held_slot;
        true
    }

    pub(crate) fn change_held_slot(
        &mut self,
        held_slot: i32,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if !(0..=8).contains(&held_slot) {
            return false;
        }
        let old_slot = self.get_held_slot();
        let mut event = PlayerChangeHeldSlotEvent::new(self as *mut Player, old_slot, held_slot);
        event.dispatch(server, client);
        if event.is_cancelled() {
            let _ = SetHeldSlotPacket {
                slot: old_slot as i8,
            }
            .dispatch(client);
            return false;
        }
        let new_slot = event.new_slot();
        if new_slot != held_slot {
            let _ = SetHeldSlotPacket {
                slot: new_slot as i8,
            }
            .dispatch(client);
        }
        if !self.set_held_slot_with_client(new_slot, client) {
            return false;
        }
        if self.get_item_use_hand() != Some(PlayerHand::Off) {
            self.refresh_active_hand(false, false, false);
            self.clear_item_use();
        }
        true
    }

    pub fn get_item_in_hand(&self, hand: PlayerHand) -> ItemStack {
        let equipment_slot = match hand {
            PlayerHand::Main => EquipmentSlot::MainHand,
            PlayerHand::Off => EquipmentSlot::OffHand,
        };
        self.get_equipment(equipment_slot)
    }

    pub fn set_item_in_hand(&mut self, hand: PlayerHand, item_stack: ItemStack) -> bool {
        let equipment_slot = match hand {
            PlayerHand::Main => EquipmentSlot::MainHand,
            PlayerHand::Off => EquipmentSlot::OffHand,
        };
        self.set_equipment(equipment_slot, item_stack)
    }

    pub fn get_equipment(&self, equipment_slot: EquipmentSlot) -> ItemStack {
        self.get_inventory_ref()
            .get_equipment(equipment_slot, self.held_slot)
    }

    pub fn set_equipment(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) -> bool {
        let previous_item_stack = self.get_equipment(equipment_slot);
        if !self
            .inventory
            .set_equipment(equipment_slot, self.held_slot, item_stack)
        {
            return false;
        }
        let current_item_stack = self.get_equipment(equipment_slot);
        self.living
            .get_attributes_mut()
            .update_equipment_attributes(&previous_item_stack, &current_item_stack, equipment_slot);
        let slot = self
            .inventory
            .slot_for_equipment(equipment_slot, self.held_slot);
        let _ = self.sync_active_equipment_change(slot);
        true
    }

    pub(super) fn sync_active_equipment_change(&self, slot: i32) -> bool {
        let Some(client) = self.client else {
            return false;
        };
        let attributes_packet = self.update_attributes_packet();
        let client = unsafe { &mut *(client as *mut Client) };
        let slot_is_synced = self.sync_player_inventory_slot(slot, client).is_ok();
        let attributes_are_synced = attributes_packet.dispatch(client).is_ok();
        slot_is_synced && attributes_are_synced
    }

    pub(crate) fn swap_item_hands(
        &mut self,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let main_hand_item = self.get_item_in_hand(PlayerHand::Main);
        let off_hand_item = self.get_item_in_hand(PlayerHand::Off);
        let mut event =
            PlayerSwapItemEvent::new(self as *mut Player, off_hand_item, main_hand_item);
        event.dispatch(server, client);
        if event.is_cancelled() {
            return false;
        }
        let main_hand_item = event.main_hand_item().clone();
        let off_hand_item = event.off_hand_item().clone();
        self.set_item_in_hand(PlayerHand::Main, main_hand_item);
        self.set_item_in_hand(PlayerHand::Off, off_hand_item);
        let _ = self.sync_slot(self.held_slot, client);
        let _ = self.sync_slot(crate::inventory::slot_conversion::OFFHAND_SLOT, client);
        let _ = self.sync_main_hand_attributes(client);
        true
    }

    pub(crate) fn drop_main_hand_item(
        &mut self,
        all: bool,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let hand_item = self.get_item_in_hand(PlayerHand::Main);
        if hand_item.is_air() {
            return false;
        }
        let dropped_item = if all {
            hand_item.clone()
        } else {
            hand_item.with_amount(1)
        };
        if !self.drop_item(dropped_item, server, client) {
            let _ = self.sync_inventory(client);
            return false;
        }
        let updated_item = if all {
            ItemStack::air()
        } else {
            hand_item.consume(1)
        };
        self.set_item_in_hand(PlayerHand::Main, updated_item);
        let slot_is_synced = self.sync_slot(self.held_slot, client).is_ok();
        let attributes_are_synced = self.sync_main_hand_attributes(client).is_ok();
        slot_is_synced && attributes_are_synced
    }

    pub(crate) fn drop_item(
        &mut self,
        item_stack: ItemStack,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if item_stack.is_air() {
            return false;
        }
        let mut event = ItemDropEvent::new(self as *mut Player, item_stack);
        event.dispatch(server, client);
        !event.is_cancelled()
    }
}
