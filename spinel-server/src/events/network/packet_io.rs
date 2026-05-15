use spinel_macros::event_dispatcher;
use spinel_network::ConnectionState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketFlowDirection {
    Clientbound,
    Serverbound,
}

impl PacketFlowDirection {
    pub const fn as_label(self) -> &'static str {
        match self {
            PacketFlowDirection::Clientbound => "Clientbound",
            PacketFlowDirection::Serverbound => "Serverbound",
        }
    }
}

#[event_dispatcher(with_client: true)]
pub struct PacketIoEvent {
    pub direction: PacketFlowDirection,
    pub state: ConnectionState,
    pub id: i32,
    pub packet_name: String,
    pub payload_size: usize,
}

impl PacketIoEvent {
    pub fn new(
        direction: PacketFlowDirection,
        state: ConnectionState,
        id: i32,
        packet_name: String,
        payload_size: usize,
    ) -> Self {
        Self {
            direction,
            state,
            id,
            packet_name,
            payload_size,
            connection_ptr: None,
        }
    }
}
