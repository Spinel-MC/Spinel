use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct PlayerSwapItemEvent {
    player: *mut Player,
    main_hand_item: ItemStack,
    off_hand_item: ItemStack,
    cancelled: bool,
}

impl PlayerSwapItemEvent {
    pub fn new(player: *mut Player, main_hand_item: ItemStack, off_hand_item: ItemStack) -> Self {
        Self {
            player,
            main_hand_item,
            off_hand_item,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn main_hand_item(&self) -> &ItemStack {
        &self.main_hand_item
    }

    pub fn set_main_hand_item(&mut self, main_hand_item: ItemStack) {
        self.main_hand_item = main_hand_item;
    }

    pub fn off_hand_item(&self) -> &ItemStack {
        &self.off_hand_item
    }

    pub fn set_off_hand_item(&mut self, off_hand_item: ItemStack) {
        self.off_hand_item = off_hand_item;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
