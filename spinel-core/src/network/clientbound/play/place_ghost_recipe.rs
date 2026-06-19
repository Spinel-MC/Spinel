use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeDisplayPayload {
    pub recipe_display_type_id: i32,
    pub encoded_recipe_display_data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceGhostRecipePacket {
    pub container_id: i32,
    pub recipe_display: RecipeDisplayPayload,
}

impl DataType for RecipeDisplayPayload {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.recipe_display_type_id).encode(writer)?;
        writer.write_all(&self.encoded_recipe_display_data)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let recipe_display_type_id = VarIntWrapper::decode(reader)?.0;
        let mut encoded_recipe_display_data = Vec::new();
        reader.read_to_end(&mut encoded_recipe_display_data)?;
        Ok(Self {
            recipe_display_type_id,
            encoded_recipe_display_data,
        })
    }
}

impl DataType for PlaceGhostRecipePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.container_id).encode(writer)?;
        self.recipe_display.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            container_id: VarIntWrapper::decode(reader)?.0,
            recipe_display: RecipeDisplayPayload::decode(reader)?,
        })
    }
}

impl PacketStruct for PlaceGhostRecipePacket {
    fn get_id() -> i32 {
        0x3D
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(PlaceGhostRecipePacket, spinel_network::Recipient::Client);
