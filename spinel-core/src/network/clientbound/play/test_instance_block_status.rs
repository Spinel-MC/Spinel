use spinel_macros::packet;
use spinel_network::types::Vector3i;
use spinel_utils::component::text::TextComponent;

#[packet(
    id: "test_instance_block_status",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct TestInstanceBlockStatusPacket {
    pub status: TextComponent,
    pub size: Option<Vector3i>,
}
