use crate::entity::Player;
use crate::inventory::Inventory;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct InventoryOpenEvent {
    player: *mut Player,
    inventory: Inventory,
    cancelled: bool,
}

impl InventoryOpenEvent {
    pub fn new(player: *mut Player, inventory: Inventory) -> Self {
        Self {
            player,
            inventory,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn take_inventory(self) -> Inventory {
        self.inventory
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
