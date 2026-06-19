use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "select_trade", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct SelectTradePacket {
    pub selected_trade_index: VarInt,
}
