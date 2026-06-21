use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct EditBookEvent {
    player: *mut Player,
    item_stack: ItemStack,
    pages: Vec<String>,
    title: Option<String>,
}

impl EditBookEvent {
    pub fn new(
        player: *mut Player,
        item_stack: ItemStack,
        pages: Vec<String>,
        title: Option<String>,
    ) -> Self {
        Self {
            player,
            item_stack,
            pages,
            title,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub const fn get_item_stack(&self) -> &ItemStack {
        &self.item_stack
    }

    pub fn pages(&self) -> &[String] {
        &self.pages
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}
