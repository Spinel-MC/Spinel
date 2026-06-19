use spinel_network::data_type::DataType;
use spinel_network::types::{SectionPosition, var_int::VarIntWrapper, var_long::VarLongWrapper};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionBlockStateUpdate {
    pub position_in_section: i16,
    pub block_state_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionBlocksUpdatePacket {
    pub section_pos: SectionPosition,
    pub updates: Vec<SectionBlockStateUpdate>,
}

impl DataType for SectionBlockStateUpdate {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarLongWrapper(
            ((self.block_state_id as i64) << 12) | ((self.position_in_section as i64) & 4095),
        )
        .encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let packed_update = VarLongWrapper::decode(reader)?.0;
        Ok(Self {
            position_in_section: (packed_update & 4095) as i16,
            block_state_id: (packed_update >> 12) as i32,
        })
    }
}

impl DataType for SectionBlocksUpdatePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.section_pos.encode(writer)?;
        VarIntWrapper(self.updates.len() as i32).encode(writer)?;
        self.updates
            .iter()
            .try_for_each(|update| update.encode(writer))
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let section_pos = SectionPosition::decode(reader)?;
        let update_count = VarIntWrapper::decode(reader)?.0;
        if update_count < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "section block update count cannot be negative",
            ));
        }

        let updates = (0..update_count)
            .map(|_| SectionBlockStateUpdate::decode(reader))
            .collect::<io::Result<Vec<_>>>()?;

        Ok(Self {
            section_pos,
            updates,
        })
    }
}

impl PacketStruct for SectionBlocksUpdatePacket {
    fn get_id() -> i32 {
        0x52
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(
    SectionBlocksUpdatePacket,
    spinel_network::Recipient::Client
);
