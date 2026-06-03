use crate::showcase::{
    EntityShowcase, InventoryShowcase, PlayerShowcase, ShowcaseSignCommand, ShowcaseSigns,
    WorldShowcase,
};
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
fn on_player_block_interact(event: &mut PlayerBlockInteractEvent, server: &mut MinecraftServer) {
    if let Some(command) = ShowcaseSigns::command_at_position(event.block_position()) {
        event.set_blocking_item_use(true);
        event.set_cancelled(true);
        run_showcase_sign_command(event, server, command);
        return;
    }

    if event.block() != Block::DARK_PRISMARINE {
        return;
    }

    if event.player().is_sneaking() == true {
        return;
    }

    let mut inventory = Inventory::new(
        InventoryType::Chest(3),
        Component::text("Test Inventory").build(),
    );
    inventory.set_item_stack(10, ItemStack::of(Material::DIAMOND));
    inventory.set_item_stack(12, ItemStack::of(Material::GOLDEN_APPLE).with_amount(3));
    inventory.set_item_stack(1, ItemStack::of(Material::NETHERITE_PICKAXE).with_amount(5));

    event.set_blocking_item_use(true);
    event.player().open_inventory(inventory);
}

fn run_showcase_sign_command(
    event: &mut PlayerBlockInteractEvent,
    server: &mut MinecraftServer,
    command: ShowcaseSignCommand,
) {
    match command {
        ShowcaseSignCommand::Player => {
            let _ = PlayerShowcase::apply(event.player());
        }
        ShowcaseSignCommand::Inventory => {
            let _ = InventoryShowcase::apply(event.player());
        }
        ShowcaseSignCommand::Entity => {
            let Some((world_id, position, player_id)) = sign_player_context(event) else {
                return;
            };
            let _ = EntityShowcase::spawn(server, world_id, position, player_id);
        }
        ShowcaseSignCommand::World => {
            let Some((world_id, position, _)) = sign_player_context(event) else {
                return;
            };
            let _ = WorldShowcase::apply(server, world_id, position);
        }
    }
}

fn sign_player_context(
    event: &mut PlayerBlockInteractEvent,
) -> Option<(
    spinel::uuid::Uuid,
    spinel::server::entity::EntityPosition,
    spinel::server::entity::EntityId,
)> {
    let player = event.player();
    Some((
        player.world()?.uuid(),
        player.position(),
        player.entity_id(),
    ))
}
