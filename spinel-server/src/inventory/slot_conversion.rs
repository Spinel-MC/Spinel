pub const WINDOW_0_OFFSET: i32 = 9;
pub const CRAFT_RESULT: i32 = 36;
pub const CRAFT_SLOT_1: i32 = 37;
pub const CRAFT_SLOT_2: i32 = 38;
pub const CRAFT_SLOT_3: i32 = 39;
pub const CRAFT_SLOT_4: i32 = 40;
pub const HELMET_SLOT: i32 = 41;
pub const CHESTPLATE_SLOT: i32 = 42;
pub const LEGGINGS_SLOT: i32 = 43;
pub const BOOTS_SLOT: i32 = 44;
pub const OFFHAND_SLOT: i32 = 45;

pub fn is_hotbar_or_offhand_slot(slot: i32) -> bool {
    (0..9).contains(&slot) || slot == OFFHAND_SLOT
}

pub fn convert_window_0_slot_to_minestom_slot(slot: i32) -> i32 {
    match slot {
        0 => CRAFT_RESULT,
        1 => CRAFT_SLOT_1,
        2 => CRAFT_SLOT_2,
        3 => CRAFT_SLOT_3,
        4 => CRAFT_SLOT_4,
        5 => HELMET_SLOT,
        6 => CHESTPLATE_SLOT,
        7 => LEGGINGS_SLOT,
        8 => BOOTS_SLOT,
        _ => convert_window_slot_to_minestom_slot(slot, WINDOW_0_OFFSET),
    }
}

pub fn convert_window_slot_to_minestom_slot(slot: i32, offset: i32) -> i32 {
    let slot = slot - offset;
    if (27..36).contains(&slot) {
        slot % 9
    } else {
        slot + 9
    }
}

pub fn is_player_inventory_slot(slot: i32) -> bool {
    !(CRAFT_RESULT..=CRAFT_SLOT_4).contains(&slot)
}

pub fn convert_minestom_slot_to_player_inventory_slot(slot: i32) -> i32 {
    match slot {
        HELMET_SLOT..=BOOTS_SLOT => (3 - (slot - HELMET_SLOT)) + 36,
        OFFHAND_SLOT => 40,
        _ => slot,
    }
}

pub fn convert_minestom_slot_to_window_slot(slot: i32) -> i32 {
    match slot {
        0..=8 => slot + 36,
        9..=35 => slot,
        CRAFT_RESULT..=CRAFT_SLOT_4 => slot - 36,
        HELMET_SLOT..=BOOTS_SLOT => slot - 36,
        OFFHAND_SLOT => 45,
        _ => slot,
    }
}

pub fn convert_player_inventory_slot_to_minestom_slot(slot: i32) -> i32 {
    match slot {
        0..=35 => slot,
        36 => BOOTS_SLOT,
        37 => LEGGINGS_SLOT,
        38 => CHESTPLATE_SLOT,
        39 => HELMET_SLOT,
        40 => OFFHAND_SLOT,
        _ => -1,
    }
}
