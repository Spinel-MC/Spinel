use spinel_network::network_buffer::NetworkBuffer;
use spinel_network::{ConnectionState, DataType, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatPacket {
    pub message: String,
    pub timestamp: i64,
    pub salt: i64,
    pub signature: Option<ChatSignature>,
    pub ack_offset: i32,
    pub ack_list: [u8; 3],
    pub checksum: i8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatSignature(pub [u8; 256]);

impl ChatPacket {
    pub const fn get_id() -> i32 {
        0x08
    }

    pub const fn get_id_const() -> i32 {
        Self::get_id()
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn encode_to_buffer(&self) -> io::Result<NetworkBuffer> {
        let mut buffer = NetworkBuffer::new();
        buffer.encode(self)?;
        Ok(buffer)
    }
}

impl DataType for ChatPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.message.encode(writer)?;
        self.timestamp.encode(writer)?;
        self.salt.encode(writer)?;
        self.signature.encode(writer)?;
        VarIntWrapper(self.ack_offset).encode(writer)?;
        writer.write_all(&self.ack_list)?;
        self.checksum.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let message = String::decode(reader)?;
        let timestamp = i64::decode(reader)?;
        let salt = i64::decode(reader)?;
        let signature = Option::<ChatSignature>::decode(reader)?;
        let ack_offset = VarIntWrapper::decode(reader)?.0;
        let mut ack_list = [0; 3];
        reader.read_exact(&mut ack_list)?;
        let checksum = i8::decode(reader)?;
        Ok(Self {
            message,
            timestamp,
            salt,
            signature,
            ack_offset,
            ack_list,
            checksum,
        })
    }
}

impl DataType for ChatSignature {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut value = [0; 256];
        reader.read_exact(&mut value)?;
        Ok(Self(value))
    }
}

impl PacketStruct for ChatPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(ChatPacket, spinel_network::Recipient::Server);
