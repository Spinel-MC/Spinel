use crate::entity::EquipmentSlot;
use crate::inventory::slot_conversion::{
    OFFHAND_SLOT, convert_minestom_slot_to_player_inventory_slot,
    convert_minestom_slot_to_window_slot, is_player_inventory_slot,
};
use spinel_registry::ItemStack;
use std::collections::HashSet;
use uuid::Uuid;

pub const INVENTORY_SIZE: usize = 46;
pub const INNER_INVENTORY_SIZE: usize = 36;

#[derive(Clone)]
pub struct PlayerInventory {
    pub(super) item_stacks: Vec<ItemStack>,
    cursor_item: ItemStack,
    viewers: HashSet<Uuid>,
}

impl PlayerInventory {
    pub fn new() -> Self {
        Self {
            item_stacks: vec![ItemStack::air(); INVENTORY_SIZE],
            cursor_item: ItemStack::air(),
            viewers: HashSet::new(),
        }
    }

    pub fn item_stack(&self, slot: usize) -> Option<&ItemStack> {
        self.item_stacks.get(slot)
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
        let Some(item_stack) = self.item_stacks.get(slot).cloned() else {
            return false;
        };
        self.set_item_stack(slot, operator(item_stack))
    }

    pub fn item_stacks(&self) -> &[ItemStack] {
        &self.item_stacks
    }

    pub fn size(&self) -> usize {
        INVENTORY_SIZE
    }

    pub fn inner_size(&self) -> usize {
        INNER_INVENTORY_SIZE
    }

    pub fn window_id(&self) -> i32 {
        0
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

    pub fn cursor_item(&self) -> &ItemStack {
        &self.cursor_item
    }

    pub fn set_cursor_item(&mut self, cursor_item: ItemStack) {
        self.cursor_item = cursor_item;
    }

    pub fn set_cursor_item_with_packet(&mut self, cursor_item: ItemStack, _send_packet: bool) {
        self.set_cursor_item(cursor_item);
    }

    pub fn equipment(&self, equipment_slot: EquipmentSlot, held_slot: i32) -> ItemStack {
        let slot = self.slot_for_equipment(equipment_slot, held_slot);
        if slot < 0 {
            return ItemStack::air();
        }
        self.item_stack(slot as usize)
            .cloned()
            .unwrap_or_else(ItemStack::air)
    }

    pub fn set_equipment(
        &mut self,
        equipment_slot: EquipmentSlot,
        held_slot: i32,
        item_stack: ItemStack,
    ) -> bool {
        let slot = self.slot_for_equipment(equipment_slot, held_slot);
        if slot < 0 {
            return false;
        }
        self.set_item_stack(slot as usize, item_stack)
    }

    pub fn send_slot_refresh(
        &self,
        slot: i32,
        _item_stack: &ItemStack,
    ) -> Option<PlayerInventoryPacketSlot> {
        if !(0..=INVENTORY_SIZE as i32).contains(&slot) {
            return None;
        }
        Some(Self::packet_slot(slot))
    }

    pub fn update(&self) -> bool {
        true
    }

    pub fn clear(&mut self) {
        self.cursor_item = ItemStack::air();
        self.item_stacks.fill(ItemStack::air());
    }

    pub fn packet_slot(slot: i32) -> PlayerInventoryPacketSlot {
        if is_player_inventory_slot(slot) {
            return PlayerInventoryPacketSlot::PlayerInventory(
                convert_minestom_slot_to_player_inventory_slot(slot),
            );
        }
        PlayerInventoryPacketSlot::Window(convert_minestom_slot_to_window_slot(slot))
    }

    pub fn slot_for_equipment(&self, equipment_slot: EquipmentSlot, held_slot: i32) -> i32 {
        match equipment_slot {
            EquipmentSlot::MainHand => held_slot,
            EquipmentSlot::OffHand => OFFHAND_SLOT,
            _ => equipment_slot.armor_slot(),
        }
    }

    pub fn equipment_slot_for_slot(&self, slot: i32, held_slot: i32) -> Option<EquipmentSlot> {
        [
            EquipmentSlot::MainHand,
            EquipmentSlot::OffHand,
            EquipmentSlot::Boots,
            EquipmentSlot::Leggings,
            EquipmentSlot::Chestplate,
            EquipmentSlot::Helmet,
            EquipmentSlot::Body,
            EquipmentSlot::Saddle,
        ]
        .into_iter()
        .find(|equipment_slot| self.slot_for_equipment(*equipment_slot, held_slot) == slot)
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerInventoryPacketSlot {
    PlayerInventory(i32),
    Window(i32),
}
