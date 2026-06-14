use spinel_network::network_buffer::NetworkBuffer;
use spinel_network::{ConnectionState, DataType, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedCommandChatPacket {
    pub command: String,
    pub timestamp: i64,
    pub salt: i64,
    pub signatures: Vec<CommandArgumentSignature>,
    pub ack_offset: i32,
    pub ack_list: [u8; 3],
    pub checksum: i8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandArgumentSignature {
    pub argument_name: String,
    pub signature: SignedCommandArgumentSignature,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedCommandArgumentSignature(pub [u8; 256]);

impl SignedCommandChatPacket {
    pub const fn get_id() -> i32 {
        0x07
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

impl DataType for SignedCommandChatPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.command.encode(writer)?;
        self.timestamp.encode(writer)?;
        self.salt.encode(writer)?;
        self.signatures.encode(writer)?;
        VarIntWrapper(self.ack_offset).encode(writer)?;
        writer.write_all(&self.ack_list)?;
        self.checksum.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let command = String::decode(reader)?;
        let timestamp = i64::decode(reader)?;
        let salt = i64::decode(reader)?;
        let signatures = Vec::<CommandArgumentSignature>::decode(reader)?;
        let ack_offset = VarIntWrapper::decode(reader)?.0;
        let mut ack_list = [0; 3];
        reader.read_exact(&mut ack_list)?;
        let checksum = i8::decode(reader)?;
        Ok(Self {
            command,
            timestamp,
            salt,
            signatures,
            ack_offset,
            ack_list,
            checksum,
        })
    }
}

impl DataType for CommandArgumentSignature {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.argument_name.encode(writer)?;
        self.signature.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            argument_name: String::decode(reader)?,
            signature: SignedCommandArgumentSignature::decode(reader)?,
        })
    }
}

impl DataType for SignedCommandArgumentSignature {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut value = [0; 256];
        reader.read_exact(&mut value)?;
        Ok(Self(value))
    }
}

impl PacketStruct for SignedCommandChatPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(SignedCommandChatPacket, spinel_network::Recipient::Server);
