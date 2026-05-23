#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryType {
    Chest(u8),
    Window3x3,
    Crafter3x3,
    Anvil,
    Beacon,
    BlastFurnace,
    BrewingStand,
    Crafting,
    Enchantment,
    Furnace,
    Grindstone,
    Hopper,
    Lectern,
    Loom,
    Merchant,
    ShulkerBox,
    Smithing,
    Smoker,
    Cartography,
    StoneCutter,
}

impl InventoryType {
    pub const fn window_type(self) -> i32 {
        match self {
            Self::Chest(rows) => rows as i32 - 1,
            Self::Window3x3 => 6,
            Self::Crafter3x3 => 7,
            Self::Anvil => 8,
            Self::Beacon => 9,
            Self::BlastFurnace => 10,
            Self::BrewingStand => 11,
            Self::Crafting => 12,
            Self::Enchantment => 13,
            Self::Furnace => 14,
            Self::Grindstone => 15,
            Self::Hopper => 16,
            Self::Lectern => 17,
            Self::Loom => 18,
            Self::Merchant => 19,
            Self::ShulkerBox => 20,
            Self::Smithing => 21,
            Self::Smoker => 22,
            Self::Cartography => 23,
            Self::StoneCutter => 24,
        }
    }

    pub const fn size(self) -> usize {
        match self {
            Self::Chest(rows) => rows as usize * 9,
            Self::Window3x3 | Self::Crafter3x3 => 9,
            Self::ShulkerBox => 27,
            Self::Anvil | Self::BlastFurnace | Self::Furnace | Self::Merchant | Self::Smoker => 3,
            Self::Beacon | Self::Lectern => 1,
            Self::BrewingStand => 5,
            Self::Crafting => 10,
            Self::Enchantment | Self::StoneCutter => 2,
            Self::Grindstone => 3,
            Self::Hopper => 5,
            Self::Loom | Self::Smithing => 4,
            Self::Cartography => 3,
        }
    }
}
