use spinel_network::data_type::DataType;
use spinel_network::types::{Identifier, var_int::VarIntWrapper};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreCookiePacket {
    pub key: Identifier,
    pub payload: Vec<u8>,
}

impl StoreCookiePacket {
    const MAX_PAYLOAD_LENGTH: usize = 5120;

    pub const fn get_id_const() -> i32 {
        0x0a
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Configuration
    }
}

impl DataType for StoreCookiePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if self.payload.len() > Self::MAX_PAYLOAD_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "cookie payload exceeds maximum length",
            ));
        }

        self.key.encode(writer)?;
        VarIntWrapper(self.payload.len() as i32).encode(writer)?;
        writer.write_all(&self.payload)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let key = Identifier::decode(reader)?;
        let payload_length = VarIntWrapper::decode(reader)?.0;
        if payload_length < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "cookie payload length cannot be negative",
            ));
        }

        let payload_length = payload_length as usize;
        if payload_length > Self::MAX_PAYLOAD_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "cookie payload exceeds maximum length",
            ));
        }

        let mut payload = vec![0; payload_length];
        reader.read_exact(&mut payload)?;
        Ok(Self { key, payload })
    }
}

impl PacketStruct for StoreCookiePacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(StoreCookiePacket, spinel_network::Recipient::Client);
