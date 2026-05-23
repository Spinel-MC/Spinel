use crate::entity::Player;
use crate::inventory::ClickType;
use crate::inventory::Inventory;
use crate::inventory::PlayerInventory;
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct InventoryClickEvent {
    player: *mut Player,
    in_open_inventory: bool,
    slot: i32,
    click_type: ClickType,
    clicked_item: ItemStack,
    cursor_item: ItemStack,
}

impl InventoryClickEvent {
    pub fn new(
        player: *mut Player,
        in_open_inventory: bool,
        slot: i32,
        click_type: ClickType,
        clicked_item: ItemStack,
        cursor_item: ItemStack,
    ) -> Self {
        Self {
            player,
            in_open_inventory,
            slot,
            click_type,
            clicked_item,
            cursor_item,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn slot(&self) -> i32 {
        self.slot
    }

    pub fn in_open_inventory(&self) -> bool {
        self.in_open_inventory
    }

    pub fn inventory(&mut self) -> Option<&Inventory> {
        if !self.in_open_inventory {
            return None;
        }
        self.player().opened_inventory()
    }

    pub fn player_inventory(&mut self) -> Option<&PlayerInventory> {
        if self.in_open_inventory {
            return None;
        }
        Some(self.player().inventory_ref())
    }

    pub fn click_type(&self) -> ClickType {
        self.click_type
    }

    pub fn clicked_item(&self) -> &ItemStack {
        &self.clicked_item
    }

    pub fn cursor_item(&self) -> &ItemStack {
        &self.cursor_item
    }
}
