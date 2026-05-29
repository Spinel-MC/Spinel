use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UpdateRecipesPacket;

impl UpdateRecipesPacket {
    pub const fn get_id() -> i32 {
        0x83
    }

    pub const fn get_id_const() -> i32 {
        0x83
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn encode_to_buffer(&self) -> io::Result<spinel_network::encoder::NetworkBuffer> {
        let mut buffer = spinel_network::encoder::NetworkBuffer::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let payload_bytes = self.encode_to_buffer()?.into_buffer();
        sender.send_packet(Self::get_id(), &payload_bytes)
    }
}

impl DataType for UpdateRecipesPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(0).encode(writer)?;
        VarIntWrapper(0).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let item_property_count = VarIntWrapper::decode(reader)?.0;
        if item_property_count != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "non-empty update recipes packets are not implemented",
            ));
        }
        let stonecutter_recipe_count = VarIntWrapper::decode(reader)?.0;
        if stonecutter_recipe_count != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "non-empty update recipes packets are not implemented",
            ));
        }
        Ok(Self)
    }
}

impl PacketStruct for UpdateRecipesPacket {
    fn get_id() -> i32 {
        0x83
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

#[cfg(test)]
mod tests {
    use super::UpdateRecipesPacket;
    use spinel_network::DataType;

    #[test]
    fn empty_update_recipes_packet_matches_minestom_empty_recipe_manager_shape() {
        let mut payload = Vec::new();

        UpdateRecipesPacket.encode(&mut payload).unwrap();

        assert_eq!(UpdateRecipesPacket::get_id(), 0x83);
        assert_eq!(payload, vec![0, 0]);
    }
}
