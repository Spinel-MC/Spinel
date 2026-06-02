use spinel_network::data_type::DataType;
use spinel_network::types::Position;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSignPacket {
    pub block_position: Position,
    pub is_front_text: bool,
    pub lines: [String; 4],
}

impl UpdateSignPacket {
    pub const fn get_id_const() -> i32 {
        0x3A
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for UpdateSignPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.block_position.encode(writer)?;
        self.is_front_text.encode(writer)?;
        self.lines.iter().try_for_each(|line| line.encode(writer))
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            block_position: Position::decode(reader)?,
            is_front_text: bool::decode(reader)?,
            lines: [
                String::decode(reader)?,
                String::decode(reader)?,
                String::decode(reader)?,
                String::decode(reader)?,
            ],
        })
    }
}

impl PacketStruct for UpdateSignPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

#[cfg(test)]
mod tests {
    use super::UpdateSignPacket;
    use spinel_network::data_type::DataType;
    use spinel_network::types::Position;

    #[test]
    fn update_sign_packet_matches_minestom_fixed_four_line_shape() {
        let packet = UpdateSignPacket {
            block_position: Position { x: 1, y: 2, z: 3 },
            is_front_text: true,
            lines: ["a".into(), "b".into(), "c".into(), "d".into()],
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = UpdateSignPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(UpdateSignPacket::get_id_const(), 0x3A);
        assert_eq!(decoded_packet, packet);
    }
}
