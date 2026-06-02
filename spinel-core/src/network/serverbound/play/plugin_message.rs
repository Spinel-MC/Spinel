use spinel_network::encoder::NetworkBuffer;
use spinel_network::{ConnectionState, DataType, PacketSender, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerboundPlayCustomPayloadPacket {
    pub channel: String,
    pub data: Vec<u8>,
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
        writer.write_all(&self.data)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let channel = String::decode(reader)?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(Self { channel, data })
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

#[cfg(test)]
mod tests {
    use super::ServerboundPlayCustomPayloadPacket;
    use spinel_network::DataType;
    use std::io::{Cursor, Read};

    #[test]
    fn serverbound_play_plugin_message_decodes_raw_remaining_payload() {
        let mut payload = Vec::new();
        "minecraft:brand".to_string().encode(&mut payload).unwrap();
        payload.extend([1, 2, 3, 4]);

        let mut reader = Cursor::new(payload);
        let packet = ServerboundPlayCustomPayloadPacket::decode(&mut reader).unwrap();
        let mut remaining = Vec::new();
        reader.read_to_end(&mut remaining).unwrap();

        assert_eq!(packet.channel, "minecraft:brand");
        assert_eq!(packet.data, vec![1, 2, 3, 4]);
        assert!(remaining.is_empty());
        assert_eq!(ServerboundPlayCustomPayloadPacket::get_id(), 0x15);
    }
}
