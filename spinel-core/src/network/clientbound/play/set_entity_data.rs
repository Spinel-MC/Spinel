use spinel_macros::packet;
use spinel_network::data_type::DataType;
use spinel_network::types::VarInt;
use spinel_network::types::entity_metadata::MetadataEntry;
use std::io::{self, Read, Write};

#[packet(id: "set_entity_data", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetEntityDataPacket {
    pub entity_id: VarInt,
    pub entries: EntityMetadataEntries,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EntityMetadataEntries(pub Vec<MetadataEntry>);

impl DataType for EntityMetadataEntries {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.0.iter().try_for_each(|entry| entry.encode(writer))?;
        0xFFu8.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut entries = Vec::new();
        loop {
            let index = u8::decode(reader)?;
            if index == 0xFF {
                return Ok(Self(entries));
            }
            let value = spinel_network::types::entity_metadata::MetadataValue::decode(reader)?;
            entries.push(MetadataEntry { index, value });
        }
    }
}

impl SetEntityDataPacket {
    pub fn new(entity_id: i32, entries: Vec<MetadataEntry>) -> Self {
        Self {
            entity_id,
            entries: EntityMetadataEntries(entries),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SetEntityDataPacket;
    use spinel_network::DataType;
    use spinel_network::types::entity_metadata::{MetadataEntry, MetadataValue};

    #[test]
    fn set_entity_data_packet_uses_minestom_terminator_shape() {
        let packet = SetEntityDataPacket::new(
            7,
            vec![MetadataEntry {
                index: 1,
                value: MetadataValue::VarInt(300),
            }],
        );
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = SetEntityDataPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(SetEntityDataPacket::get_id(), 0x61);
        assert_eq!(decoded_packet.entity_id, 7);
        assert_eq!(decoded_packet.entries.0, packet.entries.0);
        assert_eq!(payload.last().copied(), Some(0xFF));
    }
}
