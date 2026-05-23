use spinel_registry::ItemStack;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Add,
    Take,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionOption {
    All,
    AllOrNothing,
    DryRun,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TransactionResult {
    Remaining(ItemStack),
    Complete(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InventoryTransaction {
    remaining_item: ItemStack,
    changed_slots: BTreeMap<usize, ItemStack>,
}

impl TransactionType {
    pub fn process(
        &self,
        item_stacks: &[ItemStack],
        item_stack: ItemStack,
    ) -> InventoryTransaction {
        match self {
            Self::Add => add_item_stack(item_stacks, item_stack),
            Self::Take => take_item_stack(item_stacks, item_stack),
        }
    }
}

impl InventoryTransaction {
    pub fn apply(
        self,
        item_stacks: &mut [ItemStack],
        option: TransactionOption,
    ) -> TransactionResult {
        match option {
            TransactionOption::All => self.apply_all(item_stacks),
            TransactionOption::AllOrNothing => self.apply_all_or_nothing(item_stacks),
            TransactionOption::DryRun => TransactionResult::Complete(self.remaining_item.is_air()),
        }
    }

    fn apply_all(self, item_stacks: &mut [ItemStack]) -> TransactionResult {
        apply_changed_slots(item_stacks, &self.changed_slots);
        TransactionResult::Remaining(self.remaining_item)
    }

    fn apply_all_or_nothing(self, item_stacks: &mut [ItemStack]) -> TransactionResult {
        if !self.remaining_item.is_air() {
            return TransactionResult::Complete(false);
        }
        apply_changed_slots(item_stacks, &self.changed_slots);
        TransactionResult::Complete(true)
    }
}

fn add_item_stack(
    item_stacks: &[ItemStack],
    mut remaining_item: ItemStack,
) -> InventoryTransaction {
    let mut changed_slots = BTreeMap::new();
    for (slot, item_stack) in item_stacks.iter().enumerate() {
        if remaining_item.is_air() {
            break;
        }
        if item_stack.is_air() || !item_stack.is_similar(&remaining_item) {
            continue;
        }
        let available_amount = item_stack.max_stack_size() - item_stack.amount();
        if available_amount <= 0 {
            continue;
        }
        let moved_amount = available_amount.min(remaining_item.amount());
        changed_slots.insert(
            slot,
            item_stack.with_amount(item_stack.amount() + moved_amount),
        );
        remaining_item = remaining_item.consume(moved_amount);
    }
    for (slot, item_stack) in item_stacks.iter().enumerate() {
        if remaining_item.is_air() {
            break;
        }
        if !item_stack.is_air() {
            continue;
        }
        let moved_amount = remaining_item.max_stack_size().min(remaining_item.amount());
        changed_slots.insert(slot, remaining_item.with_amount(moved_amount));
        remaining_item = remaining_item.consume(moved_amount);
    }
    InventoryTransaction {
        remaining_item,
        changed_slots,
    }
}

fn take_item_stack(
    item_stacks: &[ItemStack],
    mut remaining_item: ItemStack,
) -> InventoryTransaction {
    let mut changed_slots = BTreeMap::new();
    for (slot, item_stack) in item_stacks.iter().enumerate() {
        if remaining_item.is_air() {
            break;
        }
        if item_stack.is_air() || !item_stack.is_similar(&remaining_item) {
            continue;
        }
        let moved_amount = item_stack.amount().min(remaining_item.amount());
        changed_slots.insert(slot, item_stack.consume(moved_amount));
        remaining_item = remaining_item.consume(moved_amount);
    }
    InventoryTransaction {
        remaining_item,
        changed_slots,
    }
}

fn apply_changed_slots(item_stacks: &mut [ItemStack], changed_slots: &BTreeMap<usize, ItemStack>) {
    for (slot, item_stack) in changed_slots {
        item_stacks[*slot] = item_stack.clone();
    }
}
