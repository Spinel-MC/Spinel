use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct CreativeInventoryActionEvent {
    player: *mut Player,
    slot: i32,
    item_stack: ItemStack,
    cancelled: bool,
}

impl CreativeInventoryActionEvent {
    pub fn new(player: *mut Player, slot: i32, item_stack: ItemStack) -> Self {
        Self {
            player,
            slot,
            item_stack,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn slot(&self) -> i32 {
        self.slot
    }

    pub fn get_item_stack(&self) -> &ItemStack {
        &self.item_stack
    }

    pub fn set_item_stack(&mut self, item_stack: ItemStack) {
        self.item_stack = item_stack;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
