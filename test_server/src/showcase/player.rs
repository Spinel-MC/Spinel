use spinel::{
    registry::{ItemStack, Material},
    server::{
        entity::{EquipmentSlot, Player, PlayerHand},
        inventory::{Inventory, InventoryType},
        world::{BossBar, WorldBossBarColor, WorldBossBarOverlay},
    },
    utils::component::Component,
};
use std::io;

pub struct PlayerShowcase;

impl PlayerShowcase {
    pub fn apply(player: &mut Player) -> io::Result<()> {
        player.set_permission_level(4)?;
        player.set_allow_flying(true)?;
        player.set_flying_speed(0.1)?;
        player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::DIAMOND_PICKAXE));
        player.set_equipment(
            EquipmentSlot::Helmet,
            ItemStack::of(Material::DIAMOND_HELMET),
        );
        player.set_equipment(
            EquipmentSlot::Chestplate,
            ItemStack::of(Material::DIAMOND_CHESTPLATE),
        );
        player.set_equipment(
            EquipmentSlot::Leggings,
            ItemStack::of(Material::DIAMOND_LEGGINGS),
        );
        player.set_equipment(EquipmentSlot::Boots, ItemStack::of(Material::DIAMOND_BOOTS));
        player.send_system_message(Component::text("Player showcase applied.").build())?;
        player.send_action_bar(
            Component::text("Inventory, permissions, flight, equipment").build(),
        )?;
        player.send_title(Component::text("Spinel Player API").build())?;
        player.send_subtitle(Component::text("Current implemented surface").build())?;
        player.show_boss_bar(&BossBar::new(
            Component::text("Player API").build(),
            1.0,
            WorldBossBarColor::Purple,
            WorldBossBarOverlay::Progress,
            0,
        ))?;
        player.open_inventory(Self::inventory());
        Ok(())
    }

    fn inventory() -> Inventory {
        let mut inventory = Inventory::new(
            InventoryType::Chest(3),
            Component::text("Spinel Showcase").build(),
        );
        inventory.set_item_stack(10, ItemStack::of(Material::DIAMOND));
        inventory.set_item_stack(12, ItemStack::of(Material::GOLDEN_APPLE).with_amount(3));
        inventory.set_item_stack(14, ItemStack::of(Material::NETHERITE_PICKAXE));
        inventory
    }
}
