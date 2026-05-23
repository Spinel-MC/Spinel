use spinel_registry::ItemStack;

#[derive(Clone, Debug, PartialEq)]
pub struct InventoryClickResult {
    clicked: ItemStack,
    cursor: ItemStack,
    cancelled: bool,
}

impl InventoryClickResult {
    pub fn new(clicked: ItemStack, cursor: ItemStack) -> Self {
        Self {
            clicked,
            cursor,
            cancelled: false,
        }
    }

    pub fn cancelled(mut self) -> Self {
        self.cancelled = true;
        self
    }

    pub fn clicked(&self) -> &ItemStack {
        &self.clicked
    }

    pub fn cursor(&self) -> &ItemStack {
        &self.cursor
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_clicked(&mut self, clicked: ItemStack) {
        self.clicked = clicked;
    }

    pub fn set_cursor(&mut self, cursor: ItemStack) {
        self.cursor = cursor;
    }
}
