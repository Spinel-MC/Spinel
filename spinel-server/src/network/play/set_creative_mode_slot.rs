use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::{
    events::creative_inventory_action::CreativeInventoryActionEvent, inventory::slot_conversion,
};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::serverbound::play::set_creative_mode_slot::SetCreativeModeSlotPacket;
use spinel_macros::packet_listener;
use spinel_registry::ItemStack;

#[packet_listener]
fn on_set_creative_mode_slot(
    client: &mut Client,
    packet: SetCreativeModeSlotPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let player = unsafe { &mut *player };
    if player.get_game_mode() != GameMode::Creative {
        return false;
    }

    let sent_item_stack = packet.clicked_item.to_item_stack();
    if packet.slot == -1 {
        let mut event = CreativeInventoryActionEvent::new(
            player as *mut crate::entity::Player,
            packet.slot as i32,
            sent_item_stack,
        );
        event.dispatch(server, client);
        if event.is_cancelled() {
            return false;
        }
        return player.drop_item(event.get_item_stack().clone(), server, client);
    }

    if packet.slot < 1 || packet.slot as i32 > slot_conversion::OFFHAND_SLOT {
        return false;
    }

    let slot = slot_conversion::convert_window_0_slot_to_minestom_slot(packet.slot as i32);
    let mut event = CreativeInventoryActionEvent::new(
        player as *mut crate::entity::Player,
        slot,
        sent_item_stack.clone(),
    );
    event.dispatch(server, client);

    let previous_item_stack = player
        .get_inventory_ref()
        .get_item_stack(slot as usize)
        .cloned()
        .unwrap_or_else(ItemStack::air);
    if event.is_cancelled() {
        return player.sync_player_inventory_slot(slot, client).is_ok();
    }

    let set_item_stack = event.get_item_stack().clone();
    let set_item_stack_matches_sent_item_stack = set_item_stack == sent_item_stack;
    if previous_item_stack == sent_item_stack && set_item_stack_matches_sent_item_stack {
        return true;
    }

    let inventory_slot_changed = previous_item_stack != set_item_stack;
    if inventory_slot_changed {
        if !player
            .get_inventory()
            .set_item_stack(slot as usize, set_item_stack.clone())
        {
            return false;
        }
        player.update_inventory_slot_attributes(slot, &previous_item_stack, &set_item_stack);
        if player.get_slot_is_held_main_hand(slot)
            && player.sync_main_hand_attributes(client).is_err()
        {
            return false;
        }
        if server
            .refresh_player_visible_equipment_in_world(client)
            .is_err()
        {
            return false;
        }
    }

    if inventory_slot_changed || !set_item_stack_matches_sent_item_stack {
        return player.sync_player_inventory_slot(slot, client).is_ok();
    }

    true
}
