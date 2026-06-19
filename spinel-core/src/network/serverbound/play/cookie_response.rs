use spinel_network::data_type::DataType;
use spinel_network::types::{Identifier, var_int::VarIntWrapper};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CookieResponsePacket {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>,
}

impl CookieResponsePacket {
    const MAX_PAYLOAD_LENGTH: usize = 5120;

    pub const fn get_id_const() -> i32 {
        0x14
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for CookieResponsePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.key.encode(writer)?;
        match &self.payload {
            Some(payload) => {
                if payload.len() > Self::MAX_PAYLOAD_LENGTH {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "cookie payload exceeds maximum length",
                    ));
                }

                true.encode(writer)?;
                VarIntWrapper(payload.len() as i32).encode(writer)?;
                writer.write_all(payload)
            }
            None => false.encode(writer),
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let key = Identifier::decode(reader)?;
        let payload = if bool::decode(reader)? {
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
            Some(payload)
        } else {
            None
        };

        Ok(Self { key, payload })
    }
}

impl PacketStruct for CookieResponsePacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(CookieResponsePacket, spinel_network::Recipient::Server);
