use crate::inventory::click::InventoryClickResult;
use spinel_registry::ItemStack;

pub struct InventoryClickProcessor;

impl InventoryClickProcessor {
    pub fn left_click(clicked: ItemStack, cursor: ItemStack) -> InventoryClickResult {
        if !cursor.is_similar(&clicked) {
            return InventoryClickResult::new(cursor, clicked);
        }
        let total_amount = cursor.amount() + clicked.amount();
        let max_size = cursor.max_stack_size();
        if total_amount > clicked.max_stack_size() {
            return InventoryClickResult::new(
                clicked.with_amount(max_size),
                cursor.with_amount(total_amount - max_size),
            );
        }
        InventoryClickResult::new(clicked.with_amount(total_amount), ItemStack::air())
    }

    pub fn right_click(clicked: ItemStack, cursor: ItemStack) -> InventoryClickResult {
        if clicked.is_similar(&cursor) {
            return stack_one(clicked, cursor);
        }
        if cursor.is_air() {
            return take_half(clicked);
        }
        if clicked.is_air() {
            return place_one(cursor);
        }
        InventoryClickResult::new(cursor, clicked)
    }

    pub fn change_held(clicked: ItemStack, held_item: ItemStack) -> InventoryClickResult {
        InventoryClickResult::new(held_item, clicked)
    }
}

fn stack_one(clicked: ItemStack, cursor: ItemStack) -> InventoryClickResult {
    let amount = clicked.amount() + 1;
    if amount > clicked.max_stack_size() {
        return InventoryClickResult::new(clicked, cursor);
    }
    InventoryClickResult::new(clicked.with_amount(amount), cursor.consume(1))
}

fn take_half(clicked: ItemStack) -> InventoryClickResult {
    let amount = (clicked.amount() + 1) / 2;
    InventoryClickResult::new(
        clicked.with_amount(clicked.amount() / 2),
        clicked.with_amount(amount),
    )
}

fn place_one(cursor: ItemStack) -> InventoryClickResult {
    InventoryClickResult::new(cursor.with_amount(1), cursor.consume(1))
}
