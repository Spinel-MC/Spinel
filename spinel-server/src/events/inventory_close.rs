use crate::entity::Player;
use crate::inventory::Inventory;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct InventoryCloseEvent {
    player: *mut Player,
    inventory: Inventory,
    from_client: bool,
}

impl InventoryCloseEvent {
    pub fn new(player: *mut Player, inventory: Inventory, from_client: bool) -> Self {
        Self {
            player,
            inventory,
            from_client,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn from_client(&self) -> bool {
        self.from_client
    }
}
