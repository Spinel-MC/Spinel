use spinel_network::data_type::DataType;
use spinel_network::types::{ScoreNumberFormat, var_int::VarIntWrapper};
use spinel_network::wrappers::JsonTextComponent;
use spinel_network::{ConnectionState, PacketStruct};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct SetScorePacket {
    pub owner: String,
    pub objective_name: String,
    pub score: i32,
    pub display: Option<TextComponent>,
    pub number_format: Option<ScoreNumberFormat>,
}

impl DataType for SetScorePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.owner.encode(writer)?;
        self.objective_name.encode(writer)?;
        VarIntWrapper(self.score).encode(writer)?;
        self.display.clone().map(JsonTextComponent).encode(writer)?;
        self.number_format.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            owner: String::decode(reader)?,
            objective_name: String::decode(reader)?,
            score: VarIntWrapper::decode(reader)?.0,
            display: Option::<JsonTextComponent>::decode(reader)?.map(|display| display.0),
            number_format: Option::<ScoreNumberFormat>::decode(reader)?,
        })
    }
}

impl PacketStruct for SetScorePacket {
    fn get_id() -> i32 {
        0x6C
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(SetScorePacket, spinel_network::Recipient::Client);
