use spinel_macros::packet;
use spinel_network::VarInt;
use spinel_network::types::Slot;

#[packet(id: "set_player_inventory", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetPlayerInventoryPacket {
    pub slot: VarInt,
    pub item: Slot,
}
