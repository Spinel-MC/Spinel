use crate::inventory::InventoryType;
use crate::inventory::{TransactionOption, TransactionResult, TransactionType};
use spinel_nbt::{TagHandler, Taggable};
use spinel_registry::ItemStack;
use spinel_utils::component::text::TextComponent;
use std::collections::HashSet;
use std::sync::atomic::{AtomicI32, Ordering};
use uuid::Uuid;

static INVENTORY_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

#[derive(Clone)]
pub struct Inventory {
    id: i32,
    inventory_type: InventoryType,
    title: TextComponent,
    item_stacks: Vec<ItemStack>,
    tag_handler: TagHandler,
    viewers: HashSet<Uuid>,
}

impl Inventory {
    pub fn new(inventory_type: InventoryType, title: TextComponent) -> Self {
        Self {
            id: next_inventory_id(),
            inventory_type,
            title,
            item_stacks: vec![ItemStack::air(); inventory_type.size()],
            tag_handler: TagHandler::new_handler(),
            viewers: HashSet::new(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn inventory_type(&self) -> InventoryType {
        self.inventory_type
    }

    pub fn title(&self) -> TextComponent {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: TextComponent) {
        self.title = title;
    }

    pub fn set_item_stack(&mut self, slot: usize, item_stack: ItemStack) -> bool {
        let Some(stored_item_stack) = self.item_stacks.get_mut(slot) else {
            return false;
        };
        *stored_item_stack = item_stack;
        true
    }

    pub fn replace_item_stack(
        &mut self,
        slot: usize,
        operator: impl FnOnce(ItemStack) -> ItemStack,
    ) -> bool {
        let Some(stored_item_stack) = self.item_stacks.get(slot).cloned() else {
            return false;
        };
        self.set_item_stack(slot, operator(stored_item_stack))
    }

    pub fn item_stack(&self, slot: usize) -> Option<&ItemStack> {
        self.item_stacks.get(slot)
    }

    pub fn item_stacks(&self) -> &[ItemStack] {
        &self.item_stacks
    }

    pub fn size(&self) -> usize {
        self.item_stacks.len()
    }

    pub fn inner_size(&self) -> usize {
        self.size()
    }

    pub fn window_id(&self) -> i32 {
        self.id
    }

    pub fn viewers(&self) -> &HashSet<Uuid> {
        &self.viewers
    }

    pub fn add_viewer(&mut self, player: Uuid) -> bool {
        self.viewers.insert(player)
    }

    pub fn remove_viewer(&mut self, player: Uuid) -> bool {
        self.viewers.remove(&player)
    }

    pub fn process_item_stack(
        &mut self,
        item_stack: ItemStack,
        transaction_type: TransactionType,
        transaction_option: TransactionOption,
    ) -> TransactionResult {
        transaction_type
            .process(self.item_stacks(), item_stack)
            .apply(&mut self.item_stacks, transaction_option)
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

    pub fn send_slot_refresh(&self, _slot: usize, _item_stack: &ItemStack) -> bool {
        true
    }

    pub fn update(&self) -> bool {
        true
    }

    pub fn clear(&mut self) {
        self.item_stacks.fill(ItemStack::air());
    }
}

impl Taggable for Inventory {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}

fn next_inventory_id() -> i32 {
    INVENTORY_ID_COUNTER
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |id| {
            Some(if id + 1 >= 128 { 1 } else { id + 1 })
        })
        .map(|id| if id + 1 >= 128 { 1 } else { id + 1 })
        .unwrap_or(1)
}
