use spinel_macros::event_dispatcher;
use spinel_network::ConnectionState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboundPacketErrorStage {
    FrameRead,
    PacketIdDecode,
    PacketDecode,
}

#[event_dispatcher(with_client: true)]
pub struct InboundPacketErrorEvent {
    pub stage: InboundPacketErrorStage,
    pub state: ConnectionState,
    pub packet_id: Option<i32>,
    pub packet_name: Option<String>,
    pub message: String,
}

impl InboundPacketErrorEvent {
    pub fn new(
        stage: InboundPacketErrorStage,
        state: ConnectionState,
        packet_id: Option<i32>,
        packet_name: Option<String>,
        message: String,
    ) -> Self {
        Self {
            stage,
            state,
            packet_id,
            packet_name,
            message,
            connection_ptr: None,
        }
    }
}
