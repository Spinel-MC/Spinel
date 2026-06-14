use spinel_network::data_type::DataType;
use spinel_network::{ConnectionState, PacketStruct, VarIntWrapper};
use std::collections::BTreeSet;
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugSubscriptionRequestPacket {
    pub subscriptions: BTreeSet<i32>,
}

impl DebugSubscriptionRequestPacket {
    pub const fn get_id_const() -> i32 {
        0x0E
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for DebugSubscriptionRequestPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.subscriptions.len() as i32).encode(writer)?;
        self.subscriptions
            .iter()
            .try_for_each(|subscription| VarIntWrapper(*subscription).encode(writer))
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let subscription_count = VarIntWrapper::decode(reader)?.0.max(0) as usize;
        let subscriptions = (0..subscription_count)
            .map(|_| VarIntWrapper::decode(reader).map(|subscription| subscription.0))
            .collect::<io::Result<BTreeSet<_>>>()?;
        Ok(Self { subscriptions })
    }
}

impl PacketStruct for DebugSubscriptionRequestPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}
spinel_network::register_packet_codec!(
    DebugSubscriptionRequestPacket,
    spinel_network::Recipient::Server
);
