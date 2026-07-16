use spinel_network::encoder::NetworkBuffer;
use spinel_network::{ConnectionState, DataType, PacketSender, PacketStruct, RawBytes};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerboundPlayCustomPayloadPacket {
    pub channel: String,
    pub data: RawBytes,
}

impl ServerboundPlayCustomPayloadPacket {
    pub const fn get_id() -> i32 {
        0x15
    }

    pub const fn get_id_const() -> i32 {
        0x15
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn encode_to_buffer(&self) -> io::Result<NetworkBuffer> {
        let mut buffer = NetworkBuffer::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let payload_bytes = self.encode_to_buffer()?.into_buffer();
        sender.send_packet(Self::get_id(), &payload_bytes)
    }
}

impl DataType for ServerboundPlayCustomPayloadPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.channel.encode(writer)?;
        self.data.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            channel: String::decode(reader)?,
            data: RawBytes::decode(reader)?,
        })
    }
}

impl PacketStruct for ServerboundPlayCustomPayloadPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(
    ServerboundPlayCustomPayloadPacket,
    spinel_network::Recipient::Server
);