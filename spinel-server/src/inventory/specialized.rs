use crate::inventory::{Inventory, InventoryType};
use spinel_utils::component::text::TextComponent;

pub fn anvil(title: TextComponent) -> Inventory {
    Inventory::new(InventoryType::Anvil, title)
}

pub fn beacon(title: TextComponent) -> Inventory {
    Inventory::new(InventoryType::Beacon, title)
}

pub fn brewing_stand(title: TextComponent) -> Inventory {
    Inventory::new(InventoryType::BrewingStand, title)
}

pub fn enchantment_table(title: TextComponent) -> Inventory {
    Inventory::new(InventoryType::Enchantment, title)
}

pub fn furnace(title: TextComponent) -> Inventory {
    Inventory::new(InventoryType::Furnace, title)
}

pub fn villager(title: TextComponent) -> Inventory {
    Inventory::new(InventoryType::Merchant, title)
}
