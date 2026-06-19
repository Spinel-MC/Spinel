use spinel_network::data_type::DataType;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDialogPacket {
    pub encoded_dialog_payload: Vec<u8>,
}

impl DataType for ShowDialogPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.encoded_dialog_payload)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut encoded_dialog_payload = Vec::new();
        reader.read_to_end(&mut encoded_dialog_payload)?;
        Ok(Self {
            encoded_dialog_payload,
        })
    }
}

impl PacketStruct for ShowDialogPacket {
    fn get_id() -> i32 {
        0x12
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Configuration
    }
}
spinel_network::register_packet_codec!(ShowDialogPacket, spinel_network::Recipient::Client);
