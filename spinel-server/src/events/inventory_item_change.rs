use crate::entity::Player;
use crate::inventory::Inventory;
use crate::inventory::PlayerInventory;
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct InventoryItemChangeEvent {
    player: *mut Player,
    in_open_inventory: bool,
    slot: i32,
    previous_item: ItemStack,
    item: ItemStack,
}

impl InventoryItemChangeEvent {
    pub fn new(
        player: *mut Player,
        in_open_inventory: bool,
        slot: i32,
        previous_item: ItemStack,
        item: ItemStack,
    ) -> Self {
        Self {
            player,
            in_open_inventory,
            slot,
            previous_item,
            item,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn in_open_inventory(&self) -> bool {
        self.in_open_inventory
    }

    pub fn inventory(&mut self) -> Option<&Inventory> {
        if !self.in_open_inventory {
            return None;
        }
        self.player().get_opened_inventory()
    }

    pub fn player_inventory(&mut self) -> Option<&PlayerInventory> {
        if self.in_open_inventory {
            return None;
        }
        Some(self.player().get_inventory_ref())
    }

    pub fn slot(&self) -> i32 {
        self.slot
    }

    pub fn previous_item(&self) -> &ItemStack {
        &self.previous_item
    }

    pub fn item(&self) -> &ItemStack {
        &self.item
    }
}
