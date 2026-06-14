use spinel_macros::event_dispatcher;
use spinel_network::{ConnectionState, Recipient};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PacketErrorStage {
    FrameRead,
    PacketIdDecode,
    PacketDecode,
    PacketWrite,
}

#[event_dispatcher(with_server: true)]
pub struct PacketErrorEvent {
    pub recipient: Recipient,
    pub stage: PacketErrorStage,
    pub state: ConnectionState,
    pub packet_id: Option<i32>,
    pub packet_name: Option<String>,
    pub message: String,
}

impl PacketErrorEvent {
    pub fn new(
        recipient: Recipient,
        stage: PacketErrorStage,
        state: ConnectionState,
        packet_id: Option<i32>,
        packet_name: Option<String>,
        message: String,
    ) -> Self {
        Self {
            recipient,
            stage,
            state,
            packet_id,
            packet_name,
            message,
            connection_ptr: None,
        }
    }
}
