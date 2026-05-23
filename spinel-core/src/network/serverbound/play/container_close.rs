use spinel_macros::packet;

#[packet(id: "container_close", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ContainerClosePacket {
    pub container_id: u8,
}
