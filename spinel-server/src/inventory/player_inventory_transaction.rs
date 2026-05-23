use super::player_inventory::INNER_INVENTORY_SIZE;
use crate::inventory::{PlayerInventory, TransactionOption, TransactionResult, TransactionType};
use spinel_registry::ItemStack;

impl PlayerInventory {
    pub fn process_item_stack(
        &mut self,
        item_stack: ItemStack,
        transaction_type: TransactionType,
        transaction_option: TransactionOption,
    ) -> TransactionResult {
        let transaction =
            transaction_type.process(&self.item_stacks[..INNER_INVENTORY_SIZE], item_stack);
        transaction.apply(
            &mut self.item_stacks[..INNER_INVENTORY_SIZE],
            transaction_option,
        )
    }

    pub fn add_item_stack(&mut self, item_stack: ItemStack) -> bool {
        matches!(
            self.process_item_stack(
                item_stack,
                TransactionType::Add,
                TransactionOption::AllOrNothing,
            ),
            TransactionResult::Complete(true)
        )
    }

    pub fn add_item_stack_with_option(
        &mut self,
        item_stack: ItemStack,
        transaction_option: TransactionOption,
    ) -> TransactionResult {
        self.process_item_stack(item_stack, TransactionType::Add, transaction_option)
    }

    pub fn add_item_stacks(&mut self, item_stacks: Vec<ItemStack>) -> Vec<bool> {
        item_stacks
            .into_iter()
            .map(|item_stack| self.add_item_stack(item_stack))
            .collect()
    }

    pub fn take_item_stack(
        &mut self,
        item_stack: ItemStack,
        transaction_option: TransactionOption,
    ) -> TransactionResult {
        self.process_item_stack(item_stack, TransactionType::Take, transaction_option)
    }

    pub fn take_item_stacks(
        &mut self,
        item_stacks: Vec<ItemStack>,
        transaction_option: TransactionOption,
    ) -> Vec<TransactionResult> {
        item_stacks
            .into_iter()
            .map(|item_stack| self.take_item_stack(item_stack, transaction_option))
            .collect()
    }

    pub fn copy_contents(&mut self, item_stacks: Vec<ItemStack>) -> bool {
        if item_stacks.len() != self.size() {
            return false;
        }
        self.item_stacks = item_stacks;
        true
    }
}
