use spinel::{
    macros::event_listener,
    registry::{ItemStack, Material},
    server::{
        MinecraftServer,
        events::player_block_interact::PlayerBlockInteractEvent,
        inventory::{Inventory, InventoryType},
        world::Block,
    },
    utils::component::Component,
};

#[event_listener()]
fn on_player_block_interact(event: &mut PlayerBlockInteractEvent, _server: &mut MinecraftServer) {
    if event.block() != Block::DARK_PRISMARINE {
        return;
    }

    let mut inventory = Inventory::new(
        InventoryType::Chest(3),
        Component::text("Test Inventory").build(),
    );
    inventory.set_item_stack(10, ItemStack::of(Material::DIAMOND));
    inventory.set_item_stack(12, ItemStack::of(Material::GOLDEN_APPLE).with_amount(3));
    inventory.set_item_stack(
        1,
        ItemStack::of(Material::NETHERITE_PICKAXE).with_amount(5),
    );

    event.set_blocking_item_use(true);
    event.player().open_inventory(inventory);
}
