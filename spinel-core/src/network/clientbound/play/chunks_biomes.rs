use spinel_macros::packet;
use spinel_network::data_type::DataType;
use spinel_network::types::ChunkPos;
use std::io::{self, Read, Write};

#[packet(id: "chunks_biomes", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ChunksBiomesPacket {
    pub chunk_biome_data: Vec<ChunkBiomeData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkBiomeData {
    pub pos: ChunkPos,
    pub buffer: Vec<u8>,
}

impl DataType for ChunkBiomeData {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.pos.encode(writer)?;
        self.buffer.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            pos: ChunkPos::decode(reader)?,
            buffer: Vec::<u8>::decode(reader)?,
        })
    }
}
