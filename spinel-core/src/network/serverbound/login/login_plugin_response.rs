use spinel_network::encoder::NetworkBuffer;
use spinel_network::{ConnectionState, DataType, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoginPluginResponsePacket {
    pub message_id: i32,
    pub data: Option<Vec<u8>>,
}

impl LoginPluginResponsePacket {
    pub const fn get_id_const() -> i32 {
        0x02
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Login
    }

    pub fn encode_to_buffer(&self) -> io::Result<NetworkBuffer> {
        let mut buffer = NetworkBuffer::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }
}

impl DataType for LoginPluginResponsePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.message_id).encode(writer)?;
        self.data.is_some().encode(writer)?;
        if let Some(data) = &self.data {
            writer.write_all(data)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let message_id = VarIntWrapper::decode(reader)?.0;
        let has_data = bool::decode(reader)?;
        let data = if has_data {
            let mut data = Vec::new();
            reader.read_to_end(&mut data)?;
            Some(data)
        } else {
            None
        };
        Ok(Self { message_id, data })
    }
}

impl PacketStruct for LoginPluginResponsePacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}
spinel_network::register_packet_codec!(
    LoginPluginResponsePacket,
    spinel_network::Recipient::Server
);
