use crate::events::edit_book::EditBookEvent;
use crate::inventory::slot_conversion;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::edit_book::EditBookPacket;
use spinel_macros::fn_packet_listener;
use spinel_registry::ItemStack;

#[fn_packet_listener]
fn on_edit_book(client: &mut Client, packet: EditBookPacket, server: &mut MinecraftServer) -> bool {
    let internal_slot =
        slot_conversion::convert_player_inventory_slot_to_internal_slot(packet.slot.0);
    if !slot_conversion::is_hotbar_or_offhand_slot(internal_slot) {
        return true;
    }
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let item_stack = unsafe { &*player }
        .get_inventory_ref()
        .get_item_stack(internal_slot as usize)
        .cloned()
        .unwrap_or_else(ItemStack::air);
    let mut event = EditBookEvent::new(player, item_stack, packet.pages, packet.title);
    event.dispatch(server, client);
    true
}
