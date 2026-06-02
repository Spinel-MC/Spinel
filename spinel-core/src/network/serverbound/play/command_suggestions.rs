use spinel_network::{ConnectionState, DataType, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandSuggestionsRequestPacket {
    pub transaction_id: i32,
    pub text: String,
}

impl CommandSuggestionsRequestPacket {
    pub const fn get_id_const() -> i32 {
        0x0f
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for CommandSuggestionsRequestPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.transaction_id).encode(writer)?;
        self.text.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            transaction_id: VarIntWrapper::decode(reader)?.0,
            text: String::decode(reader)?,
        })
    }
}

impl PacketStruct for CommandSuggestionsRequestPacket {
    fn get_id() -> i32 {
        0x0f
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

#[cfg(test)]
mod tests {
    use super::CommandSuggestionsRequestPacket;
    use spinel_network::{DataType, PacketStruct};

    #[test]
    fn command_suggestions_request_round_trips_minestom_shape() {
        let packet = CommandSuggestionsRequestPacket {
            transaction_id: 6,
            text: "/spawn zo".to_string(),
        };

        let mut payload = Vec::new();
        packet.encode(&mut payload).unwrap();
        let decoded_packet =
            CommandSuggestionsRequestPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(CommandSuggestionsRequestPacket::get_id(), 0x0f);
        assert_eq!(decoded_packet, packet);
    }
}
