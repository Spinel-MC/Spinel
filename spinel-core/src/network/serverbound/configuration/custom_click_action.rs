use spinel_network::data_type::DataType;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationCustomClickActionPacket {
    pub key: Identifier,
    pub payload: Vec<u8>,
}

impl ConfigurationCustomClickActionPacket {
    pub const fn get_id_const() -> i32 {
        0x08
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Configuration
    }

    pub fn payload_without_end_tag(&self) -> Option<&[u8]> {
        if self.payload.first() == Some(&0) {
            return None;
        }
        Some(&self.payload)
    }
}

impl DataType for ConfigurationCustomClickActionPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.key.encode(writer)?;
        writer.write_all(&self.payload)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let key = Identifier::decode(reader)?;
        let mut payload = Vec::new();
        reader.read_to_end(&mut payload)?;
        Ok(Self { key, payload })
    }
}

impl PacketStruct for ConfigurationCustomClickActionPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigurationCustomClickActionPacket;
    use spinel_network::data_type::DataType;
    use spinel_network::types::Identifier;

    #[test]
    fn configuration_custom_click_action_keeps_raw_payload_and_maps_end_tag_to_none() {
        let packet = ConfigurationCustomClickActionPacket {
            key: Identifier::minecraft("dialog"),
            payload: vec![0],
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet =
            ConfigurationCustomClickActionPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(ConfigurationCustomClickActionPacket::get_id_const(), 0x08);
        assert_eq!(decoded_packet.key, Identifier::minecraft("dialog"));
        assert_eq!(decoded_packet.payload_without_end_tag(), None);
    }
}
