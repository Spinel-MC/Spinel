use crate::showcase::{
    EntityShowcase, InventoryShowcase, PlayerShowcase, ShowcaseSignCommand, ShowcaseSigns,
    WorldShowcase,
};
use spinel::{
    macros::event_listener,
    registry::{ItemStack, Material},
    server::{
        MinecraftServer,
        entity::PlayerHand,
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

    if pathfind_showcase_zombie(event, server) {
        event.set_blocking_item_use(true);
        event.set_cancelled(true);
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
            let Some((world_id, position, player_uuid)) = sign_player_context(event) else {
                return;
            };
            let Ok(pathfinding_stick) = EntityShowcase::spawn(server, world_id, position) else {
                return;
            };
            let Some(world) = server.world_manager.world_mut(world_id) else {
                return;
            };
            let Some(player) = world.player_by_uuid_mut(player_uuid) else {
                return;
            };
            player.set_item_in_hand(PlayerHand::Main, pathfinding_stick);
        }
        ShowcaseSignCommand::World => {
            let Some((world_id, position, _)) = sign_player_context(event) else {
                return;
            };
            let _ = WorldShowcase::apply(server, world_id, position);
        }
    }
}

fn pathfind_showcase_zombie(
    event: &mut PlayerBlockInteractEvent,
    server: &mut MinecraftServer,
) -> bool {
    let hand = event.hand();
    let block_position = event.block_position();
    let player = event.player();
    let pathfinding_stick = player.item_in_hand(hand);
    let Some(world_id) = player.world().map(|world| world.uuid()) else {
        return false;
    };
    let Some(world) = server.world_manager.world_mut(world_id) else {
        return false;
    };
    EntityShowcase::pathfind(world, &pathfinding_stick, block_position)
}

fn sign_player_context(
    event: &mut PlayerBlockInteractEvent,
) -> Option<(
    spinel::uuid::Uuid,
    spinel::server::entity::EntityPosition,
    spinel::uuid::Uuid,
)> {
    let player = event.player();
    Some((player.current_world()?, player.position(), player.uuid()))
}
