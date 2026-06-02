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

#[cfg(test)]
mod tests {
    use super::DebugSubscriptionRequestPacket;
    use spinel_network::data_type::DataType;
    use std::collections::BTreeSet;

    #[test]
    fn debug_subscription_request_uses_minestom_varint_set_shape() {
        let packet = DebugSubscriptionRequestPacket {
            subscriptions: BTreeSet::from([2, 5, 8]),
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet =
            DebugSubscriptionRequestPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(DebugSubscriptionRequestPacket::get_id_const(), 0x0E);
        assert_eq!(decoded_packet, packet);
        assert_eq!(payload, vec![3, 2, 5, 8]);
    }
}
