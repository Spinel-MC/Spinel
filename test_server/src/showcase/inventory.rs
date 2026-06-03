use spinel::{
    registry::{ItemStack, Material},
    server::{
        entity::{EquipmentSlot, Player, PlayerHand},
        inventory::{Inventory, InventoryType},
    },
    utils::component::Component,
};
use std::io;

pub struct InventoryShowcase;

impl InventoryShowcase {
    pub fn apply(player: &mut Player) -> io::Result<()> {
        player.set_item_in_hand(PlayerHand::Main, Self::tool_stack());
        player.set_item_in_hand(PlayerHand::Off, Self::offhand_stack());
        player.set_equipment(EquipmentSlot::Helmet, Self::helmet_stack());
        player.open_inventory(Self::inventory());
        player.send_system_message(Component::text("Inventory showcase opened.").build())
    }

    fn inventory() -> Inventory {
        let mut inventory = Inventory::new(
            InventoryType::Chest(3),
            Component::text("ItemStack Showcase").build(),
        );
        inventory.set_item_stack(9, Self::resource_stack());
        inventory.set_item_stack(10, Self::food_stack());
        inventory.set_item_stack(11, Self::currency_stack());
        inventory.set_item_stack(12, Self::thrown_stack());
        inventory.set_item_stack(13, Self::book_stack());
        inventory.set_item_stack(14, Self::utility_stack());
        inventory.set_item_stack(15, Self::helmet_stack());
        inventory.set_item_stack(16, Self::tool_stack());
        inventory.set_item_stack(17, Self::offhand_stack());
        inventory
    }

    fn resource_stack() -> ItemStack {
        ItemStack::of(Material::DIAMOND).with_amount(12)
    }

    fn food_stack() -> ItemStack {
        ItemStack::of(Material::GOLDEN_APPLE).with_amount(3)
    }

    fn currency_stack() -> ItemStack {
        ItemStack::of(Material::EMERALD).with_amount(24)
    }

    fn thrown_stack() -> ItemStack {
        ItemStack::of(Material::ENDER_PEARL).with_amount(4)
    }

    fn book_stack() -> ItemStack {
        ItemStack::of(Material::WRITTEN_BOOK)
    }

    fn utility_stack() -> ItemStack {
        ItemStack::of(Material::COMPASS)
    }

    fn helmet_stack() -> ItemStack {
        ItemStack::of(Material::DIAMOND_HELMET)
    }

    fn tool_stack() -> ItemStack {
        ItemStack::of(Material::NETHERITE_AXE)
    }

    fn offhand_stack() -> ItemStack {
        ItemStack::of(Material::SHIELD)
    }
}
