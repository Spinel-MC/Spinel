use spinel_macros::packet;
use spinel_network::types::{Position, TestBlockMode};

#[packet(id: "set_test_block", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct SetTestBlockPacket {
    pub position: Position,
    pub mode: TestBlockMode,
    pub message: String,
}
