use spinel_macros::packet;
use spinel_network::types::Slot;

#[packet(id: "set_creative_mode_slot", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct SetCreativeModeSlotPacket {
    pub slot: i16,
    pub clicked_item: Slot,
}
