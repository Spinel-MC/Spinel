use spinel_macros::packet;

#[packet(id: "set_carried_item", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct SetCarriedItemPacket {
    pub slot: i16,
}
