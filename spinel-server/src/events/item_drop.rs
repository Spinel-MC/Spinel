use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct ItemDropEvent {
    player: *mut Player,
    item: ItemStack,
    cancelled: bool,
}

impl ItemDropEvent {
    pub fn new(player: *mut Player, item: ItemStack) -> Self {
        Self {
            player,
            item,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn item(&self) -> &ItemStack {
        &self.item
    }

    pub fn set_item(&mut self, item: ItemStack) {
        self.item = item;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
