#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Click {
    Left(i32),
    Right(i32),
    Middle(i32),
    LeftShift(i32),
    RightShift(i32),
    Double(i32),
    LeftDrag(Vec<i32>),
    RightDrag(Vec<i32>),
    MiddleDrag(Vec<i32>),
    LeftDropCursor,
    RightDropCursor,
    MiddleDropCursor,
    DropSlot { slot: i32, all: bool },
    HotbarSwap { hotbar_slot: i32, slot: i32 },
    OffhandSwap(i32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClickWindow {
    in_open_inventory: bool,
    click: Click,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClickType {
    LeftClick,
    RightClick,
    MiddleClick,
    ShiftClick,
    DoubleClick,
    LeftDragging,
    RightDragging,
    MiddleDragging,
    Drop,
    ChangeHeld,
}

impl Click {
    pub fn to_window(self, open_inventory_size: Option<i32>) -> ClickWindow {
        match self {
            Self::Left(slot) => single_slot_window_click(slot, open_inventory_size, Self::Left),
            Self::Right(slot) => single_slot_window_click(slot, open_inventory_size, Self::Right),
            Self::Middle(slot) => single_slot_window_click(slot, open_inventory_size, Self::Middle),
            Self::LeftShift(slot) => {
                single_slot_window_click(slot, open_inventory_size, Self::LeftShift)
            }
            Self::RightShift(slot) => {
                single_slot_window_click(slot, open_inventory_size, Self::RightShift)
            }
            Self::Double(slot) => single_slot_window_click(slot, open_inventory_size, Self::Double),
            Self::OffhandSwap(slot) => {
                single_slot_window_click(slot, open_inventory_size, Self::OffhandSwap)
            }
            Self::DropSlot { slot, all } => {
                single_slot_window_click(slot, open_inventory_size, |slot| Self::DropSlot {
                    slot,
                    all,
                })
            }
            Self::HotbarSwap { hotbar_slot, slot } => {
                single_slot_window_click(slot, open_inventory_size, |slot| Self::HotbarSwap {
                    hotbar_slot,
                    slot,
                })
            }
            Self::LeftDrag(slots) => {
                multiple_slot_window_click(slots, open_inventory_size, Self::LeftDrag)
            }
            Self::RightDrag(slots) => {
                multiple_slot_window_click(slots, open_inventory_size, Self::RightDrag)
            }
            Self::MiddleDrag(slots) => {
                multiple_slot_window_click(slots, open_inventory_size, Self::MiddleDrag)
            }
            click @ (Self::LeftDropCursor | Self::RightDropCursor | Self::MiddleDropCursor) => {
                ClickWindow::new(false, click)
            }
        }
    }

    pub fn slot(&self) -> i32 {
        match self {
            Self::Left(slot)
            | Self::Right(slot)
            | Self::Middle(slot)
            | Self::LeftShift(slot)
            | Self::RightShift(slot)
            | Self::Double(slot)
            | Self::OffhandSwap(slot) => *slot,
            Self::DropSlot { slot, .. } | Self::HotbarSwap { slot, .. } => *slot,
            _ => -999,
        }
    }
}

impl ClickWindow {
    pub fn new(in_open_inventory: bool, click: Click) -> Self {
        Self {
            in_open_inventory,
            click,
        }
    }

    pub fn from_window(self, open_inventory_size: Option<i32>) -> Click {
        let in_open_inventory = self.in_open_inventory;
        match self.click {
            Click::Left(slot) => {
                restore_single_slot(in_open_inventory, slot, open_inventory_size, Click::Left)
            }
            Click::Right(slot) => {
                restore_single_slot(in_open_inventory, slot, open_inventory_size, Click::Right)
            }
            Click::Middle(slot) => {
                restore_single_slot(in_open_inventory, slot, open_inventory_size, Click::Middle)
            }
            Click::LeftShift(slot) => restore_single_slot(
                in_open_inventory,
                slot,
                open_inventory_size,
                Click::LeftShift,
            ),
            Click::RightShift(slot) => restore_single_slot(
                in_open_inventory,
                slot,
                open_inventory_size,
                Click::RightShift,
            ),
            Click::Double(slot) => {
                restore_single_slot(in_open_inventory, slot, open_inventory_size, Click::Double)
            }
            Click::OffhandSwap(slot) => restore_single_slot(
                in_open_inventory,
                slot,
                open_inventory_size,
                Click::OffhandSwap,
            ),
            Click::DropSlot { slot, all } => {
                restore_single_slot(in_open_inventory, slot, open_inventory_size, |slot| {
                    Click::DropSlot { slot, all }
                })
            }
            Click::HotbarSwap { hotbar_slot, slot } => {
                restore_single_slot(in_open_inventory, slot, open_inventory_size, |slot| {
                    Click::HotbarSwap { hotbar_slot, slot }
                })
            }
            Click::LeftDrag(slots) => restore_multiple_slots(
                in_open_inventory,
                slots,
                open_inventory_size,
                Click::LeftDrag,
            ),
            Click::RightDrag(slots) => restore_multiple_slots(
                in_open_inventory,
                slots,
                open_inventory_size,
                Click::RightDrag,
            ),
            Click::MiddleDrag(slots) => restore_multiple_slots(
                in_open_inventory,
                slots,
                open_inventory_size,
                Click::MiddleDrag,
            ),
            click @ (Click::LeftDropCursor | Click::RightDropCursor | Click::MiddleDropCursor) => {
                click
            }
        }
    }

    pub fn in_open_inventory(&self) -> bool {
        self.in_open_inventory
    }

    pub fn click(&self) -> &Click {
        &self.click
    }

    pub fn set_click(&mut self, click: Click) {
        self.click = click;
    }
}

fn single_slot_window_click(
    slot: i32,
    open_inventory_size: Option<i32>,
    constructor: impl FnOnce(i32) -> Click,
) -> ClickWindow {
    match open_inventory_size {
        None => ClickWindow::new(false, constructor(slot)),
        Some(size) if slot < size => ClickWindow::new(true, constructor(slot)),
        Some(size) => ClickWindow::new(false, constructor(slot - size)),
    }
}

fn multiple_slot_window_click(
    slots: Vec<i32>,
    open_inventory_size: Option<i32>,
    constructor: impl FnOnce(Vec<i32>) -> Click,
) -> ClickWindow {
    match open_inventory_size {
        None => ClickWindow::new(false, constructor(slots)),
        Some(size) if slots.iter().any(|slot| *slot < size) => {
            ClickWindow::new(true, constructor(slots))
        }
        Some(size) => ClickWindow::new(
            false,
            constructor(slots.into_iter().map(|slot| slot - size).collect()),
        ),
    }
}

fn restore_single_slot(
    in_open_inventory: bool,
    slot: i32,
    open_inventory_size: Option<i32>,
    constructor: impl FnOnce(i32) -> Click,
) -> Click {
    match open_inventory_size {
        None => constructor(slot),
        Some(_) if in_open_inventory => constructor(slot),
        Some(size) => constructor(slot + size),
    }
}

fn restore_multiple_slots(
    in_open_inventory: bool,
    slots: Vec<i32>,
    open_inventory_size: Option<i32>,
    constructor: impl FnOnce(Vec<i32>) -> Click,
) -> Click {
    match open_inventory_size {
        None => constructor(slots),
        Some(_) if in_open_inventory => constructor(slots),
        Some(size) => constructor(slots.into_iter().map(|slot| slot + size).collect()),
    }
}
