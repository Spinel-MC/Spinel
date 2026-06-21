use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct PlayerChangeHeldSlotEvent {
    player: *mut Player,
    old_slot: i32,
    new_slot: i32,
    cancelled: bool,
}

impl PlayerChangeHeldSlotEvent {
    pub fn new(player: *mut Player, old_slot: i32, new_slot: i32) -> Self {
        Self {
            player,
            old_slot,
            new_slot,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn old_slot(&self) -> i32 {
        self.old_slot
    }

    pub fn new_slot(&self) -> i32 {
        self.new_slot
    }

    pub fn set_new_slot(&mut self, new_slot: i32) -> bool {
        if !(0..=8).contains(&new_slot) {
            return false;
        }
        self.new_slot = new_slot;
        true
    }

    pub fn item_in_old_slot(&mut self) -> ItemStack {
        let old_slot = self.old_slot;
        self.player()
            .get_inventory_ref()
            .get_item_stack(old_slot as usize)
            .cloned()
            .unwrap_or_else(ItemStack::air)
    }

    pub fn item_in_new_slot(&mut self) -> ItemStack {
        let new_slot = self.new_slot;
        self.player()
            .get_inventory_ref()
            .get_item_stack(new_slot as usize)
            .cloned()
            .unwrap_or_else(ItemStack::air)
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
