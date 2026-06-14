use spinel_network::data_type::DataType;
use spinel_network::encoder::NetworkBuffer;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::wrappers::JsonTextComponent;
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveMode {
    Create,
    Remove,
    Update,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveRenderType {
    Integer,
    Hearts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoreboardObjectivePacket {
    pub objective_name: String,
    pub mode: ObjectiveMode,
    pub objective_value: Option<TextComponent>,
    pub render_type: Option<ObjectiveRenderType>,
}

impl ScoreboardObjectivePacket {
    pub const fn get_id() -> i32 {
        0x68
    }

    pub fn create(
        objective_name: impl Into<String>,
        objective_value: TextComponent,
        render_type: ObjectiveRenderType,
    ) -> Self {
        Self {
            objective_name: objective_name.into(),
            mode: ObjectiveMode::Create,
            objective_value: Some(objective_value),
            render_type: Some(render_type),
        }
    }

    pub fn remove(objective_name: impl Into<String>) -> Self {
        Self {
            objective_name: objective_name.into(),
            mode: ObjectiveMode::Remove,
            objective_value: None,
            render_type: None,
        }
    }

    pub fn encode_to_buffer(&self) -> io::Result<NetworkBuffer> {
        let mut buffer = NetworkBuffer::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let payload_bytes = self.encode_to_buffer()?.into_buffer();
        sender.send_packet(Self::get_id(), &payload_bytes)
    }
}

impl ObjectiveMode {
    const fn protocol_id(self) -> i8 {
        match self {
            Self::Create => 0,
            Self::Remove => 1,
            Self::Update => 2,
        }
    }

    const fn from_protocol_id(protocol_id: i8) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Create),
            1 => Some(Self::Remove),
            2 => Some(Self::Update),
            _ => None,
        }
    }

    const fn has_payload(self) -> bool {
        matches!(self, Self::Create | Self::Update)
    }
}

impl ObjectiveRenderType {
    const fn protocol_id(self) -> i32 {
        match self {
            Self::Integer => 0,
            Self::Hearts => 1,
        }
    }

    const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Integer),
            1 => Some(Self::Hearts),
            _ => None,
        }
    }
}

impl DataType for ScoreboardObjectivePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.objective_name.encode(writer)?;
        self.mode.protocol_id().encode(writer)?;
        if self.mode.has_payload() {
            JsonTextComponent(self.objective_value.clone().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "objective value is required")
            })?)
            .encode(writer)?;
            VarIntWrapper(
                self.render_type
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "objective render type is required",
                        )
                    })?
                    .protocol_id(),
            )
            .encode(writer)?;
            Option::<VarIntWrapper>::None.encode(writer)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let objective_name = String::decode(reader)?;
        let mode = ObjectiveMode::from_protocol_id(i8::decode(reader)?).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "unknown scoreboard objective mode",
            )
        })?;
        if !mode.has_payload() {
            return Ok(Self {
                objective_name,
                mode,
                objective_value: None,
                render_type: None,
            });
        }

        let objective_value = JsonTextComponent::decode(reader)?.0;
        let render_type = ObjectiveRenderType::from_protocol_id(VarIntWrapper::decode(reader)?.0)
            .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "unknown scoreboard render type")
        })?;
        let _number_format = Option::<VarIntWrapper>::decode(reader)?;
        Ok(Self {
            objective_name,
            mode,
            objective_value: Some(objective_value),
            render_type: Some(render_type),
        })
    }
}

impl PacketStruct for ScoreboardObjectivePacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(
    ScoreboardObjectivePacket,
    spinel_network::Recipient::Client
);
