use crate::inventory::click::Click;
use crate::inventory::player_inventory::{INNER_INVENTORY_SIZE, INVENTORY_SIZE};
use crate::inventory::slot_conversion::{
    convert_window_0_slot_to_minestom_slot, convert_window_slot_to_minestom_slot,
};
use spinel_core::network::serverbound::play::container_click::ContainerClickPacket;

#[derive(Clone, Default)]
pub struct ClickPreprocessor {
    left_drag: Vec<i32>,
    right_drag: Vec<i32>,
    middle_drag: Vec<i32>,
}

impl ClickPreprocessor {
    pub fn clear_cache(&mut self) {
        self.left_drag.clear();
        self.right_drag.clear();
        self.middle_drag.clear();
    }

    pub fn process_click(
        &mut self,
        packet: &ContainerClickPacket,
        container_size: Option<usize>,
    ) -> Option<Click> {
        let slot = converted_slot(packet.slot, container_size);
        let max_size = container_size.map_or(INVENTORY_SIZE, |size| size + INNER_INVENTORY_SIZE);
        let slot_is_valid = slot >= 0 && slot < max_size as i32;
        if slot_is_valid {
            return self.process(packet.click_type, slot, packet.button);
        }
        if slot == -999 {
            return self.process_invalid_slot(packet.click_type, packet.button);
        }
        None
    }

    pub fn is_creative_click(&self, click: &Click, has_cursor_item: bool) -> bool {
        matches!(click, Click::Middle(_)) && !has_cursor_item
            || matches!(click, Click::MiddleDrag(_))
    }

    fn process_invalid_slot(&mut self, click_type: i32, button: i8) -> Option<Click> {
        match (click_type, button) {
            (0 | 4, 0) => Some(Click::LeftDropCursor),
            (0 | 4, 1) => Some(Click::RightDropCursor),
            (3, 2) => Some(Click::MiddleDropCursor),
            (5, 0 | 4 | 8) => self.clear_drag(button),
            (5, 2) => Some(Click::LeftDrag(std::mem::take(&mut self.left_drag))),
            (5, 6) => Some(Click::RightDrag(std::mem::take(&mut self.right_drag))),
            (5, 10) => Some(Click::MiddleDrag(std::mem::take(&mut self.middle_drag))),
            _ => None,
        }
    }

    fn process(&mut self, click_type: i32, slot: i32, button: i8) -> Option<Click> {
        match click_type {
            0 => pickup_click(slot, button),
            1 => Some(if button == 0 {
                Click::LeftShift(slot)
            } else {
                Click::RightShift(slot)
            }),
            2 => swap_click(slot, button),
            3 => Some(Click::Middle(slot)),
            4 => Some(Click::DropSlot {
                slot,
                all: button == 1,
            }),
            5 => self.drag_click(slot, button),
            6 => Some(Click::Double(slot)),
            _ => None,
        }
    }

    fn clear_drag(&mut self, button: i8) -> Option<Click> {
        if button == 0 {
            self.left_drag.clear();
        }
        if button == 4 {
            self.right_drag.clear();
        }
        if button == 8 {
            self.middle_drag.clear();
        }
        None
    }

    fn drag_click(&mut self, slot: i32, button: i8) -> Option<Click> {
        if button == 1 {
            push_unique_drag_slot(&mut self.left_drag, slot);
        }
        if button == 5 {
            push_unique_drag_slot(&mut self.right_drag, slot);
        }
        if button == 9 {
            push_unique_drag_slot(&mut self.middle_drag, slot);
        }
        None
    }
}

fn push_unique_drag_slot(drag_slots: &mut Vec<i32>, slot: i32) {
    if drag_slots.contains(&slot) {
        return;
    }
    drag_slots.push(slot);
}

fn converted_slot(slot: i16, container_size: Option<usize>) -> i32 {
    if slot == -999 {
        return -999;
    }
    match container_size {
        None => convert_window_0_slot_to_minestom_slot(slot as i32),
        Some(size) if slot as usize >= size => {
            size as i32 + convert_window_slot_to_minestom_slot(slot as i32, size as i32)
        }
        Some(_) => slot as i32,
    }
}

fn pickup_click(slot: i32, button: i8) -> Option<Click> {
    match button {
        0 => Some(Click::Left(slot)),
        1 => Some(Click::Right(slot)),
        _ => None,
    }
}

fn swap_click(slot: i32, button: i8) -> Option<Click> {
    match button {
        0..=8 => Some(Click::HotbarSwap {
            hotbar_slot: button as i32,
            slot,
        }),
        40 => Some(Click::OffhandSwap(slot)),
        _ => None,
    }
}
