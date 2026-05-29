use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecipeBookAddPacket {
    pub should_replace: bool,
}

impl RecipeBookAddPacket {
    pub const fn reset_empty() -> Self {
        Self {
            should_replace: true,
        }
    }

    pub const fn get_id() -> i32 {
        0x48
    }

    pub const fn get_id_const() -> i32 {
        0x48
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

impl DataType for RecipeBookAddPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(0).encode(writer)?;
        self.should_replace.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let entry_count = VarIntWrapper::decode(reader)?.0;
        if entry_count != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "non-empty recipe book add packets are not implemented",
            ));
        }
        Ok(Self {
            should_replace: bool::decode(reader)?,
        })
    }
}

impl PacketStruct for RecipeBookAddPacket {
    fn get_id() -> i32 {
        0x48
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

#[cfg(test)]
mod tests {
    use super::RecipeBookAddPacket;
    use spinel_network::DataType;

    #[test]
    fn empty_recipe_book_reset_packet_matches_minestom_empty_recipe_manager_shape() {
        let mut payload = Vec::new();

        RecipeBookAddPacket::reset_empty()
            .encode(&mut payload)
            .unwrap();

        assert_eq!(RecipeBookAddPacket::get_id(), 0x48);
        assert_eq!(payload, vec![0, 1]);
    }
}
