use spinel_network::data_type::DataType;
use spinel_network::{ConnectionState, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugSubscriptionUpdate {
    pub subscription: i32,
    pub encoded_value: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugEntityValuePacket {
    pub entity_id: i32,
    pub update: DebugSubscriptionUpdate,
}

impl DataType for DebugSubscriptionUpdate {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.subscription).encode(writer)?;
        self.encoded_value.is_some().encode(writer)?;
        if let Some(encoded_value) = &self.encoded_value {
            writer.write_all(encoded_value)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let subscription = VarIntWrapper::decode(reader)?.0;
        let encoded_value = if bool::decode(reader)? {
            let mut encoded_value = Vec::new();
            reader.read_to_end(&mut encoded_value)?;
            Some(encoded_value)
        } else {
            None
        };
        Ok(Self {
            subscription,
            encoded_value,
        })
    }
}

impl DataType for DebugEntityValuePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.entity_id).encode(writer)?;
        self.update.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            entity_id: VarIntWrapper::decode(reader)?.0,
            update: DebugSubscriptionUpdate::decode(reader)?,
        })
    }
}

impl PacketStruct for DebugEntityValuePacket {
    fn get_id() -> i32 {
        0x1C
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(DebugEntityValuePacket, spinel_network::Recipient::Client);
