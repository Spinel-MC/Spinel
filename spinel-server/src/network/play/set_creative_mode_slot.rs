use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::{
    events::creative_inventory_action::CreativeInventoryActionEvent, inventory::slot_conversion,
};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::serverbound::play::set_creative_mode_slot::SetCreativeModeSlotPacket;
use spinel_macros::packet_listener;

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
    if player.game_mode() != GameMode::Creative {
        let _ = player.sync_inventory(client);
        return false;
    }
    if packet.slot < 1 || packet.slot as i32 > slot_conversion::OFFHAND_SLOT {
        let _ = player.sync_inventory(client);
        return false;
    }
    let slot = slot_conversion::convert_window_0_slot_to_minestom_slot(packet.slot as i32);
    let item_stack = packet.clicked_item.to_item_stack();
    let mut event = CreativeInventoryActionEvent::new(
        player as *mut crate::entity::Player,
        slot,
        item_stack.clone(),
    );
    event.dispatch(server, client);
    if event.is_cancelled() {
        return false;
    }
    let previous_item_stack = player
        .inventory_ref()
        .item_stack(slot as usize)
        .cloned()
        .unwrap_or_else(spinel_registry::ItemStack::air);
    if !player.inventory().set_item_stack(slot as usize, item_stack) {
        return false;
    }
    let current_item_stack = player
        .inventory_ref()
        .item_stack(slot as usize)
        .cloned()
        .unwrap_or_else(spinel_registry::ItemStack::air);
    player.update_inventory_slot_attributes(slot, &previous_item_stack, &current_item_stack);
    if player.slot_is_held_main_hand(slot) && player.sync_main_hand_attributes(client).is_err() {
        return false;
    }
    server
        .refresh_player_visible_equipment_in_world(client)
        .is_ok()
}
