use spinel_network::data_type::DataType;
use spinel_network::{ConnectionState, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugSubscriptionEvent {
    pub subscription: i32,
    pub encoded_value: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugEventPacket {
    pub event: DebugSubscriptionEvent,
}

impl DataType for DebugSubscriptionEvent {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.subscription).encode(writer)?;
        writer.write_all(&self.encoded_value)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let subscription = VarIntWrapper::decode(reader)?.0;
        let mut encoded_value = Vec::new();
        reader.read_to_end(&mut encoded_value)?;
        Ok(Self {
            subscription,
            encoded_value,
        })
    }
}

impl DataType for DebugEventPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.event.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            event: DebugSubscriptionEvent::decode(reader)?,
        })
    }
}

impl PacketStruct for DebugEventPacket {
    fn get_id() -> i32 {
        0x1D
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(DebugEventPacket, spinel_network::Recipient::Client);
