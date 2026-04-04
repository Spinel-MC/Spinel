pub mod decoder;
pub mod encoder;
pub mod network_buffer;
pub mod packet_names;
pub mod types;

pub mod data_type;
pub mod wrappers;

pub use data_type::DataType;
pub use types::var_int::VarIntWrapper;
pub use types::var_long::VarLongWrapper;
pub use types::{Array, Position, Slot, VarInt, VarLong};
pub use wrappers::{JsonTextComponent, NbtTextComponent};

pub use decoder::PacketDecoder;
pub use encoder as encoder_module;
pub use encoder::PacketEncoder;

pub use inventory;
pub use spinel_utils::Priority;
pub use tokio;

pub trait PacketSender {
    fn send_packet(&mut self, id: i32, payload: &[u8]);
}

pub trait PacketStruct: DataType {
    fn get_id() -> i32;
    fn get_state() -> ConnectionState;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

impl ConnectionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionState::Handshaking => "Handshaking",
            ConnectionState::Status => "Status",
            ConnectionState::Login => "Login",
            ConnectionState::Configuration => "Configuration",
            ConnectionState::Play => "Play",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Recipient {
    Client,
    Server,
}
