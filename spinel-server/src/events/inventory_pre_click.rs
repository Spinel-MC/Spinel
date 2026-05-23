use crate::entity::Player;
use crate::inventory::PlayerInventory;
use crate::inventory::{Click, Inventory};
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct InventoryPreClickEvent {
    player: *mut Player,
    in_open_inventory: bool,
    click: Click,
    cancelled: bool,
}

impl InventoryPreClickEvent {
    pub fn new(player: *mut Player, in_open_inventory: bool, click: Click) -> Self {
        Self {
            player,
            in_open_inventory,
            click,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn click(&self) -> &Click {
        &self.click
    }

    pub fn slot(&self) -> i32 {
        self.click.slot()
    }

    pub fn clicked_item(&mut self) -> Option<&ItemStack> {
        let slot = self.slot();
        if slot == -999 {
            return Some(self.player().inventory_ref().cursor_item());
        }
        if self.in_open_inventory {
            return self
                .player()
                .opened_inventory()
                .and_then(|inventory| inventory.item_stack(slot as usize));
        }
        self.player().inventory_ref().item_stack(slot as usize)
    }

    pub fn set_click(&mut self, click: Click) {
        self.click = click;
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

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
