use crate::events::player_anvil_input::PlayerAnvilInputEvent;
use crate::inventory::InventoryType;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::rename_item::RenameItemPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_rename_item(
    client: &mut Client,
    packet: RenameItemPacket,
    server: &mut MinecraftServer,
) -> bool {
    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };
    let player = unsafe { &mut *player };
    let Some(inventory) = player.get_opened_inventory() else {
        return false;
    };
    if inventory.inventory_type() != InventoryType::Anvil {
        return false;
    }
    let mut event = PlayerAnvilInputEvent::new(player as *mut _, packet.item_name);
    event.dispatch(server, client);
    if event.is_cancelled() {
        return false;
    }
    player.set_anvil_rename_text(event.input().to_string());
    true
}
