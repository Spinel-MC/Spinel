use spinel_macros::packet;

#[packet(id: "set_held_slot", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetHeldSlotPacket {
    pub slot: i8,
}
